use super::*;

#[test]
fn test_new_constructor() {
  let v = Vec2::new(3.0, 4.0);
  assert_eq!(v, Vec2 { x: 3.0, y: 4.0 });
}

#[test]
fn test_default() {
  let v = Vec2::zero();
  assert_eq!(v, Vec2 { x: 0.0, y: 0.0 });
}

#[test]
fn test_add_mutates_in_place() {
  let mut a = Vec2::new(1.0, 2.0);
  a.add(Vec2 { x: 3.0, y: 4.0 });
  assert_eq!(a, Vec2 { x: 4.0, y: 6.0 });
}

#[test]
fn test_add_with_negatives() {
  let mut a = Vec2::new(5.0, 10.0);
  a.add(Vec2 { x: -3.0, y: -7.0 });
  assert_eq!(a, Vec2 { x: 2.0, y: 3.0 });
}

#[test]
fn test_add_with_zeros() {
  let mut a = Vec2::new(1.0, 2.0);
  a.add(Vec2 { x: 0.0, y: 0.0 });
  assert_eq!(a, Vec2 { x: 1.0, y: 2.0 });
}

#[test]
fn test_add_new_returns_sum() {
  let a = Vec2 { x: 1.0, y: 2.0 };
  let b = Vec2 { x: 3.0, y: 4.0 };
  let result = a.add_new(b);
  assert_eq!(result, Vec2 { x: 4.0, y: 6.0 });
}

#[test]
fn test_add_new_with_negatives() {
  let a = Vec2 { x: -1.0, y: -2.0 };
  let b = Vec2 { x: -3.0, y: -4.0 };
  assert_eq!(a.add_new(b), Vec2 { x: -4.0, y: -6.0 });
}

#[test]
fn test_add_new_with_zeros() {
  let a = Vec2 { x: 0.0, y: 0.0 };
  let b = Vec2 { x: 0.0, y: 0.0 };
  assert_eq!(a.add_new(b), Vec2 { x: 0.0, y: 0.0 });
}
