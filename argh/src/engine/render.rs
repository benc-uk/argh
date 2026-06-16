// ==============================================================================================
// Module & file:   engine / render.rs
// Purpose:         Core rendering methods for 3D meshes and instances
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use slotmap::SlotMap;

use crate::{
  baked_mesh::BakedMesh,
  buffer::Buffer,
  camera::Camera,
  colour::{BLACK, Colour},
  engine::LightHandle,
  helpers::{OUT_NEAR, compute_outcode},
  light::Light,
  material::{BlendMode, Material},
  math::{Mat3, Mat4, Vec2, Vec3, Vec4},
  scene::Scene,
};

use super::{Engine, InstanceHandle};

const TRI_AREA_EPS: f32 = 1e-8;

// ProcessedVert collects various data from the processing/transformation of mesh vertex in the rendering pass
// It holds data ready for rasterization, shading etc
pub(super) struct ProcessedVert {
  clip: Vec4, // Pre-divide clip space position, used for near-plane clipping
  screen: ScreenVert,
  outcode: u8,
  uv: Vec2, // Raw UV only used by fast path
}

// ClipVert is the per-vertex payload that flows through near-plane clipping.
// It carries everything needed to (a) interpolate at the clip boundary, and
// (b) reconstruct a ScreenVert + lighting afterwards. All attributes get
// lerped together with the same `t` so the clipped vertex is self-consistent.
#[derive(Copy, Clone)]
struct ClipVert {
  clip: Vec4,
  uv: Vec2,
  light: Colour, // pre-computed per-vertex lighting (Gouraud), carried through interp
}

impl ClipVert {
  // A standard lerp (t: 0~1) between two ClipVert which blends all values inside
  #[inline(always)]
  fn lerp(&self, other: &Self, t: f32) -> Self {
    let inv_t = 1.0 - t;
    Self {
      clip: self.clip * inv_t + other.clip * t,
      uv: self.uv * inv_t + other.uv * t,
      light: self.light * inv_t + other.light * t,
    }
  }
}

impl From<&ProcessedVert> for ClipVert {
  fn from(vert: &ProcessedVert) -> Self {
    Self {
      clip: vert.clip,
      uv: vert.uv,
      light: vert.screen.light,
    }
  }
}

// Sutherland-Hodgman clip of a single triangle against the near plane.
// Output is written into `out` and length (0, 3 or 4) returned.
// This is a very hard algorithm to understand see: misc/clip_triangle_near.png for a visual guide
fn clip_triangle_near(tri: &[ClipVert; 3], out: &mut [ClipVert; 4]) -> usize {
  // Distance from the near clip plane
  let d = [tri[0].clip.w - tri[0].clip.z, tri[1].clip.w - tri[1].clip.z, tri[2].clip.w - tri[2].clip.z];
  // We use reverse-Z, so near plane sits at clip.z = clip.w
  let inside = [d[0] >= 0.0, d[1] >= 0.0, d[2] >= 0.0];

  let mut count = 0usize;
  for i in 0..3 {
    let j = (i + 1) % 3;
    // Check edges, with verts i and j
    match (inside[i], inside[j]) {
      // Both verts are inside: copy the j vert
      (true, true) => {
        out[count] = tri[j];
        count += 1;
      }

      // First vert inside: add a new vert, lerped between i & j
      (true, false) => {
        let t = d[i] / (d[i] - d[j]);
        out[count] = tri[i].lerp(&tri[j], t);
        count += 1;
      }

      // Second vert inside: add two new verts, 1) lerped between i & j, 2) copy of j
      (false, true) => {
        let t = d[i] / (d[i] - d[j]);
        out[count] = tri[i].lerp(&tri[j], t);
        count += 1;
        out[count] = tri[j];
        count += 1;
      }

      // Both verts outside
      (false, false) => {}
    }
  }
  count
}

// This is used internally to represent a vertex transformed into screen space (after perspective divide)
// It's a hybrid of x & Y being screen pixel values, and z being a float representing depth in 0-1 range
#[derive(Copy, Clone)]
struct ScreenVert {
  x: f32,        // pixel coordinate, [0, width]
  y: f32,        // pixel coordinate, [0, height], origin top-left
  z: f32,        // NDC depth [0, +1] (D3D/Vulkan/WebGPU convention, near=0, far=+1)
  light: Colour, // Lighting per vertex (for Gouraud shading)
  inv_w: f32,    // Inverse of w
  u_w: f32,      // PRE-DIVIDED, not raw u and v.
  v_w: f32,      // PRE-DIVIDED, not raw u and v.
}

