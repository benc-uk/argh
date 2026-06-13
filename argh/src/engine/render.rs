// ==============================================================================================
// Module & file:   engine / render.rs
// Purpose:         Core rendering methods for 3D meshes and instances
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  baked_mesh::BakedMesh,
  buffer::Buffer,
  camera::Camera,
  colour::{BLACK, Colour},
  helpers::{OUT_NEAR, compute_outcode, shade_vert, shade_vert_diffuse},
  material::Material,
  math::{Mat3, Mat4, Vec3, Vec4},
  scene::Scene,
};

use super::{Engine, InstanceHandle};

const TRI_AREA_EPS: f32 = 1e-8;

// ProcessedVert collects various data from the processing/transformation of mesh vertex in the rendering pass
// It holds data ready for rasterization, shading etc
pub(super) struct ProcessedVert {
  world: Vec3,
  screen: ScreenVert,
  outcode: u8,
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

  /// Render a [Scene] from given [Camera], this is the most common thing to call inside your [App] update
  pub fn render(&mut self, cam: &Camera, scn: &Scene) {
    // render all static stuff
    for mesh in &scn.baked_meshes {
      self.render_static(mesh, cam.pers_mat * cam.view_mat, scn);
    }

    // render all dynamic instances
    for handle in 0..scn.instance_keys.len() {
      let hdl = scn.instance_keys[handle];
      self.render_instance(hdl, cam, scn);
    }
  }

  /// Renders a 3D [Instance] (ref by [InstanceHandle]) onto the screen from given camera position
  /// This triggers the full rendering pipeline
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
    let mvp = cam.pers_mat * cam.view_mat * m;

    // Model is made of meshes, iterate over them
    for mesh in model.meshes.iter() {
      // Clear vert vector cache
      self.verts.clear();
      self.normals.clear();

      // 2. Process verts, into world space, clip space and screen space
      self.verts.extend(mesh.verts.iter().enumerate().map(|(i, vert)| {
        // World space: vert transformed by model matrix M
        let world = m.transform_point(vert);

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
        let sx = (ndc_x * 0.5 + 0.5) * self.size.0 as f32;
        let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * self.size.1 as f32; // Flip Y here

        // Reach out to the UV array and pre-mult by 1/w, it MUST be the same length
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
            light: BLACK, // Mutated later
            u_w,
            v_w,
          },
          outcode,
        }
      }));

      // 3. Process normals using the Mat3 inverse transpose of the model matrix
      self.normals.extend(mesh.normals.iter().map(|n| (m_inv_t * n).normalize_new()));

      // 4. Now to rendering triangles
      // Walk the index list 3 at a time for triangles, cull, shade & rasterize
      for tri in mesh.indices.chunks(3) {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        let mut sv0 = self.verts[i0].screen;
        let mut sv1 = self.verts[i1].screen;
        let mut sv2 = self.verts[i2].screen;
        let wv0 = self.verts[i0].world;
        let wv1 = self.verts[i1].world;
        let wv2 = self.verts[i2].world;

        // It's convention that the normals list is in the same order as the verts list
        // Otherwise we're in impossible mess TBH
        let mut n0 = self.normals[i0];
        let n1 = self.normals[i1];
        let n2 = self.normals[i2];

        // Trivial reject: all three vertices outside the SAME plane.
        let combined_out = self.verts[i0].outcode & self.verts[i1].outcode & self.verts[i2].outcode;
        if combined_out != 0 {
          continue;
        }

        // Strict near-plane discard (any vertex behind near). This will back objects "pop" in/out near camera
        // TODO: Sutherland-Hodgman near-plane clipping which is complex as hell
        let any_near = (self.verts[i0].outcode | self.verts[i1].outcode | self.verts[i2].outcode) & OUT_NEAR;
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

        let mat = &mesh.material;

        // Cludge n0 to be a normal for the whole face for flat shading
        if !instance.smooth {
          n0 = (wv1 - wv0).cross(wv2 - wv0).normalize_new();
        }

        // Ambient light
        let amb = scn.ambient_light * mat.diffuse;
        let eye = cam.pos();

        // Always shade vert 0
        let (d0, s0) = shade_vert(&scn.lights, wv0, n0, eye, mat.hardness);
        sv0.light = (d0 * mat.diffuse) + (s0 * mat.specular) + amb;

        // Only shade vert 1 & 2 when smooth shading
        if instance.smooth {
          let (d1, s1) = shade_vert(&scn.lights, wv1, n1, eye, mat.hardness);
          let (d2, s2) = shade_vert(&scn.lights, wv2, n2, eye, mat.hardness);
          sv1.light = (d1 * mat.diffuse) + (s1 * mat.specular) + amb;
          sv2.light = (d2 * mat.diffuse) + (s2 * mat.specular) + amb;
        }

        // Finally draw the damn triangle based on the screen verts and interpolate
        fill_triangle(&mut self.buffer, sv0, sv1, sv2, mat, instance.smooth);
        self.stat_rend_tri_frame += 1;
      }
    }
  }

  /// Renders a 3D mesh onto the screen from given camera position
  /// This triggers a rendering pipeline
  fn render_static(&mut self, baked_mesh: &BakedMesh, vp: Mat4, scn: &Scene) {
    self.verts.clear();
    self.normals.clear();

    // Prevents a panic but very likely result in absolutely nothing being rendered
    if baked_mesh.baked_lighting.is_empty() {
      return;
    }

    // Similar but different code from processing the verts in a dynamic instance
    self.verts.extend(baked_mesh.verts.iter().enumerate().map(|(i, world)| {
      let clip = vp * &Vec4::new(world.x, world.y, world.z, 1.0);
      let outcode = compute_outcode(&clip);
      let inv_w = 1.0 / clip.w;
      let ndc_x = clip.x * inv_w;
      let ndc_y = clip.y * inv_w;
      let ndc_z = clip.z * inv_w;
      let sx = (ndc_x * 0.5 + 0.5) * self.size.0 as f32;
      let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * self.size.1 as f32;
      let uv = baked_mesh.uvs[i];

      ProcessedVert {
        world: *world, // no transform!
        screen: ScreenVert {
          x: sx,
          y: sy,
          z: ndc_z,
          inv_w,
          light: baked_mesh.baked_lighting[i],
          u_w: uv.x * inv_w,
          v_w: uv.y * inv_w,
        },
        outcode,
      }
    }));

    // 3. Copy normals
    // self.normals.extend_from_slice(&baked_mesh.normals);

    // This is also similar but different!
    for tri in baked_mesh.indices.chunks(3) {
      let i0 = tri[0] as usize;
      let i1 = tri[1] as usize;
      let i2 = tri[2] as usize;
      let mut sv0 = self.verts[i0].screen;
      let mut sv1 = self.verts[i1].screen;
      let mut sv2 = self.verts[i2].screen;
      let wv0 = self.verts[i0].world;
      let wv1 = self.verts[i1].world;
      let wv2 = self.verts[i2].world;

      let n0 = baked_mesh.normals[i0];
      let n1 = baked_mesh.normals[i1];
      let n2 = baked_mesh.normals[i2];

      // Trivial reject: all three vertices outside the SAME plane.
      let combined_out = self.verts[i0].outcode & self.verts[i1].outcode & self.verts[i2].outcode;
      if combined_out != 0 {
        continue;
      }

      // Strict near-plane discard (any vertex behind near). This will back objects "pop" in/out near camera
      // TODO: Sutherland-Hodgman near-plane clipping which is complex as hell
      let any_near = (self.verts[i0].outcode | self.verts[i1].outcode | self.verts[i2].outcode) & OUT_NEAR;
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

      // No lighting calc, just grab the baked values, wow so speedy
      sv0.light = baked_mesh.baked_lighting[i0] + shade_vert_diffuse(&scn.lights, wv0, n0);
      sv1.light = baked_mesh.baked_lighting[i1] + shade_vert_diffuse(&scn.lights, wv1, n1);
      sv2.light = baked_mesh.baked_lighting[i2] + shade_vert_diffuse(&scn.lights, wv2, n2);

      // Finally draw the damn triangle based on the screen verts and interpolate/fill between them
      // Note we force smooth to true
      fill_triangle(&mut self.buffer, sv0, sv1, sv2, &baked_mesh.material, true);
      self.stat_rend_tri_frame += 1;
    }
  }
}

