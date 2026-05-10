// ==============================================================================================
// Module & file:   math / matrix4.rs
// Purpose:         4x4 affine transformation matrix for 3D graphics
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::{Quat, Vec3, Vec4};
use core::fmt;
use std::fmt::Formatter;
use std::ops::{Mul, MulAssign};

#[cfg(test)]
#[path = "matrix4_tests.rs"]
mod matrix4_tests;

/// A classic 4x4 affine transformation matrix, designed for transformations on [Vec3] and [Vec4]
/// Rotations are based on quaternions
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Mat4 {
  ele: [[f64; 4]; 4],
}

#[derive(thiserror::Error, Debug)]
pub enum Mat4Error {
  #[error("near plane cannot be zero")]
  NearPlaneZero,
}

impl Mat4 {
  /// New identity matrix
  pub fn new() -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with scale transform set
  pub fn new_scale(sx: f64, sy: f64, sz: f64) -> Self {
    Self {
      ele: [[sx, 0.0, 0.0, 0.0], [0.0, sy, 0.0, 0.0], [0.0, 0.0, sz, 0.0], [0.0, 0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with rotation transform set
  pub fn new_rot(r: Quat) -> Self {
    let x = r.x;
    let y = r.y;
    let z = r.z;
    let w = r.w;

    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;

    Self {
      ele: [
        [1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0],
        [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0],
        [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }

  /// New matrix with translation transform set
  pub fn new_trans(x: f64, y: f64, z: f64) -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [x, y, z, 1.0]],
    }
  }

  /// Convenience & optimisation method - new matrix with scale, rotate, and translation transform set
  pub fn new_scale_rot_trans(sx: f64, sy: f64, sz: f64, r: Quat, tx: f64, ty: f64, tz: f64) -> Self {
    let x = r.x;
    let y = r.y;
    let z = r.z;
    let w = r.w;

    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;

    Self {
      ele: [
        [(1.0 - 2.0 * (yy + zz)) * sx, (2.0 * (xy + wz)) * sx, (2.0 * (xz - wy)) * sx, 0.0],
        [(2.0 * (xy - wz)) * sy, (1.0 - 2.0 * (xx + zz)) * sy, (2.0 * (yz + wx)) * sy, 0.0],
        [(2.0 * (xz + wy)) * sz, (2.0 * (yz - wx)) * sz, (1.0 - 2.0 * (xx + yy)) * sz, 0.0],
        [tx, ty, tz, 1.0],
      ],
    }
  }

  /// Create a matrix which performs perspective transformation
  /// # Arguments:
  /// * `fovy`   - Field of view in radians
  /// * `aspect` - Aspect ratio of the camera
  /// * `near`   - Near clipping plane
  /// * `far`    - Far clipping plane
  /// Clip space is (-w, -w, 0) to (w, w, w) on all three axis
  pub fn new_perspective(fovy: f64, aspect: f64, near: f64, far: f64) -> Result<Self, Mat4Error> {
    if near == 0.0 {
      return Err(Mat4Error::NearPlaneZero);
    }

    let f = 1.0 / (fovy * 0.5).tan(); // cotangent of half-FOV
    let nf = 1.0 / (near - far);

    let mut m = Self::zero();
    m.ele[0][0] = f / aspect; // x scale
    m.ele[1][1] = f; // y scale
    m.ele[2][2] = far * nf; // z remap
    m.ele[2][3] = -1.0; // copies -z_view into clip.w (this is the "perspective" bit)
    m.ele[3][2] = far * near * nf; // z offset

    Ok(m)
  }

  /// Create a right-handed view matrix that places the camera at `eye`, pointing at `target`. Transforms world-space points into view space (camera at origin, looking down -Z).
  /// # Arguments:
  /// * `eye` - Position of the eye or origin of view space
  /// * `target` - What to point or look at
  /// * `up_hint` - as the rough world-up direction (typically (0, 1, 0))
  pub fn new_look_at(eye: Vec3, target: Vec3, up_hint: Vec3) -> Self {
    // Camera basis vectors in world space (right-handed, camera looks down -Z).
    // normalize() mutates in place, so these bindings must be mut.
    let mut forward = eye - target;
    forward.normalize(); // +Z of camera (points away from target)
    let mut right = up_hint.cross(forward);
    right.normalize();
    let up = forward.cross(right); // already unit length (forward and right are orthonormal)

    // Column-major layout: ele[col][row].
    // The basis vectors form the rows of the rotation part (= transpose of camera-to-world).
    // Column 3 is -R * eye, i.e. -dot(axis, eye) per row.
    Self {
      ele: [
        // Column 0: x-components of each basis vector
        [right.x, up.x, forward.x, 0.0],
        // Column 1: y-components
        [right.y, up.y, forward.y, 0.0],
        // Column 2: z-components
        [right.z, up.z, forward.z, 0.0],
        // Column 3: translation
        [-right.dot(eye), -up.dot(eye), -forward.dot(eye), 1.0],
      ],
    }
  }

  /// Create a zero matrix which is of almost no use
  pub fn zero() -> Self {
    Self {
      ele: [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]],
    }
  }

  /// Matrix will translate by the given x & y amounts
  pub fn trans(&mut self, x: f64, y: f64, z: f64) {
    self.ele[3][0] = x;
    self.ele[3][1] = y;
    self.ele[3][2] = z;
  }

  /// Matrix will scale by the given x & y scaling factors
  pub fn scale(&mut self, sx: f64, sy: f64, sz: f64) {
    self.ele[0][0] = sx;
    self.ele[1][1] = sy;
    self.ele[2][2] = sz;
  }
}

impl fmt::Display for Mat4 {
  /// Output this matrix in readable form
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}, {}\n {}, {}, {}, {}\n {}, {}, {}, {},\n {}, {}, {}, {}]",
      self.ele[0][0],
      self.ele[1][0],
      self.ele[2][0],
      self.ele[3][0],
      self.ele[0][1],
      self.ele[1][1],
      self.ele[2][1],
      self.ele[3][1],
      self.ele[0][2],
      self.ele[1][2],
      self.ele[2][2],
      self.ele[3][2],
      self.ele[0][3],
      self.ele[1][3],
      self.ele[2][3],
      self.ele[3][3]
    )
  }
}

