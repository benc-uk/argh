# Transparency design

Design notes for adding translucency to Argh. Scope: a handful of
transparent objects per scene (glass panes, water surfaces, a coloured
visor), not particle clouds or AAA-grade volumetrics. The aim is something
that looks correct for the common case and degrades predictably in the
awkward cases, without rewriting the rasteriser.

This is a write-up to implement from, not the implementation. No code in
the tree has changed.

---

## The two flavours of "transparent"

These get conflated, but they're different problems and need different
machinery:

1. **Alpha cutout (binary)** — a pixel is fully opaque or fully discarded.
   Used for foliage, chain link, decals. Already implemented today via
   `Texture::alpha_cutout` + `cutoff` (see `texture.rs`, sampled inside
   `rasterize_tri`). Writes depth, needs no sorting, no extra passes.
   **Nothing in this doc changes that path.**

2. **Alpha blended (translucent)** — a pixel partially covers what's
   behind it. Output is
   `out = src * α + dst * (1 - α)` ("source-over" / straight alpha).
   That formula requires `dst` to already contain everything behind `src`,
   which is where sorting and the two-pass split come in.

Treat cutouts as opaque draws for ordering purposes. Treat translucent
materials as a separate class that has to be drawn after everything else.

---

## A note on depth in Argh

There is **no toggleable depth-write state** in Argh today. `Buffer` has
two write paths:

- `set_pixel(x, y, c)` — colour only, no depth involvement at all.
  Used by 2D / UI draw code.
- `set_pixel_depth(x, y, c, z)` — fused depth-test-and-write. If the
  pixel passes the reverse-Z test, it writes both the colour AND the
  new depth value. There is no way to ask it to test without writing.

The opaque and cutout paths in `rasterize_tri` both end at
`set_pixel_depth`, which is correct for them: they always want a test
and they always want a write on pass.

Blended transparency needs a **third** behaviour: depth-test but no
depth-write. That doesn't exist today and has to be added (see the
`blend_pixel_depth` sketch later in this doc). Throughout this
document, phrases like "depth write OFF" mean "use the test-only
blended write path", not "flip a state bit".

## The two-pass painter's approach

```
PASS 1 — opaque (and cutout)
   for each opaque baked mesh:     rasterise via set_pixel_depth
   for each opaque instance mesh:  rasterise via set_pixel_depth
   // this is exactly what render() does today

PASS 2 — transparent (blended)
   collect a list of transparent draws (baked + instance) into Engine
   compute view-space Z centroid for each
   sort back-to-front (furthest first)
   for each draw, rasterise via the NEW blend_pixel_depth which:
       - tests depth (so a glass pane is occluded by the wall in front)
       - does NOT write depth (so the next transparent behind doesn't self-reject)
       - performs src OVER dst alpha blend on the existing framebuffer pixel
```

### Why depth-test-but-no-write for blended pixels

- **Test the existing depth**: a transparent surface still has to respect
  opaque geometry in front of it. Pass 1 has already filled the depth
  buffer with all the opaque surfaces, so testing against it gives the
  right occlusion for free. Without the test, a glass window draws over
  a wall that was supposed to occlude it.
- **Don't write into the depth buffer**: if a transparent surface writes
  its own depth, a _second_ transparent surface drawn behind it gets
  rejected by the test (the first pane's `z` is now in the depth
  buffer), and you get wrong compositing or vanishing surfaces. The
  pass-1 opaque depth values must remain in place undisturbed across
  pass 2.

### Summary of the three rasteriser write paths

The complete picture once the feature lands:

| Category                   | Pass | Buffer function called                                    | Depth test | Depth write                 | Sort needed        |
| -------------------------- | ---- | --------------------------------------------------------- | ---------- | --------------------------- | ------------------ |
| Opaque                     | 1    | `set_pixel_depth`                                         | YES        | YES                         | No                 |
| Cutout (binary alpha)      | 1    | `set_pixel_depth` (after `continue` for discarded texels) | YES        | YES (surviving pixels only) | No                 |
| Blended (continuous alpha) | 2    | `blend_pixel_depth` _(new)_                               | YES        | NO                          | Yes, back-to-front |

Cutout sits in pass 1 alongside opaque on purpose: surviving cutout
pixels behave identically to opaque pixels (they write depth), which
means cutout self-occludes correctly and later opaque draws are
correctly occluded by it, without any sort. Discarded cutout texels
hit `continue` before any write happens, which is already how
`rasterize_tri` works today.

