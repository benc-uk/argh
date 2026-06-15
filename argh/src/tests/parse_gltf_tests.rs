// ==============================================================================================
// Module & file:   engine / parse_gltf_tests.rs
// Purpose:         Tests for the glTF / GLB parser.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           Uses real glTF assets from the repo.
//                  See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use crate::engine::Engine;
use crate::test_helpers::asset_path;

// --- Happy path with GLB ---

#[test]
fn test_load_gltf_duck_succeeds() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/duck.glb");
  let h = e.load_gltf(p.to_str().unwrap()).expect("duck.glb should load");
  let m = e.model(h);
  assert!(m.tri_count > 0, "duck should have triangles");
}

#[test]
fn test_load_gltf_teapot_low_succeeds() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/utah_teapot_low.glb");
  let h = e.load_gltf(p.to_str().unwrap()).expect("teapot_low should load");
  assert!(e.model(h).tri_count > 0);
}

#[test]
fn test_load_gltf_teapot_lods_increase_in_tri_count() {
  let mut e = Engine::new(64, 64);
  let h_lo = e.load_gltf(asset_path("gltf/utah_teapot_low.glb").to_str().unwrap()).expect("low");
  let h_md = e.load_gltf(asset_path("gltf/utah_teapot_med.glb").to_str().unwrap()).expect("med");
  let h_hi = e.load_gltf(asset_path("gltf/utah_teapot_high.glb").to_str().unwrap()).expect("high");
  let lo = e.model(h_lo).tri_count;
  let md = e.model(h_md).tri_count;
  let hi = e.model(h_hi).tri_count;
  assert!(lo < md, "low ({lo}) should be less than med ({md})");
  assert!(md < hi, "med ({md}) should be less than high ({hi})");
}

// --- Happy path with .gltf + side-car bin ---

#[test]
fn test_load_gltf_potion_bottle_succeeds() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/potion/bottle_A_labeled_green.gltf");
  let h = e.load_gltf(p.to_str().unwrap()).expect("bottle.gltf should load");
  assert!(e.model(h).tri_count > 0);
}

#[test]
fn test_load_gltf_potion_bottle_has_texture() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/potion/bottle_A_labeled_green.gltf");
  let h = e.load_gltf(p.to_str().unwrap()).expect("bottle should load");
  let m = e.model(h);
  let has_tex = m.meshes.iter().any(|mesh| mesh.material.texture().is_some());
  assert!(has_tex, "bottle should have at least one textured mesh");
}

// --- Engine registry side-effects ---

#[test]
fn test_load_gltf_adds_to_engine_model_registry() {
  let mut e = Engine::new(64, 64);
  let count_before = e.models.len();
  let p = asset_path("gltf/duck.glb");
  let _ = e.load_gltf(p.to_str().unwrap()).expect("duck should load");
  assert_eq!(e.models.len(), count_before + 1);
}

#[test]
fn test_load_gltf_duck_name_picked_up_from_glb() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/duck.glb");
  let h = e.load_gltf(p.to_str().unwrap()).expect("duck should load");
  // glTF root node is unnamed in duck.glb so we fall back to "no_name".
  let name = e.model(h).name();
  assert!(!name.is_empty());
}

// --- load_gltf_bytes equivalent path ---

#[test]
fn test_load_gltf_bytes_matches_load_gltf() {
  let p = asset_path("gltf/duck.glb");
  let bytes = std::fs::read(&p).expect("duck bytes");
  let mut e_path = Engine::new(64, 64);
  let mut e_bytes = Engine::new(64, 64);
  let h_path = e_path.load_gltf(p.to_str().unwrap()).expect("path-load duck");
  let h_bytes = e_bytes.load_gltf_bytes(&bytes).expect("bytes-load duck");
  assert_eq!(e_path.model(h_path).tri_count, e_bytes.model(h_bytes).tri_count);
}

// --- Negative cases ---

#[test]
fn test_load_gltf_missing_file_returns_error() {
  let mut e = Engine::new(64, 64);
  assert!(e.load_gltf("definitely/does/not/exist.glb").is_err());
}

