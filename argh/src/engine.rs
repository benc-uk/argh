// ==============================================================================================
// Module & file:   engine.rs
// Purpose:         Core rendering engine, window management and drawing operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use slotmap::{SlotMap, new_key_type};
use std::time::Instant;

use crate::camera::Camera;
use crate::colour::{BLACK, Colour, WHITE};
use crate::light::Light;
use crate::math::{Quat, VEC3_ONE, VEC3_ZERO, Vec2, Vec3, Vec4};
use crate::models::{Instance, Material, Mesh};
use crate::{buffer::Buffer, helpers};
use minifb::{Window, WindowOptions};

/// All users of argh are expected to provide their own Scene implementation
pub trait Scene {
  /// This method will be called every frame by the main loop, use it to draw and render your scene
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64);
}

// Re-export some of the minifb enums for inputs
pub use minifb::{Key, MouseButton};

new_key_type! {
  /// A handle to reference instances held by the engine
  pub struct InstanceHandle;
}

new_key_type! {
  /// A handle to reference materials held by the engine
  pub struct MaterialHandle;
}

new_key_type! {
  /// A handle to reference meshes held by the engine
  pub struct MeshHandle;
}

#[derive(thiserror::Error)]
pub enum EngineError {
  #[error("no mesh registered with the name: '{0}'")]
  MeshNotFound(String),
}

impl std::fmt::Debug for EngineError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self, f)
  }
}

/// This is the heart of argh, create an instance of the Engine to use the library
pub struct Engine {
  win_size: (usize, usize),
  win_title: String,
  aspect: f64,
  buffer: Buffer,
  t: f64,
  scale: minifb::Scale,
  fps: f64,
  lights: Vec<Light>,
  exit: bool,

  // Things tracked & cached by the engine
  meshes: SlotMap<MeshHandle, Mesh>,
  materials: SlotMap<MaterialHandle, Material>,
  instances: SlotMap<InstanceHandle, Instance>,

  // Inputs
  keys: Vec<Key>,
  keys_pressed: Vec<Key>,

  // Public fields...
  /// Rate to try to update the buffer, used at engine start only
  pub target_fps: usize,

  /// Output debug info like FPS onto the top right of the screen
  pub debug: bool,

  /// Ambient light colour, defaults to [0.1, 0.1, 0.1], beware setting this too high it will look washed out
  pub ambient_light: Colour,
}

// This is used internally to represent a vertex transformed into screen space (after perspective divide)
// It's a hybrid of x & Y being screen pixel values, and z being a float representing depth in 0-1 range
#[derive(Copy, Clone)]
pub struct ScreenVert {
  pub(crate) x: f64,         // pixel coordinate, [0, width]
  pub(crate) y: f64,         // pixel coordinate, [0, height], origin top-left
  pub(crate) z: f64,         // NDC depth [0, +1] (D3D/Vulkan/WebGPU convention, near=0, far=+1)
  pub(crate) colour: Colour, // Gouraud shading needs colour per vertex
  pub(crate) inv_w: f64,     // Inverse of w
  pub(crate) u_w: f64,       // PRE-DIVIDED, not raw u and v.
  pub(crate) v_w: f64,       // PRE-DIVIDED, not raw u and v.
}

// VertexOut collects various data from the processing/transformation of mesh vertex in the rendering pass
// It holds data ready for rasterization, shading etc
pub struct ProcessedVert {
  world: Vec3,
  screen: ScreenVert,
  outcode: u8,
}

impl Engine {
  /// Constructor for a new Engine
  /// # Arguments
  /// * `w` - Width of the window in pixels
  /// * `h` - Height of the window in pixels
  /// * `title` - Title of the window
  pub fn new(w: i32, h: i32, title: String, scale: i32) -> Self {
    let scl = match scale {
      n if n <= 0 => minifb::Scale::FitScreen,
      1 => minifb::Scale::X1,
      2 => minifb::Scale::X2,
      4 => minifb::Scale::X4,
      8 => minifb::Scale::X8,
      _ => minifb::Scale::X1,
    };

    Self {
      win_size: (w as usize, h as usize),
      buffer: Buffer::new(w as usize, h as usize),
      win_title: title,
      t: 0.0,
      scale: scl,
      fps: 0.0,
      debug: false,
      target_fps: 60,
      aspect: w as f64 / h as f64,
      exit: false,
      meshes: SlotMap::with_key(),
      materials: SlotMap::with_key(),
      instances: SlotMap::with_key(),

      lights: vec![] as std::vec::Vec<Light>,
      ambient_light: Colour::new(0.1, 0.1, 0.1),

      keys: vec![],
      keys_pressed: vec![],
    }
  }