### Why back-to-front

Source-over blending is not commutative. The "behind" surface has to
already be in the framebuffer when the "in-front" surface gets composited,
or the colours come out wrong. Sort by camera-space distance, furthest
first, render in that order. With reverse-Z view space ("camera looks down
-Z", per `architecture.md`), the furthest object has the **most negative**
`view_z`, so sort **ascending** to draw far first.

### Sort granularity

| Sort unit                          | Pros                                         | Cons                                                                                             | Verdict                         |
| ---------------------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------- |
| Per mesh / per instance (centroid) | Trivial, tiny `N`, fast                      | Wrong when two transparent surfaces interpenetrate, or one is concave (e.g. a transparent torus) | **Start here**                  |
| Per triangle                       | Always correct between surfaces              | Triangle list grows, more bookkeeping per frame                                                  | Only if a specific scene breaks |
| BSP tree                           | Mathematically perfect order without sorting | Pre-process cost, painful to maintain                                                            | Overkill                        |

For "a few transparent objects per scene", per-mesh centroid is the
correct trade-off. Document the interpenetration caveat and move on. If
one scene breaks, split the offending mesh at author time first; only
escalate to per-triangle if it keeps happening.

---

## Material API changes

Transparency is a surface property, which means it belongs on `Material`,
not on `Mesh` and not on `Texture`. (Texture alpha keeps its existing
binary-cutout role.)

**`argh/src/material.rs`** — add:

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
  Opaque,        // pass 1, depth write, no blend  (default)
  AlphaBlend,    // pass 2, no depth write, src OVER dst
  Additive,      // pass 2, no depth write, src + dst  (fire, sparks; optional)
}

pub struct Material {
  pub diffuse: Colour,
  pub specular: Colour,
  pub hardness: f32,
  pub opacity: f32,            // NEW: 1.0 = fully opaque, 0.0 = invisible
  pub blend_mode: BlendMode,   // NEW: defaults to Opaque
  pub(crate) texture: Option<Rc<Texture>>,
}

impl Material {
  #[inline]
  pub fn is_opaque(&self) -> bool {
    matches!(self.blend_mode, BlendMode::Opaque)
  }
}
```

Defaults: `opacity = 1.0`, `blend_mode = Opaque`. Existing materials keep
behaving exactly as they do today.

Effective per-pixel alpha is `material.opacity * texture_alpha` (or just
`material.opacity` if untextured).

**Why an enum and not a `transparent: bool`?** Because additive blending
(common for sparks, fire, light shafts) uses a different formula and is
worth keeping in the type. If you only ever want straight alpha, collapse
to a bool and skip `Additive`.

---

## Data model recap

For clarity, the ownership hierarchy in Argh is:

```
Engine
  └── Scene
        ├── instances[]  (transform + ModelHandle)
        │     └── Model  (shared, looked up by handle)
        │           └── meshes[]  (each Mesh has exactly one Material)
        └── baked_meshes[]  (world-space, each has one Material)
```

A `Model` holds one or more `Mesh`. Each `Mesh` has exactly one
`Material`. An `Instance` is a placed copy of a `Model` (transform +
handle). `BakedMesh` is a pre-transformed mesh already in world space.

---

## AABB (axis-aligned bounding box)

Neither `Mesh` nor `Model` currently carries a bounding box. Add one to
both. The AABB is computed once at construction time and never changes
(the geometry is immutable after build).

**`Mesh`** — compute AABB from vertex positions at the end of the
constructor (min/max sweep over all positions):

```rust
pub struct Aabb {
  pub min: Vec3,
  pub max: Vec3,
}

impl Aabb {
  pub fn from_points(positions: &[Vec3]) -> Self { /* min/max sweep */ }

  /// Midpoint of the box. Cheap representative point for sorting.
  #[inline]
  pub fn centroid(&self) -> Vec3 {
    (self.min + self.max) * 0.5
  }
}
```

Store as `pub(crate) aabb: Aabb` on `Mesh`.

**`Model`** — compute from the union of all child mesh AABBs. Recompute
whenever `add_mesh` is called (just expand min/max):

```rust
pub struct Model {
  meshes: Vec<Mesh>,
  pub(crate) aabb: Aabb,       // NEW: union of all mesh AABBs (model space)
  pub(crate) is_opaque: bool,  // NEW: see below
}
```

**`BakedMesh`** — same as `Mesh`, but the AABB is in world space
(because the vertices are already world-space).

The AABB is useful beyond transparency (frustum culling, broad-phase
picking, debug visualisation), so it earns its place regardless.

---

## Centroid (for transparency sorting)

The centroid is simply `aabb.centroid()`, the midpoint of the bounding
box. Not the arithmetic mean of vertex positions (which is biased by
vertex density and gives worse results for uneven meshes).

At transparency-sort time, the centroid is transformed into view space to
produce the sort key:

```rust
// Instance mesh: centroid is in model space, transform to view space
let world_centroid = instance.model_mat().transform_point(mesh.aabb.centroid());
let view_z = (cam.view_mat * world_centroid.to_vec4(1.0)).z;

