// ==============================================================================================
// Module & file:   engine / parse_gltf.rs
// Purpose:         Parser for OBJ and MTL
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           There's a naming confusion between Mesh and Model
// ==============================================================================================

use gltf::{Material, Node, material::AlphaMode::Mask};

use crate::{
  engine::ModelHandle,
  material::{MATERIAL_PLACEHOLDER, Material as ArghMaterial},
  math::{Mat3, Mat4},
  mesh::Mesh,
  model::Model,
  prelude::{col, v2, v3},
  texture::Texture,
};

use super::Engine;

// #[derive(thiserror::Error)]
// pub enum GltfError {
//   #[error("texture loading error {0}")]
//   Texture(#[from] gltf::Error),
// }

// impl std::fmt::Debug for GltfError {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     std::fmt::Display::fmt(self, f)
//   }
// }

impl Engine {
  /// Load & parse an GLTF or GLB file
  /// Resulting model will be loaded into the engine and handle returned
  pub fn load_gltf(&mut self, path: &str) -> Result<ModelHandle, gltf::Error> {
    let (doc, buffers, images) = gltf::import(path)?;

    if doc.scenes().len() != 1 {
      println!("Warning GLTF/GLB file with multiple scenes, only default will be used")
    }

    let scene = doc.default_scene().unwrap();

    println!("Loading GLTF '{}', found default scene: {}", path, scene.name().unwrap_or("no_name"));

    if scene.nodes().len() > 1 {
      println!("Warning GLTF/GLB scene with multiple nodes, only first will be parsed as a model")
    }

    let root_node = scene.nodes().next().expect("GLTF scene has no nodes");

    let mut model = Model::new(root_node.name().unwrap_or("no_name"));

    // Recurse over the nodes in the scene, each one contains meshes etc
    // We start with an "blank" parent transformation matrix for root node
    process_node(&root_node, &mut model, &buffers, &images, Mat4::new());

    let hdl = self.add_model(model);

    Ok(hdl)
  }
}

fn process_node(node: &Node, model: &mut Model, buffers: &Vec<gltf::buffer::Data>, images: &Vec<gltf::image::Data>, parent_trans: Mat4) {
  println!("Parsing node: {}", node.name().unwrap_or("no_name"));

  let node_trans = Mat4::from(node.transform().matrix());
  let trans = parent_trans * node_trans; // Combine with parent
  let normal_trans = Mat3::from_mat4_upper(&trans).inverse_transpose().unwrap_or_default();

  if let Some(mesh) = node.mesh() {
    let mesh_name = match mesh.name() {
      Some(n) => n.to_string(),
      None => format!("mesh_{}", mesh.index()),
    };

    println!("  Mesh: {}", mesh_name);

    for prim in mesh.primitives() {
      let mut out_mesh = Mesh::new();
      out_mesh.name = format!("{}_{}", mesh_name, prim.index());
      println!("    Primitive: {} (Type: {:?})", prim.index(), prim.mode());

      let in_mat = prim.material();
      println!("      Material: {}", in_mat.name().unwrap_or("no_name"));

      let reader = prim.reader(|buf| Some(&buffers[buf.index()]));

      if let Some(iter) = reader.read_positions() {
        for [x, y, z] in iter {
          out_mesh.positions.push(trans.transform_point(&v3(x, y, z)));
        }
      }

      if let Some(iter) = reader.read_normals() {
        for [x, y, z] in iter {
          out_mesh.normals.push(normal_trans * &v3(x, y, z));
        }
      }

      if let Some(iter) = reader.read_tex_coords(0) {
        for [u, v] in iter.into_f32() {
          out_mesh.tex_coords.push(v2(u, v));
        }
      }

      if let Some(iter) = reader.read_indices() {
        for i in iter.into_u32() {
          out_mesh.indices.push(i);
        }
      } else {
        // No indices!? Apparently a 'Non-indexed primitive' is thing in glTF, who-knew
        for i in 0..out_mesh.positions.len() {
          out_mesh.indices.push(i as u32);
        }
      }
      println!("    Verts: {} Indices: {}", out_mesh.positions.len(), out_mesh.indices.len());

      out_mesh.material = parse_material(in_mat, images);
      model.add_mesh(out_mesh);
    }
  }

  // Recurse!
  for child_node in node.children() {
    process_node(&child_node, model, buffers, images, trans);
  }
}

fn parse_material(mat: Material, images: &[gltf::image::Data]) -> ArghMaterial {
  let mut out_mat = MATERIAL_PLACEHOLDER;

  // Textured material uses pbr_metallic_roughness().base_color_texture()
  // Don't fucking ask ok...

  if let Some(base_tex) = mat.pbr_metallic_roughness().base_color_texture() {
    // Find the image data this is why we've lugged images Vec all the way down here
    let img_idx = base_tex.texture().source().index();
    let img_data = &images[img_idx];

    let mut tex = Texture::from_raw_rgba8(&img_data.pixels, img_data.width, img_data.height);
    tex.alpha_cutout = mat.alpha_mode() == Mask;

    out_mat = ArghMaterial::new_textured(tex);
  } else {
    // Without a texture the base colour is in base_color_factor()
    let bcf = mat.pbr_metallic_roughness().base_color_factor();
    out_mat.diffuse = col(bcf[0], bcf[1], bcf[2]);
  }

  out_mat
}
