// ==============================================================================================
// Module & file:   math / mod.rs
// Purpose:         Module root for vector and matrix maths types
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

//! This module holds structs & methods for working with vectors and transformation matrices.
//! It follows classic patterns for working with these in a computer graphics context

mod matrix3;
mod vector2;
mod vector3;

pub use matrix3::Mat3;
pub use vector2::Vec2;
pub use vector3::Vec3;