impl ScreenVert {
  // Finish a ClipVert into a ScreenVert by doing the perspective divide and
  // viewport mapping (origin top-left, Y flipped). Mirrors the per-vertex code
  // in the vertex-processing loop so screen-space results match for unchanged verts.
  #[inline(always)]
  fn from_clip(cv: &ClipVert, size: (usize, usize)) -> Self {
    let inv_w = 1.0 / cv.clip.w;
    let ndc_x = cv.clip.x * inv_w;
    let ndc_y = cv.clip.y * inv_w;
    let ndc_z = cv.clip.z * inv_w;

    Self {
      x: (ndc_x * 0.5 + 0.5) * size.0 as f32,
      y: (1.0 - (ndc_y * 0.5 + 0.5)) * size.1 as f32,
      z: ndc_z,
      inv_w,
      light: cv.light,
      u_w: cv.uv.x * inv_w,
      v_w: cv.uv.y * inv_w,
    }
  }
}

impl Engine {
  /// Set the colour of a single pixel in the frame buffer, this is the bedrock of the system
  /// # Arguments
  /// * `x` - X position of pixel
  /// * `y` - Y position of pixel
  /// * `colour` - New colour of the pixel
  #[inline(always)]
  pub fn set_pixel(&mut self, x: usize, y: usize, colour: Colour) {
    self.buffer.set_pixel(x, y, colour);
  }

  /// Access the current pixels in the frame buffer
  pub fn buffer_content(&self) -> &[u32] {
    &self.buffer.pixels
  }

  /// Used by WASM only to re-encode/pack the internal frame buffer as RGBA (rather than ARGB)
  /// Output array should be pre-allocated and sized (W * H * 4)
  pub fn buffer_copy_bytes(&self, out: &mut [u8]) {
    debug_assert_eq!(out.len(), self.buffer.pixels.len() * 4);

    for (chunk, &p) in out.chunks_exact_mut(4).zip(&self.buffer.pixels) {
      let [_, r, g, b] = p.to_be_bytes();
      chunk[0] = r;
      chunk[1] = g;
      chunk[2] = b;
      chunk[3] = 0xFF;
    }
  }

  /// Render a [Scene] from given [Camera], this is the most common thing to call inside your [App][crate::app::App] update
  pub fn render(&mut self, cam: &Camera, scn: &Scene) {
    // render all static stuff
    for mesh in &scn.baked_meshes {
      self.render_static(mesh, cam.pers_mat * cam.view_mat, scn, cam.pos());
    }

    // render all dynamic instances
    for handle in 0..scn.instance_keys.len() {
      let hdl = scn.instance_keys[handle];
      self.render_instance(hdl, cam, scn);
    }
  }

