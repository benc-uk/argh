# Static geometry and baked vertex lighting

A design proposal for two complementary optimisations that together transform
the renderer's handling of non-moving world geometry (walls, floors, scenery,
props). Inspired by the Quake-era software rasteriser pipeline.

The two changes stack: static geometry baking eliminates the per-frame model
transform work, baked vertex lighting eliminates the per-frame shading work.
Combined, a static vertex per frame costs a single `VP·v` matmul + screen
mapping + one array lookup. No matrix builds, no inverse-transpose, no
per-vertex transforms, no sqrt, no powf, no light loop.

Scope note: this doc covers only what's needed for the first cut. Spatial
chunking, AABBs, and frustum culling are deliberately out of scope. The data
layout is just `Vec<StaticMesh>` on `Scene`. Those optimisations are a
natural next step once this is working (see the very end).

## Why this is worth doing

In a typical scene, 70 to 90 percent of geometry is static (level walls,
props, scenery). Today, that geometry pays the full transform-and-shade cost
every frame even though the answers are deterministic. The wins are real and
compound.

### Per-frame cost today, per static vertex

Looking at `engine/render.rs` (`render_instance`) and `helpers.rs`
(`shade_vert`), every static vertex pays the following every frame:

| Stage | Work | Per-vertex cost |
|---|---|---|
| Build model matrix | `instance.get_model_mat()` | per-instance, one-off |
| Build inverse-transpose for normals | 3x3 inverse + transpose | per-instance, one-off |
| Combine MVP | `pers · view · model` | per-instance, 2 Mat4 matmuls |
| World transform | `m.transform_point(vert)` | 12 muls + 9 adds per vert |
| Normal transform | `(m_inv_t · n).normalize_new()` | 9 muls + 6 adds + **sqrt** per normal |
| Clip transform | `mvp · Vec4` | per vert anyway, can't eliminate |
| Shade vertex | `shade_vert(...)` per light | ~30 flops + 2 sqrt + 2 div + **powf** per light per shade |

The shade cost is per-triangle-corner, not per unique vertex, so a vertex
shared between 6 triangles is shaded 6 times every frame producing the same
answer.

### After both bakes

| Stage | Work |
|---|---|
| World transform | none, vertex is stored in world space |
| Normal transform | none, normal is stored in world space |
| Clip transform | `VP · Vec4` per vert (VP computed once per frame, shared) |
| Shade vertex | `sm.baked_lighting[i]` array lookup |

For a modest scene of 1000 static verts, 5 static lights, 60 FPS, smooth
shaded:

- ~1.8 million `powf` calls per second eliminated
- ~360,000 sqrt per second eliminated from the normal transform path
- All per-instance matrix building gone
- Per-vertex world transform gone

The `powf` calls alone are usually 30 to 50 percent of total CPU time in a
software rasteriser with multiple lights.

---

## Part one: static geometry

### Concept

Static geometry is pre-baked into world space at scene load. The model
matrix is permanently the identity for the life of the scene, which means it
can be elided entirely from the per-frame path. MVP collapses to VP and is
computed once per camera per frame, shared across all static geometry.

### Data layout

A new type, clearly distinct from `Instance` which keeps its transform:

```rust
pub struct StaticMesh {
    pub(crate) material: Material,      // owned by value, same as Mesh today
    pub(crate) verts: Vec<Vec3>,        // already in WORLD space
    pub(crate) normals: Vec<Vec3>,      // already in WORLD space, normalised
    pub(crate) uvs: Vec<Vec2>,
    pub(crate) indices: Vec<i32>,

    pub(crate) baked_lighting: Vec<Colour>,  // see part two
}
```

On `Scene`, just a flat list:

```rust
pub(super) static_meshes: Vec<StaticMesh>,
```

One `StaticMesh` per source mesh, mirroring the current `Mesh` -> `Material`
ownership model. No cross-source merging by material in this design.

### Bake API

```rust
impl Scene {
    pub fn add_static(&mut self, engine: &Engine, model_h: ModelHandle,
                      pos: Vec3, rot: Quat, scale: Vec3) {
        let model = engine.models.get(model_h).unwrap();
        let m = Mat4::new_scale_rot_trans(scale.x, scale.y, scale.z,
                                          rot, pos.x, pos.y, pos.z);
        let m_inv_t = Mat3::from_mat4_upper(&m)
                           .inverse_transpose()
                           .unwrap_or_default();

        for mesh in &model.meshes {
            let verts: Vec<Vec3> = mesh.verts.iter()
                .map(|v| m.transform_point(v))
                .collect();
            let normals: Vec<Vec3> = mesh.normals.iter()
                .map(|n| (m_inv_t * n).normalize_new())
                .collect();

            // Take ownership of the material clone for this static mesh
            let sm = StaticMesh {
                material: clone_material(&mesh.material),
                verts, normals,
                uvs: mesh.uvs.clone(),
                indices: mesh.indices.clone(),
                baked_lighting: vec![],   // populated in second pass
            };
            self.static_meshes.push(sm);
        }
    }
}
```

