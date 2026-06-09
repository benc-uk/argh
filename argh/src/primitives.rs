// ==============================================================================================
// Module & file:   primitives.rs
// Purpose:         Helper functions for generating meshes of primitive and simple shapes
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{core::Material, core::Mesh, core::Model, math::*};

/// Create a mesh for a unit cube, two triangles per face.
///
/// Verts are duplicated per face (24 verts total) so each face can carry its
/// own outward-facing normal. This keeps the cube cleanly flat-shaded under
/// per-vertex (Gouraud) lighting; a shared-vertex cube would smooth across
/// the edges and look like a wonky sphere.
pub fn new_cube(mat: Material) -> Model {
  let mut mesh = Mesh::new_with_material(mat);

  // 6 faces * 4 verts each = 24 verts. Order per face: bl, br, tr, tl
  // (relative to looking AT the face from outside the cube).
  mesh.verts = vec![
    // front (+Z)
    Vec3::new(-0.5, -0.5, 0.5),
    Vec3::new(0.5, -0.5, 0.5),
    Vec3::new(0.5, 0.5, 0.5),
    Vec3::new(-0.5, 0.5, 0.5),
    // back (-Z)
    Vec3::new(0.5, -0.5, -0.5),
    Vec3::new(-0.5, -0.5, -0.5),
    Vec3::new(-0.5, 0.5, -0.5),
    Vec3::new(0.5, 0.5, -0.5),
    // left (-X)
    Vec3::new(-0.5, -0.5, -0.5),
    Vec3::new(-0.5, -0.5, 0.5),
    Vec3::new(-0.5, 0.5, 0.5),
    Vec3::new(-0.5, 0.5, -0.5),
    // right (+X)
    Vec3::new(0.5, -0.5, 0.5),
    Vec3::new(0.5, -0.5, -0.5),
    Vec3::new(0.5, 0.5, -0.5),
    Vec3::new(0.5, 0.5, 0.5),
    // top (+Y)
    Vec3::new(-0.5, 0.5, 0.5),
    Vec3::new(0.5, 0.5, 0.5),
    Vec3::new(0.5, 0.5, -0.5),
    Vec3::new(-0.5, 0.5, -0.5),
    // bottom (-Y)
    Vec3::new(-0.5, -0.5, -0.5),
    Vec3::new(0.5, -0.5, -0.5),
    Vec3::new(0.5, -0.5, 0.5),
    Vec3::new(-0.5, -0.5, 0.5),
  ];

  // One outward normal per vert, all four corners of a face share the same value.
  let face_normals: [Vec3; 6] = [
    Vec3::new(0.0, 0.0, 1.0),  // front
    Vec3::new(0.0, 0.0, -1.0), // back
    Vec3::new(-1.0, 0.0, 0.0), // left
    Vec3::new(1.0, 0.0, 0.0),  // right
    Vec3::new(0.0, 1.0, 0.0),  // top
    Vec3::new(0.0, -1.0, 0.0), // bottom
  ];

  mesh.normals = Vec::with_capacity(24);
  for n in face_normals {
    for _ in 0..4 {
      mesh.normals.push(n);
    }
  }

  // Per-face UVs: same texture on every face, mapped 0..1 in both axes.
  // Order matches the vert order per face: bl, br, tr, tl.
  // v=0 at top of texture (image-memory convention), v=1 at bottom.
  let face_uvs: [Vec2; 4] = [
    Vec2::new(0.0, 1.0), // bl
    Vec2::new(1.0, 1.0), // br
    Vec2::new(1.0, 0.0), // tr
    Vec2::new(0.0, 0.0), // tl
  ];

  mesh.uvs = Vec::with_capacity(24);
  for _ in 0..6 {
    mesh.uvs.extend_from_slice(&face_uvs);
  }

  // 12 triangles, CCW winding when viewed from outside the cube.
  // Each face's verts are at base = face_index * 4, with corners (bl, br, tr, tl)
  // at offsets (0, 1, 2, 3): so (bl, br, tr) and (bl, tr, tl) give two CCW tris.
  mesh.indices = Vec::with_capacity(36);
  for face in 0..6_i32 {
    let b = face * 4;
    mesh.indices.extend_from_slice(&[b, b + 1, b + 2, b, b + 2, b + 3]);
  }

  Model::from_mesh(mesh, "cube")
}

