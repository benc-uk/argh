// ==============================================================================================
// Purpose:         Attempt at a first person camera with mouse look
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// ==============================================================================================

use argh::{
  camera::Camera,
  engine::{Engine, Key},
  prelude::v3,
};

pub(super) struct FpsCamera {
  pub(super) ang_x: f32,
  pub(super) ang_y: f32,
  pub(super) vel_x: f32,
  pub(super) vel_z: f32,
  pub(super) camera: Camera,
}

// Movement with inertia: accelerate while held, then coast and decay
const ACCEL: f32 = 2.1;
const MAX_SPEED: f32 = 0.4;
const FRICTION: f32 = 0.8;
const YAW_SCALE: f32 = 2.9;
const PITCH_SCALE: f32 = 2.1;
const MOUSE_DEAD_ZONE: f32 = 0.25;

/// Remap a value in [-1, 1] with a dead zone so the output ramps smoothly
/// from 0 at the dead-zone boundary up to +/-1 at the edges.
fn dead_zone(v: f32, dz: f32) -> f32 {
  let abs_v = v.abs();
  if abs_v < dz {
    return 0.0;
  }
  v.signum() * (abs_v - dz) / (1.0 - dz)
}

impl FpsCamera {
  pub fn new(start_ang: f32, camera: Camera) -> Self {
    Self {
      ang_x: 0.0,
      ang_y: start_ang,
      vel_x: 0.0,
      vel_z: 0.0,
      camera,
    }
  }

  pub fn update(&mut self, eng: &Engine, dt: f32) {
    // Mouse look
    if let Some(mpos) = eng.mouse_pos() {
      let (mx, my) = (mpos.0, mpos.1);
      let (w, h) = (eng.size().0 as f32, eng.size().1 as f32);
      let dx = dead_zone(((mx / w) * 2.0) - 1.0, MOUSE_DEAD_ZONE);
      let dy = dead_zone(((my / h) * 2.0) - 1.0, MOUSE_DEAD_ZONE);

      // Apply sensitivity
      self.ang_y -= dx * YAW_SCALE * dt;
      self.ang_x -= dy * PITCH_SCALE * dt;
      self.ang_x = self.ang_x.clamp(-2.5, 2.5);
    }

    // Current look/forward direction
    let d_z = -f32::cos(self.ang_y);
    let d_x = -f32::sin(self.ang_y);
    let d_y = f32::sin(self.ang_x);

    if eng.is_pressed(Key::W) {
      self.vel_x += d_x * ACCEL * dt;
      self.vel_z += d_z * ACCEL * dt;
    }
    if eng.is_pressed(Key::S) {
      self.vel_x -= d_x * ACCEL * dt;
      self.vel_z -= d_z * ACCEL * dt;
    }

    // Strafe: right vector is forward rotated 90 deg in XZ -> (-d_z, d_x)
    if eng.is_pressed(Key::A) {
      self.vel_x += d_z * ACCEL * dt;
      self.vel_z -= d_x * ACCEL * dt;
    }
    if eng.is_pressed(Key::D) {
      self.vel_x -= d_z * ACCEL * dt;
      self.vel_z += d_x * ACCEL * dt;
    }

    // Clamp horizontal speed magnitude
    let speed_sq = self.vel_x * self.vel_x + self.vel_z * self.vel_z;
    if speed_sq > MAX_SPEED * MAX_SPEED {
      let scale = MAX_SPEED / speed_sq.sqrt();
      self.vel_x *= scale;
      self.vel_z *= scale;
    }

    // Apply velocity every frame (so released keys still coast)
    let mut p = self.camera.pos();
    p.x += self.vel_x;
    p.z += self.vel_z;
    self.camera.set_pos(p);
    self.camera.set_look_at(v3(p.x + d_x, p.y + d_y, p.z + d_z));

    // Apply friction every frame
    self.vel_x *= FRICTION;
    self.vel_z *= FRICTION;
  }
}