  /// Begin the main loop, open the window and blocks until the window is closed or escape is pressed
  /// # Arguments
  /// * `scene` - Implementation of Scene with your own `update()` function
  pub fn start<S: Scene>(mut self, mut scene: S) {
    let opt = WindowOptions {
      scale_mode: minifb::ScaleMode::Stretch,
      resize: false,
      scale: self.scale,
      ..Default::default()
    };

    let mut window = Window::new(&self.win_title, self.win_size.0, self.win_size.1, opt).unwrap_or_else(|e| {
      panic!("{}", e);
    });

    if self.target_fps > 0 {
      window.set_target_fps(self.target_fps);
    }

    // Lights check
    if self.lights.is_empty() {
      println!("You have added no lights, there will be no shading just flat colours!")
    }

    let mut last_time = Instant::now();

    while window.is_open() {
      if self.exit {
        break;
      }

      let now = Instant::now();
      let dt = now.duration_since(last_time).as_secs_f64();
      self.t += dt;
      self.fps = 1.0 / dt;
      last_time = now;

      self.keys = window.get_keys();
      self.keys_pressed = window.get_keys_pressed(minifb::KeyRepeat::No);

      // This is the hook, the user does their rendering here.
      let t = self.t;
      scene.update(&mut self, dt, t);

      if self.debug {
        self.draw_string(&format!("FPS: {:.2}", self.fps), 2, 2, BLACK);
        self.draw_string(&format!("FPS: {:.2}", self.fps), 1, 1, WHITE);
      }

      if let Err(e) = window.update_with_buffer(&self.buffer.pixels, self.win_size.0, self.win_size.1) {
        println!("Error updating buffer: {}", e);
      }
    }
  }

  /// Stop running and exit the running process
  pub fn stop(&mut self) {
    self.exit = true;
  }

  /// Return the width & height of the window
  pub fn get_size(&self) -> (usize, usize) {
    (self.win_size.0, self.win_size.1)
  }

  /// Get the aspect ratio of the viewport and window
  pub fn get_aspect(&self) -> f64 {
    self.aspect
  }