  /// Renders an Instance (ref by [InstanceHandle]) onto the screen from given camera position
  /// This starts the full rendering pipeline
  pub fn render_instance(&mut self, hdl: InstanceHandle, cam: &Camera, scn: &Scene) {
    // Shorthand for finding the instance
    let Some(instance) = scn.instances.get(hdl) else {
      return;
    };

    // We unwrap here, as model existence is checked when instance is created
    let model = self.models.get(instance.model_handle).unwrap();

    // 0. Get the matrices we need, model and inverse transpose
    let m = instance.model_mat();
    // Inverse transpose of the model matrix in a Mat3 for normals
    let m_inv_t = Mat3::from_mat4_upper(&m).inverse_transpose().unwrap_or_default();

    // 1. Combine MVP (model, view, perspective) matrix
    let mvp = cam.pers_mat * cam.view_mat * m; // full transforms

    // 2. Model is made of meshes, iterate over them
    for mesh in model.meshes.iter() {
      // Clear vert vector cache
      self.verts.clear();

      // 3. Main vertex processing loop, does just about everything
      // Prepares both processed verts and screen vert
      for i in 0..mesh.positions.len() {
        let pos = &mesh.positions[i];
        let world = m.transform_point(&mesh.positions[i]);
        let clip = mvp * &Vec4::new(pos.x, pos.y, pos.z, 1.0);
        let normal = (m_inv_t * &mesh.normals[i]).normalize_new();
        let uv = mesh.tex_coords[i];

        // We shade early, probably faster than after back face culling due to tris sharing verts
        let (d, s) = shade_vert(&scn.lights, world, normal, cam.pos(), mesh.material.hardness);
        let light = d * mesh.material.diffuse + s * mesh.material.specular + scn.ambient_light;

        let inv_w = 1.0 / clip.w;
        let ndc_x = clip.x * inv_w;
        let ndc_y = clip.y * inv_w;
        let ndc_z = clip.z * inv_w;

        self.verts.push(ProcessedVert {
          clip,
          uv,
          screen: ScreenVert {
            x: (ndc_x * 0.5 + 0.5) * self.size.0 as f32,
            y: (1.0 - (ndc_y * 0.5 + 0.5)) * self.size.1 as f32,
            z: ndc_z,
            inv_w,
            light,
            u_w: uv.x * inv_w,
            v_w: uv.y * inv_w,
          },
          outcode: compute_outcode(&clip),
        });
      }

      // 4. Now to rendering triangles
      // Walk the index list 3 at a time for triangles, cull, shade & rasterize
      for tri in mesh.indices.chunks(3) {
        render_tri(tri, &mesh.material, &self.verts, &mut self.buffer, self.size);
        self.stat_rend_tri_frame += 1;
      }
    }
  }

  // Renders a static [BakedMesh] onto the screen from given camera position
  // This starts the full rendering pipeline. Note not public
  fn render_static(&mut self, mesh: &BakedMesh, vp: Mat4, scn: &Scene, eye: Vec3) {
    // Prevents a panic but very likely result in absolutely nothing being rendered
    if mesh.lighting.is_empty() {
      return;
    }

    self.verts.clear();

    // Main vertex processing loop, does just about everything
    // Prepares both processed verts and screen vert
    for i in 0..mesh.positions.len() {
      let world = mesh.positions[i]; // Pretransformed
      let clip = vp * &Vec4::new(world.x, world.y, world.z, 1.0);
      let uv = mesh.tex_coords[i];

      // Dynamic lights also get lighting from dynamic lights too
      let (d, s) = shade_vert(&scn.lights, world, mesh.normals[i], eye, mesh.material.hardness);
      let light = d * mesh.material.diffuse + s * mesh.material.specular + scn.ambient_light;

      let inv_w = 1.0 / clip.w;
      let ndc_x = clip.x * inv_w;
      let ndc_y = clip.y * inv_w;
      let ndc_z = clip.z * inv_w;

      self.verts.push(ProcessedVert {
        clip,
        uv,
        screen: ScreenVert {
          x: (ndc_x * 0.5 + 0.5) * self.size.0 as f32,
          y: (1.0 - (ndc_y * 0.5 + 0.5)) * self.size.1 as f32,
          z: ndc_z,
          inv_w,
          light: mesh.lighting[i] + light,
          u_w: uv.x * inv_w,
          v_w: uv.y * inv_w,
        },
        outcode: compute_outcode(&clip),
      });
    }

    // 3. Walk the index list 3 at a time for triangles, pass down to shared render_tri()
    for tri in mesh.indices.chunks(3) {
      render_tri(tri, &mesh.material, &self.verts, &mut self.buffer, self.size);
      self.stat_rend_tri_frame += 1;
    }
  }
}

// ===== Standalone functions ===========

// Standard edge function, it's basically a cross product
#[inline(always)]
fn edge_function(a: ScreenVert, b: ScreenVert, px: f32, py: f32) -> f32 {
  (b.x - a.x) * (py - a.y) - (b.y - a.y) * (px - a.x)
}

// Back-face cull. We use Y-flipped screen space, the signed area test is inverted from OpenGL
// Back faces (mesh CW or back of CCW) have POSITIVE area; So we discard anything non-negative.
#[inline(always)]
fn is_back_facing(sv0: &ScreenVert, sv1: &ScreenVert, sv2: &ScreenVert) -> bool {
  let area = (sv1.x - sv0.x) * (sv2.y - sv0.y) - (sv1.y - sv0.y) * (sv2.x - sv0.x);
  area >= 0.0
}

