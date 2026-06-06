# Renderer improvements

A punch-list of correctness, quality and performance work for the software
rasteriser, captured from a code-review pass over the engine, math and
rendering modules. Roughly ordered "biggest visible impact for least effort"
first, but each item stands alone and can be picked off independently.

Items already done are listed at the bottom for completeness.

## Suggested order to tackle

1. Near-plane Sutherland-Hodgman clip (kills triangle popping near camera)
2. Unify f32/f64 colour handling
3. Tile-based rasteriser (bigger refactor; do once 1 and 2 are in)

---

## Correctness

### 1. Strict near-plane discard pops triangles in and out

**File:** `argh/src/engine/render.rs`, lines 179 to 184.

Today, any triangle with even one vertex behind the near plane is discarded
whole. Result: triangles pop visibly when you fly the camera close to
geometry. The TODO already calls this out.

The fix is **Sutherland-Hodgman clipping against the near plane only**. You
don't need to clip against all six frustum planes (the others can stay as
"discard the whole triangle"; the visual artefact there is invisible because
the geometry is already off-screen). Near is special because the perspective
divide blows up as `w -> 0`.

Algorithm in summary:

- For each triangle, check the near plane outcode bits.
- If all three verts are in front of near, pass through.
- If all three are behind, discard.
- Otherwise, clip the polygon against the single near plane in _clip space_
  (before the perspective divide). You'll emit 1 or 2 sub-triangles
  depending on how many verts were behind.
- New verts get linearly interpolated positions, UVs, and (if you have it)
  per-vertex shading.

Reference: Foley & van Dam, or Fabien Sanglard's Doom 3 BFG notes.

---

## Performance

### 2. Lift the texture/flat branch out of the inner loop (probably not worth it)

**File:** `argh/src/engine/render.rs`.

This was on the list because the inner loop currently does
`match &mat.texture { ... }` per pixel. But honest reassessment: that branch
is on a value that's **constant for the whole triangle**, so the CPU branch
predictor hits 100% accuracy after one iteration. The cost is invisible
next to the `1.0 / inv_w` divide and the texture cache misses.

If profiling ever shows it actually mattering, the cleanest fix is the
boring one: split into `fill_inner_textured` and `fill_inner_flat` with a
single `if let Some(tex) = &mat.texture` between bbox setup and the inner
loop. Duplicates the loop scaffolding, but each function reads
top-to-bottom with no generics or captured-closure puzzles.

**Avoid** the "generic closure" trick (`fn fill_triangle<F: Fn(...)>(..., f: F)`).
It monomorphises correctly, but the call sites become noisy ceremony and the
clarity loss is not paid for by any measurable speedup. Reach for that
pattern only when SIMD-rewriting a known-hot loop, not as default style.

### 3. f32/f64 colour mixing

**Files:** `argh/src/colour.rs`, `argh/src/helpers.rs`.

`Colour` stores `f32`. Lighting multiplies through `f64` (the `Mul<f64>`
impl), then casts back. Every per-pixel shade does an `as f32`. Inconsistent
and slightly wasteful.

Two paths:

- Make `Colour` fields `f64` to match the rest of the maths. Doubles colour
  storage cost (probably not measurable).
- Or: keep `Colour` as `f32`, audit `shade_vert` and friends to work in f32
  end-to-end, and remove the `Mul<f64> for Colour` impl. The world-space
  geometry stays f64; only the colour pipeline becomes f32.

The second is what most software renderers do.

### 4. Tile-based rasteriser

**Biggest refactor on the list; do it last.**

Replace the bbox + scanline walk in `fill_triangle` with a tile-based
traversal:

- Divide the screen into 8x8 or 16x16 tiles.
- For each triangle, find the tiles its bbox touches.
- For each tile, test the four corners against the three edge functions.
  - All corners inside all three edges: every pixel is inside; skip the
    per-pixel test.
  - All corners outside any one edge: discard the whole tile.
  - Mixed: per-pixel test as today.

Pays off in three ways:

- Better cache locality (depth buffer and colour buffer accessed in
  contiguous chunks rather than long thin scanlines).