impl Mul<&Vec3> for Mat4 {
  type Output = Vec3;

  /// Multiply and transform given Vec3 by this matrix, assumes that w = 1, so will be treated like a point and translated
  fn mul(self, v: &Vec3) -> Vec3 {
    Vec3 {
      x: self.ele[0][0] * v.x + self.ele[1][0] * v.y + self.ele[2][0] * v.z + self.ele[3][0], // implicit * 1 removed
      y: self.ele[0][1] * v.x + self.ele[1][1] * v.y + self.ele[2][1] * v.z + self.ele[3][1], // implicit * 1 removed
      z: self.ele[0][2] * v.x + self.ele[1][2] * v.y + self.ele[2][2] * v.z + self.ele[3][2], // implicit * 1 removed
    }
  }
}

impl Mul<&Vec4> for Mat4 {
  type Output = Vec4;

  /// Multiply and transform given Vec4 by this matrix
  fn mul(self, v: &Vec4) -> Vec4 {
    Vec4 {
      x: self.ele[0][0] * v.x + self.ele[1][0] * v.y + self.ele[2][0] * v.z + self.ele[3][0] * v.w,
      y: self.ele[0][1] * v.x + self.ele[1][1] * v.y + self.ele[2][1] * v.z + self.ele[3][1] * v.w,
      z: self.ele[0][2] * v.x + self.ele[1][2] * v.y + self.ele[2][2] * v.z + self.ele[3][2] * v.w,
      w: self.ele[0][3] * v.x + self.ele[1][3] * v.y + self.ele[2][3] * v.z + self.ele[3][3] * v.w,
    }
  }
}

impl Mul<Self> for Mat4 {
  type Output = Self;

  /// Multiply together two Mat4 to combine or compose them
  fn mul(self, m: Self) -> Self {
    let mut r = Self::zero();
    for col in 0..4 {
      for row in 0..4 {
        r.ele[col][row] = self.ele[0][row] * m.ele[col][0] + self.ele[1][row] * m.ele[col][1] + self.ele[2][row] * m.ele[col][2] + self.ele[3][row] * m.ele[col][3];
      }
    }
    r
  }
}

impl MulAssign<Self> for Mat4 {
  /// Multiply together two Mat3 to combine or compose them, mutate & store in place
  fn mul_assign(&mut self, m: Self) {
    *self = *self * m;
  }
}

impl Mul<&Vec<Vec3>> for Mat4 {
  type Output = Vec<Vec3>;

  /// Multiply a list of points by a matrix
  fn mul(self, points: &Vec<Vec3>) -> Vec<Vec3> {
    let mut out = Vec::with_capacity(points.len());

    for p in points {
      out.push(self * p);
    }

    out
  }
}

impl Mul<&Vec<Vec4>> for Mat4 {
  type Output = Vec<Vec4>;

  /// Multiply a list of points by a matrix
  fn mul(self, points: &Vec<Vec4>) -> Vec<Vec4> {
    let mut out = Vec::with_capacity(points.len());

    for p in points {
      out.push(self * p);
    }

    out
  }
}
