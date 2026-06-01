# Renderer improvements

A punch-list of correctness, quality and performance work for the software
rasteriser, captured from a code-review pass over the engine, math and
rendering modules. Roughly ordered "biggest visible impact for least effort"
first, but each item stands alone and can be picked off independently.

Items already done are listed at the bottom for completeness.

## Suggested order to tackle

1. Top-left fill rule + epsilon-degenerate guard (rasteriser correctness)
2. Gamma correction (visual quality, biggest "this looks better" lever)
3. Near-plane Sutherland-Hodgman clip (kills triangle popping near camera)
4. Reuse scratch vectors in `render_instance` (allocator pressure)
5. Step `inv_w`, `u_w`, `v_w` incrementally across the bbox (rasteriser speed)
6. Light attenuation in `shade_vert` (visual quality)
7. Proper normal transform (inverse-transpose) when non-uniform scale exists
8. UV-length `debug_assert` when registering a mesh
9. Split `Mat4 * Vec3` into `transform_point` vs `transform_dir`
10. Unify f32/f64 colour handling
11. Tile-based rasteriser (bigger refactor; do once 1 to 9 are in)

---

## Correctness

### 1. Top-left fill rule

**File:** `argh/src/engine/render.rs`, inside `fill_triangle`.

The current inside test is `if w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0`. Pixels
that fall *exactly* on a shared edge between two triangles will be drawn by
both, by neither, or by one of them depending on float rounding. Visually:
intermittent black or doubled pixels along seams inside a connected mesh.

The standard fix is the **top-left rule** (D3D / Microsoft Raster Spec):

- A pixel on an edge is considered inside if that edge is a "top" edge
  (perfectly horizontal and above the triangle's interior) OR a "left" edge
  (the edge goes downwards on the screen, i.e. its y component is positive
  in screen space).
- For all other on-edge pixels, the pixel is *not* inside that triangle.

Implement by pre-computing three booleans per triangle (one per edge) and
flipping `<= 0.0` to `< 0.0` for non top-left edges. Negligible cost in the
inner loop, kills seam artefacts.

### 2. Epsilon-degenerate triangle guard

**File:** `argh/src/engine/render.rs`, `fill_triangle`.

Currently:

```rust
if area == 0.0 { return; }
```

A triangle with `area = 1e-15` sails through, then `inv_area = 1e15`,
propagating into barycentric weights and producing either flashes or NaNs.
Change to `if area.abs() < EPS { return; }` where `EPS` is something like
`1e-8` for f64.

### 3. Strict near-plane discard pops triangles in and out

**File:** `argh/src/engine/render.rs`, lines 177 to 182.

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
- Otherwise, clip the polygon against the single near plane in *clip space*
  (before the perspective divide). You'll emit 1 or 2 sub-triangles
  depending on how many verts were behind.
- New verts get linearly interpolated positions, UVs, and (if you have it)
  per-vertex shading.

Reference: Foley & van Dam, or Fabien Sanglard's Doom 3 BFG notes.

### 4. Normal transform ignores non-uniform scale

**File:** `argh/src/engine/render.rs`, line 150.

`instance.rot.rotate_vec3(*n)` rotates the normal by the rotation quaternion
only. Correct iff the scale is uniform. The instance API exposes `scale_x`,
`scale_y`, `scale_z` independently, so anyone using non-uniform scale will
get subtly wrong lighting (highlights drift off-surface, edges shade wrong).

The textbook fix is to transform normals by the **inverse-transpose of the
upper 3x3 of the model matrix**, then re-normalise.

Two practical paths:

- **Cheap path**: detect non-uniform scale and only build the inverse-
  transpose then. Uniform scale path can keep using the quaternion. This is
  what most engines do.
- **General path**: always build a `Mat3` for normals as
  `transpose(inverse(M3))` where `M3` is the upper 3x3 of the model matrix.
  Simpler to reason about; slightly more cost per instance per frame.

Either way, normalise after transform.

### 5. `Mat4 * Vec3` silently treats it as a point

**File:** `argh/src/math/matrix4.rs`, `Mul<&Vec3> for Mat4`.

The current implementation does `M * (v.x, v.y, v.z, 1.0)`, which adds the
translation column. Fine for points. Wrong for directions (normals, light
vectors). You're sidestepping this by rotating normals via the quaternion
directly, so you've never been bitten, but the operator is a latent footgun.

Suggested change: drop the `Mul<Vec3>` impl in favour of two named methods:

```rust
fn transform_point(self, v: Vec3) -> Vec3 { ... }   // implicit w = 1
fn transform_dir  (self, v: Vec3) -> Vec3 { ... }   // implicit w = 0
```

Forces every call site to declare intent. Catches the bug at compile time
if someone refactors a normal through `mat * v`.

### 6. UV array length coupled to verts length

**File:** `argh/src/engine/render.rs`, line 126; `argh/src/engine/parse.rs`.

The render path does `mesh.uvs[i]` unconditionally. The OBJ parser pads
zeros if UVs are missing, but anything else that builds a `Mesh` directly
must remember to do the same or risk panic.

Cheap fix: `debug_assert_eq!(mesh.uvs.len(), mesh.verts.len(), "UVs must
match vert count")` inside `Engine::add_mesh`. Loud failure at the boundary
beats a confusing panic in the render loop.

### 7. Z-buffer test uses `<` only

**File:** `argh/src/buffer.rs`, `set_pixel_depth`.

`z < self.depth[idx]` means "first triangle wins" for perfectly coplanar
polygons. That's a defensible choice (skips later draws, saves overdraw
cost), but it's also non-obvious behaviour. Worth a one-line comment to
document the convention, even if not changing it. If you ever want "last
draw wins" for things like decals, you'll need a `<=` variant.

