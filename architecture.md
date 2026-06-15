# Argh architecture

High-level ownership & reference map of the main runtime components.

```
                  ┌─────────────────────────┐
                  │   App   (your code)     │
                  └──┬──────────┬──────────┬┘
            owns /   │          │          │
            drives   ▼          ▼          ▼
                ┌────────┐ ┌────────┐ ┌────────┐
                │ Engine │ │ Scene  │ │ Camera │
                └────────┘ └────────┘ └────────┘

Engine
  └── models : SlotMap<ModelHandle, Model>
        │
        └── Model
              ├── name
              ├── tri_count
              └── meshes : Vec<Mesh>
                    │
                    └── Mesh
                          ├── verts, normals, uvs, indices
                          └── material : Material
                                ├── diffuse, specular, hardness
                                └── texture : Option<Rc<Texture>>
                                              │
                                              └── Texture  (shared via Rc)

Scene
  ├── ambient : Colour
  ├── instances : SlotMap<InstanceHandle, Instance>
  │     │
  │     └── Instance
  │           ├── pos, rot, scale, smooth
  │           └── model_handle  ──► refs into Engine.models
  │
  ├── lights : SlotMap<LightHandle, Light>
  │     │
  │     └── Light  (pos, colour, brightness, atten, is_static)
  │
  └── baked : Vec<BakedMesh>     ──► derived from a Model at
        │                            add_static() time (world space,
        └── BakedMesh                lighting pre-baked into verts)
              ├── verts, normals  (WORLD space)
              ├── uvs, indices
              ├── material : Material
              └── baked_lighting : Vec<Colour>

Camera
  ├── pos, look_at
  ├── view_mat
  └── pers_mat
```

Handle types live in `engine/mod.rs` as `new_key_type!` slotmap keys:
`ModelHandle`, `InstanceHandle`, `LightHandle`.

## Notes

- `Instance` and `BakedMesh` both originate from `Model` but in opposite
  ways: `Instance` keeps a live handle and applies a transform each frame,
  `BakedMesh` snapshots the geometry into world space and forgets the
  handle.
- `Material` is the only thing currently shared between `Mesh` and
  `BakedMesh` (via `Clone`), and `Texture` is shared via `Rc` so cloning a
  material is cheap.

## Rendering pipeline

`Engine::render(cam, scn)` is the per-frame entrypoint. It walks the scene
in two passes: first every `BakedMesh` via `render_static`, then every
`Instance` via `render_instance`. Both paths end up in the same rasteriser
(`rasterize_tri`) and write to the same framebuffer + depth buffer.

```
                       Scene + Camera
                             │
                ┌────────────┴────────────┐
                ▼                         ▼
        render_static                render_instance
       (BakedMesh, VP)              (Instance, Cam, Scene)
                │                         │
                ▼                         ▼
     vertex processing loop      vertex processing loop
     (world is pre-baked)        (Model -> M, MVP, Minv^T)
                │                         │
                └────────────┬────────────┘
                             ▼
                    Vec<ProcessedVert>
                             │
                             ▼
              triangle loop over mesh.indices
                             │
              trivial-reject (combined outcode)
                             │
            ┌────────────────┴────────────────┐
            │                                 │
       any_near == 0                     any_near != 0
      (fast path)                        (slow path)
            │                                 │
            │                       clip_triangle_near
            │                       (Sutherland-Hodgman,
            │                        produces 0, 3 or 4 verts)
            │                                 │
            │                       fan-triangulate + perspective
            │                       divide -> ScreenVert
            │                                 │
            └────────────────┬────────────────┘
                             ▼
                       rasterize_tri
                  (back-face cull, edge fn,
                   barycentrics, perspective-correct
                   UV, linear NDC-z depth, Gouraud light)
                             │
                             ▼
                   Buffer.set_pixel_depth
```

### 1. Vertex processing

Per vertex (in `mesh.positions`):

1. **Transform.** Build `MVP = pers * view * model` once per mesh, then
   `clip = MVP * (x, y, z, 1)`. Normals use the upper-3x3 inverse-transpose
   of the model matrix so non-uniform scale doesn't skew them.
2. **Shade early.** Lighting is computed once per vertex (Gouraud), before
   back-face cull, because tris share verts and per-vert is cheaper than
   per-tri. `shade_vert` walks `scn.lights`, skipping any light with
   `!is_dynamic || !is_enabled`, and returns (diffuse, specular). The mesh's
   `material.diffuse / specular` modulate those and `scn.ambient_light` is
   added in. The `render_static` path additionally adds the per-vert colour
   from `BakedMesh::baked_lighting` (which was produced earlier by
   `BakedMesh::bake_lighting`, walking only `is_static` lights).
3. **Perspective divide + viewport map.** `inv_w = 1 / clip.w`, then
   `ndc = clip.xyz * inv_w`. Screen X/Y come from the standard
   `(ndc * 0.5 + 0.5) * size` map, with Y flipped (origin is top-left).