// Baked mesh: centroid is already world space
let view_z = (cam.view_mat * baked.aabb.centroid().to_vec4(1.0)).z;
```

Sort ascending (most negative first) for back-to-front with reverse-Z.

### Centroid is an approximation

Pure centroid sorting fails when meshes overlap or when one mesh is
inside another's bounding box. Live with it for now. The alternative is
per-triangle sort, which means flattening all transparent triangles into
the sort list. Save that for the day it actually breaks.

---

## `is_opaque` flag on Model

Transparency is a surface property (lives on `Material`), but the
renderer needs to quickly decide per-model whether it has any transparent
content. Rather than walking every mesh's material each frame, cache one
boolean on `Model`:

```rust
impl Model {
  /// True when ALL meshes in the model have opaque materials.
  pub(crate) is_opaque: bool,
}
```

**When it's computed:** at the point a mesh is added to the model
(`Model::add_mesh`) and whenever a material is changed
(`set_mesh_material`, `set_all_material`). One private helper
`recompute_opaque()` that walks `meshes` and ANDs
`mesh.material.is_opaque()`. Call it from every mutation site.

**Why on Model and not elsewhere:**

- Not on `Instance`: instances share a model, so the flag would just
  mirror the model's value with no extra signal.
- Not on `Mesh`: `mesh.material.is_opaque()` already gives the answer
  per-mesh. Duplicating it as a separate bool on Mesh adds an
  invalidation point for zero gain.
- On `Model`: lets the renderer skip per-mesh dispatch entirely for
  fully-opaque models (the common case), which is the only level where
  caching actually saves work.

**Render-time dispatch using the flag:**

```
render():
  for instance in scn.instances:
    let model = get_model(instance.handle);
    if model.is_opaque:
      // fast path: all meshes are opaque, no per-mesh routing
      for mesh in model.meshes: rasterise_opaque(...)
    else:
      // mixed model: check each mesh
      for mesh in model.meshes:
        if mesh.material.is_opaque():
          rasterise_opaque(...)
        else:
          transparent_list.push(TransparentDraw { ... })

  for baked in scn.baked_meshes:
    if baked.material.is_opaque():
      rasterise_opaque(...)
    else:
      transparent_list.push(TransparentDraw { ... })

  transparent_list.sort_by(|a, b| a.view_z.partial_cmp(&b.view_z).unwrap())
  for draw in &transparent_list:
    rasterise_blended(draw)
```

This is the only design. No alternatives to choose between. The flag
gives a fast path for opaque models, the per-mesh fallback handles mixed
models, and the material remains the single source of truth.

---

## Shape of the collected draw

The pass-2 list needs enough info to (a) sort and (b) render the right
geometry with the right material. The natural shape:

```rust
// in engine/render.rs or alongside it
enum TransparentSource {
  Instance { instance: InstanceHandle, mesh_idx: usize },
  Baked    { baked_idx: usize },
}

