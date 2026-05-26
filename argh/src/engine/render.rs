// ==============================================================================================
// Module & file:   engine / render.rs
// Purpose:         Core rendering methods for 3D meshes and instances
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  buffer::Buffer,
  camera::Camera,
  colour::{BLACK, Colour},
  helpers::{OUT_NEAR, compute_outcode, shade_vert},
  math::{Vec3, Vec4},
  models::Texture,
};

use super::{Engine, InstanceHandle};

// ProcessedVert collects various data from the processing/transformation of mesh vertex in the rendering pass
// It holds data ready for rasterization, shading etc
struct ProcessedVert {
  world: Vec3,
  screen: ScreenVert,
  outcode: u8,
}

// This is used internally to represent a vertex transformed into screen space (after perspective divide)
// It's a hybrid of x & Y being screen pixel values, and z being a float representing depth in 0-1 range
#[derive(Copy, Clone)]
struct ScreenVert {
  x: f64,         // pixel coordinate, [0, width]
  y: f64,         // pixel coordinate, [0, height], origin top-left
  z: f64,         // NDC depth [0, +1] (D3D/Vulkan/WebGPU convention, near=0, far=+1)
  colour: Colour, // Gouraud shading needs colour per vertex
  inv_w: f64,     // Inverse of w
  u_w: f64,       // PRE-DIVIDED, not raw u and v.
  v_w: f64,       // PRE-DIVIDED, not raw u and v.
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
  pub fn write_rgba_bytes(&self, out: &mut [u8]) {
    debug_assert_eq!(out.len(), self.buffer.pixels.len() * 4);

    for (chunk, &p) in out.chunks_exact_mut(4).zip(&self.buffer.pixels) {
      let [_, r, g, b] = p.to_be_bytes();
      chunk[0] = r;
      chunk[1] = g;
      chunk[2] = b;
      chunk[3] = 0xFF;
    }
  }

  /// Render all instances available
  pub fn render_all(&mut self, cam: &Camera) {
    let keys: Vec<_> = self.instances.keys().collect();

    for i in keys {
      self.render_instance(cam, i);
    }
  }

  /// Renders a 3D mesh onto the screen from given camera position
  /// This triggers a rendering pipeline
  pub fn render_instance(&mut self, cam: &Camera, h: InstanceHandle) {
    // Shorthand for finding the instance
    let Some(instance) = self.instances.get(h) else {
      return;
    };

    // Get texture via the material
    let mat = self.materials.get(instance.material_handle).unwrap();
    let tex = &mat.texture;

    // 1. Combine MVP (model, view, perspective) matrix
    let m = instance.get_model_mat();
    let mvp = cam.pers_mat * cam.view_mat * m;

    // We unwrap here, as mesh existence is checked when instance is created
    let mesh = self.meshes.get(instance.mesh_handle).unwrap();

    // 2. Process verts, into world space, clip space and screen space
    let verts: Vec<ProcessedVert> = mesh
      .verts
      .iter()
      .enumerate()
      .map(|(i, vert)| {
        // World space: vert transformed by model matrix M
        let world = m * vert;

        // Clip space: vert transformed by MVP
        let clip = mvp * &Vec4::new(vert.x, vert.y, vert.z, 1.0);

        // Calc how vertex is clipped or not
        let outcode = compute_outcode(&clip);

        // 1/w used for perspective and we store it for other reasons too
        let inv_w = 1.0 / clip.w;

        // Clip space -> NDCs, which get us towards screen space
        let ndc_x = clip.x * inv_w;
        let ndc_y = clip.y * inv_w;
        let ndc_z = clip.z * inv_w;
        // Screen space: is NDC [-1,+1] -> pixels. IMPORTANT! flip Y because screen origin is top-left.
        let sx = (ndc_x * 0.5 + 0.5) * self.size.0 as f64;
        let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * self.size.1 as f64; // Flip Y here

        // Reach out to the UV array and pre-mult by 1/w
        let uv = mesh.uvs[i];
        let u_w = uv.x * inv_w;
        let v_w = uv.y * inv_w;

        // Bundle up processed data into a single struct
        ProcessedVert {
          world,
          screen: ScreenVert {
            x: sx,
            y: sy,
            z: ndc_z,
            inv_w,
            colour: BLACK, // Mutated later
            u_w,
            v_w,
          },
          outcode,
        }
      })
      .collect();

    // 3. Process normals, we don't do any fancy 3x3 matrix extraction, transpose blah blah
    // We just rotate them by the model rotation quat for now
    // TODO: We probably do need to do this the proper way at some point
    let normals: Vec<Vec3> = mesh.normals.iter().map(|n| instance.rot.rotate_vec3(*n)).collect();

    // 3. Now to rendering triangles
    // Walk the index list 3 at a time for triangles, cull, shade & rasterize
    for tri in mesh.indices.chunks(3) {
      let i0 = tri[0] as usize;
      let i1 = tri[1] as usize;
      let i2 = tri[2] as usize;
      let mut sv0 = verts[i0].screen;
      let mut sv1 = verts[i1].screen;
      let mut sv2 = verts[i2].screen;
      let wv0 = verts[i0].world;
      let wv1 = verts[i1].world;
      let wv2 = verts[i2].world;

      // It's convention that the normals list is in the same order as the verts list
      // Otherwise we're in impossible mess TBH
      let n0 = normals[i0];
      let n1 = normals[i1];
      let n2 = normals[i2];

      // Trivial reject: all three vertices outside the SAME plane.
      let combined_out = verts[i0].outcode & verts[i1].outcode & verts[i2].outcode;
      if combined_out != 0 {
        continue;
      }

      // Strict near-plane discard (any vertex behind near). This will back objects "pop" in/out near camera
      // TODO: Sutherland-Hodgman near-plane clipping which is complex as hell
      let any_near = (verts[i0].outcode | verts[i1].outcode | verts[i2].outcode) & OUT_NEAR;
      if any_near != 0 {
        continue;
      }

      // Back-face cull. We use Y-flipped screen space, the signed area test is inverted from OpenGL
      //  - Back faces (mesh CW or back of CCW) have POSITIVE area
      // So we discard anything non-negative.
      let area = (sv1.x - sv0.x) * (sv2.y - sv0.y) - (sv1.y - sv0.y) * (sv2.x - sv0.x);
      if area >= 0.0 {
        continue;
      }

      // Ambient light
      let amb = self.ambient_light * mat.diffuse;

      let eye = cam.get_pos();

      // Calc shading & lighting at each world vertex, and set into screen vert
      let (d0, s0) = shade_vert(&self.lights, wv0, n0, eye, mat.hardness);
      sv0.colour = (d0 * mat.diffuse) + (s0 * mat.specular) + amb;
      if instance.smooth {
        let (d1, s1) = shade_vert(&self.lights, wv1, n1, eye, mat.hardness);
        let (d2, s2) = shade_vert(&self.lights, wv2, n2, eye, mat.hardness);
        sv1.colour = (d1 * mat.diffuse) + (s1 * mat.specular) + amb;
        sv2.colour = (d2 * mat.diffuse) + (s2 * mat.specular) + amb;
      }

      // Finally draw the damn triangle based on the screen verts and interpolate
      fill_triangle(&mut self.buffer, sv0, sv1, sv2, tex, instance.smooth);
    }
  }
}

