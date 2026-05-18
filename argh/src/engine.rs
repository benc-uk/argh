// ==============================================================================================
// Module & file:   engine.rs
// Purpose:         Core rendering engine, window management and drawing operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use std::collections::HashMap;
use std::time::Instant;

use crate::camera::Camera;
use crate::colour::{BLACK, Colour, WHITE};
use crate::light::Light;
use crate::math::{Quat, VEC3_ONE, VEC3_ZERO, Vec2, Vec3, Vec4};
use crate::models::{Instance, Material, Mesh, SimpleColourTexture};
use crate::{buffer::Buffer, helpers};
use minifb::{Window, WindowOptions};

/// All users of argh are expected to provide their own Scene implementation
pub trait Scene {
  /// This method will be called every frame by the main loop, use it to draw and render your scene
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64);
}

// Re-export some of the minifb enums for inputs
pub use minifb::{Key, MouseButton};

/// This is an integer handle to reference instances
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct InstanceHandle(usize);

/// This is a internal way to lookup Meshes
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct MeshHandle(usize);

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

  // Meshes & instances
  meshes: Vec<Mesh>,
  mesh_lookup: HashMap<String, MeshHandle>,
  instances: Vec<Instance>,

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
  pub(crate) x: f64, // pixel coordinate, [0, width]
  pub(crate) y: f64, // pixel coordinate, [0, height], origin top-left
  pub(crate) z: f64, // NDC depth [0, +1] (D3D/Vulkan/WebGPU convention, near=0, far=+1)
  // pub(crate) inv_w: f64,    // 1/w from clip space, for perspective-correct interp
  pub(crate) colour: Colour, // Gouraud shading needs colour per vertex
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
      mesh_lookup: HashMap::new(),
      meshes: vec![],
      instances: vec![],

      lights: vec![] as std::vec::Vec<Light>,
      ambient_light: Colour::new(0.1, 0.1, 0.1),

      keys: vec![],
      keys_pressed: vec![],
    }
  }

  /// Clear the entire window and buffer with the given colour
  /// Also clears the depth buffer
  pub fn clear(&mut self, colour: Colour) {
    self.buffer.clear(colour);
  }

  /// Return the width & height of the window
  pub fn get_size(&self) -> (usize, usize) {
    (self.win_size.0, self.win_size.1)
  }

  /// Get the aspect ratio of the viewport and window
  pub fn get_aspect(&self) -> f64 {
    self.aspect
  }

  /// Stop running and exit the running process
  pub fn stop(&mut self) {
    self.exit = true;
  }

  /// Add a mesh to the cache and give it a name
  pub fn add_mesh(&mut self, name: &str, mesh: Mesh) {
    self.meshes.push(mesh);
    let handle = MeshHandle(self.meshes.len() - 1);
    self.mesh_lookup.insert(name.to_string(), handle);
  }

  /// Add a light to the scene, used by 3D rendering
  pub fn add_light(&mut self, light: Light) {
    self.lights.push(light);
  }

  /// Create an instance of a mesh with given name, instance will have default values & material
  pub fn add_instance(&mut self, mesh_name: &str) -> InstanceHandle {
    let tex = SimpleColourTexture::new(WHITE);
    let mat = Material::new(tex);

    let i = Instance {
      material: mat,
      pos: VEC3_ZERO,
      scale: VEC3_ONE,
      rot: Quat::ident(),
      smooth: true,
      mesh: self.get_mesh_handle(mesh_name),
    };

    self.instances.push(i);
    InstanceHandle(self.instances.len() - 1)
  }

  pub fn instance_mut(&mut self, h: InstanceHandle) -> &mut Instance {
    &mut self.instances[h.0]
  }

  pub fn instance(&self, h: InstanceHandle) -> &Instance {
    &self.instances[h.0]
  }

  /// Set the colour of a single pixel in the frame buffer
  /// # Arguments
  /// * `x` - X position of pixel
  /// * `y` - Y position of pixel
  /// * `colour` - New colour of the pixel
  #[inline(always)]
  pub fn set_pixel(&mut self, x: usize, y: usize, colour: Colour) {
    self.buffer.set_pixel(x, y, colour);
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

  /// Returns the keys held down this frame. Snapshot taken once per frame before scene.update().
  pub fn get_keys(&self) -> &[Key] {
    &self.keys
  }

  /// Returns the keys held down this frame. Snapshot taken once per frame before scene.update().
  pub fn get_keys_pressed(&self) -> &[Key] {
    &self.keys_pressed
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

  /// Render all instances available
  pub fn render_all(&mut self, cam: &Camera) {
    for i in 0..self.instances.len() {
      self.render_instance(cam, InstanceHandle(i));
    }
  }

  pub(crate) fn get_mesh_handle(&self, name: &str) -> MeshHandle {
    *self.mesh_lookup.get(name).unwrap()
  }

  /// Renders a 3D mesh onto the screen from given camera position
  /// This triggers a rendering pipeline
  pub fn render_instance(&mut self, cam: &Camera, h: InstanceHandle) {
    let instance = &self.instances[h.0];

    // Get the colour of the mesh
    let mat = instance.get_material();
    let colour = mat.texture.get_colour_at(0.0, 0.0);

    // 1. Combine MVP (model, view, perspective) matrix
    let m = instance.get_model_mat();
    let mvp = cam.pers_mat * cam.view_mat * m;

    let mesh = &self.meshes[instance.mesh.0];

    // 2. Process verts, into world space, clip space and screen space
    let verts: Vec<ProcessedVert> = mesh
      .verts
      .iter()
      .map(|vert| {
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

        // Bundle up processed data into a single struct
        ProcessedVert {
          world,
          screen: ScreenVert {
            x: sx,
            y: sy,
            z: ndc_z,
            // inv_w,
            colour: BLACK, // Mutated later
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
      let (d0, s0) = shade_vert(&self.lights, wv0, n0, eye, mat.hardness);
      sv0.colour = (d0 * colour * mat.diffuse) + (s0 * mat.specular) + amb;
      if instance.smooth {
        let (d1, s1) = shade_vert(&self.lights, wv1, n1, eye, mat.hardness);
        let (d2, s2) = shade_vert(&self.lights, wv2, n2, eye, mat.hardness);
        sv1.colour = (d1 * colour * mat.diffuse) + (s1 * mat.specular) + amb;
        sv2.colour = (d2 * colour * mat.diffuse) + (s2 * mat.specular) + amb;
      }

      // Finally draw the damn triangle based on the screen verts and interpolate
      self.buffer.fill_triangle(sv0, sv1, sv2, instance.smooth);
    }
  }
}

// Internal function for calculating the lighting and colour at a vertex in world space
fn shade_vert(lights: &Vec<Light>, world: Vec3, n: Vec3, eye: Vec3, hardness: f64) -> (Colour, Colour) {
  // Shading & lighting over multiple lights
  let mut diff_sum = BLACK;
  let mut spec_sum = BLACK;

  // if !lights.is_empty() {
  for light in lights {
    // Vectors to and from the surface and the light
    let l = (light.pos - world).normalize_new();
    let li = l.invert();

    // Diffuse lighting
    let n_dot_l = n.dot(l).max(0.0);
    let diff_col = light.colour * light.brightness * n_dot_l;
    diff_sum += diff_col;

    // Specular
    if n_dot_l > 0.0 {
      let v = (eye - world).normalize_new();
      let r = li.reflect(n);
      let v_dot_r = v.dot(r).max(0.0);
      let spec = v_dot_r.powf(hardness);
      let spec_col = light.colour * spec * light.brightness;
      spec_sum += spec_col;
    }
  }
  // }
  (diff_sum, spec_sum)
}