// Shared render path for instances & statics - process triangles deal with clipping etc the call rasterize_tri
fn render_tri(tri: &[u32], mat: &Material, verts: &[ProcessedVert], buffer: &mut Buffer, size: (usize, usize)) {
  let i0 = tri[0] as usize;
  let i1 = tri[1] as usize;
  let i2 = tri[2] as usize;

  // Trivial reject: all three vertices outside the SAME plane if they all share the same bitmask
  let combined_out = verts[i0].outcode & verts[i1].outcode & verts[i2].outcode;
  if combined_out != 0 {
    return;
  }

  // Near-plane handling: if any vert is past the near plane we have to clip the triangle with Sutherland-Hodgman
  // Common case (nothing crossing near) hits the fast path below with clip overhead
  let any_near = (verts[i0].outcode | verts[i1].outcode | verts[i2].outcode) & OUT_NEAR;
  if any_near == 0 {
    // ---- FAST PATH: no clipping needed, screen verts are already valid ----
    rasterize_tri(buffer, verts[i0].screen, verts[i1].screen, verts[i2].screen, mat);
    return;
  }

  // ---- SLOW PATH: near-plane clipping via Sutherland-Hodgman ----
  let cv_in = [ClipVert::from(&verts[i0]), ClipVert::from(&verts[i1]), ClipVert::from(&verts[i2])];
  let mut cv_out = [cv_in[0]; 4];
  let vert_tot = clip_triangle_near(&cv_in, &mut cv_out);

  // Fan out triangles might be 1 or 2 triangles in cv_out
  for ti in 0..(vert_tot - 2) {
    let sv0 = ScreenVert::from_clip(&cv_out[0], size);
    let sv1 = ScreenVert::from_clip(&cv_out[ti + 1], size);
    let sv2 = ScreenVert::from_clip(&cv_out[ti + 2], size);

    rasterize_tri(buffer, sv0, sv1, sv2, mat);
  }
}