The `clone_material` helper sidesteps `Material` not being `Clone` today
(it contains `Texture` which holds a big `Vec<u32>`). Two simple options:

- Make `Material` derive `Clone` (cheap if textures are small or rarely
  duplicated; the pixel `Vec` clones with it).
- Or wrap the texture in `Rc<Texture>` so cloning the material is cheap.

Pick the simpler one for now; both are local changes inside `models.rs`.

### Per-frame render path

A second function alongside `render_instance`, walking the flat list of
static meshes:

```rust
pub fn render_static(&mut self, cam: &Camera, scn: &Scene) {
    let vp = cam.pers_mat * cam.view_mat;   // ONE matmul for the entire static world

    for sm in &scn.static_meshes {
        self.render_static_mesh(sm, &vp, scn);
    }
}
```

Inside `render_static_mesh`, the vertex stage becomes:

```rust
self.verts.extend(sm.verts.iter().enumerate().map(|(i, world)| {
    let clip = vp * &Vec4::new(world.x, world.y, world.z, 1.0);
    let outcode = compute_outcode(&clip);
    let inv_w = 1.0 / clip.w;
    let ndc_x = clip.x * inv_w;
    let ndc_y = clip.y * inv_w;
    let ndc_z = clip.z * inv_w;
    let sx = (ndc_x * 0.5 + 0.5) * self.size.0 as f32;
    let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * self.size.1 as f32;
    let uv = sm.uvs[i];

    ProcessedVert {
        world: *world,                          // no transform!
        screen: ScreenVert { x: sx, y: sy, z: ndc_z, inv_w,
                             light: sm.baked_lighting[i],  // see part two
                             u_w: uv.x*inv_w, v_w: uv.y*inv_w },
        outcode,
    }
}));
```

Normals are not needed in the per-frame loop at all because lighting is
baked. The `self.normals` scratch vector is unused for static geometry.

### What this eliminates per frame

- `instance.get_model_mat()` (one per instance)
- `Mat3::from_mat4_upper(&m).inverse_transpose()` (one per instance, includes a determinant + 3x3 inverse)
- `cam.pers_mat * cam.view_mat * m` (collapses to a single shared `VP` for all static)
- `m.transform_point(vert)` per vert (12 muls + 9 adds gone)
- `(m_inv_t * n).normalize_new()` per normal (gone, including the sqrt)

---

## Part two: baked vertex lighting

### Concept

A scene's lights are split into static (sun, room sconces, fixed lamps) and
dynamic (player torch, muzzle flash, projectile glows). At scene load, after
all static lights and static geometry are registered, a one-pass bake walks
every static vertex and computes the contribution of every static light.
The result is stored per-vertex as a `Colour`.

At render time, the per-frame shade loop is replaced with a single array
lookup. Dynamic lights, if any, are accumulated on top per frame.

### Storage

One `Colour` (12 bytes) per unique static vertex. For 1000 verts, 12 KB.
Trivial compared to the geometry itself.

```rust
// On StaticMesh
pub(crate) baked_lighting: Vec<Colour>,
```

### Light classification

```rust
pub struct Light {
    // ... existing fields ...
    pub is_static: bool,    // baked into static geometry
    pub is_dynamic: bool,   // applied per-frame even to static geometry
}
```

A light can be both, but the common patterns are:

- Sun, room lights: `is_static = true, is_dynamic = false`
- Player torch, fireball: `is_static = false, is_dynamic = true`
- Pulsing pickup glow that also affects static walls: both true (rare)

### Effect matrix

What a light contributes, by classification and target geometry type. The
two flags only change the static-geometry path; `render_instance` continues
to loop the full light set for dynamic models unless you explicitly teach
it to honour the flags.