---

## Visual quality

### 8. Gamma correction

**Biggest "why doesn't my renderer look as nice" lever there is.**

Currently:

- Lighting is done in linear RGB (correct).
- The result is written straight to the 8-bit framebuffer as linear values
  (wrong).
- minifb's window displays those 8-bit values as sRGB.

Net effect: highlights get crushed, mid-tones are too dark, the whole image
looks muddy compared to a "proper" renderer.

Fixes, in order of correctness vs cost:

- **Quick and dirty**: `pow(x, 1.0/2.2)` in `Colour::to_packed_0rgb` before
  the multiply-by-255. Two pows per pixel. Looks dramatically better.
- **Slightly correct**: `pow(x, 1.0/2.2)` *and* `pow(x, 2.2)` when reading
  texture pixels in `Texture::sample`, because PNG/JPG textures are stored
  in sRGB and need to be linearised before the linear lighting maths.
- **Proper**: replace the gamma 2.2 approximation with the real piecewise
  sRGB transfer function. Negligible visual difference, slightly more cost.
  Worth doing if you ever care about photographic accuracy.

A 256-entry lookup table for the linear->sRGB conversion eliminates the
per-pixel `pow` entirely. The output side is 8-bit so 256 entries is exact.

### 9. Lights have no distance attenuation

**File:** `argh/src/helpers.rs`, `shade_vert`.

Right now a light at distance 1 lights a surface identically to the same
light at distance 100. Scenes look "evenly bright" with no sense of depth
or proximity to light sources.

Standard quadratic falloff:

```
attenuation = 1.0 / (1.0 + k_l * d + k_q * d * d)
```

where `d` is distance from light to fragment, `k_l` is linear coefficient,
`k_q` is quadratic. Defaults of `k_l = 0.09`, `k_q = 0.032` are reasonable
for a unit-scale scene. Multiply both `diff_col` and `spec_col` by this
factor.

Add fields to `Light` for the two coefficients (or one combined "range"
value if you want a simpler API).

---

## Performance

### 10. Per-frame heap allocations in `render_instance`

**File:** `argh/src/engine/render.rs`.

Every frame, for every instance, you allocate two `Vec`s:

- `Vec<ProcessedVert>` of length `mesh.verts.len()`
- `Vec<Vec3>` for transformed normals

One wine bottle: fine. Hundreds of instances: real allocator pressure.

Standard pattern: store two reusable scratch buffers on `Engine`,
`clear()` them at the start of each `render_instance`, and grow them as
needed. Saves the allocator on the hot path.

### 11. Step `inv_w`, `u_w`, `v_w` per pixel incrementally

**File:** `argh/src/engine/render.rs`, inside `fill_triangle`.

Today, for every textured pixel inside the triangle, the inner loop does:

```rust
let inv_w = b0 * v0.inv_w + b1 * v1.inv_w + b2 * v2.inv_w;
let u = (b0 * v0.u_w + b1 * v1.u_w + b2 * v2.u_w) * w;
let v = (b0 * v0.v_w + b1 * v1.v_w + b2 * v2.v_w) * w;
```

That's three weighted sums per pixel. But `inv_w`, `u_w` and `v_w` are
**linear in screen space** (that's the whole point of pre-dividing by w).
So just like the edge functions `w0`, `w1`, `w2` already step incrementally
across the bbox, these can too:

- Compute the value and the per-pixel `dx`/`dy` deltas at triangle setup.
- Step by `dx` along x in the inner loop, by `dy` per scanline.

Removes 6 multiplies and 6 adds per textured pixel.

### 12. Lift the texture/flat branch out of the inner loop (probably not worth it)

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

### 13. Redundant bounds checks in `set_pixel_depth`

**File:** `argh/src/buffer.rs`, line 50.

After the bbox clamp in `fill_triangle`, every pixel is guaranteed inside
the buffer. `set_pixel_depth`'s `x < self.w && y < self.h` check is dead
weight on the hot path. Two options:

- Split into a safe public `set_pixel_depth` and an unchecked
  `set_pixel_depth_unchecked` (with `debug_assert!`s) that the rasteriser
  calls.
- Or: drop the check entirely in `set_pixel_depth` and rely on the
  rasteriser's clamp. Less robust if anything else ever calls it.

Either way, also stop computing `y * self.w + x` twice.

### 14. f32/f64 colour mixing

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

### 15. Tile-based rasteriser

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
