# Killing the `&dyn Texture` overhead in the rasteriser

Notes on why `&dyn Texture` is costing us per-pixel performance in `fill_triangle`,
and what to do about it.

## What `&dyn Texture` is at the CPU level

When you write:

```rust
let tex: &dyn Texture = mat.texture.as_ref();
tex.sample(u, v);
```

`&dyn Texture` is a **fat pointer**, two words wide:

```
&dyn Texture
+--------------------+
| data pointer       |  --> the actual SimpleColourTexture / ImageTexture struct
+--------------------+
| vtable pointer     |  --> [size, align, drop_fn, sample_fn, ...]
+--------------------+
```

Calling `tex.sample(u, v)` becomes, roughly:

```
mov  rax, [tex_vtable_ptr]   ; load vtable address
mov  rcx, [rax + offset]     ; load sample fn pointer from vtable
mov  rdi, [tex_data_ptr]     ; pass &self
call rcx                     ; indirect call through function pointer
```

The indirect call itself isn't the killer (modern CPUs predict these reasonably
well once the loop is warm). **The killer is what the compiler can't do because
of it.**

## Why this is worse than it looks

When `sample` is behind a vtable, the optimiser sees an opaque function call in
the middle of the hot loop. So:

1. **No inlining.** `SimpleColourTexture::sample` is literally "read a struct
   field". If inlined into the loop, the compiler would notice `self.colour`
   is loop-invariant and hoist it **out of the inner loop entirely**, making
   texture sampling free. With `&dyn`, you pay the function call cost **per
   pixel**. For flat-coloured triangles this is pure overhead.

2. **Register state must be preserved across the call.** Caller-saved registers
   get spilled before the call and reloaded after, choking edge-function
   stepping and barycentric maths.

3. **Autovectorisation is dead.** The compiler will never SIMD across pixels if
   it can't see what happens inside the per-pixel call.

4. **Inlining is transitive.** `ImageTexture::sample` calls
   `self.image.get_pixel(...)`. `DynamicImage::get_pixel` is itself an enum
   dispatch over pixel formats, so we have **two layers of dispatch per pixel**.
   Once inlining stops at the outer boundary, none of the inner stuff can be
   specialised either.

## What monomorphisation buys you

Generics in Rust are **monomorphised**: the compiler stamps out a separate copy
of the function per concrete type used. Same source, multiple specialised
binaries.

```rust
pub fn fill_triangle<T: Texture>(&mut self, v0: ScreenVert, ..., tex: &T, smooth: bool) {
    // tex.sample(u, v) is now a STATIC dispatch -> inlinable
}
```

The compiler can inline, see through, hoist invariants, the lot.

## The catch

`Material` stores `Box<dyn Texture>`, which is the right design choice for the
public API (a material can hold any texture type, decided at runtime). At the
rasteriser call site we still only have `&dyn Texture`. The trick is to convert
dyn to concrete **once per triangle**, not per pixel.

### Pattern 1: Enum dispatch (good)

Replace the trait + Box with a concrete enum:

```rust
pub enum Texture {
    Solid(Colour),
    Image(ImageTexture),
}

impl Texture {
    #[inline(always)]
    pub fn sample(&self, u: f64, v: f64) -> Colour {
        match self {
            Texture::Solid(c) => *c,
            Texture::Image(img) => img.sample_direct(u, v),
        }
    }
}
```

Now `Material.texture: Texture`. The match is still per-pixel inside
`fill_triangle`, but because everything is concrete and `#[inline(always)]` is
there, the compiler can:

- Inline `sample()` into the loop
- See both arms and optimise each
- The match is predicted perfectly within a triangle anyway

Big win vs `&dyn`. Crucially this closes the API: users can't add their own
texture types. For a personal renderer, totally fine.

### Pattern 2: Enum + per-triangle dispatch + generic inner loop (best)

Hoist the texture-type decision **out of the per-pixel loop entirely**:

```rust
pub trait Sampler {
    fn sample(&self, u: f64, v: f64) -> Colour;
}

impl Sampler for Colour {
    #[inline(always)] fn sample(&self, _u: f64, _v: f64) -> Colour { *self }
}
impl Sampler for ImageTexture {
    #[inline(always)] fn sample(&self, u: f64, v: f64) -> Colour { /* direct */ }
}

impl Buffer {
    fn fill_triangle_inner<S: Sampler>(&mut self, v0: ScreenVert, ..., s: &S, smooth: bool) {
        // inner loop: s.sample(u, v) is a direct, inlinable call
        // compiler makes a dedicated version per S
    }

    pub fn fill_triangle(&mut self, v0: ScreenVert, ..., tex: &Texture, smooth: bool) {
        match tex {
            Texture::Solid(c)   => self.fill_triangle_inner(v0, .., c, smooth),
            Texture::Image(img) => self.fill_triangle_inner(v0, .., img, smooth),
        }
    }
}
```