| Light flags | Static geometry (`StaticMesh`) | Dynamic instances (`Instance`) |
|---|---|---|
| `is_static = true`, `is_dynamic = false` (sun, sconce) | Baked into `baked_lighting` once at scene load. Free at runtime. | Applied every frame via `shade_vert` in `render_instance`. |
| `is_static = false`, `is_dynamic = true` (torch, muzzle flash) | Accumulated per frame on top of `baked_lighting` via the dynamic overlay. | Applied every frame via `shade_vert` in `render_instance`. |
| `is_static = true`, `is_dynamic = true` (rare; pulsing fixture) | Baked AND added by the overlay each frame. Double-counts unless the overlay explicitly skips `is_static` lights. Avoid unless intentional. | Applied every frame via `shade_vert` in `render_instance`. |
| `is_static = false`, `is_dynamic = false` | Ignored, neither baked nor overlaid. | Still applied via `shade_vert` (the flags are invisible to `render_instance`). Usually a configuration mistake. |

Rule of thumb: every light should have at least one flag set, and lights
flagged both static and dynamic want the overlay to filter on
`is_dynamic && !is_static` to avoid the double-count.

### Bake function

Runs once after the scene is populated. Per-vertex, per-static-light, no eye
vector (specular is intentionally view-dependent, see trade-offs below).
`material` is read directly from `self`, so the method takes no extra
parameter for it (this also avoids a borrow conflict at the call site).

```rust
impl StaticMesh {
    pub fn bake_lighting(&mut self,
                         lights: &SlotMap<LightHandle, Light>,
                         ambient: Colour) {
        self.baked_lighting.clear();
        self.baked_lighting.reserve(self.verts.len());

        for (vert, normal) in self.verts.iter().zip(&self.normals) {
            let mut diffuse = BLACK;

            for light in lights.values() {
                if !light.is_static { continue; }

                let l_raw = light.pos - *vert;
                let d = l_raw.len();
                let l = l_raw.normalize_new();
                let atten = 1.0 / (1.0 + light.atten_linear * d
                                       + light.atten_quad * d * d);
                let n_dot_l = normal.dot(l).max(0.0);
                diffuse += light.colour * light.brightness * n_dot_l * atten;
            }

            let amb = ambient * self.material.diffuse;
            self.baked_lighting.push(diffuse * self.material.diffuse + amb);
        }
    }
}
```

Notice: this is per **unique vertex**, not per triangle corner. The existing
6x-shading-of-shared-verts inefficiency is eliminated structurally, not just
in cost.

Driver pass at scene-load time:

```rust
impl Scene {
    pub fn bake_static_lighting(&mut self) {
        for sm in &mut self.static_meshes {
            sm.bake_lighting(&self.lights, self.ambient_light);
        }
    }
}
```

User flow:

```rust
let static_lights = /* set up sun, sconces, etc with is_static = true */;
scene.add_static(engine, wall_model, ...);
scene.add_static(engine, floor_model, ...);
scene.bake_static_lighting();  // call ONCE after all static lights + geometry added
```

### Render path

In `render_static`, the per-triangle shading section collapses from

```rust
let (d0, s0) = shade_vert(&scn.lights, wv0, n0, eye, mat.hardness);
sv0.light = (d0 * mat.diffuse) + (s0 * mat.specular) + amb;
if instance.smooth {
    let (d1, s1) = shade_vert(&scn.lights, wv1, n1, eye, mat.hardness);
    let (d2, s2) = shade_vert(&scn.lights, wv2, n2, eye, mat.hardness);
    sv1.light = (d1 * mat.diffuse) + (s1 * mat.specular) + amb;
    sv2.light = (d2 * mat.diffuse) + (s2 * mat.specular) + amb;
}
```

to

```rust
sv0.light = sm.baked_lighting[i0];
sv1.light = sm.baked_lighting[i1];
sv2.light = sm.baked_lighting[i2];
```

Three array lookups. That is the entire lighting cost for a static triangle.

### Optional dynamic-light overlay

For moving lights that still need to affect static geometry (a player
torch), accumulate their contribution per-frame on top of the bake:

```rust
sv0.light = sm.baked_lighting[i0]
          + accumulate_dynamic_lights(&scn.lights, wv0, n0);
// etc
```

`accumulate_dynamic_lights` walks only lights flagged `is_dynamic`, usually
1 to 3 lights total. Tiny cost compared to today's full-light-set loop.

### Trade-offs

| Lost | Why | Mitigation |
|---|---|---|
| Specular highlights on static geometry | Specular depends on eye position, changes every camera move | Either accept matte static surfaces (Quake 1 did, looked fine) or layer a per-frame specular pass on top using `eye` and the stored normal. Quad the cost of the dynamic overlay if so. |
| Dynamic lights on static geometry | Bake is per static light, doesn't track moving ones | Use the dynamic overlay above. Static + dynamic light list is the standard Quake-era hybrid. |
| Movable static lights | "Static" means truly static. Move the light and the bake is wrong | If a light moves, reclassify it dynamic and stop baking. Or re-bake on change (slow, scene-wide pass). |
| Harsh linear interpolation across large flat polygons | One vert per corner of an 8m wall means a light in the middle produces an X-shaped artefact | Tessellate large flat surfaces at bake time so vert density captures light variation. Or, as a next step, progress to lightmaps (per-texel baked lighting via a low-resolution texture). |
| Flat shading on static geometry | Storing per-vertex colour assumes Gouraud-style interpolation | Either commit to smooth shading on static geometry, or additionally bake a `face_lighting: Vec<Colour>` (one per triangle) for flat-shaded materials. |