/// Create a UV sphere mesh of unit diameter (radius 0.5) at the origin.
///
/// # Arguments
/// * `stacks` - Latitude divisions (rings between the poles). Min 2.
/// * `sectors` - Longitude divisions (slices around the Y axis). Min 3.
///
/// Higher counts mean smoother silhouette, at the cost of triangle count.
/// A reasonable default is (16, 24). Triangle count is roughly
/// 2 * stacks * sectors (less at the poles).
///
/// Normals are per-vertex and point radially outward from the origin, ready
/// for smooth (Gouraud) shading. Seam and pole verts are intentionally
/// duplicated to keep indexing uniform.
pub fn new_sphere(mat: Material, stacks: usize, sectors: usize) -> Model {
  let stacks = stacks.max(2);
  let sectors = sectors.max(3);
  let radius: f32 = 0.5;

  let pi = std::f32::consts::PI;

  let mut mesh = Mesh::new_with_material(mat);

  // --- 1. Generate verts + per-vert radial normals.
  // (stacks+1) rings of (sectors+1) verts.
  let vert_count = (stacks + 1) * (sectors + 1);
  mesh.verts = Vec::with_capacity(vert_count);
  mesh.normals = Vec::with_capacity(vert_count);
  mesh.uvs = Vec::with_capacity(vert_count);

  for i in 0..=stacks {
    let phi = pi * (i as f32) / (stacks as f32); // 0 at +Y pole, pi at -Y pole
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let v = i as f32 / stacks as f32; // 0 at top pole, 1 at bottom pole

    for j in 0..=sectors {
      let theta = 2.0 * pi * (j as f32) / (sectors as f32);
      let x = sin_phi * theta.cos();
      let y = cos_phi;
      let z = sin_phi * theta.sin();
      let u = j as f32 / sectors as f32; // 0..1 around the sphere, seam at the back

      // (x, y, z) is already a unit vector on the sphere, so it doubles as the
      // outward normal. Position is just that scaled by the radius.
      mesh.verts.push(Vec3::new(x * radius, y * radius, z * radius));
      mesh.normals.push(Vec3::new(x, y, z));
      mesh.uvs.push(Vec2::new(u, v));
    }
  }

  // --- 2. Build triangles.
  // Each quad cell (i,j) -> (i+1,j+1) emits two triangles, except at the
  // poles where one of them is degenerate and is skipped.
  let row_stride = sectors + 1;
  mesh.indices = Vec::with_capacity(stacks * sectors * 6);

  for i in 0..stacks {
    for j in 0..sectors {
      let v00 = (i * row_stride + j) as i32; // top-left
      let v01 = (i * row_stride + j + 1) as i32; // top-right
      let v10 = ((i + 1) * row_stride + j) as i32; // bottom-left
      let v11 = ((i + 1) * row_stride + j + 1) as i32; // bottom-right

      if i != stacks - 1 {
        mesh.indices.extend_from_slice(&[v00, v11, v10]);
      }

      if i != 0 {
        mesh.indices.extend_from_slice(&[v00, v01, v11]);
      }
    }
  }

  Model::from_mesh(mesh, format!("sphere_{}_{}", stacks, sectors).as_str())
}

/// Utah teapot!
///
/// File format: each triangle is six lines (vert, normal, vert, normal, vert, normal)
/// terminated by a blank line. We push indices (n-3, n-2, n-1) at the blank line,
/// where n is the current vert count, so they reference the three verts we just parsed.
/// A final flush after the loop handles the last triangle if the file has no trailing blank.
pub fn new_teapot(mat: Material) -> Model {
  let txt = include_str!("data/teapot.txt");

  let mut mesh = Mesh::new_with_material(mat);
  let mut lc = 0;

  let flush_tri = |out: &mut Mesh| {
    let n = out.verts.len() as i32;
    out.indices.push(n - 3);
    out.indices.push(n - 2);
    out.indices.push(n - 1);
  };

  for line in txt.lines() {
    if line.is_empty() && lc == 6 {
      flush_tri(&mut mesh);

      lc = 0;
      continue;
    }

    let eles: Vec<f32> = line.split_whitespace().filter_map(|word| word.parse::<f32>().ok()).collect();

    let v = Vec3 {
      x: eles[0],
      y: eles[1],
      z: eles[2],
    };

    if lc % 2 == 0 {
      mesh.verts.push(v);
      mesh.uvs.push(Vec2 { x: 0.0, y: 0.0 });
    } else {
      mesh.normals.push(v);
    }

    lc += 1;
  }

  // Flush trailing triangle if the file didn't end with a blank line.
  if lc == 6 {
    flush_tri(&mut mesh);
  }

  Model::from_mesh(mesh, "teapot")
}