#[test]
fn test_load_gltf_bytes_garbage_returns_error() {
  let mut e = Engine::new(64, 64);
  assert!(e.load_gltf_bytes(b"not a glb at all").is_err());
}

// --- Material mapping (PBR -> Phong) ---

#[test]
fn test_load_gltf_duck_material_has_clamped_hardness() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/duck.glb");
  let h = e.load_gltf(p.to_str().unwrap()).expect("duck should load");
  let m = e.model(h);
  // Hardness range is (1.0 - roughness)^2 * 63.0 + 1.0, so always in 1..=64.
  for mesh in &m.meshes {
    let hardness = mesh.material.hardness;
    assert!((1.0 - 1e-4..=64.0 + 1e-4).contains(&hardness), "hardness {hardness} out of expected range");
  }
}

#[test]
fn test_load_gltf_duck_specular_clamped_in_unit_range() {
  // spec_strength = 1 - r^2 is in [0,1], and for a non-metallic material the
  // per-channel result is also in [0,1].
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/duck.glb");
  let h = e.load_gltf(p.to_str().unwrap()).expect("duck should load");
  let m = e.model(h);
  for mesh in &m.meshes {
    let (sr, sg, sb) = (mesh.material.specular.r(), mesh.material.specular.g(), mesh.material.specular.b());
    for c in [sr, sg, sb] {
      assert!((-1e-5..=1.0 + 1e-5).contains(&c), "spec channel {c} out of unit range");
    }
  }
}

#[test]
fn test_load_gltf_mesh_uvs_match_positions() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/duck.glb");
  let h = e.load_gltf(p.to_str().unwrap()).expect("duck should load");
  let m = e.model(h);
  for mesh in &m.meshes {
    assert_eq!(mesh.tex_coords.len(), mesh.positions.len());
    assert_eq!(mesh.normals.len(), mesh.positions.len());
  }
}

#[test]
fn test_load_gltf_indices_triangulated() {
  let mut e = Engine::new(64, 64);
  let p = asset_path("gltf/duck.glb");
  let h = e.load_gltf(p.to_str().unwrap()).expect("duck should load");
  for mesh in &e.model(h).meshes {
    assert!(mesh.indices.len() % 3 == 0, "indices should be a multiple of 3");
  }
}

// --- Default roughness behaviour ---
//
// The glTF 2.0 spec defines `pbrMetallicRoughness.roughnessFactor` to default to
// 1.0 when omitted. The `gltf` crate honours that default. Our PBR -> Phong
// mapping then produces:
//   hardness      = (1.0 - 1.0)^2 * 63.0 + 1.0 = 1.0
//   spec_strength = 1.0 - 1.0^2               = 0.0  =>  specular = (0, 0, 0)
//
// This is a spec-compliant matte material, but it surprises people who expect
// "default = shiny". Lock it in so any future change to the mapping is noticed.

