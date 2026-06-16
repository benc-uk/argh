// ==============================================================================================
// Module & file:   engine / parse_obj.rs
// Purpose:         Parser for OBJ and MTL
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           There's a naming confusion between Mesh and Model
// ==============================================================================================

use std::path::Path;
use tobj::Material;

use crate::{
  colour::Colour,
  engine::ModelHandle,
  material::{BlendMode::AlphaBlend, Material as ArghMaterial},
  mesh::Mesh as ArghMesh,
  model::Model as ArghModel,
  prelude::{v2, v3},
  texture::{Texture, TextureError},
};

use super::Engine;

#[cfg(test)]
#[path = "../tests/parse_obj_tests.rs"]
mod parse_obj_tests;

#[derive(thiserror::Error)]
pub enum ObjError {
  #[error("failed to load .obj file: {0}")]
  Load(#[from] tobj::LoadError),

  #[error("texture loading error {0}")]
  Texture(#[from] TextureError),
}

impl std::fmt::Debug for ObjError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self, f)
  }
}

impl Engine {
  /// Load & parse an OBJ file, this will also load any referenced MTL files
  /// Resulting model will be loaded into the engine and handle returned
  /// If material loading fails a placeholder will be used
  pub fn load_obj(&mut self, path: &str) -> Result<ModelHandle, ObjError> {
    let loaded_obj = tobj::load_obj(
      path,
      &tobj::LoadOptions {
        single_index: true,
        triangulate: true,
        ignore_points: true,
        ignore_lines: true,
      },
    )?;

    let (models, materials) = loaded_obj;

    // Materials might report a separate error
    let materials = match materials {
      Ok(m) => m,
      Err(e) => {
        println!("Material error: {}. Will fallback to placeholder material", e);
        vec![]
      }
    };

    println!("Parsing {}", path);
    println!("  meshes: {}", models.len());
    println!("  materials: {}", materials.len());

    let mut out_model = ArghModel::new(Path::new(path).file_stem().unwrap().to_str().unwrap());

    // Now supporting multiple models (aka meshes!)
    for (i, in_model) in models.iter().enumerate() {
      println!("  Mesh {}: {}", i, &in_model.name);
      let mut out_mesh = ArghMesh::new();
      let in_mesh = &in_model.mesh;

      // Positions: flat [x,y,z, x,y,z, ...] so walk in non-overlapping chunks of 3
      for chunk in in_mesh.positions.chunks_exact(3) {
        out_mesh.positions.push(v3(chunk[0], chunk[1], chunk[2]));
      }
      println!("    pos verts: {}", out_mesh.positions.len());

      if !in_mesh.texcoords.is_empty() {
        // Texture coords (UVs): flat [u,v, u,v, ...]
        for chunk in in_mesh.texcoords.chunks_exact(2) {
          // We flip Y or V texture coord, super important
          out_mesh.tex_coords.push(v2(chunk[0], 1.0 - chunk[1]));
        }
        println!("    tex uvs: {}", out_mesh.tex_coords.len());
      } else {
        println!("    tex uvs: none");
        out_mesh.tex_coords = vec![v2(0.0, 0.0); out_mesh.positions.len()];
      }

      // Normals: flat [x,y,z, x,y,z, ...]
      for chunk in in_mesh.normals.chunks_exact(3) {
        out_mesh.normals.push(v3(chunk[0], chunk[1], chunk[2]));
      }
      println!("    normals: {}", out_mesh.normals.len());

      // Thanks to single_index in load options we just need to do this
      for i in &in_mesh.indices {
        out_mesh.indices.push(*i);
      }

      out_mesh.tri_count = out_mesh.indices.len() as u32 / 3;
      println!("    tris: {}", out_mesh.tri_count);

      // Add matching material to this mesh
      if let Some(mat_id) = in_mesh.material_id {
        let in_material = materials.get(mat_id).unwrap();
        let m = parse_mtl(in_material, path);
        out_mesh.material = m?;
      }

      out_mesh.name = format!("{}_{}", in_model.name, i);

      out_model.add_mesh(out_mesh);
    }

    Ok(self.add_model(out_model))
  }
}

// Convert tobj::Material (MTL) into argh::Material
pub fn parse_mtl(in_material: &Material, path: &str) -> Result<ArghMaterial, ObjError> {
  println!("  Material: {}", in_material.name);

  // Default to flat colour
  let diff_col = in_material.diffuse.unwrap_or([1.0, 1.0, 1.0]);
  let mut mat = ArghMaterial::new_flat(Colour::from_slice(diff_col));
  println!("    diffuse: {}", mat.diffuse);

  // We only support `map_Kd` (diffuse texture)
  if let Some(diffuse_texture_file) = &in_material.diffuse_texture {
    let dir = Path::new(path).parent().unwrap_or(Path::new(""));
    let tex_path = dir.join(diffuse_texture_file);
    println!("    texture: {}", tex_path.display());

    let tex = Texture::new(tex_path.to_str().unwrap())?;
    mat = ArghMaterial::new_textured(tex);
  }

  mat.diffuse = Colour::from_slice(diff_col);
  mat.specular = Colour::from_slice(in_material.specular.unwrap_or([0.0, 0.0, 0.0]));
  mat.hardness = in_material.shininess.unwrap_or(20.0);
  mat.opacity = in_material.dissolve.unwrap_or(1.0);

  // Default to enabling AlphaBlend when opacity below threshold
  if mat.opacity < 0.9 {
    mat.blend_mode = AlphaBlend
  }

  println!("        D:{} S:{} H:{}", mat.diffuse, mat.specular, mat.hardness);
  println!("        O:{} B:{:?}", mat.opacity, mat.blend_mode);

  Ok(mat)
}
