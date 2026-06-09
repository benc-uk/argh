use rand::random_range;
use std::f32::consts::PI;

use argh::{engine::Key, prelude::*};

pub struct MyApp {
  camera: Camera,
  scn: Scene,
  ang_x: f32,
  ang_y: f32,
  vel_x: f32,
  vel_z: f32,
}

// Movement with inertia: accelerate while held, then coast and decay
const ACCEL: f32 = 0.08;
const MAX_SPEED: f32 = 0.4;
const FRICTION: f32 = 0.8;
const YAW_SCLAE: f32 = 0.07;
const PITCH_SCALE: f32 = 0.05;

impl App for MyApp {
  fn update(&mut self, eng: &mut Engine, _dt: f64, _t: f64) {
    eng.clear(BLACK);

    // Mouse look
    if let Some(mpos) = eng.get_mouse_pos() {
      let (mx, my) = (mpos.0, mpos.1);
      let (w, h) = (eng.get_size().0 as f32, eng.get_size().1 as f32);
      let mut dx = ((mx / w) * 2.0) - 1.0;
      let mut dy = ((my / h) * 2.0) - 1.0;
      if f32::abs(dx) < 0.25 {
        dx = 0.0;
      }
      if f32::abs(dy) < 0.25 {
        dy = 0.0;
      }

      // Apply sensitivity
      self.ang_y -= dx * YAW_SCLAE;
      self.ang_x -= dy * PITCH_SCALE;
      self.ang_x = self.ang_x.clamp(-2.5, 2.5);
    }

    // Current look/forward direction
    let d_z = -f32::cos(self.ang_y);
    let d_x = -f32::sin(self.ang_y);
    let d_y = f32::sin(self.ang_x);

    if eng.is_pressed(Key::W) {
      self.vel_x += d_x * ACCEL;
      self.vel_z += d_z * ACCEL;
    }
    if eng.is_pressed(Key::S) {
      self.vel_x -= d_x * ACCEL;
      self.vel_z -= d_z * ACCEL;
    }

    // Clamp horizontal speed magnitude
    let speed_sq = self.vel_x * self.vel_x + self.vel_z * self.vel_z;
    if speed_sq > MAX_SPEED * MAX_SPEED {
      let scale = MAX_SPEED / speed_sq.sqrt();
      self.vel_x *= scale;
      self.vel_z *= scale;
    }

    // Apply velocity every frame (so released keys still coast)
    let mut p = self.camera.get_pos();
    p.x += self.vel_x;
    p.z += self.vel_z;
    self.camera.set_pos(p);
    self.camera.set_look_at(v3(p.x + d_x, p.y + d_y, p.z + d_z));

    // Apply friction every frame
    self.vel_x *= FRICTION;
    self.vel_z *= FRICTION;

    // self.scn.light_mut(self.lh).pos = p;

    eng.render(&self.camera, &self.scn);

    if !eng.get_keys_pressed().is_empty() {
      // Quit on escape
      if eng.is_pressed(Key::Escape) {
        eng.stop();
      }
    }
  }
}

pub fn new(eng: &mut Engine) -> MyApp {
  let mut scn = Scene::new();
  scn.ambient_light = BLACK;

  let floor = eng.load_obj("assets/models/dungeon/floor_tile_large.obj").expect("obj loading failed");
  let grate = eng.load_obj("assets/models/dungeon/floor_tile_big_grate.obj").expect("obj loading failed");
  let floor_rocks = eng.load_obj("assets/models/dungeon/floor_tile_large_rocks.obj").expect("obj loading failed");

  let wall = eng.load_obj("assets/models/dungeon/wall.obj").expect("obj loading failed");

  let barrel_small = eng.load_obj("assets/models/dungeon/barrel_large.obj").expect("obj loading failed");
  let trunk = eng.load_obj("assets/models/dungeon/trunk_large_C.obj").expect("obj loading failed");
  let stool = eng.load_obj("assets/models/dungeon/stool.obj").expect("obj loading failed");
  let boxobj = eng.load_obj("assets/models/dungeon/box_small_decorated.obj").expect("obj loading failed");

  let grid: Vec<Vec<char>> = DUNGEON_MAP.lines().map(|line| line.chars().collect()).collect();

  for y in 0..grid.len() {
    for x in 0..grid.len() {
      let cell = grid[x][y];
      let xf = x as f32;
      let yf = y as f32;
      if cell != '.' {
        let floor_mesh = match random_range(1..=6) {
          1 => grate,
          2 => floor_rocks,
          _ => floor,
        };
        scn.add_instance_trans(floor_mesh, v3(xf * CELL_SIZE, 0.0, yf * CELL_SIZE), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));

        if floor_mesh == floor {
          let decoh: Option<ModelHandle> = match random_range(1..=10) {
            1 => Some(trunk),
            2 => Some(barrel_small),
            3 => Some(stool),
            4 => Some(boxobj),
            _ => None,
          };

          if let Some(deco) = decoh {
            scn.add_instance_trans(deco, v3(xf * CELL_SIZE, 0.0, yf * CELL_SIZE), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
          }

          if random_range(1..=10) < 5 {
            let mut l = Light::new(v3(xf * CELL_SIZE, 3.0, yf * CELL_SIZE), 5.0, col8(255, 250, 200));
            l.atten_linear = 0.0;
            l.atten_quad = 3.0;
            scn.add_light(l);
          }
        }

        let north = grid[x][y - 1];
        let south = grid[x][y + 1];
        let east = grid[x + 1][y];
        let west = grid[x - 1][y];
        if north == '.' {
          scn.add_instance_trans(wall, v3(xf * CELL_SIZE, 0.0, yf * CELL_SIZE - 2.25), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if south == '.' {
          scn.add_instance_trans(wall, v3(xf * CELL_SIZE, 0.0, yf * CELL_SIZE + 2.25), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if east == '.' {
          scn.add_instance_trans(wall, v3(xf * CELL_SIZE + 2.25, 0.0, yf * CELL_SIZE), v3(0.0, PI / 2.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if west == '.' {
          scn.add_instance_trans(wall, v3(xf * CELL_SIZE - 2.25, 0.0, yf * CELL_SIZE), v3(0.0, PI / 2.0, 0.0), v3(1.0, 1.0, 1.0));
        }
      }
    }
    println!()
  }

  let camera = Camera::new_perspective(eng.get_aspect(), v3(10.0, 2.5, 10.0), v3(0.0, 2.0, -1.0), 70.0, 0.01, 300.0).unwrap();

  let mut l = Light::new(camera.get_pos(), 5.0, col8(255, 250, 200));
  l.atten_linear = 0.0;
  l.atten_quad = 3.0;
  scn.add_light(l);

  MyApp {
    camera,
    scn,
    ang_x: 0.0,
    ang_y: -2.0,

    vel_x: 0.0,
    vel_z: 0.0,
  }
}

const CELL_SIZE: f32 = 4.0_f32;
const DUNGEON_MAP: &str = "..........
..........
..###..#..
..###.###.
..###..#..
....#..##.
....#..##.
.#######..
.##....#..
..........";