struct TransparentDraw {
  source: TransparentSource,
  view_z: f32,   // sort key, computed once at collection time
}
```

Lives on `Engine` as `Vec<TransparentDraw>`, `clear()`ed at the top of
`render()`, reused frame-to-frame. Same pattern as the existing
`self.verts`.

You **don't** need to bake the matrices or vertex data into the draw at
collection time. The renderer can re-fetch model + mesh by handle/index
during pass 2 — exactly as `render_instance` does today. The list is
just an ordering hint.

---

---

## The blend itself

**`argh/src/buffer.rs`** — currently has `set_pixel_depth` which does
fused test-and-write. Add a sibling that tests but does not write, and
performs the blend:

```rust
#[inline(always)]
pub(crate) fn blend_pixel_depth(&mut self, x: usize, y: usize, src: Colour, a: f32, z: f32) {
  let idx = y * self.w + x;
  // Depth TEST (greater wins, reverse-Z) but no depth WRITE.
  // Unlike set_pixel_depth, self.depth[idx] is never touched.
  if z <= self.depth[idx] { return; }

  let dst = Colour::from_packed_0rgb(self.pixels[idx]);
  let out = src * a + dst * (1.0 - a);   // straight alpha "src OVER dst"
  self.pixels[idx] = out.to_packed_0rgb();
  // deliberately NOT touching self.depth
}
```

This is the only place in the renderer that depth is tested without
being written. Keep them as two distinct functions rather than adding a
"write?" parameter to `set_pixel_depth` — separate inlined paths beat a
runtime branch in the hot loop, and it's clearer at the call site.

Two pitfalls worth knowing in advance:

- **Gamma.** `from_packed_0rgb` decodes via your sRGB LUT into linear,
  blending happens in linear, `to_packed_0rgb` re-encodes with the cheap
  `sqrt()` gamma. That round-trip is the right answer — blending should
  always be in linear light, never in sRGB-encoded values. Just be aware
  the read-modify-write is doing real work per blended pixel.
- **Straight vs premultiplied alpha.** Above is straight. If you ever
  want additive (`out = src + dst`) for sparks/fire, it's the
  `BlendMode::Additive` branch. Premultiplied alpha
  (`out = src + dst * (1 - a)`) is the better default for advanced use
  cases but means the texture pipeline has to multiply RGB by A at load.
  Skip for now.

### Reuse the rasteriser

You do **not** need a second rasteriser. The cleanest approach is to
parameterise `rasterize_tri` on the write callback or pass through a
`BlendMode`:

```rust
fn rasterize_tri(
  buff: &mut Buffer,
  v0: ScreenVert, v1: ScreenVert, v2: ScreenVert,
  mat: &Material,
) {
  // ...existing setup...
  let opacity = mat.opacity;
  let blend = mat.blend_mode;
  // ...existing inner loop...

  // at the write site:
  match blend {
    BlendMode::Opaque => {
      // existing path, also handles alpha_cutout via texture
      buff.set_pixel_depth(x as usize, y as usize, surface_colour * lighting, z);
    }
    BlendMode::AlphaBlend => {
      let a = opacity * texel_alpha;     // texel_alpha = 1.0 if untextured
      if a < 1.0/255.0 { /* skip */ }    // optional early-out
      buff.blend_pixel_depth(x as usize, y as usize, surface_colour * lighting, a, z);
    }
    BlendMode::Additive => {
      // out = src + dst, ignore alpha for composite, depth test on / write off
      buff.add_pixel_depth(x as usize, y as usize, surface_colour * lighting, z);
    }
  }
}
```

The `match` on `blend` is on a value that's **constant for the whole
triangle**, so the branch predictor pins it after the first pixel. Cost is
invisible. (Same logic as the texture/no-texture match already in the
loop.)

---

## Integration touchpoints — checklist

In rough order:

1. **`material.rs`**
   - Add `BlendMode` enum.
   - Add `opacity: f32` and `blend_mode: BlendMode` to `Material`.
   - Update `MATERIAL_PLACEHOLDER`, `new_textured`, `new_flat` to default
     to `opacity = 1.0`, `blend_mode = Opaque`.
   - Add `is_opaque()` helper.

2. **New: `aabb.rs`** (or inline in `mesh.rs`)
   - Add `Aabb { min, max }` struct with `from_points` and `centroid()`.

3. **`mesh.rs`**
   - Add `pub(crate) aabb: Aabb`, computed from vertex positions in the
     constructor.

4. **`baked_mesh.rs`**
   - Same: compute and store `aabb` (in world space).

5. **`model.rs`**
   - Add `pub(crate) aabb: Aabb` (union of child mesh AABBs, model space).
   - Add `pub(crate) is_opaque: bool`.
   - Recompute both in `add_mesh`, `set_mesh_material`, `set_all_material`.

6. **`buffer.rs`**
   - Add `blend_pixel_depth` (and `add_pixel_depth` if you want additive).

7. **`engine/render.rs`**
   - In `rasterize_tri`, branch on `mat.blend_mode` at the write site,
     pass `mat.opacity` through, compute effective alpha as
     `opacity * texel_alpha`.
   - Add `transparent_list: Vec<TransparentDraw>` to `Engine` struct.
   - Refactor `Engine::render`:
     - clear `transparent_list`.
     - pass 1: check `model.is_opaque` for the fast path; for mixed
       models, route per-mesh. Baked meshes check material directly.
     - compute `view_z` from `mesh.aabb.centroid()` transformed to view
       space.
     - sort `transparent_list` ascending by `view_z`.
     - pass 2: for each entry, look up the geometry and rasterise via
       the blended write path.

8. **Tests / scene**
   - Add a simple transparent quad in `examples/` to eyeball it. Two
     overlapping coloured quads at different `z` is the minimum useful
     case. Three confirms the sort.

---

## Edge cases to know about up front

- **Baked lighting + transparency.** `BakedMesh::bake_lighting` snapshots
  static-light contribution into per-vert colour at add-time. That's
  fine for transparent baked meshes — opacity is material-level, baked
  lighting is unaffected. But if you ever want a transparent surface
  whose opacity is itself baked (e.g. a stained-glass window with
  multiplied light), you'll need a bake-time hook. Not a concern for the
  v1 feature.

- **Transparent + cutout in the same mesh.** Technically possible
  (`BlendMode::AlphaBlend` + a texture with alpha-cutout enabled), but
  meaningless: the cutout discards before the blend gets a chance. Pick
  one. Document this and either ignore cutout when blend mode is non-
  Opaque, or assert.

- **Order of two transparent things at exactly the same `view_z`.**
  Centroid sort isn't stable across the comparator if both centroids
  project to identical `view_z`. Won't crash, will flicker by frame.
  Acceptable for a v1; if it bites, secondary-sort by `InstanceHandle`
  or mesh index for stability.

- **Camera inside a transparent mesh.** The centroid is on the wrong
  side of the camera, sort order is undefined. Document as
  "don't fly the camera through the glass." Real fix is per-triangle
  sort or per-pixel ordering (depth peel / OIT), neither of which is
  in scope here.

- **Back faces.** `is_back_facing` runs in screen space and is unaware
  of blend mode. For glass you usually still want back-face culling
  (you only see the front of a window). For something like a fish tank
  where the back wall is visible through the front, you'd want
  _two-sided_ rendering: draw the mesh twice, back faces first, then
  front faces, both in pass 2. Out of scope for v1 but worth knowing.

- **Specular on transparent surfaces.** Existing Gouraud specular still
  applies and that's correct — glass highlights are exactly the point.
  No change needed.

---

## What was deliberately rejected

- **Weighted-blended OIT (McGuire/Bavoil 2013)**: order-independent,
  one geometry pass, but needs two extra full-screen float buffers and
  a composite pass. Worth it for hundreds of overlapping smoke
  particles. Total overkill for a few panes of glass. Revisit only if
  the scene profile changes radically.
- **Depth peeling**: correct order-independent transparency by
  rendering the scene N times, peeling the nearest unrendered fragment
  per pass. Beautiful, but N× render cost. Pointless here.
- **A-buffer / per-pixel linked lists**: capture every fragment per
  pixel, sort, composite. Memory-heavy, complex. No.
- **Screen-door / dithered transparency**: zero-sort alternative
  (`if alpha < bayer4[x&3][y&3] { discard }`). Bulletproof,
  order-independent, integrates with the existing fast path with no
  blending and no extra buffer. **Worth keeping in mind as a fallback**
  for cases where the centroid sort breaks (e.g. interpenetrating
  glass). Could even coexist with the blended path: per-material
  choice of "blend or dither". Not required for v1 but a 6-line
  fallback if needed.

---

## Suggested implementation order

1. `BlendMode` + `Material::is_opaque` + `opacity` field. Compile, all
   existing materials default to Opaque, nothing observably changes.
2. `Aabb` struct + `Mesh::aabb` + `BakedMesh::aabb` + `Model::aabb`.
   Compute once at construction, unused so far. Compile.
3. `Model::is_opaque` flag, recomputed in `add_mesh` /
   `set_mesh_material` / `set_all_material`. Compile.
4. `Buffer::blend_pixel_depth`. Untested.
5. Branch `rasterize_tri` on `BlendMode` at the write site, route alpha
   blend through `blend_pixel_depth`. Author a single test scene with
   one transparent quad over an opaque ground plane to confirm the
   blend formula and the depth-write-off behaviour.
6. Add `Engine::transparent_list`, refactor `Engine::render` into the
   two-pass collect-sort-draw flow using `model.is_opaque` fast path
   and `mesh.aabb.centroid()` for the sort key.
7. Multi-quad test scene (three quads at different `view_z`, partially
   overlapping) to confirm sort direction is correct.

Each step compiles and runs cleanly on its own; you can pause at any
point and the renderer still works.
