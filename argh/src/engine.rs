// ==============================================================================================
// Module & file:   engine.rs
// Purpose:         Core rendering engine, window management and drawing operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::camera::Camera;
use crate::colour::{BLACK, Colour, WHITE};
use crate::light::Light;
use crate::math::{Vec2, Vec3, Vec4};
use crate::models::Mesh;
use crate::{buffer::Buffer, helpers};
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

/// All users of argh are expected to provide their own Scene implementation
pub trait Scene {
  /// This method will be called every frame by the main loop, use it to draw and render your scene
  fn update(&mut self, engine: &mut Engine, dt: f64);
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

  pub target_fps: usize,
  pub debug: bool,
}

// This is used internally to represent a vertex transformed into screen space (after perspective divide)
// It's a hybrid of x & Y being screen pixel values, and z being a float representing depth in 0-1 range
#[derive(Copy, Clone)]
pub struct ScreenVertex {
  pub x: f64,     // pixel coordinate, [0, width]
  pub y: f64,     // pixel coordinate, [0, height], origin top-left
  pub z: f64,     // NDC depth [0, +1] (D3D/Vulkan/WebGPU convention, near=0, far=+1)
  pub inv_w: f64, // 1/w from clip space, for perspective-correct interp
}

// VertexOut collects various data from the processing of mesh vertex in the rendering pass, transformed ready for rasterization
pub struct VertexOut {
  world: Vec3,
  screen: ScreenVertex,
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
      win_title: title,
      buffer: Buffer::new(w as usize, h as usize),
      t: 0.0,
      scale: scl,
      fps: 0.0,
      debug: false,
      target_fps: 60,
      aspect: w as f64 / h as f64,
      lights: vec![] as std::vec::Vec<Light>,
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

  /// Get the current time in seconds
  #[inline(always)]
  pub fn t(&self) -> f64 {
    self.t
  }

  /// Set the colour of a
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
      topmost: true,
      resize: true,
      scale: self.scale,
      ..Default::default()
    };

    let mut window = Window::new(&self.win_title, self.win_size.0, self.win_size.1, opt).unwrap_or_else(|e| {
      panic!("{}", e);
    });

    if self.target_fps > 0 {
      window.set_target_fps(self.target_fps);
    }
    let mut last_time = Instant::now();

    // Lights check
    if self.lights.is_empty() {
      println!("You have added no lights, there will be no shading just flat colours!")
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
      let now = Instant::now();
      let dt = now.duration_since(last_time).as_secs_f64();
      self.t += dt;
      self.fps = 1.0 / dt;
      last_time = now;

      scene.update(&mut self, dt);

      if self.debug {
        self.draw_string(&format!("FPS: {:.2}", self.fps), 2, 2, crate::colour::BLACK);
        self.draw_string(&format!("FPS: {:.2}", self.fps), 1, 1, crate::colour::WHITE);
      }

      let res = window.update_with_buffer(&self.buffer.pixels, self.win_size.0, self.win_size.1);
      if res.is_err() {
        println!("Error updating buffer: {}", res.err().unwrap());
      }
    }
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

  /// Add a light to the scene, used by 3D rendering
  pub fn add_light(&mut self, light: Light) {
    self.lights.push(light);
  }

  /// Renders a 3D mesh onto the screen from given camera position
  pub fn render_mesh(&mut self, cam: &Camera, mesh: &Mesh) {
    // Get the colour of the mesh
    let colour = match &mesh.material {
      None => WHITE,
      Some(m) => m.texture.get_colour_at(0.0, 0.0) * m.diffuse,
    };

    // 1. Combine MVP (model, view, perspective) matrix
    let m = mesh.get_model_mat();
    let mvp = cam.pers_mat * cam.view_mat * m;

    // 2. Process verts, into world space, clip space and screen space
    let verts: Vec<VertexOut> = mesh
      .verts
      .iter()
      .map(|v| {
        // World space
        let world = m * v;

        // Clip space
        let clip = mvp * &Vec4::new(v.x, v.y, v.z, 1.0);

        // Calc vertex is clipped or not
        let outcode = helpers::compute_outcode(&clip);

        // 1/w used for perspective and we store it for other reasons too
        let inv_w = 1.0 / clip.w;

        // NDCs to get us towards screen space
        let ndc_x = clip.x * inv_w;
        let ndc_y = clip.y * inv_w;
        let ndc_z = clip.z * inv_w;
        // Screen space is NDC [-1,+1] -> pixels. IMPORTANT! flip Y because screen origin is top-left.
        let sx = (ndc_x * 0.5 + 0.5) * self.win_size.0 as f64;
        let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * self.win_size.1 as f64; // Flip Y here

        // Finally output processed vert data bundle
        VertexOut {
          world,
          screen: ScreenVertex { x: sx, y: sy, z: ndc_z, inv_w },
          outcode,
        }
      })
      .collect();

    let normals: Vec<Vec3> = mesh.normals.iter().map(|n| mesh.rot.rotate_vec3(*n)).collect();

    // 3. Walk the index list 3 at a time for triangles, cull, shade & rasterize
    for (tri_index, tri) in mesh.indices.chunks(3).enumerate() {
      let i0 = tri[0] as usize;
      let i1 = tri[1] as usize;
      let i2 = tri[2] as usize;
      let sv0 = verts[i0].screen;
      let sv1 = verts[i1].screen;
      let sv2 = verts[i2].screen;
      let wv0 = verts[i0].world;
      let wv1 = verts[i1].world;
      let wv2 = verts[i2].world;

      // One per triangle
      let n = normals[tri_index];

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
      //  - Front faces (mesh CCW in 3D) have NEGATIVE area in screen space
      //  - Back faces (mesh CW or back of CCW) have POSITIVE area
      // So we discard anything non-negative.
      let area = (sv1.x - sv0.x) * (sv2.y - sv0.y) - (sv1.y - sv0.y) * (sv2.x - sv0.x);
      if area >= 0.0 {
        continue;
      }

      // Shading & lighting over multiple lights
      let mut light_col_sum = BLACK;
      if !self.lights.is_empty() {
        let world_v = (wv0 + wv1 + wv2) / 3.0; // Centroid of the triangle in world space

        for light in &self.lights {
          let l = (light.pos - world_v).normalize_new();
          let diff = n.dot(l);
          let col = light.colour * light.brightness * diff;
          light_col_sum += col;
        }
      } else {
        // I figured this was a better fallback than a totally black window!
        light_col_sum = WHITE;
      }

      let out_colour = colour * light_col_sum;

      self.buffer.fill_triangle(sv0, sv1, sv2, out_colour);
    }
  }
}