  /// Add a mesh to the cache and give it a name
  pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
    self.meshes.insert(mesh)
  }

  /// Add a light to the scene, used by 3D rendering
  pub fn add_light(&mut self, light: Light) {
    self.lights.push(light);
  }

  /// Create an instance of a mesh with given name, using the material
  pub fn add_instance(&mut self, mesh_handle: MeshHandle, mat_handle: MaterialHandle) -> InstanceHandle {
    let i = Instance {
      material_handle: mat_handle,
      pos: VEC3_ZERO,
      scale: VEC3_ONE,
      rot: Quat::ident(),
      smooth: true,
      mesh_handle: mesh_handle,
    };

    self.instances.insert(i)
  }

  pub fn instance_mut(&mut self, h: InstanceHandle) -> &mut Instance {
    self.instances.get_mut(h).expect("instance not found")
  }

  pub fn instance(&self, h: InstanceHandle) -> &Instance {
    self.instances.get(h).expect("instance not found")
  }

  pub fn remove_instance(&mut self, h: InstanceHandle) {
    self.instances.remove(h);
  }

  pub fn add_material(&mut self, mat: Material) -> MaterialHandle {
    self.materials.insert(mat)
  }

  pub fn material_mut(&mut self, h: MaterialHandle) -> &mut Material {
    self.materials.get_mut(h).expect("material not found")
  }

  pub fn material(&self, h: MaterialHandle) -> &Material {
    self.materials.get(h).expect("material not found")
  }

  /// Render all instances available
  pub fn render_all(&mut self, cam: &Camera) {
    //self.instances.iter().map(|(ih, _)| &self.render_instance(cam, ih));
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

    // Get texture
    let mh = instance.get_material();
    let mat = &self.materials.get(mh).unwrap();
    let tex = mat.texture.as_ref();

    // 1. Combine MVP (model, view, perspective) matrix
    let m = instance.get_model_mat();
    let mvp = cam.pers_mat * cam.view_mat * m;

    // We unwrap here, as mesh existence is checked when instance is created
    let mesh = &self.meshes.get(instance.mesh_handle).unwrap();

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
        let outcode = helpers::compute_outcode(&clip);

        // 1/w used for perspective and we store it for other reasons too
        let inv_w = 1.0 / clip.w;

        // Clip space -> NDCs, which get us towards screen space
        let ndc_x = clip.x * inv_w;
        let ndc_y = clip.y * inv_w;
        let ndc_z = clip.z * inv_w;
        // Screen space: is NDC [-1,+1] -> pixels. IMPORTANT! flip Y because screen origin is top-left.
        let sx = (ndc_x * 0.5 + 0.5) * self.win_size.0 as f64;
        let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * self.win_size.1 as f64; // Flip Y here

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
      let any_near = (verts[i0].outcode | verts[i1].outcode | verts[i2].outcode) & helpers::OUT_NEAR;
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
      let (d0, s0) = helpers::shade_vert(&self.lights, wv0, n0, eye, mat.hardness);
      sv0.colour = (d0 * mat.diffuse) + (s0 * mat.specular) + amb;
      if instance.smooth {
        let (d1, s1) = helpers::shade_vert(&self.lights, wv1, n1, eye, mat.hardness);
        let (d2, s2) = helpers::shade_vert(&self.lights, wv2, n2, eye, mat.hardness);
        sv1.colour = (d1 * mat.diffuse) + (s1 * mat.specular) + amb;
        sv2.colour = (d2 * mat.diffuse) + (s2 * mat.specular) + amb;
      }

      // Finally draw the damn triangle based on the screen verts and interpolate
      self.buffer.fill_triangle(sv0, sv1, sv2, tex, instance.smooth);
    }
  }

  /// Returns the keys held down this frame. Snapshot taken once per frame before scene.update().
  pub fn get_keys(&self) -> &[Key] {
    &self.keys
  }

  /// Returns the keys held down this frame. Snapshot taken once per frame before scene.update().
  pub fn get_keys_pressed(&self) -> &[Key] {
    &self.keys_pressed
  }

  /// Set the colour of a single pixel in the frame buffer, this is the bedrock of the system
  /// # Arguments
  /// * `x` - X position of pixel
  /// * `y` - Y position of pixel
  /// * `colour` - New colour of the pixel
  #[inline(always)]
  pub fn set_pixel(&mut self, x: usize, y: usize, colour: Colour) {
    self.buffer.set_pixel(x, y, colour);
  }

  /// Clear the entire window and buffer with the given colour
  /// Also clears the depth buffer
  pub fn clear(&mut self, colour: Colour) {
    self.buffer.clear(colour);
  }

  /// Draw text onto the screen
  /// # Arguments
  /// * `s` - String to draw
  /// * `x` - X position of start of text
  /// * `y` - Y position of text
  /// * `colour` - Colour to draw the text
  pub fn draw_string(&mut self, s: &str, x: usize, y: usize, colour: Colour) {
    for (i, ch) in s.chars().enumerate() {
      self.buffer.draw_char(ch, x + (i * (crate::text::glyph_size().0 + 1)), y, colour);
    }
  }

  /// Draw a filled 2D rectangle
  /// # Arguments
  /// * `x` - X coord of top left corner of rectangle
  /// * `y` - Y coord of top left corner of rectangle
  /// * `w` - Width of rectangle in pixels
  /// * `h` - Height of rectangle in pixels
  /// * `colour` - Colour to fill the rectangle
  pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, colour: Colour) {
    self.buffer.fill_rect(x, y, w, h, colour);
  }

  /// Draw a 2D line between two points
  pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, colour: Colour) {
    let mut x0 = x0;
    let mut y0 = y0;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
      self.buffer.set_pixel(x0 as usize, y0 as usize, colour);
      let e2 = 2 * error;
      if e2 >= dy {
        if x0 == x1 {
          break;
        }
        error += dy;
        x0 += sx;
      }
      if e2 <= dx {
        if y0 == y1 {
          break;
        }
        error += dx;
        y0 += sy;
      }
    }
  }

  /// Draw a series of 2D lines between a list of Vec2 points, designed for drawing a polygon but this method does not ensure the shape is closed
  pub fn draw_poly_line(&mut self, points: Vec<Vec2>, colour: Colour) {
    for p in 0..points.len() {
      if p + 1 >= points.len() {
        break;
      }

      self.draw_line(points[p].x as i32, points[p].y as i32, points[p + 1].x as i32, points[p + 1].y as i32, colour);
    }
  }
}