One branch per triangle, then jump into a fully specialised rasteriser tuned
for that texture type. For `Solid`, the optimiser will likely lift the sample
call entirely out of the inner loop.

This is the trick GPU drivers and serious software renderers use, just done at
compile time via generics instead of runtime via shader recompilation.

## The other elephant: `ImageTexture::sample` is slow on its own

```rust
fn sample(&self, u: f64, v: f64) -> Colour {
    let x = u * self.w as f64;
    let y = v * self.h as f64;
    let pix = self.image.get_pixel(x as u32, y as u32);     // enum dispatch + bounds check
    Colour::from_rgb8(pix.0[0], pix.0[1], pix.0[2])         // 3 u8->f64 + 3 divides by 255
}
```

Problems:

1. **`DynamicImage::get_pixel`** is generic over pixel format internally. Even
   after inlining `sample`, this layer is enum-dispatch and does its own bounds
   checking.
2. **No wrap mode.** `u=1.0` or `u<0.0` will panic from out-of-bounds.
3. **`from_rgb8` divides three f64s by 255** for every texel.

Flatten the texture once at load time:

```rust
pub struct ImageTexture {
    pixels: Vec<u32>,   // packed 0RGB to match buffer format
    w: u32,
    h: u32,
    w_mask: u32,        // if power-of-two, free wrap with bitwise AND
    h_mask: u32,
}

impl ImageTexture {
    pub fn new(path: &str) -> Result<Self, TextureError> {
        let img = ImageReader::open(path)?.decode()?.to_rgb8();
        let (w, h) = img.dimensions();
        let pixels = img.pixels()
            .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32))
            .collect();
        Ok(Self { pixels, w, h, w_mask: w - 1, h_mask: h - 1 })
    }

    #[inline(always)]
    pub fn sample_direct(&self, u: f64, v: f64) -> Colour {
        let x = ((u * self.w as f64) as i32 as u32) & self.w_mask;
        let y = ((v * self.h as f64) as i32 as u32) & self.h_mask;
        let p = unsafe { *self.pixels.get_unchecked((y * self.w + x) as usize) };
        Colour::from_packed_0rgb(p)
    }
}
```

Caveats:

- **Power-of-two assumption** for the `& mask` wrap. Relax with `% w` (slower)
  or `clamp` (different visual behaviour) for arbitrary sizes.
- **`get_unchecked`** skips bounds checking. Safe only because `(x, y)` are
  masked into range. Test with bounds checking first, drop them after.
- **Pre-converted to packed `u32`** matching the buffer format. No divides.

## Realistic expected wins

At 1080p on a textured scene:

| Change | Approximate speedup of texturing path |
|---|---|
| Just enum-dispatch (Pattern 1) | 1.3 to 1.6x |
| Enum + generic inner (Pattern 2) | 1.8 to 2.5x |
| Flat `Vec<u32>` ImageTexture | 2 to 4x on top of above |
| All combined | 3 to 6x texturing path |

Whole-frame speedup will be less dramatic because of non-texturing work (edge
functions, shading, depth test). Realistically 1.5 to 2x on `render_instance`,
which on a CPU rasteriser is the difference between 30 and 60 FPS.

## Verify with a profiler

```bash
cargo install flamegraph
cargo flamegraph --bin <your_demo>
```

Look for `sample` and `get_pixel` in the flamegraph. If chunky, the analysis
above applies. If thin, the bottleneck is elsewhere (edge function stepping,
depth test, inner branch).

Or quick-and-dirty: render a scene full of large flat-coloured quads, time it
with `dyn` vs Pattern 2. If Solid texturing gets dramatically faster, the
compiler is doing the loop-invariant hoist predicted above, and the diagnosis
is confirmed.

## TL;DR

1. Make `Texture` an enum, not a trait. Drop `Box<dyn Texture>` in `Material`.
2. Either accept per-pixel match (Pattern 1, simplest) or hoist with generics
   (Pattern 2, fastest).
3. Rewrite `ImageTexture` to hold a flat `Vec<u32>` with direct indexed access
   and packed colour conversion. Probably the single biggest win.
4. Profile before and after on a real scene.

Start with step 3 alone (rewrite `ImageTexture::sample`) since it's lowest risk
and biggest win, then assess whether step 1+2 is worth the API churn.