// Standard edge function
#[inline(always)]
fn edge_function(a: ScreenVert, b: ScreenVert, px: f64, py: f64) -> f64 {
  (b.x - a.x) * (py - a.y) - (b.y - a.y) * (px - a.x)
}

// Fill a 3D triangle between three ScreenVertex points which form a triangle
// Not public outside the crate
#[inline(always)]
fn fill_triangle(buff: &mut Buffer, v0: ScreenVert, v1: ScreenVert, v2: ScreenVert, tex: &Texture, smooth: bool) {
  let area = edge_function(v1, v2, v0.x, v0.y);
  if area == 0.0 {
    return;
  } // degenerate triangle, save ourselves a NaN

  // We need inverse area for Barycentric gubbins later
  let inv_area = 1.0 / area;

  let min_x = v1.x.min(v0.x).min(v2.x).max(0.0);
  let min_y = v1.y.min(v0.y).min(v2.y).max(0.0);
  let max_x = v1.x.max(v0.x).max(v2.x).min(buff.w as f64 - 1.0);
  let max_y = v1.y.max(v0.y).max(v2.y).min(buff.h as f64 - 1.0);
  let min_xi = min_x.floor() as i32;
  let max_xi = max_x.ceil() as i32;
  let min_yi = min_y.floor() as i32;
  let max_yi = max_y.ceil() as i32;

  // Sample at the centre of the first pixel in the loop range
  let start_x = min_xi as f64 + 0.5;
  let start_y = min_yi as f64 + 0.5;

  // CONVENTION: triangles arrive here AFTER back-face cull, in screen space
  // (Y-down). Our cull keeps triangles with NEGATIVE screen-space signed area
  // (CW on screen, from CCW-in-world meshes after the viewport Y-flip).
  //
  // We use the textbook CCW edge setup. Because our triangles are CW on screen,
  // inside points produce NEGATIVE edge values, so the inside test is `w <= 0`.
  // If the cull/Y convention ever changes, flip all three test signs back to >=.

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

        // Texture mapping requires voodoo with inv_w
        let inv_w = b0 * v0.inv_w + b1 * v1.inv_w + b2 * v2.inv_w;
        let w = 1.0 / inv_w; // one divide instead of two
        let u = (b0 * v0.u_w + b1 * v1.u_w + b2 * v2.u_w) * w;
        let v = (b0 * v0.v_w + b1 * v1.v_w + b2 * v2.v_w) * w;
        let texel = tex.sample(u, v);

        let mut colour = v0.colour;
        if smooth {
          // Gouraud shading interpolates between colours at each vert
          colour = v0.colour * b0 + v1.colour * b1 + v2.colour * b2;
        }

        buff.set_pixel_depth(x as usize, y as usize, texel * colour, z as f32);
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
