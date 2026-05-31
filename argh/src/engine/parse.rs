use std::path::Path;

use tobj;

use crate::{
  colour::Colour,
  engine::{MaterialHandle, MeshHandle},
  models::{Material as ArghMaterial, Mesh as ArghMesh, Texture, TextureError},
  prelude::{v2, v3},
};

use super::Engine;

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
  pub fn load_obj(&mut self, path: &str) -> Result<(MeshHandle, MaterialHandle), ObjError> {
    let loaded_obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;

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
    println!("  models: {}", models.len());
    println!("  materials: {}", materials.len());

    if models.len() > 1 {
      println!("  warning! multiple models found, only the first will be parsed");
    }

    // Not supporting multiple models/parts, so only get first
    let model = models.first().unwrap(); // Will never panic we check first 

    println!("\n  Model: {}", &model.name);
    let mut out_mesh = ArghMesh::new(&model.name);
    let in_mesh = &model.mesh;

    // Positions: flat [x,y,z, x,y,z, ...] so walk in non-overlapping chunks of 3
    for chunk in in_mesh.positions.chunks_exact(3) {
      out_mesh.verts.push(v3(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
    }
    println!("  pos verts: {}", out_mesh.verts.len());

    // Texture coords (UVs): flat [u,v, u,v, ...]
    for chunk in in_mesh.texcoords.chunks_exact(2) {
      // We flip Y or V texture coord
      out_mesh.uvs.push(v2(chunk[0] as f64, 1.0 - chunk[1] as f64));
    }
    println!("  tex uvs: {}", out_mesh.uvs.len());

    // Normals: flat [x,y,z, x,y,z, ...]
    for chunk in in_mesh.normals.chunks_exact(3) {
      out_mesh.normals.push(v3(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
    }
    println!("  normals: {}", out_mesh.normals.len());

    // Faces Indexes
    for i in &in_mesh.indices {
      out_mesh.indices.push(*i as i32);
    }

    // Get matching material
    let mut mat_hdl = self.mat_placeholder;
    if let Some(mat_id) = in_mesh.material_id {
      let in_material = materials.get(mat_id).unwrap();
      println!("\n  Material: {}", in_material.name);

      let diff_col = in_material.diffuse.unwrap_or([1.0, 1.0, 1.0]);
      let mut tex = Texture::Solid(Colour::from_slice(diff_col));

      // We only support `map_Kd` (diffuse texture)
      if let Some(diffuse_texture_file) = &in_material.diffuse_texture {
        let dir = Path::new(path).parent().unwrap_or(Path::new(""));
        let tex_path = dir.join(diffuse_texture_file);
        println!("    texture: {}", tex_path.display());

        tex = Texture::image(tex_path.to_str().unwrap())?;
      }

      let mut mat = ArghMaterial::new(tex);
      mat.hardness = in_material.shininess.unwrap_or(20.0) as f64;
      mat_hdl = self.add_material(mat);
    }

    Ok((self.add_mesh(out_mesh), mat_hdl))
  }
}
