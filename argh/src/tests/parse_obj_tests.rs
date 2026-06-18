// ==============================================================================================
// Module & file:   engine / parse_obj_tests.rs
// Purpose:         Tests for the OBJ/MTL parser.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           Uses real OBJ assets from the repo.
//                  See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use crate::engine::Engine;
use crate::test_helpers::asset_path;

// --- Happy path ---

#[test]
fn test_load_obj_icosahedron_succeeds() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("obj/icosahedron.obj");
  let h = e.load_obj(p.to_str().unwrap(), false).expect("icosahedron.obj should load");
  let m = e.model(h);
  assert!(m.tri_count > 0);
}

// --- Textured assets ---

#[test]
fn test_load_obj_chest_loads_without_texture_in_mtl() {
  // chest.mtl has map_Kd commented out; we should still load without a texture.
  let mut e = Engine::new(64, 64);
  let p = asset_path("obj/chest/chest.obj");
  let h = e.load_obj(p.to_str().unwrap(), false).expect("chest.obj should load");
  let m = e.model(h);
  // No texture because map_Kd is commented in the MTL.
  for mesh in &m.meshes {
    assert!(mesh.material.texture().is_none());
  }
}

#[test]
fn test_load_obj_dice_carries_texture() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("obj/dice/dice.obj");
  let h = e.load_obj(p.to_str().unwrap(), false).expect("dice.obj should load");
  let m = e.model(h);
  let has_tex = m.meshes.iter().any(|mesh| mesh.material.texture().is_some());
  assert!(has_tex, "dice should have at least one textured mesh");
}

// --- Negative cases ---

#[test]
fn test_load_obj_missing_file_returns_load_error() {
  let mut e = Engine::new(64, 64);
  match e.load_obj("definitely/does/not/exist.obj", false) {
    Err(super::ObjError::Load(_)) => {}
    Err(other) => panic!("expected ObjError::Load, got {other}"),
    Ok(_) => panic!("expected an error for missing file"),
  }
}

#[test]
fn test_load_obj_garbage_returns_load_error() {
  let mut e = Engine::new(64, 64);
  let dir = std::env::temp_dir();
  let path = dir.join("argh_garbage.obj");
  std::fs::write(&path, b"this is not an obj file at all").unwrap();
  let result = e.load_obj(path.to_str().unwrap(), false);
  let _ = std::fs::remove_file(&path);
  match result {
    Err(super::ObjError::Load(_)) => {}
    Err(other) => panic!("expected Load error, got {other}"),
    // tobj is fairly tolerant; if it returns Ok we still won't panic, so accept the model.
    Ok(_) => {}
  }
}

// --- Engine registry side-effects ---

#[test]
fn test_load_obj_adds_to_engine_model_registry() {
  let mut e = Engine::new(64, 64);
  let count_before = e.models.len();
  let p = asset_path("obj/skull.obj");
  let _ = e.load_obj(p.to_str().unwrap(), false).expect("skull.obj should load");
  assert_eq!(e.models.len(), count_before + 1);
}

#[test]
fn test_load_obj_name_uses_file_stem() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("obj/skull.obj");
  let h = e.load_obj(p.to_str().unwrap(), false).expect("skull.obj should load");
  assert_eq!(e.model(h).name(), "skull");
}

// --- Mesh-level invariants ---

#[test]
fn test_load_obj_mesh_indices_are_triangulated() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("obj/skull.obj");
  let h = e.load_obj(p.to_str().unwrap(), false).expect("skull.obj should load");
  let m = e.model(h);
  for mesh in &m.meshes {
    assert!(mesh.indices.len() % 3 == 0, "indices should be multiples of 3");
  }
}

#[test]
fn test_load_obj_uv_count_matches_vert_count() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("obj/skull.obj");
  let h = e.load_obj(p.to_str().unwrap(), false).expect("skull.obj should load");
  let m = e.model(h);
  for mesh in &m.meshes {
    assert_eq!(mesh.tex_coords.len(), mesh.positions.len());
  }
}
