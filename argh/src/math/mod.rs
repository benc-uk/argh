// ==============================================================================================
// Module & file:   math / mod.rs
// Purpose:         Module root for vector and matrix maths types
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

//! This module holds structs & methods for working with 2D and 3D vectors plus transformation matrices, in order to support most 3D graphics operations and algorithms

mod affine2;
mod matrix3;
mod matrix4;
mod quat;
mod vector2;
mod vector3;
mod vector4;

pub use affine2::Affine2;
pub use matrix3::Mat3;
pub use matrix4::Mat4;
pub use quat::Quat;
pub use vector2::Vec2;
pub use vector3::*;
pub use vector4::Vec4;

#[cfg(test)]
#[path = "../tests/math_audit_tests.rs"]
mod audit_tests;
