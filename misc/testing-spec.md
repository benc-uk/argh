# argh — Test Specification

This document is the canonical guide for writing tests against the `argh` software-rendering
engine. It exists so a generative tool (or a human) can produce a comprehensive, internally
consistent test suite without re-deriving conventions from scratch.

The suite targets **behaviour, not pixels**. We verify state, counts, transforms, parser
output, and edge/negative cases. We do **not** golden-image or hash framebuffers.

---

## Table of contents

1. [House style and conventions](#house-style-and-conventions)
2. [Test placement and module wiring](#test-placement-and-module-wiring)
3. [Shared test helpers](#shared-test-helpers)
4. [Assets available](#assets-available)
5. [Crate-internal access](#crate-internal-access)
6. [Module-by-module test specifications](#module-by-module-test-specifications)
    * [`colour`](#colour)
    * [`texture`](#texture)
    * [`material`](#material)
    * [`mesh`](#mesh-pubcrate)
    * [`model`](#model)
    * [`instance`](#instance)
    * [`camera`](#camera)
    * [`light`](#light)
    * [`scene`](#scene)
    * [`baked_mesh`](#baked_mesh-pubcrate)
    * [`buffer`](#buffer-pubcrate)
    * [`primitives`](#primitives)
    * [`engine` core](#engine-core)
    * [`engine::draw2d`](#enginedraw2d)
    * [`helpers`](#helpers-pubcrate)
    * [`engine::parse_obj`](#engineparse_obj)
    * [`engine::parse_gltf`](#engineparse_gltf)
    * [`math` audit checklist](#math-audit-checklist)
7. [Gotchas, traps and quirks](#gotchas-traps-and-quirks)

---

## House style and conventions

Match the existing math test style. Look at `argh/src/math/vector3_tests.rs` for the
reference example.

### File header

Every test file starts with the same banner:

```rust
// ==============================================================================================
// Module & file:   <module> / <file>_tests.rs
// Purpose:         Tests for <thing>
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;
```

### Section banners

Group related tests under `// --- <Area> ---` banners. Areas should be derived from the
public API surface, e.g. `Constructors`, `Setters`, `Arithmetic`, `Edge cases`,
`Negative cases`, `Round-trips`, `Defaults`.

### Test function naming

```
test_<thing>_<scenario>
```

Examples:

* `test_new`
* `test_new_with_negatives`
* `test_normalize_zero_vector`
* `test_scene_remove_instance_handle_no_longer_resolves`
* `test_load_gltf_missing_file_errors`

Keep names readable; do not abbreviate to the point of obscurity.

### Test body conventions

* One conceptual assertion per test wherever possible. A test may contain multiple
  `assert_eq!` calls if they verify the same fact (e.g. all fields of a struct that should
  match defaults).
* Prefer `assert_eq!` over `assert!` when comparing values.
* Use the helper `assert_vec3_near` / `assert_mat4_near` for any float comparison that goes
  through arithmetic. Direct `==` on f32 is only acceptable for values that were never
  computed (constants, exact-bit constructors).
* Avoid loops in tests unless verifying behaviour over a range (e.g. wrap-around addressing
  at multiple UVs). Prefer flat, copy-paste-style tests so each failure points at exactly
  one scenario.
* When asserting an error variant, match it: don't just check `is_err()`.
* When asserting a panic, use `#[should_panic]` or `#[should_panic(expected = "...")]`.

### Tolerance

For computed float comparisons use `1e-5` as the default epsilon (matches existing math
tests). For trig-heavy comparisons (rotations, perspective projections) use `1e-4`.

### Floating-point edge values

Test these explicitly where the type accepts a float:

* `f32::NAN`, `f32::INFINITY`, `f32::NEG_INFINITY`
* `0.0` and `-0.0`
* `f32::MIN_POSITIVE`, `f32::MAX`
* Boundary values for the type (e.g. roughness ∈ `[0,1]` → test 0.0 and 1.0)

### Commenting

Comments are sparse. Each `// --- Section ---` banner is enough. Single-line comments above
a non-obvious test scenario are welcome but never mandatory. No corporate-speak.

---

## Test placement and module wiring

We use **inline `#[cfg(test)] mod foo_tests`** with `#[path]` indirection, matching the
existing math pattern.

For each source file `argh/src/foo.rs` (or `argh/src/sub/foo.rs`) that needs tests:

1. Add at the top of `foo.rs`, after the existing `use` statements:

   ```rust
   #[cfg(test)]
   #[path = "foo_tests.rs"]
   mod foo_tests;
   ```

2. Create `argh/src/foo_tests.rs` next to it.

3. The test file imports parent items via `use super::*;`.

This pattern gives tests full access to `pub(crate)` items and any private helpers the
test file needs, without exposing anything to consumers of the crate.

For `engine/*.rs` files (e.g. `engine/render.rs`, `engine/draw2d.rs`), tests live in
`engine/<file>_tests.rs` with the same convention.

The `math` module already uses this pattern; do not change its layout.

---

## Shared test helpers

A new module at `argh/src/test_helpers.rs` provides shared utilities. Wire it in `lib.rs`
under `#[cfg(test)]`:

```rust
#[cfg(test)]
pub(crate) mod test_helpers;
```

### Required helpers

```rust
use std::path::PathBuf;
use crate::math::{Mat4, Vec3};

/// Resolves a path relative to the crate's manifest dir, then walks up one to find
/// the workspace root, then appends `rel`. Use this for any test that loads from `assets/`.
///
/// Example: `asset_path("assets/gltf/duck.glb")`
pub(crate) fn asset_path(rel: &str) -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // argh/src lives at <root>/argh, so go up one to reach the workspace root.
    let workspace_root = manifest.parent().expect("workspace root not found");
    workspace_root.join(rel)
}

/// Build a flat raw RGBA8 buffer for the given dimensions, all pixels set to `fill`.
pub(crate) fn tiny_rgba8(w: u32, h: u32, fill: [u8; 4]) -> Vec<u8> {
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for _ in 0..(w * h) { out.extend_from_slice(&fill); }
    out
}

/// Build a flat raw RGB8 buffer for the given dimensions, all pixels set to `fill`.
pub(crate) fn tiny_rgb8(w: u32, h: u32, fill: [u8; 3]) -> Vec<u8> {
    let mut out = Vec::with_capacity((w * h * 3) as usize);
    for _ in 0..(w * h) { out.extend_from_slice(&fill); }
    out
}

/// Assert two Vec3s are equal within `eps`. Use for any computed Vec3.
pub(crate) fn assert_vec3_near(a: Vec3, b: Vec3, eps: f32) {
    assert!(
        (a.x - b.x).abs() < eps && (a.y - b.y).abs() < eps && (a.z - b.z).abs() < eps,
        "expected ~{}, got {}", b, a
    );
}

/// Assert two Mat4s are equal within `eps` per element.
pub(crate) fn assert_mat4_near(a: &Mat4, b: &Mat4, eps: f32) {
    let ar = a.raw();
    let br = b.raw();
    for r in 0..4 {
        for c in 0..4 {
            assert!(
                (ar[r][c] - br[r][c]).abs() < eps,
                "mismatch at [{},{}]: got {}, expected {}", r, c, ar[r][c], br[r][c]
            );
        }
    }
}
```

`Mat4::raw()` is `pub(super)` so this helper must live somewhere the math module can
expose it through. If `Mat4::raw()` isn't accessible from `test_helpers`, add a
`#[cfg(test)] pub(crate) fn raw_for_test(&self) -> &[[f32;4]; 4]` on `Mat4` that just
returns `&self.ele`. Same for `Mat3` if needed.

### Optional helpers (add only if a test really needs them)

* `dummy_textured_material()` — returns a `Material` built from `tiny_rgba8(2, 2, ...)`.
* `dummy_mesh(verts, normals, uvs, indices)` — builds a `Mesh` with the four buffers set.
  Useful for `model_tests` and `baked_mesh_tests`.

---

## Assets available

All paths below are relative to the **workspace root**. Use `asset_path("assets/...")`.

The asset directory is `assets/gltf/`. Earlier drafts of this doc claimed it was `glft` (a repo typo) but the repo uses the correct `gltf` spelling.

### glTF / GLB (`assets/gltf/`)

| File                                            | Notes                                       |
|-------------------------------------------------|---------------------------------------------|
| `duck.glb`                                      | Single mesh "LOD3spShape", one material "blinn3-fx". Declares `roughnessFactor ≈ 0.198`, so after PBR-to-Phong mapping `hardness ≈ 41.32` and `specular ≈ (0.96, 0.96, 0.96)`. |
| `utah_teapot_low.glb`                           | Low-poly teapot. Useful for LOD comparisons. |
| `utah_teapot_med.glb`                           | Mid-poly teapot. tri_count > low.            |
| `utah_teapot_high.glb`                          | High-poly teapot. tri_count > med.           |
| `potion/bottle_A_labeled_green.gltf`            | Textured, external `.bin` + PNG, material name "texture". |
| `potion/bottle_A_labeled_green.bin`             | Buffer for the bottle.                       |
| `potion/dungeon_texture.png`                    | Texture used by the bottle.                  |

### OBJ + MTL (`assets/obj/`)

| File / dir                | Notes                                                     |
|---------------------------|-----------------------------------------------------------|
| `icosahedron.obj`         | Untextured icosahedron, no MTL. tri_count = 20.           |
| `model.obj`               | Generic untextured.                                       |
| `skull.obj`               | Untextured, larger mesh.                                  |
| `chest/`                  | Textured (`chest_diffuse.png`), single MTL.               |
| `cola_can/`               | Multi-material (`cocacola.png`, `top.png`, `bottom.png`). |
| `dice/`                   | Textured, has a normal map (which argh ignores).          |
| `dungeon/`                | Many small banners and props; good multi-mesh fodder.     |
| `hamburger/`              | Multi-material.                                           |
| `house_plant/`            | Textured.                                                 |
| `table/`                  | Textured.                                                 |
| `teapot/`                 | OBJ-format teapot (separate from the glTF ones).          |
| `wine/`                   | Textured.                                                 |

### Textures (`assets/textures/`)

| File              | Notes                                              |
|-------------------|----------------------------------------------------|
| `checker_256.png` | 256×256 checker. Useful for sampling tests.        |
| `crate.png`       | Textured crate. Largeish.                          |
| `crate_2.jpg`     | JPEG variant (tests JPEG decoding).                |
| `earth.png`       | Earth map (~1MB).                                  |
| `uv_check.png`    | UV grid, intended for visual verification.         |

---

## Crate-internal access

Several types we want to test are `pub(crate)` and have no public constructor:

* `Mesh`, `Buffer`, `BakedMesh`, `Instance` (must go via `Scene::add_instance`)
* The `engine::render` helpers like `ProcessedVert`, `ScreenVert`
* Members of `Buffer` (depth, pixels, w, h are `pub(crate)`)

Because our tests live inside the crate (`#[cfg(test)] mod`), `pub(crate)` is fine —
just `use super::*;` or absolute paths from `crate::`. No new public API needed.

### When you genuinely cannot reach a thing

If a private helper or a struct field is needed by a test and there's no `pub(crate)`
accessor, add a strictly test-only one:

```rust
#[cfg(test)]
impl Mat4 {
    pub(crate) fn raw_for_test(&self) -> &[[f32; 4]; 4] {
        &self.ele
    }
}
```

Keep these gated by `#[cfg(test)]` so they never compile into release builds. Prefer
extending what's there over inventing parallel APIs.

---

## Module-by-module test specifications

For each module:

* **Functional coverage**: tests that prove correct happy-path behaviour.
* **Edge cases**: boundary values, empty inputs, extreme magnitudes, NaN/inf.
* **Negative cases**: invalid input that must error or panic.
* **Round-trips / invariants**: properties that hold across operations.

### `colour`

API: `Colour { r, g, b }`, `new`, `from_rgb8`, `from_slice`, `from_packed_0rgb`,
`to_packed_0rgb`, `rand`, named constants (`BLACK`, `WHITE`, `RED`, `GREEN`, `BLUE`,
`MAGENTA`, `CYAN`, `YELLOW`), `Display`, `Mul<f32/f64/Colour>`, `Add`, `*Assign`.

#### Functional
* Each constructor produces the expected struct.
* Named constants have expected values (e.g. `WHITE == Colour::new(1.0, 1.0, 1.0)`).
* `Display` produces `"[r, g, b]"`.

#### sRGB ↔ linear
* `from_rgb8(0, 0, 0)` equals `BLACK`.
* `from_rgb8(255, 255, 255)` ≈ `WHITE` (within 1e-5).
* `from_packed_0rgb(0xFFFFFF)` ≈ `WHITE`.
* `from_packed_0rgb(0x000000)` equals `BLACK`.
* `from_rgb8(128, 128, 128)` produces a linear value < 0.5 (gamma squashes mid-tones).

#### Round-trips (gamma-tolerant)
* `Colour::new(0.5, 0.5, 0.5).to_packed_0rgb()` decodes via `from_packed_0rgb` to something
  within `0.02` of `0.5` (gamma round-trip loss).

#### Arithmetic
* `+`, `*` (f32, f64, Colour), and their `*Assign` variants behave as expected.
* `Colour::new(0.5, 0.5, 0.5) * 2.0 == Colour::new(1.0, 1.0, 1.0)`.
* Component-wise `Colour * Colour`.

#### Out-of-range
* `Colour::new(2.0, -1.0, NaN)` constructs without panic — does not clamp.
* Arithmetic does not clamp: `WHITE * 2.0` has components = 2.0.
* `to_packed_0rgb` does clamp to `[0, 1]` before packing.

#### Negative
* (None really; `Colour` is a permissive value type.)

### `texture`

API: `Texture::new(path)`, `from_bytes(&[u8])`, `from_raw_rgba8(&[u8], w, h)`,
`from_raw_rgb8(&[u8], w, h)`, `sample(u, v) -> (Colour, alpha)`, `enable_cutout(bool)`,
fields `alpha_cutout`, `cutoff` (both `pub(crate)`).

#### Functional
* `from_raw_rgba8` with `tiny_rgba8(2, 2, [255, 0, 0, 255])` gives 4 red pixels, alpha 1.0,
  `alpha_cutout == true`.
* `from_raw_rgb8` with `tiny_rgb8(2, 2, [0, 255, 0])` gives 4 green pixels, alpha 1.0,
  `alpha_cutout == false`.
* `Texture::new(asset_path("assets/textures/checker_256.png").to_str().unwrap())` succeeds.
* `Texture::from_bytes(include_bytes!("../../assets/textures/checker_256.png"))` succeeds.
* `enable_cutout(false)` flips `alpha_cutout`.

#### Sampling
* `sample(0.0, 0.0)` returns the (0, 0) texel.
* `sample(1.0, 1.0)` wraps to (0, 0) texel (because of `u - u.floor()`).
* `sample(2.7, 2.7)` equals `sample(0.7, 0.7)`.
* `sample(-0.3, -0.3)` equals `sample(0.7, 0.7)` (wrap is floor-based).
* Asymmetric corners: `sample(0.5, 0.5)` lands in the centre texel for a 2×2 texture.

#### Edge cases
* `from_raw_rgba8` with empty slice gives 0 pixels.
* `from_raw_rgba8` with 5 bytes (not multiple of 4) silently drops the trailing byte
  (chunks_exact behaviour).
* `from_raw_rgb8` likewise drops trailing partial chunk.
* Maximum reasonable size: skip; not worth allocating for a test.

#### Negative
* `Texture::new("does/not/exist.png")` → `TextureError::IoError`.
* `Texture::new(<path to malformed file>)` → `TextureError::ImageError`. Use a
  txt-renamed-png approach or skip the malformed case if no fixture exists; in that case
  do `from_bytes(b"not an image")` instead.
* `from_bytes(b"")` → `TextureError::ImageError`.

#### Error formatting
* `TextureError::Display` and `Debug` produce non-empty strings.

### `material`

API: `Material { diffuse, specular, hardness, texture (pub(crate)) }`,
`MATERIAL_PLACEHOLDER`, `new_textured`, `new_flat`, `set_texture`, `texture()`. Derives
`Clone`.

#### Functional
* `MATERIAL_PLACEHOLDER` has diffuse=WHITE, specular=WHITE, hardness=20, texture=None.
* `new_flat(RED)` sets diffuse=RED, specular=WHITE, hardness=20, texture=None.
* `new_textured(tex)` sets diffuse=WHITE, specular=WHITE, hardness=20, texture=Some.
* `set_texture(other)` overwrites the texture and `texture()` returns the new one.
* `texture()` returns None on a flat material.

#### Field mutation
* Direct writes to `diffuse`, `specular`, `hardness` are visible through getters.

#### Clone via Rc
* After `let m2 = m1.clone();`, `Rc::strong_count(&m1.texture().unwrap()) == 2`.

#### Edge values
* `m.hardness = f32::MAX` doesn't panic.
* `m.hardness = -1.0` doesn't panic.
* `m.hardness = f32::NAN` doesn't panic.

### `mesh` (`pub(crate)`)

API: `Mesh::new()`, `Mesh::new_with_material(Material)`, fields are `pub(crate)`:
`material`, `positions`, `normals`, `tex_coords`, `indices`, `name`, `tri_count`.

#### Functional
* `Mesh::new()` produces empty buffers, `name == ""`, `tri_count == 0`,
  `material` is `MATERIAL_PLACEHOLDER` (assert the relevant fields).
* `Mesh::new_with_material(new_flat(RED))` carries that material into the field.

#### Field mutation
* Pushing into each buffer is independent; growing `positions` doesn't grow others.

(Mesh has very little behaviour; ~3-5 tests is appropriate.)

### `model`

API: `Model { meshes (pub(crate)), name (pub(crate)), tri_count (pub(crate)) }`,
`Model::new(name)`, `Model::from_mesh(mesh, name)`, `Model::add_mesh(mesh)`, `name()`,
`mesh_info()`, `set_mesh_material(i, m)`, `set_all_material(m)`.

#### Functional
* `Model::new("foo")` — empty meshes, `tri_count == 0`, `name() == "foo"`.
* `Model::from_mesh(m, "foo")` — meshes has length 1, `tri_count == m.tri_count`.
* `add_mesh` accumulates `tri_count` across multiple meshes.
* `name()` returns the constructor name.
* `mesh_info()` empty for new model.
* `mesh_info()` after adding two meshes named "a" and "b" returns `{"a": 0, "b": 1}`.
* `mesh_info()` with two meshes named the same returns the later index (later wins).
* `set_mesh_material(0, m)` replaces mesh 0's material.
* `set_all_material(m)` updates every mesh.

#### Edge / negative
* `add_mesh` debug-asserts on UV count != vert count (`#[should_panic]`).
* `add_mesh` debug-asserts on normal count != vert count (`#[should_panic]`).
* `set_mesh_material(99, m)` on an empty model panics (out-of-bounds index).

Use `dummy_mesh(...)` helper here. Build well-formed meshes for the happy path and
intentionally-malformed ones for the panic cases.

### `instance`

API: `Instance` (`pub` struct but fields are `pub(crate)`). Builder methods: `pos`,
`pos_xyz`, `rot_x/y/z`, `rot_x/y/z_world`, `scale`, `scale_x/y/z`, `smooth`, `model_mat`,
`handle`. Only constructable via `Scene::add_instance` family.

#### Functional
* Default instance from `Scene::add_instance(mh)` has `pos == V3_ZERO`, `scale == V3_ONE`,
  `rot == Quat::ident()`, `smooth == true`.
* `pos_xyz(1, 2, 3)` then read via `model_mat()` confirms translation.
* `scale(2.0)` sets all three axes; `model_mat()` shows 2× scale on the diagonal.
* `scale_x(2.0)` only changes x; other axes remain 1.
* `smooth(false)` toggles.
* `handle()` returns the same handle the engine returned when adding.

#### Rotations
* Two `rot_x(0.5)` calls accumulate (compare to single `rot_x(1.0)`).
* `rot_x` vs `rot_x_world` produce different `model_mat` after a preceding `rot_y(0.5)`.
* `rot_z(0.0)` is a no-op.

#### model_mat composition
* With pos=(1,2,3), no rotation, scale=2 — `model_mat()` equals
  `Mat4::new_scale_rot_trans(2, 2, 2, ident, 1, 2, 3)`.

### `camera`

API: `Camera::new_perspective(aspect, pos, look_at, fov, near, far) -> Result<...>`,
`pos()`, `look_at()`, `set_pos`, `set_look_at`. Fields `pers_mat` and `view_mat` are
`pub(crate)`.

#### Functional
* `new_perspective` happy path returns Ok and the matrices are non-default.
* `pos()` / `look_at()` return the constructor inputs.
* `set_pos(p)` updates `view_mat`.
* `set_look_at(la)` updates `view_mat`.
* Setting pos to the same value is a no-op for `pos()` but still recomputes `view_mat`
  (compare with `assert_mat4_near` against the old matrix to confirm).

#### Negative
* `new_perspective` with `near = 0` → `Mat4Error::NearPlaneZero`.
* `new_perspective` with `far <= near` → `Mat4Error::FarNotGreaterThanNear`.

#### Edge
* `new_perspective` with extreme aspect (0.01, 100.0) doesn't panic.
* `new_perspective` with `fov = 179.0` degrees doesn't panic.
* `new_perspective` with `near = 1e-5, far = 1e9` doesn't panic.
* `pos == look_at` (degenerate look-at) — verify whether it produces NaN matrix or
  panics, and lock that behaviour with the test.

### `light`

API: `Light { pos, brightness, colour, atten_linear, atten_quad, is_static, is_dynamic }`,
`Light::new(...)`, `Light::new_default()`. Derives Debug, Clone, Copy.

#### Functional
* `Light::new(...)` sets all fields exactly.
* `Light::new_default()` matches the documented values: pos=V3_ZERO, brightness=1.0,
  colour=WHITE, atten_linear=0.09, atten_quad=0.032, is_static=false, is_dynamic=false.
* `Copy`: assignment leaves the original usable.
* `Clone`: explicit `.clone()` yields equal fields.
* `Debug` produces a non-empty string containing "Light".

### `scene`

Big surface area. Several `pub(crate)` fields including `lights`, `instances`,
`instance_keys`, `light_keys`, `baked_meshes`, and `pub ambient_light`.

API: `Scene::new` / `Scene::default`, `add_light`, `remove_light`, `light_mut`, `light`,
`add_instance`, `add_instance_mut`, `add_instance_world(mh, pos, rot_xyz, scale)`,
`instance_mut`, `instance`, `remove_instance`, `instances`, `instances_mut`,
`add_static(eng, mh, pos, rot_xyz, scale)`, `bake_static_lighting`, `stats(eng)`.

#### Functional
* `Scene::new` and `Scene::default()` produce equivalent state; `ambient_light` is
  `Colour::new(0.008, 0.008, 0.008)`.
* `add_light(l)` returns a handle; `scene.light(h) == l` field-wise.
* `light_mut(h).brightness = 0.5` then `light(h).brightness == 0.5`.
* `remove_light(h)` then `scene.light(h)` panics (or `should_panic`-test it).
* `add_instance(mh)` returns a handle; defaults verified (see instance tests).
* `add_instance_mut(mh)` returns `&mut Instance` for the same handle as `add_instance` would.
* `remove_instance(h)` removes from `instances` slotmap AND from `instance_keys` Vec.
* `instances().count()` matches the slotmap len after adds/removes.

#### add_instance_world quirk
The current implementation creates `Quat::ident()` *inside* the closure and then applies
`rot_x/y/z` from the input. Verify that the resulting model_mat matches the documented
order: scale, then rot X, then rot Y, then rot Z, then translation. The point of the
test is to lock current behaviour, not necessarily what the user *intended*; if the
behaviour is wrong, surface it (don't silently fix it).

#### Static geometry
* `add_static(eng, mh, V3_ZERO, V3_ZERO, V3_ONE)` adds one baked mesh per mesh in the model.
* `add_static` with non-identity pos/scale transforms verts into world space (sample one
  vert and assert via `assert_vec3_near`).
* `bake_static_lighting()` populates `baked_lighting.len() == verts.len()`.
* Re-baking after light changes updates colours (compare two snapshots).

#### Stats
* `stats(eng)` on an empty scene returns `(0, 0, 0, 0)`.
* `stats(eng)` after adding one instance of a cube returns `(1, 0, 0, 12)`.
* `stats(eng)` after adding one instance + one static of the same cube returns
  `(1, 1, 0, 24)`.
* `stats(eng)` after `remove_instance` decreases the tri count.
* `stats(eng)` after multiple lights counts them in slot 2.

#### Edge / negative
* `light(unknown_handle)` panics.
* `instance(unknown_handle)` panics.
* `light_mut(stale_handle_after_remove)` panics.
* Adding 1000 instances doesn't break iteration counts.

### `baked_mesh` (`pub(crate)`)

API: `BakedMesh { material, verts, normals, uvs, indices, baked_lighting }`,
`bake_lighting(lights, ambient)`.

Construction is only done from `Scene::add_static`, so most tests go through `Scene`.
For direct `BakedMesh` tests, construct one manually (it's `pub(crate)` with public
fields to crate code).

#### Functional
* Empty `lights` SlotMap → every entry of `baked_lighting` equals `ambient * material.diffuse`.
* Single static light directly above a single vert with normal +Y → baked_lighting at that
  vert is bright (close to light.colour * material.diffuse + ambient).
* Single dynamic-only light → ignored (only static contributes to baking).
* `is_static` and `is_dynamic` both true → still baked (static-ness is what gates baking).
* Distance attenuation: doubling distance reduces baked contribution.
* Back-facing normal (n · l < 0) → only ambient term remains.
* `baked_lighting.len() == verts.len()` after baking.
* Re-baking clears the old buffer (it doesn't accumulate).

### `buffer` (`pub(crate)`)

API: `Buffer { pixels, depth, w, h }` (all `pub(crate)`), `new`, `clear`, `clear_depth`,
`set_pixel`, `set_pixel_depth`, `fill_rect`, `draw_char`.

#### Functional
* `Buffer::new(10, 5)` — `pixels.len() == 50`, `depth.len() == 50`, all zero.
* `clear(RED)` fills all pixels with `RED.to_packed_0rgb()` and depth with 0.0.
* `clear_depth` leaves pixels alone, fills depth with 0.0.
* `set_pixel(2, 3, RED)` writes the packed RED value at the right linear index
  (`3 * w + 2 == 32` for w=10).
* `set_pixel(99, 99, RED)` on a 10×5 buffer is a no-op (does not panic).
* `set_pixel_depth(x, y, c, z)` only writes when `z > existing depth` (reverse-Z convention).
* Second `set_pixel_depth` with smaller z does nothing.
* `fill_rect(2, 2, 4, 2, RED)` writes the expected packed value to those pixels and
  leaves others untouched.
* `fill_rect` clipped at right/bottom edge: `(8, 4, 100, 100, RED)` on a 10×5 buffer
  writes only the in-bounds tail.
* `draw_char` on a known printable glyph doesn't panic.
* `draw_char` on an unknown glyph (e.g. `'\u{200B}'`) is a no-op.

### `primitives`

API: `new_cube(mat)`, `new_sphere(mat, stacks, sectors)`,
`new_cylinder(mat, sectors, caps: bool)`, `new_cone(mat, sectors, cap: bool)`.
All return `Model`.

#### Cube
* `meshes.len() == 1`, `tri_count == 12`.
* Mesh has 24 positions, 24 normals, 24 uvs, 36 indices.
* Every position has all components in `[-0.5, 0.5]`.
* Every normal is a unit axis: one of (±1, 0, 0), (0, ±1, 0), (0, 0, ±1).
* The front face (first 4 verts) all have normal (0, 0, 1).
* The back face (verts 4-7) all have normal (0, 0, -1).
* Index winding: every triangle is CCW from outside (compute cross product of
  (v1-v0) × (v2-v0) and assert it dots with the face normal positively).
* `model.name() == "cube"`.

#### Sphere
* Default-ish params `new_sphere(mat, 16, 24)`:
  * `vert_count == (16+1)*(24+1)` (= 425).
  * `tri_count == 2*(16-1)*24` (= 720).
  * All normals have length ≈ 1.0 (assert_vec3_near to a freshly normalised copy).
  * All positions have length ≈ 0.5.
  * UVs lie in `[0, 1]`.
* Stacks clamp: `new_sphere(mat, 0, 24)` → behaves as if stacks=2.
* Stacks clamp: `new_sphere(mat, 1, 24)` → behaves as if stacks=2.
* Sectors clamp: `new_sphere(mat, 16, 0)` → behaves as if sectors=3.
* Sectors clamp: `new_sphere(mat, 16, 2)` → behaves as if sectors=3.
* `model.name() == "sphere_16_24"`.

#### Cylinder
* `new_cylinder(mat, 24, true)`:
  * Side vert count = `2 * (24+1)`. Cap verts = `2 * (1 + 24)`. Total = 100.
  * `tri_count == 4 * 24` (= 96).
  * Side normals (first `2*(sectors+1)` verts) all have y ≈ 0 and length ≈ 1.
  * Cap normals: top cap normals == (0, 1, 0); bottom cap normals == (0, -1, 0).
* `new_cylinder(mat, 24, false)`:
  * Vert count = `2 * (24+1)` (= 50).
  * `tri_count == 2 * 24` (= 48).
* Sectors clamp: `new_cylinder(mat, 0, true)` → sectors=3.
* Names: `cylinder_24_capped`, `cylinder_24_open`.

#### Cone
* `new_cone(mat, 24, true)`:
  * Side vert count = `2 * (24+1)`. Cap verts = `1 + 24`. Total = 75.
  * `tri_count == 2 * 24` (= 48).
  * Apex verts (the second ring) all sit at exactly `(0, 0.5, 0)`.
  * Side normals have positive y component (tilted up).
  * Cap normals all equal (0, -1, 0).
* `new_cone(mat, 24, false)`:
  * Vert count = `2 * (24+1)` (= 50).
  * `tri_count == 24`.
* Sectors clamp: `new_cone(mat, 0, true)` → sectors=3.
* Names: `cone_24_capped`, `cone_24_open`.

### `engine` core

API on `Engine`: `new(w, h)`, `size()`, `aspect()`, `tick(dt)`, `time()`, `add_model`,
`model`, `model_mut`, `stats()`, `draw_debug()`. (Plus `start_window`, `stop` which we
skip because they require minifb and a live window.)

#### Functional
* `Engine::new(800, 600)` — `size() == (800, 600)`, `aspect() == 800.0/600.0`,
  `time() == 0.0`.
* `tick(0.016)` — `time()` returns 0.016. `stats()` returns 0 (reset every frame).
* Two `tick(0.016)` calls — `time()` returns 0.032.
* `tick(0.0)` — `time()` unchanged. The fps averager is not updated for dt=0 (it returns
  early because of `if dt > 0.0`).
* `add_model(Model::new("foo"))` returns a handle; `model(h).name() == "foo"`.
* `model_mut(h).set_all_material(new_flat(RED))` is visible through `model(h)`.
* `draw_debug()` doesn't panic on a freshly created engine.

#### Edge / negative
* `Engine::new(0, 0)` — verify documented behaviour (probably allowed; produces an empty
  buffer). Lock it with a test.
* `model(stale_handle)` panics (existing behaviour: `unwrap`).
* `model_mut(stale_handle)` panics.

### `engine::draw2d`

API: `clear`, `draw_string`, `draw_rect`, `draw_line`. (All operate on the buffer.)

#### Functional (state-only)
* `clear(RED)` then `engine.buffer_content()[0] == RED.to_packed_0rgb()`.
* `clear(BLACK)` then the last pixel equals 0.
* `draw_rect(2, 2, 4, 2, RED)` updates the expected pixels (sample two: one inside, one
  outside).
* `draw_rect(8, 4, 100, 100, RED)` on a 10×5 buffer is clipped (no panic).
* `draw_line(0, 0, 9, 4, RED)` doesn't panic.
* `draw_line(9, 4, 0, 0, RED)` (swapped endpoints) doesn't panic.
* `draw_string("hi", 0, 0, RED)` doesn't panic. (Cannot easily verify glyph pixels
  without baking text knowledge into the test; smoke is fine.)

### `helpers` (`pub(crate)`)

API: `compute_outcode(&Vec4)`, `shade_vert(lights, world, n, eye, hardness)`,
`shade_vert_diffuse(lights, world, n)`, `FpsAveragerEight::new`/`add_fps`/`avg_fps`,
clip-bit constants `OUT_LEFT`/`OUT_RIGHT`/`OUT_BOTTOM`/`OUT_TOP`/`OUT_NEAR`/`OUT_FAR`.

#### compute_outcode
* Vec4 strictly inside the frustum → outcode = 0.
* Vec4 with `x < -w` → `OUT_LEFT` only.
* Vec4 with `x > w` → `OUT_RIGHT` only.
* Vec4 with `y < -w` → `OUT_BOTTOM`.
* Vec4 with `y > w` → `OUT_TOP`.
* Vec4 with `z < 0` → `OUT_FAR`. (Note: reverse Z convention.)
* Vec4 with `z > w` → `OUT_NEAR`.
* Corner case: `x < -w` AND `y < -w` → both bits set.

#### shade_vert
* Empty lights → returns (BLACK, BLACK).
* Single static-at-vert light, normal facing the light → high diffuse, high spec.
* Single light behind the normal (n · l < 0) → zero diffuse, zero spec.
* Doubling distance reduces diffuse via attenuation.
* `hardness` controls highlight tightness: higher hardness → smaller spec.

#### shade_vert_diffuse
* Only dynamic lights contribute.
* Static-only lights produce zero diffuse.
* Mixed: only the dynamic ones contribute.

#### FpsAveragerEight
* `new()` then `avg_fps() == 0.0` (count == 0).
* `add_fps(60.0)` once → `avg_fps() == 60.0`.
* 8 samples of 60.0 → avg = 60.0.
* 8 samples of (10..80) increments → avg = mean of those 8.
* 9th sample evicts the first: 8 samples of 60.0 then `add_fps(100.0)` →
  avg = (60.0 * 7 + 100.0) / 8.

### `engine::parse_obj`

API: `Engine::load_obj(path) -> Result<ModelHandle, ObjError>`, `parse_mtl`.

#### Functional
* `load_obj(asset_path("assets/obj/icosahedron.obj"))` returns Ok, model has 20 triangles.
* `load_obj` on a textured asset (e.g. `assets/obj/chest/chest.obj`) — model added, mesh
  has a textured material.
* `load_obj` on a multi-mesh asset (e.g. several `assets/obj/dungeon/banner_*.obj`
  loaded successively) — each adds independently, distinct handles.
* `parse_mtl` happy path with a built-in `tobj::Material` fixture (construct directly
  in the test) — diffuse and hardness applied.
* `parse_mtl` with `shininess = None` → hardness defaults to 20.0.
* `parse_mtl` with `diffuse = None` → diffuse defaults to WHITE.

#### Negative
* `load_obj("does/not/exist.obj")` → `ObjError::Load`.
* `load_obj` on a non-obj file (e.g. a `.png`) → `ObjError::Load`.

#### Quirks
* UVs are V-flipped at parse time (`1.0 - v`). Verify by loading a small fixture with
  known UVs if possible, or asserting that a `(0, 1)` MTL becomes `(0, 0)` in the mesh.
* If MTL file is missing, parser logs and continues with `Vec::new()` materials —
  meshes get the placeholder material. Verify with a hand-crafted OBJ that references
  a missing MTL.

### `engine::parse_gltf`

API: `Engine::load_gltf(path) -> Result<ModelHandle, gltf::Error>`. Internal `GltfData`
and `parse_material` are file-local.

#### Functional
* `load_gltf(asset_path("assets/gltf/duck.glb"))` returns Ok; model is added.
* The duck model has `tri_count > 0` and one mesh (LOD3spShape with one primitive in the
  source file).
* `load_gltf` on each teapot LOD; assert `low.tri_count < med.tri_count < high.tri_count`.
* `load_gltf` on `assets/gltf/potion/bottle_A_labeled_green.gltf` — loads successfully,
  texture is parsed and set on the material.

#### Material derivation
The duck declares `roughnessFactor ≈ 0.198` and a base colour of `[1,1,1,1]`. After
parsing through the PBR-to-Phong mapping:
* `hardness ≈ 41.32` (within the expected `1..=64` clamp).
* `specular ≈ (0.96, 0.96, 0.96)` (because `spec_strength = 1 - r*r ≈ 0.96` and the
  duck is non-metallic).
* `diffuse == WHITE` (from base_color_factor `[1,1,1,1]`).

A separate lock-down test covers the default-roughness case (see "glTF default
roughness behaviour" in the gotchas section) using a tiny in-memory GLB that omits
`pbrMetallicRoughness` entirely.

Test by inspecting the model's first mesh material after load. Use `eng.model(h)`
and reach into `meshes[0].material`.

#### Negative
* `load_gltf("does/not/exist.glb")` → `Err(gltf::Error)`.
* `load_gltf` on a non-glb file → `Err`.

### `math` audit checklist

The existing `math/*_tests.rs` are largely good. The audit phase adds these scenarios
to the relevant existing file. Group new tests under a `// --- Edge cases (audit) ---`
banner near the bottom so reviewers can see what was added.

#### `vector3_tests.rs`
* `Vec3::new(NaN, NaN, NaN)` constructs and `normalize_new()` produces NaN.
* `Vec3::zero().normalize_new()` — document current behaviour (likely NaN; lock with test).
* `Vec3::new(INF, 0, 0).len()` returns inf.
* `cross` anti-commutativity: `a.cross(b) == -b.cross(a)`.
* `cross` parallel inputs: `(1,0,0).cross((2,0,0))` is zero vector.
* `dot` with NaN propagates.
* `Display` format `"[x, y, z]"` shape.

#### `vector2_tests.rs`, `vector4_tests.rs`
* Same NaN/inf checks per type.
* `Display` format.

#### `matrix4_tests.rs`
* `new_perspective` errors:
  * `Mat4::new_perspective(1.0, 0.0_f32.to_radians(), 0.0, 100.0)` → `NearPlaneZero`.
  * `Mat4::new_perspective(1.0, 1.0, 1.0, 0.5)` → `FarNotGreaterThanNear`.
  * `Mat4::new_perspective(1.0, 1.0, 1.0, 1.0)` → `FarNotGreaterThanNear`.
* `new_perspective` extremes: tiny aspect, huge fov — no panic.
* `new_look_at` with collinear pos/look_at/up — lock current behaviour.
* `inverse` on a known-singular matrix (e.g. scale by 0) — verify Option/Result/panic.
* Round-trip: `mat.inverse() * mat ≈ identity` for a few well-conditioned matrices.

#### `quat_tests.rs`
* Repeated rotation drift: 10,000 `rot_x(0.001)` calls and verify the quat is still
  ≈ unit length (within 1e-4).
* `rot_x` vs `rot_x_world` differ after a preceding `rot_y(0.5)`.
* `Quat::ident()` rotates a vector to itself.
* Composition: rotating by `pi` around X twice ≈ identity rotation on a test vector.

#### `affine2_tests.rs`
* Inverse round-trip on a non-degenerate Affine2.
* Composition order matches matrix multiplication convention.
* Identity Affine2 maps every point to itself.

#### `matrix3_tests.rs`
* `from_mat4_upper` of a Mat4 with pure translation produces identity Mat3.
* `from_mat4_upper` of a Mat4 with scale (2,3,4) and zero translation produces the
  diagonal Mat3.
* `inverse_transpose` round-trip with a uniform scale matrix (should be 1/scale on the
  diagonal).
* `inverse_transpose` of a singular matrix returns the documented value
  (`unwrap_or_default()` is used at the call site, so it likely returns `Option<Mat3>`).
  Lock it.

---

## Gotchas, traps and quirks

A non-exhaustive list of things that will bite a test author. Read this before writing
tests; many of these have caused real bugs in the engine.

### Asset path directory
The directory is `assets/gltf/`. (Earlier drafts of this doc said `assets/glft/`.)

### glTF default roughness behaviour
The glTF 2.0 spec defines `pbrMetallicRoughness.roughnessFactor` to default to `1.0`
when omitted. Our PBR-to-Phong mapping then produces `hardness = 1.0` and
`spec_strength = 0.0` (so `specular = BLACK`). This is spec-compliant matte
behaviour, not a bug. It is **not** the duck: `duck.glb` declares
`roughnessFactor ≈ 0.198` and produces `hardness ≈ 41.32`, `specular ≈ (0.96, 0.96, 0.96)`.
The default case is locked down by `test_default_roughness_produces_hardness_one`,
`test_default_roughness_produces_zero_specular`, and
`test_default_roughness_diffuse_is_white` in `parse_gltf_tests`, using a tiny
in-memory GLB whose material omits `pbrMetallicRoughness` entirely.

### `Model::add_mesh` debug-asserts on buffer length parity
UVs and normals must match position count, otherwise the debug build panics. Tests that
build a malformed mesh and pass it to `add_mesh` need `#[should_panic]`.

### OBJ parser V-flips UVs
The OBJ parser writes `(u, 1.0 - v)` into the mesh's tex_coords. Don't be surprised by
flipped V in OBJ tests.

### `Texture::sample` floor-wraps
`sample(u, v)` does `u - u.floor()` so negative inputs wrap correctly. UVs that hit
exactly 1.0 fold to 0.0. Take this into account when picking sample coordinates.

### `Buffer::set_pixel` silently bounds-checks
Out-of-bounds writes are no-ops. `set_pixel_depth` does NOT bounds-check.

### Reverse-Z depth convention
The depth buffer starts at 0.0 and "nearer" is larger. `set_pixel_depth` writes when
`z > existing`. Tests that exercise the depth path need to use this convention or they
will silently mis-test.

### `Engine::tick(0.0)` short-circuits the FPS averager
If `dt == 0.0`, fps is not added to the averager. This is the safe-divide guard; the test
suite should not assume `tick(0.0)` updates fps stats.

### `add_instance_world` rotation bug
The current implementation creates `Quat::ident()` inside the closure and then mutates it
with `rot_x/y/z(rot.x/y/z)`. This sequence applies rotations in X-Y-Z order around the
quat's *local* axes after each preceding rotation. Verify behaviour, don't assume
intuition. If you find a real bug, write the test to lock current behaviour AND open an
issue (see "out of scope" in plan.md).

### `Scene::remove_*` is O(n) over `instance_keys` / `light_keys`
Just a perf note — not a correctness one. Don't write timing-sensitive tests.

### `Colour` doesn't clamp on arithmetic
`WHITE * 2.0` has component value 2.0. Only `to_packed_0rgb` clamps to `[0, 1]`. Test
both behaviours.

### `Material::clone` shares the texture via Rc
Two cloned materials point at the same texture; mutating the underlying texture would
be visible from both. Tests that rely on independent textures must not clone.

### `desktop` feature
By default the crate builds with `desktop` enabled. The `engine::input` and
`Engine::start_window`/`stop` are gated behind it. Tests should not require the desktop
feature unless they specifically test desktop-only behaviour. The bulk of the suite
must compile with `--no-default-features`.

### `gltf` typo in directory
Mentioned above; mentioned again because it will be the #1 cause of asset-not-found
errors in tests written from memory.

### Always include `#[cfg(test)]`-only constructors behind the cfg gate
Never add a `pub(crate) fn new_for_test(...)` without the `#[cfg(test)]` attribute.

### Test files must use `super::*;` or absolute paths
Because tests are loaded via `#[path]` into a `#[cfg(test)] mod` inside the parent module,
`use super::*;` reaches the parent module's items. For things outside the parent module
(e.g. accessing `crate::engine::Engine` from `light_tests.rs`), use absolute paths.

### Some tests will need both `Engine` and `Scene`
There is no "Engine creates a Scene" API; the user creates them separately. Tests that
need both should construct both in one go.

---

## Quick reference: writing a new test file

1. Add the `#[cfg(test)] #[path = "..."] mod ...;` declaration to the parent source file.
2. Create the `_tests.rs` file with the standard header banner.
3. Start with `use super::*;` and any extra imports needed.
4. Add `use crate::test_helpers::*;` if you'll be loading assets or asserting near.
5. Group tests under `// --- Section ---` banners.
6. Run `cargo test --lib <module_name>` to verify just that file.
7. Run `cargo test --lib` after each module to keep the whole suite green.

The full command for a focused TDD loop:

```bash
cargo test --lib texture_tests -- --nocapture
```

---

## Out of scope (do not write tests for these)

* Pixel comparisons / golden images / framebuffer hashing.
* Property-based / fuzz / random tests (we use example-based throughout).
* Performance / benchmarks.
* `start_window`, `stop`, the `Inputs` desktop subsystem (these need a live window).
* CI workflow changes (handled separately by the user).
* The `text` glyph table internals (we treat `draw_char` as a black box).

If a test you'd write falls in one of the above buckets, skip it. If you find yourself
needing to test something not described in this doc, add a test under a clear banner and
update this doc in the same PR.