- Big triangles get massive early-out wins on fully-inside tiles.
- Sets up the structure for SIMD (process 4 or 8 pixels at once inside a
  tile) and threading (one tile per thread).

Reference: Larrabee paper (Abrash & Forsyth), and the classic
"Optimising Software Occlusion Culling" series by Fabien Giesen.

---

## Smaller smells (cosmetic / consistency)

- `Camera::new_perspective` returns `Box<dyn Error>` while everything else
  uses typed `thiserror` enums. Pick one.
- `render()` walks `for i in 0..len { hdl = keys[i]; ... }` to dance around
  the borrow checker. Restructure to iterate the slice cleanly.
- `Instance::rot_x/y/z` mirror `Quat::rot_x/y/z`. Two layers of the same
  thing. Could be a single delegating macro, or just direct access to the
  underlying quat. Not urgent.

---

## Things deliberately not on this list

These would each be a "real" feature, not a fix. Worth doing later, but
don't qualify as "improvements" so much as "next chapters":

- Bilinear / trilinear texture filtering
- Mipmaps (generation + level selection)
- Shadow mapping or shadow volumes
- Proper PBR (Cook-Torrance, GGX, Fresnel)
- Multi-threaded rendering (depends on the tile refactor first)
- SIMD inner loop (depends on the tile refactor first)
- Sprite/2D blitting with alpha compositing
- Bitmap font improvements

---

## Already done

- **Quaternion world-frame rotation methods** (`rot_x_world`, `rot_y_world`,
  `rot_z_world`, plus generic `rotate_world` / `rotate_local`). Convention
  pinned by tests in `quat_tests.rs`.
- **`add_instance_trans` double-write of `pos` and `scale`** in
  `scene.rs`. Tidied.
- **Top-left fill rule + epsilon degenerate guard** in `fill_triangle`
  (`engine/render.rs`). `TRI_AREA_EPS = 1e-8` shortcut and the `tl0/tl1/tl2`
  flags on the inside test, so shared edges between adjacent triangles
  rasterise without dropouts or double-writes.
- **Proper normal transform via inverse-transpose `Mat3`**
  (`engine/render.rs`, `math/matrix3.rs`). A genuine 3D `Mat3` (the old 2D
  affine matrix was renamed to `Affine2`), with `from_mat4_upper`,
  `transpose`, `inverse`, and an `inverse_transpose` that skips the
  redundant transpose. Normals re-normalised after transform.
- **UV-length and normal-length `debug_assert`** in `Model::add_mesh`
  (`models.rs`). Loud failure at the boundary rather than a confusing
  panic inside the rasteriser.
- **Split `Mat4 * Vec3` into `transform_point` / `transform_dir`**
  (`math/matrix4.rs`). Every call site now declares intent; the
  point-vs-direction footgun is gone.
- **Gamma correction with 256-entry sRGB→linear LUT** (`colour.rs`).
  Encode (`1/2.2`) in `to_packed_0rgb`, LUT-based decode in
  `from_packed_0rgb` and `from_rgb8`, so the texture-sample hot path is
  one cache load instead of three `powf` calls.
- **Removed redundant bounds check in `Buffer::set_pixel_depth`**
  (`buffer.rs`). Rasteriser clamps the bbox, so the check was dead weight;
  `y * w + x` now computed once.
- **Light distance attenuation** with `atten_linear` and `atten_quad` fields
  on `Light` (defaults 0.09 / 0.032) and quadratic falloff in `shade_vert`
  (`light.rs`, `helpers.rs`). Diffuse and specular both attenuated.
- **Per-frame heap allocations removed in `render_instance`**
  (`engine/mod.rs`, `engine/render.rs`). Scratch `verts` and `normals` Vecs
  live on `Engine`; `clear()` + `extend()` reuses capacity so steady-state
  allocation count is zero. Instance-level matrix setup also hoisted above
  the mesh loop.
- **Perspective-correct `inv_w`, `u_w`, `v_w` stepped incrementally in
  `fill_triangle`** (`engine/render.rs`). Starting values and per-x / per-y
  deltas computed once at triangle setup; inner-loop weighted sums
  replaced by three adds. Roughly 8 multiplies and 2 adds saved per
  textured pixel.