4. **Outcode.** `compute_outcode(&clip)` packs a 6-bit frustum membership
   mask (left/right/bottom/top/near/far) into a `u8` for trivial reject.
5. **Texture coords are pre-divided.** `u_w = u * inv_w`, `v_w = v * inv_w`.
   This lets the rasteriser do perspective-correct UV with one divide per
   pixel instead of two.

Each vertex is pushed into a `Vec<ProcessedVert>` (reused per mesh, cleared
each call) which holds `clip`, `uv`, `outcode`, and the finished
`ScreenVert`.

### 2. Triangle assembly + culling

`mesh.indices.chunks(3)` walks triangles. For each:

- **Trivial reject.** `outcode[i0] & outcode[i1] & outcode[i2] != 0` means
  all three verts are outside the same frustum plane: skip.
- **Near-plane test.** `(outcode[i0] | outcode[i1] | outcode[i2]) & OUT_NEAR`
  flags whether any vert is past the near plane. This selects fast vs slow
  path.

### 3. Near-plane clipping (slow path only)

The far/left/right/top/bottom planes are handled implicitly by the pixel
loop's screen-bounds clamping. The **near plane** must be clipped in clip
space, otherwise `clip.w` can flip sign and the perspective divide
explodes.

`clip_triangle_near` is a single-plane Sutherland-Hodgman pass against
`clip.w - clip.z >= 0` (the reverse-Z near plane). Per edge it emits 0, 1
or 2 verts depending on inside/outside membership, lerping the `ClipVert`
payload (clip pos, raw UV, pre-shaded light) at the boundary with the same
`t` so all attributes stay consistent. Output is 0, 3, or 4 verts; 4 verts
get fan-triangulated into 2 tris.

Each output `ClipVert` is then re-projected to a `ScreenVert` via
`ScreenVert::from_clip`, which repeats the perspective divide + viewport
map exactly as the per-vertex loop does, so fast-path and slow-path verts
match bit-for-bit when nothing actually clipped.

### 4. Rasterisation

`rasterize_tri` is a classic half-space rasteriser:

1. **Back-face cull.** `is_back_facing` checks signed triangle area in
   screen space. Because Y is flipped, the test is inverted relative to
   OpenGL: back faces have non-negative area.
2. **Edge functions.** `edge_function` is a 2D cross product. Three edges
   give barycentric weights at every pixel by incremental addition (no
   divides in the hot loop). Inside test is `w0 <= 0 && w1 <= 0 && w2 <= 0`
   because Argh's triangles arrive CW on screen.
3. **Bounding-box clip.** Min/max of vert XY clamped to buffer bounds gives
   the loop range, sampling at pixel centres (`+ 0.5`).
4. **Depth.** `z = b0*v0.z + b1*v1.z + b2*v2.z` is interpolated linearly in
   NDC (no `/w` needed in screen space). Convention is reverse-Z: near
   plane maps to `ndc.z = 1`, far to `ndc.z = 0`, which pairs f32's
   high-precision band with the near plane.
5. **Texture sample.** If the material has a texture, `inv_w`, `u_w`, `v_w`
   are interpolated linearly, then one divide recovers perspective-correct
   `(u, v)`. `tex.sample(u, v)` returns colour + alpha; alpha-cutout
   textures `continue` past the pixel without writing depth.
6. **Lighting.** `v0.light * b0 + v1.light * b1 + v2.light * b2` blends the
   per-vertex Gouraud colour across the triangle, then `surface * lighting`
   gives the final pixel colour. For a flat-shaded look, authors call
   `Model::flatten` so each vert has its face's normal and Gouraud collapses
   to flat.
7. **Write.** `Buffer.set_pixel_depth` does the depth test (reverse-Z, so
   greater depth wins) and stores the packed sRGB colour.

### Coordinate conventions in one place

| Stage         | X         | Y                | Z                | W           |
| ------------- | --------- | ---------------- | ---------------- | ----------- |
| World / view  | RH        | RH               | looks down -Z    | -           |
| Clip          | -w..w     | -w..w            | 0..w (reverse-Z) | -view.z     |
| NDC           | -1..1     | -1..1            | 0..1 (near=1)    | -           |
| Screen        | 0..width  | 0..height (down) | 0..1 NDC depth   | inv_w cache |

### Static vs dynamic in one place

| Aspect             | `render_instance` (Mesh)           | `render_static` (BakedMesh)            |
| ------------------ | ---------------------------------- | -------------------------------------- |
| Geometry source    | `Model.meshes`, model-space        | `BakedMesh`, world-space               |
| Model matrix       | Built per frame from `Instance`    | Identity (already in world)            |
| Normal matrix      | `Mat3::from_mat4_upper(m).inv_t()` | Not needed (baked has world normals)   |
| Per-vert lighting  | `shade_vert(dynamic lights)`       | `baked_lighting[i] + shade_vert(...)`  |
| Updated when?      | Every frame                        | Bake-time + dynamic top-up each frame  |
| Light filter       | `is_dynamic && is_enabled`         | `is_static` at bake, dynamic per-frame |