// Fill a triangle between three ScreenVertex points in screen space interpolating lighting, z-depth and texture samples
fn rasterize_tri(buff: &mut Buffer, v0: ScreenVert, v1: ScreenVert, v2: ScreenVert, mat: &Material) {
  if is_back_facing(&v0, &v1, &v2) {
    return;
  }

  let area = edge_function(v1, v2, v0.x, v0.y);

  // Degenerate triangle, save ourselves a NaN panic
  if area.abs() < TRI_AREA_EPS {
    return;
  }

  // We need inverse area for Barycentric gubbins later
  let inv_area = 1.0 / area;

  let min_x = v1.x.min(v0.x).min(v2.x).max(0.0);
  let min_y = v1.y.min(v0.y).min(v2.y).max(0.0);
  let max_x = v1.x.max(v0.x).max(v2.x).min(buff.w as f32 - 1.0);
  let max_y = v1.y.max(v0.y).max(v2.y).min(buff.h as f32 - 1.0);
  let min_xi = min_x.floor() as i32;
  let max_xi = max_x.ceil() as i32;
  let min_yi = min_y.floor() as i32;
  let max_yi = max_y.ceil() as i32;

  // Sample at the centre of the first pixel in the loop range
  let start_x = min_xi as f32 + 0.5;
  let start_y = min_yi as f32 + 0.5;

  // CONVENTION: triangles arrive here AFTER back-face cull, in screen space (Y-down).
  // We use the textbook CCW edge setup. Because our triangles are CW on screen,
  // inside points produce NEGATIVE edge values, so the inside test is `w <= 0`

  // Edge values at the centre of the first pixel
  let mut w0_row = edge_function(v1, v2, start_x, start_y);
  let mut w1_row = edge_function(v2, v0, start_x, start_y);
  let mut w2_row = edge_function(v0, v1, start_x, start_y);

  // Step amounts: how much each edge value changes per pixel
  let dx0 = v1.y - v2.y;
  let dx1 = v2.y - v0.y;
  let dx2 = v0.y - v1.y;

  let dy0 = v2.x - v1.x;
  let dy1 = v0.x - v2.x;
  let dy2 = v1.x - v0.x;

  // Core pixel loop starts here

  for y in min_yi..=max_yi {
    let mut w0 = w0_row;
    let mut w1 = w1_row;
    let mut w2 = w2_row;

    for x in min_xi..=max_xi {
      if w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 {
        // Barycentric weights (positive, sum to 1)
        let b0 = w0 * inv_area;
        let b1 = w1 * inv_area;
        let b2 = w2 * inv_area;

        // Linear depth interpolation (correct in screen space, no /w needed)
        let z = b0 * v0.z + b1 * v1.z + b2 * v2.z;

        // Get diffuse colour from texture or from material basic diffuse
        let mut alpha = 1.0;
        let surface_colour = match &mat.texture {
          Some(tex) => {
            // Texture mapping requires voodoo with inv_w
            let inv_w = b0 * v0.inv_w + b1 * v1.inv_w + b2 * v2.inv_w;
            let w = 1.0 / inv_w; // one divide instead of two
            let u = (b0 * v0.u_w + b1 * v1.u_w + b2 * v2.u_w) * w;
            let v = (b0 * v0.v_w + b1 * v1.v_w + b2 * v2.v_w) * w;

            let (texel, a) = tex.sample(u, v);
            alpha = a; // Pull out

            texel * mat.diffuse
          }

          None => mat.diffuse,
        };

        // Pure Gouraud: light is always interpolated across the three verts.
        // For a flat-shaded look, author meshes with per-face normals (Model::flatten).
        let lighting = v0.light * b0 + v1.light * b1 + v2.light * b2;

        match mat.blend_mode {
          BlendMode::Opaque => {
            // existing path, also handles alpha_cutout via texture
            buff.set_pixel_depth(x as usize, y as usize, surface_colour * lighting, z);
          }
          BlendMode::AlphaBlend => {
            let a = mat.opacity * alpha; // texel_alpha = 1.0 if untextured
            buff.blend_pixel_depth(x as usize, y as usize, surface_colour * lighting, a, z);
          }
          BlendMode::Mask => {
            if alpha < mat.mask_cutoff {
              w0 += dx0;
              w1 += dx1;
              w2 += dx2;
              continue;
            }
          }
          BlendMode::Additive => { /* TODO: not implemented */ }
        }

        buff.set_pixel_depth(x as usize, y as usize, surface_colour * lighting, z);
      }

      w0 += dx0;
      w1 += dx1;
      w2 += dx2;
    }

    w0_row += dy0;
    w1_row += dy1;
    w2_row += dy2;
  }
}

// Internal function for calculating the light at a vertex in world space
// We return light values (as RGB Colours) falling on that vert, NOT the colour of the surface
#[inline(always)]
pub(crate) fn shade_vert(lights: &SlotMap<LightHandle, Light>, world: Vec3, n: Vec3, eye: Vec3, hardness: f32) -> (Colour, Colour) {
  // Shading & lighting over multiple lights
  let mut diff_sum = BLACK;
  let mut spec_sum = BLACK;

  for light in lights.values() {
    // Only apply dynamic lights
    if !light.is_dynamic {
      continue;
    }

    if !light.is_enabled {
      continue;
    }

    // Vectors to and from the surface and the light
    let l_raw = light.pos - world;
    let d = l_raw.len();
    let l = l_raw.normalize_new();
    let li = l.invert();

    // Add attenuation
    let atten = 1.0 / (1.0 + light.atten_linear * d + light.atten_quad * d * d);

    // Diffuse lighting
    let n_dot_l = n.dot(l).max(0.0);
    let diff_col = light.colour * light.brightness * n_dot_l * atten;
    diff_sum += diff_col;

    // Specular, hardness guardrail serves as a quick way to disable specular
    if n_dot_l > 0.0 && hardness > 0.0 {
      let v = (eye - world).normalize_new();
      let r = li.reflect(n);
      let v_dot_r = v.dot(r).max(0.0);
      let spec = v_dot_r.powf(hardness);
      let spec_col = light.colour * spec * light.brightness * atten;
      spec_sum += spec_col;
    }
  }

  (diff_sum, spec_sum)
}