// Standard edge function, it's basically a cross product
#[inline(always)]
fn edge_function(a: ScreenVert, b: ScreenVert, px: f32, py: f32) -> f32 {
  (b.x - a.x) * (py - a.y) - (b.y - a.y) * (px - a.x)
}

// Fill a triangle between three ScreenVertex points in screen space interpolating lighting, z-depth and texture samples
#[inline(always)]
fn fill_triangle(buff: &mut Buffer, v0: ScreenVert, v1: ScreenVert, v2: ScreenVert, mat: &Material, smooth: bool) {
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
        let surface_colour = match &mat.texture {
          Some(tex) => {
            // Texture mapping requires voodoo with inv_w
            let inv_w = b0 * v0.inv_w + b1 * v1.inv_w + b2 * v2.inv_w;
            let w = 1.0 / inv_w; // one divide instead of two
            let u = (b0 * v0.u_w + b1 * v1.u_w + b2 * v2.u_w) * w;
            let v = (b0 * v0.v_w + b1 * v1.v_w + b2 * v2.v_w) * w;

            let (texel, alpha) = tex.sample(u, v);

            // When alpha cutting, skip this pixel entirely if alpha is low
            if alpha < 0.5 && tex.alpha_cutout {
              w0 += dx0;
              w1 += dx1;
              w2 += dx2;

              continue;
            }

            texel * mat.diffuse
          }

          None => mat.diffuse,
        };

        // Default shading uses on vert 0 only (flat)
        let mut lighting = v0.light;

        // Gouraud shading interpolates between shade at all 3 verts
        if smooth {
          lighting = v0.light * b0 + v1.light * b1 + v2.light * b2;
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
