use rand::random_range;
use std::f32::consts::PI;

use argh::{engine::Key, prelude::*};

use crate::fps_camera::FpsCamera;

pub struct MyApp {
  camera: FpsCamera,
  scn: Scene,
}

const CAM_HEIGHT: f32 = 2.5;
const CELL_SIZE: f32 = 4.0_f32;
const DUNGEON_MAP: &str = "..........
.......#..
..##t..L..
..#Lb.##o.
..o##..#..
....#..L#.
....#..#o.
.##bL###..
.#o....bL.
..........";

impl App for MyApp {
  fn update(&mut self, eng: &mut Engine, dt: f64, _t: f64) {
    eng.clear(BLACK);

    // fps controls
    self.camera.update(eng, dt as f32);

    eng.render(&self.camera.camera, &self.scn);

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
  let boxobj = eng.load_obj("assets/models/dungeon/box_small_decorated.obj").expect("obj loading failed");

  let grid: Vec<Vec<char>> = DUNGEON_MAP.lines().map(|line| line.chars().collect()).collect();

  for y in 0..grid.len() {
    for x in 0..grid.len() {
      let cell = grid[y][x];
      let xf = x as f32;
      let yf = y as f32;
      if cell != '.' {
        let floor_mesh = match random_range(1..=6) {
          1 => grate,
          2 => floor_rocks,
          _ => floor,
        };
        scn.add_static(eng, floor_mesh, cell_pos(x, y, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));

        match cell {
          'o' => {
            scn.add_static(eng, barrel_small, cell_pos(x, y, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
          }
          't' => {
            scn.add_static(eng, trunk, cell_pos(x, y, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
          }
          'b' => {
            scn.add_static(eng, boxobj, cell_pos(x, y, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
          }
          'L' => {
            // Torch: range ~4 units, lights at head height for tighter falloff on floor
            let l = Light::new(cell_pos(x, y, 2.0), 12.0, col8(255, 250, 200), 1.0, 5.0, true, false);
            scn.add_light(l);
          }
          _ => {}
        };

        let west = grid[y][x - 1];
        let east = grid[y][x + 1];
        let north = grid[y - 1][x];
        let south = grid[y + 1][x];
        if north == '.' {
          scn.add_static(eng, wall, v3(xf * CELL_SIZE, 0.0, yf * CELL_SIZE - 2.24), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if south == '.' {
          scn.add_static(eng, wall, v3(xf * CELL_SIZE, 0.0, yf * CELL_SIZE + 2.24), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if east == '.' {
          scn.add_static(eng, wall, v3(xf * CELL_SIZE + 2.24, 0.0, yf * CELL_SIZE), v3(0.0, PI / 2.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if west == '.' {
          scn.add_static(eng, wall, v3(xf * CELL_SIZE - 2.24, 0.0, yf * CELL_SIZE), v3(0.0, PI / 2.0, 0.0), v3(1.0, 1.0, 1.0));
        }
      }
    }
  }

  let px = 2;
  let py = 2;

  let camera = FpsCamera::new(
    3.4,
    Camera::new_perspective(eng.get_aspect(), cell_pos(px, py, CAM_HEIGHT), cell_pos(px - 1, py, 2.5), 70.0, 0.01, 50.0).unwrap(),
  );

  scn.ambient_light = BLACK;
  scn.bake_static_lighting();

  println!("=== DUNGEON SCENE ===");
  println!("Static meshes: {}", scn.get_stats(eng).1);
  println!("Lights: {}", scn.get_stats(eng).2);
  println!("Total Tris: {}\n", scn.get_stats(eng).3);

  MyApp { camera, scn }
}

fn cell_pos(x: usize, y: usize, z: f32) -> Vec3 {
  v3(x as f32 * CELL_SIZE, z, y as f32 * CELL_SIZE)
}