---

## What the combined static path looks like

After both bakes, the per-frame static render becomes:

```
Per frame, once:
  VP = pers · view

Per static mesh:
  Per vertex:
    VP · Vec4              <-- the only real work
    outcode + screen mapping
    sm.baked_lighting[i] lookup   <-- effectively free

  Per triangle:
    Trivial reject
    Backface area test
    fill_triangle (rasterisation unchanged)
```

No model transform. No normal transform. No matrix builds per instance. No
sqrt. No powf. No light loop. Just transform position, look up colour,
rasterise.

---

## Prerequisites

The only real prerequisite:

- **A way to clone a `Material`.** Either derive `Clone` on it, or wrap the
  texture in `Rc<Texture>` so cloning is cheap. Needed because `StaticMesh`
  takes ownership of the material from the source mesh at bake time.

---

## Implementation order

A reasonable path that keeps the renderer working at every step:

1. Make `Material` cloneable (derive `Clone`, or `Rc<Texture>` inside). Tiny
   local change in `models.rs`.
2. Add `StaticMesh`. Add `static_meshes: Vec<StaticMesh>` to `Scene`.
3. Implement `Scene::add_static` that bakes geometry into world space and
   pushes one `StaticMesh` per source mesh. No baked lighting yet, leave
   `baked_lighting` empty and fall back to per-frame `shade_vert`.
4. Implement `render_static` walking `scn.static_meshes`. Verify it draws
   correctly and produces identical pixels to the equivalent
   `render_instance` calls. This is a regression-test checkpoint.
5. Add `is_static` / `is_dynamic` flags to `Light`.
6. Implement `StaticMesh::bake_lighting` and `Scene::bake_static_lighting`.
   Call from user code after all static lights and static geometry are
   added.
7. Swap `render_static`'s shading section to use `baked_lighting` lookups.
8. (Optional) Add the dynamic-light overlay for moving lights that need to
   affect static geometry.

Each step is independently shippable. The renderer is never broken in
between.

---

## Things deliberately not in scope (yet)

Natural next chapters once the flat-list version is working:

- **Spatial chunking + AABB frustum culling.** Group static geometry into
  spatial chunks, each with an enclosing AABB. Each frame, test each
  chunk's AABB against the camera frustum and skip the rest of the work
  for off-screen chunks. Required for the static path to actually scale,
  because without it every static vertex is still transformed every frame.
  Needs `Aabb` and `Plane` math primitives and a `Mat4 -> 6 planes`
  extractor (Gribb & Hartmann). Start with `Vec<Chunk>` and centroid
  binning; upgrade to a grid or BVH only if profiling demands it.
- **Inside vs Intersect rendering split.** When a chunk is fully inside the
  frustum, skip per-vertex outcode work and per-triangle trivial reject.
  Worth a separate `render_chunk_no_clip` path. Only meaningful once
  chunking exists.
- **Cross-source material merging within a chunk.** Today each source mesh
  becomes one `StaticMesh`. If two source meshes share a material, they're
  still rendered as separate batches. Merging into one big vertex/index
  buffer per material would cut per-batch overhead and improve cache
  locality. Pairs naturally with promoting `Material` to a handle-managed
  resource (`MaterialHandle` is already declared in `engine/mod.rs`
  waiting for storage).
- **Lightmaps.** Per-texel baked lighting via low-resolution textures
  layered over the static geometry. Strictly better quality than vertex
  lighting on large flat surfaces. Big implementation step beyond this
  design.
- **Shadow casting in the bake.** Today the bake doesn't account for
  occlusion (one wall blocking another's light). Adding shadow rays in
  the bake (offline ray-cast at load time) gives big visual upgrade.
  Slow to bake, free at render.
- **PVS (Potentially Visible Sets).** Quake-style precomputed cell-to-cell
  visibility. Replaces per-frame frustum culling with a table lookup.
  Heavy bake, very fast runtime. Overkill until your scene has thousands
  of chunks.
- **Dynamic-light shadows on static geometry.** Shadow volumes or per-light
  shadow maps. Outside the scope of "make static stuff fast."
