// ==============================================================================================
// Module & file:   engine / parse_gltf.rs
// Purpose:         Parser for OBJ and MTL
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           There's a naming confusion between Mesh and Model
// ==============================================================================================

use gltf::{
  Material, Node,
  image::Format::{R8G8B8, R8G8B8A8},
  material::AlphaMode::Mask,
};

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

#[cfg(test)]
#[path = "../tests/parse_gltf_tests.rs"]
mod parse_gltf_tests;

struct GltfData {
  buffers: Vec<gltf::buffer::Data>,
  textures: Vec<gltf::image::Data>,
}

impl Engine {
  /// Load & parse an GLTF or GLB file
  /// Resulting model will be loaded into the engine and handle returned
  pub fn load_gltf(&mut self, path: &str) -> Result<ModelHandle, gltf::Error> {
    // We call import not load, this will also fetch the buffers and textures
    let (doc, buffers, textures) = gltf::import(path)?;

    if doc.scenes().len() != 1 {
      println!("Warning GLTF/GLB file with multiple scenes, only default will be used")
    }

    let scene = doc.default_scene().unwrap();

    println!("Loading: {} found default scene: {}", path, scene.name().unwrap_or("no_name"));

    if scene.nodes().len() > 1 {
      println!("Warning GLTF/GLB scene with multiple nodes, only first will be parsed as a model")
    }

    let root_node = scene.nodes().next().expect("GLTF scene has no nodes");

    let mut model = Model::new(root_node.name().unwrap_or("no_name"));

    let mut data = GltfData { buffers, textures };

    // Recurse over the nodes in the scene, each one contains meshes etc
    // We start with an "blank" parent transformation matrix for root node
    data.process_node(&mut model, &root_node, Mat4::new());

    let hdl = self.add_model(model);

    Ok(hdl)
  }

  /// Load & parse an GLTF or GLB file provided as raw bytes
  /// Resulting model will be loaded into the engine and handle returned
  pub fn load_gltf_bytes(&mut self, bytes: &[u8]) -> Result<ModelHandle, gltf::Error> {
    // We call import not load, this will also fetch the buffers and textures
    let (doc, buffers, textures) = gltf::import_slice(bytes)?;

    if doc.scenes().len() != 1 {
      println!("Warning GLTF/GLB file with multiple scenes, only default will be used")
    }

    let scene = doc.default_scene().unwrap();

    println!("Loading GLTF/GLB bytes found default scene: {}", scene.name().unwrap_or("no_name"));

    if scene.nodes().len() > 1 {
      println!("Warning GLTF/GLB scene with multiple nodes, only first will be parsed as a model")
    }

    let root_node = scene.nodes().next().expect("GLTF scene has no nodes");

    let mut model = Model::new(root_node.name().unwrap_or("no_name"));

    let mut data = GltfData { buffers, textures };

    // Recurse over the nodes in the scene, each one contains meshes etc
    // We start with an "blank" parent transformation matrix for root node
    data.process_node(&mut model, &root_node, Mat4::new());

    let hdl = self.add_model(model);

    Ok(hdl)
  }
}

impl GltfData {
  fn process_node(&mut self, out: &mut Model, node: &Node, parent_trans: Mat4) {
    println!("Parsing node: {} with {} children", node.name().unwrap_or("no_name"), node.children().len());

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

        let reader = prim.reader(|buf| Some(&self.buffers[buf.index()]));

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

        out_mesh.tri_count = (out_mesh.indices.len() / 3) as u32;
        out_mesh.material = self.parse_material(prim.material());

        // Add mesh to the output model
        out.add_mesh(out_mesh);
      }
    }

    // Recurse!
    for child_node in node.children() {
      self.process_node(out, &child_node, trans);
    }
  }

  // Convert glTF material into an argh Material
  fn parse_material(&self, mat: Material) -> ArghMaterial {
    println!("      Material: {}", mat.name().unwrap_or("no_name"));
    let mut out_mat = MATERIAL_PLACEHOLDER;

    // Textured material uses pbr_metallic_roughness().base_color_texture()
    // Don't fucking ask ok...

    if let Some(base_tex) = mat.pbr_metallic_roughness().base_color_texture() {
      // Find the image data it's not inside the material but in the textures vec
      let img_idx = base_tex.texture().source().index();
      let img_data = &self.textures[img_idx];
      let (w, h) = (img_data.width, img_data.height);

      // There's many different texture formats, we just support RGBA and RGB
      let tex_opt = match img_data.format {
        R8G8B8A8 => Some(Texture::from_raw_rgba8(&img_data.pixels, w, h)),
        R8G8B8 => Some(Texture::from_raw_rgb8(&img_data.pixels, w, h)),
        _ => None,
      };

      if let Some(mut tex) = tex_opt {
        tex.alpha_cutout = mat.alpha_mode() == Mask;
        out_mat = ArghMaterial::new_textured(tex);
      }
    }

    let pbr = mat.pbr_metallic_roughness();

    let metallic = pbr.metallic_factor(); // 0..1
    let roughness = pbr.roughness_factor(); // 0..1
    let base = pbr.base_color_factor(); // [r,g,b,a]

    // Specular colour: white for dielectrics, tinted by base for metals.
    // Modulate by (1 - roughness²) so matte surfaces have weak spec but
    // moderately rough ones still show a visible highlight (gentler than 1 - r).
    let spec_strength = 1.0 - roughness * roughness;
    let spec_r = ((1.0 - metallic) * 1.0 + metallic * base[0]) * spec_strength;
    let spec_g = ((1.0 - metallic) * 1.0 + metallic * base[1]) * spec_strength;
    let spec_b = ((1.0 - metallic) * 1.0 + metallic * base[2]) * spec_strength;

    // Hardness from roughness. The classical "shininess = 2/a^4 - 2" mapping
    // is way too tight for a software Phong renderer. Use a gentle quadratic
    // ease-out capped at 64:
    //   roughness 0.0 -> 64 (tight-ish highlight)
    //   roughness 0.5 -> ~16
    //   roughness 1.0 ->  1 (broad, invisible once spec_strength = 0)
    let hardness = (1.0 - roughness).powi(2) * 63.0 + 1.0;

    out_mat.diffuse = col(base[0], base[1], base[2]);
    out_mat.specular = col(spec_r, spec_g, spec_b);
    out_mat.hardness = hardness;

    println!("        D:{} S:{} H:{}", out_mat.diffuse, out_mat.specular, out_mat.hardness);

    out_mat
  }
}
