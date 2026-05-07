use argh::colour::{BLACK, BLUE, CYAN, GREEN, RED, YELLOW};
use argh::engine::{Engine, Scene};
use argh::math::{Mat4, Quat, Vec2, Vec3, Vec4};

struct MyScene {
  cube: Mesh,
}

/// Simple mesh
struct Mesh {
  verts: Vec<Vec3>,
  indices: Vec<i32>,
}

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64) {
    engine.clear(BLACK);

    let (w, h) = engine.get_size();
    let aspect = w as f64 / h as f64;

    // --- 1. Build M, V, P and compose ---
    // Spin the cube around Y so we can see the perspective working
    let angle = engine.t();
    let mut axis = Vec3::new(0.6, 0.3, 0.9);
    axis.normalize();
    let model = Mat4::new_scale_rot_trans(1.0, 1.0, 1.0, Quat::new(axis, angle), 0.0, 0.0, 0.0);

    // Right-handed, camera at +Z=3 looking at the origin down -Z.
    // View is the INVERSE of camera-to-world. With the camera only translated
    // by (0,0,3), the inverse is a translation by (0,0,-3).
    let py = f64::sin(engine.t());
    let px = f64::sin(engine.t() * 0.7);
    let view = Mat4::new_trans(-px, py, -3.0);
    let proj = Mat4::new_perspective(60f64.to_radians(), aspect, 0.1, 100.0);
    let mvp = proj * view * model;

    // --- 2. Transform every unique vert ONCE ---
    let clip: Vec<Vec4> = self.cube.verts.iter().map(|v| mvp * &Vec4::new(v.x, v.y, v.z, 1.0)).collect();

    // --- 3. Perspective divide + viewport map ---
    // Keep z separately so we can back-face cull and (later) depth-sort.
    let screen: Vec<(Vec2, f64)> = clip
      .iter()
      .map(|c| {
        let inv_w = 1.0 / c.w;
        let ndc_x = c.x * inv_w;
        let ndc_y = c.y * inv_w;
        let ndc_z = c.z * inv_w;
        // Viewport: NDC [-1,+1] -> pixels. Flip Y because screen origin is top-left.
        let sx = (ndc_x * 0.5 + 0.5) * w as f64;
        let sy = (ndc_y * 0.5 + 0.5) * h as f64;
        (Vec2 { x: sx, y: sy }, ndc_z)
      })
      .collect();

    let mut t = 0;
    // --- 4. Walk the index list, cull, raster ---
    for tri in self.cube.indices.chunks(3) {
      t += 1;
      let (a, _) = screen[tri[0] as usize];
      let (b, _) = screen[tri[1] as usize];
      let (c, _) = screen[tri[2] as usize];

      // 2D back-face cull. Signed area of the screen-space triangle.
      let area = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
      if area <= 0.0 {
        continue;
      }

      let col = match t {
        1..=2 => RED,
        3..=4 => GREEN,
        5..=6 => BLUE,
        7..=8 => YELLOW,
        _ => CYAN,
      };

      engine.fill_triangle(a, b, c, col);
    }
  }
}

fn main() {
  let mut e = Engine::new(640, 360, String::from("Argh: cube_3d"), 2);
  e.debug = true;
  e.target_fps = 60;

  // Unit cube at the origin
  let verts = vec![
    Vec3::new(-0.5, -0.5, -0.5), // 0: back  bottom left
    Vec3::new(0.5, -0.5, -0.5),  // 1: back  bottom right
    Vec3::new(0.5, 0.5, -0.5),   // 2: back  top    right
    Vec3::new(-0.5, 0.5, -0.5),  // 3: back  top    left
    Vec3::new(-0.5, -0.5, 0.5),  // 4: front bottom left
    Vec3::new(0.5, -0.5, 0.5),   // 5: front bottom right
    Vec3::new(0.5, 0.5, 0.5),    // 6: front top    right
    Vec3::new(-0.5, 0.5, 0.5),   // 7: front top    left
  ];

  // 12 triangles, CCW winding when viewed from outside the cube
  let indices = vec![
    4, 5, 6, 4, 6, 7, // front
    1, 0, 3, 1, 3, 2, // back
    0, 4, 7, 0, 7, 3, // left
    5, 1, 2, 5, 2, 6, // right
    7, 6, 2, 7, 2, 3, // top
    0, 1, 5, 0, 5, 4, // bottom
  ];

  let s = MyScene { cube: Mesh { verts, indices } };

  e.start(s);
}
