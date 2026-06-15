// ==============================================================================================
// Purpose:         Test harness for benchmarking
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// ==============================================================================================

use std::f32::consts::PI;

use argh::{engine::Key, prelude::*};

pub struct MyApp {
  camera: Camera,
  scn: Scene,
  px: usize,
  py: usize,
  lx: usize,
  ly: usize,
  frame: u32,
}

const CAM_HEIGHT: f32 = 12.5;

impl MyApp {
  fn move_cam(&mut self, px: usize, py: usize) {
    self.px = px;
    self.py = py;
    println!("Moving camera to: {},{}", self.px, self.py);
    self.camera.set_pos(cell_pos(self.px, self.py, CAM_HEIGHT));
    self.camera.set_look_at(cell_pos(self.px + self.lx, self.py + self.ly, 2.5));
  }
}

impl App for MyApp {
  fn update(&mut self, eng: &mut Engine, _dt: f64, _t: f64) {
    eng.clear(BLACK);

    eng.render(&self.camera, &self.scn);

    // Move camera to difference places
    if self.frame == 200 {
      self.move_cam(3, 2)
    }
    if self.frame == 400 {
      self.move_cam(6, 6)
    }
    if self.frame == 600 {
      self.move_cam(7, 1)
    }
    if self.frame == 800 {
      self.move_cam(2, 7)
    }

    if !eng.keys_pressed().is_empty() {
      // Quit on escape
      if eng.is_pressed(Key::Escape) {
        eng.stop();
      }

      // Quit on escape
      if eng.is_pressed(Key::Up) {
        self.move_cam(self.px, usize::clamp(self.py + 1, 0, 10));
      }
      if eng.is_pressed(Key::Down) {
        self.move_cam(self.px, usize::clamp(self.py - 1, 0, 10));
      }
      if eng.is_pressed(Key::Left) {
        self.move_cam(usize::clamp(self.px + 1, 0, 10), self.py);
      }
      if eng.is_pressed(Key::Right) {
        self.move_cam(usize::clamp(self.px - 1, 0, 10), self.py);
      }
    }
    self.frame += 1;
  }
}

pub fn new(eng: &mut Engine) -> MyApp {
  let mut scn = Scene::new();
  scn.ambient_light = BLACK;

  let floor = eng.load_obj("assets/obj/dungeon/floor_tile_large.obj").expect("obj loading failed");
  // let grate = eng.load_obj("assets/obj/dungeon/floor_tile_big_grate.obj").expect("obj loading failed");
  // let floor_rocks = eng.load_obj("assets/obj/dungeon/floor_tile_large_rocks.obj").expect("obj loading failed");

  let wall = eng.load_obj("assets/obj/dungeon/wall.obj").expect("obj loading failed");

  let barrel_small = eng.load_obj("assets/obj/dungeon/barrel_large.obj").expect("obj loading failed");
  let trunk = eng.load_obj("assets/obj/dungeon/trunk_large_C.obj").expect("obj loading failed");
  let boxobj = eng.load_obj("assets/obj/dungeon/box_small_decorated.obj").expect("obj loading failed");

  let grid: Vec<Vec<char>> = DUNGEON_MAP.lines().map(|line| line.chars().collect()).collect();

  for y in 0..grid.len() {
    for x in 0..grid.len() {
      let cell = grid[y][x];
      let xf = x as f32;
      let yf = y as f32;
      if cell != '.' {
        // let floor_mesh = match random_range(1..=6) {
        //   1 => grate,
        //   2 => floor_rocks,
        //   _ => floor,
        // };
        scn.add_static(eng, floor, cell_pos(x, y, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));

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
            let l = Light::new_static(cell_pos(x, y, 3.0), 9.0, col8(255, 250, 200), 0.0, 2.0);
            scn.add_light(l);
          }
          _ => {}
        };

        let west = grid[y][x - 1];
        let east = grid[y][x + 1];
        let north = grid[y - 1][x];
        let south = grid[y + 1][x];
        if north == '.' {
          scn.add_static(eng, wall, v3(xf * CELL_SIZE, 0.0, yf * CELL_SIZE - 2.25), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if south == '.' {
          scn.add_static(eng, wall, v3(xf * CELL_SIZE, 0.0, yf * CELL_SIZE + 2.25), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if east == '.' {
          scn.add_static(eng, wall, v3(xf * CELL_SIZE + 2.25, 0.0, yf * CELL_SIZE), v3(0.0, PI / 2.0, 0.0), v3(1.0, 1.0, 1.0));
        }
        if west == '.' {
          scn.add_static(eng, wall, v3(xf * CELL_SIZE - 2.25, 0.0, yf * CELL_SIZE), v3(0.0, PI / 2.0, 0.0), v3(1.0, 1.0, 1.0));
        }
      }
    }
  }

  let px = 1;
  let py = 1;
  let lx = 0;
  let ly = 1;
  let camera = Camera::new_perspective(eng.aspect(), cell_pos(px, py, CAM_HEIGHT), cell_pos(px + lx, py + ly, 2.5), 70.0, 0.01, 300.0).unwrap();

  scn.bake_static_lighting();

  println!("=== DUNGEON SCENE ===");
  println!("Static meshes: {}", scn.stats(eng).1);
  println!("Lights: {}", scn.stats(eng).2);
  println!("Total Tris: {}\n", scn.stats(eng).3);

  MyApp {
    camera,
    scn,
    px,
    py,
    lx,
    ly,
    frame: 0,
  }
}

fn cell_pos(x: usize, y: usize, z: f32) -> Vec3 {
  v3(x as f32 * CELL_SIZE, z, y as f32 * CELL_SIZE)
}

const CELL_SIZE: f32 = 4.0_f32;
const DUNGEON_MAP: &str = "..........
.......#..
..L#t..L..
..#Lb.L#o.
..o##..#..
....L..#L.
....#..#o.
.L#bL##L..
.#o....bL.
..........";