// Build a minimal valid GLB (a single triangle) whose only material declares
// just a name and nothing else, so `roughnessFactor` takes its glTF default.
fn build_minimal_glb_no_roughness() -> Vec<u8> {
  // BIN payload layout (offsets in bytes):
  //   0..36   positions  : 3 * Vec3<f32>
  //  36..72   normals    : 3 * Vec3<f32>
  //  72..96   tex_coords : 3 * Vec2<f32>
  //  96..102  indices    : 3 * u16
  // Padded up to a multiple of 4 for the GLB chunk.
  let mut bin: Vec<u8> = Vec::new();
  for p in [[0.0_f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
    for f in p {
      bin.extend_from_slice(&f.to_le_bytes());
    }
  }
  for n in [[0.0_f32, 0.0, 1.0]; 3] {
    for f in n {
      bin.extend_from_slice(&f.to_le_bytes());
    }
  }
  for uv in [[0.0_f32, 0.0]; 3] {
    for f in uv {
      bin.extend_from_slice(&f.to_le_bytes());
    }
  }
  for i in [0u16, 1, 2] {
    bin.extend_from_slice(&i.to_le_bytes());
  }
  while !bin.len().is_multiple_of(4) {
    bin.push(0);
  }

  // Material has no pbrMetallicRoughness object => everything stays at glTF defaults.
  let json = r#"{
    "asset": {"version": "2.0"},
    "scene": 0,
    "scenes": [{"nodes": [0]}],
    "nodes": [{"mesh": 0}],
    "meshes": [{"primitives": [{"attributes": {"POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2}, "indices": 3, "material": 0}]}],
    "materials": [{"name": "no_pbr_set"}],
    "accessors": [
      {"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3", "min": [0.0,0.0,0.0], "max": [1.0,1.0,0.0]},
      {"bufferView": 1, "componentType": 5126, "count": 3, "type": "VEC3"},
      {"bufferView": 2, "componentType": 5126, "count": 3, "type": "VEC2"},
      {"bufferView": 3, "componentType": 5123, "count": 3, "type": "SCALAR"}
    ],
    "bufferViews": [
      {"buffer": 0, "byteOffset": 0,  "byteLength": 36, "target": 34962},
      {"buffer": 0, "byteOffset": 36, "byteLength": 36, "target": 34962},
      {"buffer": 0, "byteOffset": 72, "byteLength": 24, "target": 34962},
      {"buffer": 0, "byteOffset": 96, "byteLength": 6,  "target": 34963}
    ],
    "buffers": [{"byteLength": 102}]
  }"#;
  let mut json_bytes = json.as_bytes().to_vec();
  while !json_bytes.len().is_multiple_of(4) {
    json_bytes.push(b' ');
  }

  let total_len = 12 + 8 + json_bytes.len() + 8 + bin.len();
  let mut glb: Vec<u8> = Vec::with_capacity(total_len);
  glb.extend_from_slice(b"glTF");
  glb.extend_from_slice(&2u32.to_le_bytes());
  glb.extend_from_slice(&(total_len as u32).to_le_bytes());
  // JSON chunk
  glb.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
  glb.extend_from_slice(b"JSON");
  glb.extend_from_slice(&json_bytes);
  // BIN chunk
  glb.extend_from_slice(&(bin.len() as u32).to_le_bytes());
  glb.extend_from_slice(b"BIN\0");
  glb.extend_from_slice(&bin);
  glb
}

#[test]
fn test_default_roughness_produces_hardness_one() {
  let mut e = Engine::new(64, 64);
  let glb = build_minimal_glb_no_roughness();
  let h = e.load_gltf_bytes(&glb).expect("minimal glb should load");
  let mesh = &e.model(h).meshes[0];
  // (1 - 1)^2 * 63 + 1 == 1.0
  assert!(
    (mesh.material.hardness - 1.0).abs() < 1e-5,
    "default roughness should map to hardness 1.0, got {}",
    mesh.material.hardness
  );
}

#[test]
fn test_default_roughness_produces_zero_specular() {
  let mut e = Engine::new(64, 64);
  let glb = build_minimal_glb_no_roughness();
  let h = e.load_gltf_bytes(&glb).expect("minimal glb should load");
  let mesh = &e.model(h).meshes[0];
  // spec_strength = 1 - r^2 = 0 with r=1, so all channels must be zero.
  let (sr, sg, sb) = (mesh.material.specular.r(), mesh.material.specular.g(), mesh.material.specular.b());
  assert!(sr.abs() < 1e-6, "spec R should be 0, got {sr}");
  assert!(sg.abs() < 1e-6, "spec G should be 0, got {sg}");
  assert!(sb.abs() < 1e-6, "spec B should be 0, got {sb}");
}

#[test]
fn test_default_roughness_diffuse_is_white() {
  // baseColorFactor defaults to [1,1,1,1] when omitted, so the diffuse channel
  // should come out as pure white regardless of the missing pbr block.
  let mut e = Engine::new(64, 64);
  let glb = build_minimal_glb_no_roughness();
  let h = e.load_gltf_bytes(&glb).expect("minimal glb should load");
  let mesh = &e.model(h).meshes[0];
  let (dr, dg, db) = (mesh.material.diffuse.r(), mesh.material.diffuse.g(), mesh.material.diffuse.b());
  assert!((dr - 1.0).abs() < 1e-5);
  assert!((dg - 1.0).abs() < 1e-5);
  assert!((db - 1.0).abs() < 1e-5);
}
