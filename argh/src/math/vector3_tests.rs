// ==============================================================================================
// Module & file:   math / vector3_tests.rs
// Purpose:         Tests for Vec3 3D vector operations and Mat4 interactions
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;

// --- Constructors ---

#[test]
fn test_new() {
  let v = Vec3::new(3.0, 4.0, 5.0);
  assert_eq!(v, Vec3 { x: 3.0, y: 4.0, z: 5.0 });
}
