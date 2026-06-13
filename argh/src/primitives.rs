// ==============================================================================================
// Module & file:   primitives.rs
// Purpose:         Helper functions for generating meshes of primitive and simple shapes
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{material::Material, mesh::Mesh, math::*, model::Model};

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

  mesh.tri_count = 12;

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

  // Poles only contribute one triangle per sector instead of two, so the
  // total is 2 * sectors at each pole + 2 * sectors for each of the (stacks-2)
  // middle rings = 2 * (stacks - 1) * sectors.
  mesh.tri_count = (2 * (stacks - 1) * sectors) as u32;

  Model::from_mesh(mesh, format!("sphere_{}_{}", stacks, sectors).as_str())
}

/// Create a cylinder mesh of unit diameter (radius 0.5) and unit height (1.0),
/// centred at the origin with its axis along the Y axis.
///
/// # Arguments
/// * `sectors` - Number of subdivisions around the circumference. Min 3.
/// * `caps` - If true, generate the top and bottom cap discs. If false, the cylinder
///   is open at both ends (a tube).
///
/// Higher sector counts mean a smoother silhouette, at the cost of triangle count.
/// A reasonable default is 24. Triangle count is `2 * sectors` for an open tube,
/// or `4 * sectors` with caps (two per side quad, plus a triangle fan per cap).
///
/// Side normals point radially outward for smooth (Gouraud) shading of the curved
/// surface. Cap normals point straight up (+Y) or down (-Y) so the cap/side seam
/// stays sharp rather than smoothing across the edge. Rim verts are intentionally
/// duplicated between side and caps to allow this.
pub fn new_cylinder(mat: Material, sectors: usize, caps: bool) -> Model {
  let sectors = sectors.max(3);
  let radius: f32 = 0.5;
  let half_h: f32 = 0.5;
  let pi = std::f32::consts::PI;

  let mut mesh = Mesh::new_with_material(mat);

  // Vert buffer layout:
  //   [0 .. side_count)                              side: bottom ring then top ring, each (sectors+1)
  //   [side_count .. side_count + cap_count)         top cap: centre vert + sectors rim verts   (if caps)
  //   [side_count + cap_count .. vert_count)         bottom cap: centre vert + sectors rim verts (if caps)
  let side_count = 2 * (sectors + 1);
  let cap_count = if caps { 1 + sectors } else { 0 };
  let vert_count = side_count + 2 * cap_count;

  mesh.verts = Vec::with_capacity(vert_count);
  mesh.normals = Vec::with_capacity(vert_count);
  mesh.uvs = Vec::with_capacity(vert_count);

  // --- 1. Side: two rings of (sectors+1) verts with a seam duplicate at j=sectors
  // so UVs can wrap 0..1 without sharing a vert across the seam.
  // Normals point radially outward, ready for smooth shading on the curved face.
  for ring in 0..2 {
    let (y, v) = if ring == 0 { (-half_h, 1.0) } else { (half_h, 0.0) };
    for j in 0..=sectors {
      let theta = 2.0 * pi * (j as f32) / (sectors as f32);
      let cx = theta.cos();
      let cz = theta.sin();
      mesh.verts.push(Vec3::new(cx * radius, y, cz * radius));
      mesh.normals.push(Vec3::new(cx, 0.0, cz));
      mesh.uvs.push(Vec2::new(j as f32 / sectors as f32, v));
    }
  }

  // --- 2 & 3. Caps (optional).
  // UVs are a disc centred at (0.5, 0.5) in texture space.
  if caps {
    // Top cap: centre vert + sectors rim verts, all with +Y normals.
    mesh.verts.push(Vec3::new(0.0, half_h, 0.0));
    mesh.normals.push(Vec3::new(0.0, 1.0, 0.0));
    mesh.uvs.push(Vec2::new(0.5, 0.5));
    for j in 0..sectors {
      let theta = 2.0 * pi * (j as f32) / (sectors as f32);
      let cx = theta.cos();
      let cz = theta.sin();
      mesh.verts.push(Vec3::new(cx * radius, half_h, cz * radius));
      mesh.normals.push(Vec3::new(0.0, 1.0, 0.0));
      mesh.uvs.push(Vec2::new(0.5 + 0.5 * cx, 0.5 - 0.5 * cz));
    }

    // Bottom cap: same layout but at y=-half_h with -Y normals.
    mesh.verts.push(Vec3::new(0.0, -half_h, 0.0));
    mesh.normals.push(Vec3::new(0.0, -1.0, 0.0));
    mesh.uvs.push(Vec2::new(0.5, 0.5));
    for j in 0..sectors {
      let theta = 2.0 * pi * (j as f32) / (sectors as f32);
      let cx = theta.cos();
      let cz = theta.sin();
      mesh.verts.push(Vec3::new(cx * radius, -half_h, cz * radius));
      mesh.normals.push(Vec3::new(0.0, -1.0, 0.0));
      mesh.uvs.push(Vec2::new(0.5 + 0.5 * cx, 0.5 - 0.5 * cz));
    }
  }

  // --- 4. Indices. CCW winding when viewed from outside the cylinder.
  let tri_count = if caps { 4 * sectors } else { 2 * sectors } as u32;
  mesh.indices = Vec::with_capacity((tri_count * 3) as usize);

  // Side: each quad (j, j+1) becomes two triangles, matching sphere's winding pattern.
  let bot_base = 0_i32;
  let top_base = (sectors + 1) as i32;
  for j in 0..sectors as i32 {
    let bl = bot_base + j;
    let br = bot_base + j + 1;
    let tl = top_base + j;
    let tr = top_base + j + 1;
    mesh.indices.extend_from_slice(&[tl, br, bl, tl, tr, br]);
  }

  if caps {
    let s = sectors as i32;

    // Top cap fan: (centre, rim[j+1], rim[j]) gives the +Y outward normal.
    let top_centre = side_count as i32;
    let top_rim_base = top_centre + 1;
    for j in 0..s {
      let a = top_rim_base + j;
      let b = top_rim_base + (j + 1) % s;
      mesh.indices.extend_from_slice(&[top_centre, b, a]);
    }

    // Bottom cap fan: (centre, rim[j], rim[j+1]) gives the -Y outward normal.
    let bot_centre = (side_count + cap_count) as i32;
    let bot_rim_base = bot_centre + 1;
    for j in 0..s {
      let a = bot_rim_base + j;
      let b = bot_rim_base + (j + 1) % s;
      mesh.indices.extend_from_slice(&[bot_centre, a, b]);
    }
  }

  mesh.tri_count = tri_count;

  let name = if caps {
    format!("cylinder_{}_capped", sectors)
  } else {
    format!("cylinder_{}_open", sectors)
  };
  Model::from_mesh(mesh, name.as_str())
}

/// Create a cone mesh of unit base diameter (radius 0.5) and unit height (1.0),
/// centred at the origin with its axis along the Y axis. Base sits at y = -0.5,
/// apex at y = +0.5.
///
/// # Arguments
/// * `sectors` - Number of subdivisions around the circumference. Min 3.
/// * `cap` - If true, generate the base cap disc. If false, the cone is open at the base.
///
/// Higher sector counts mean a smoother silhouette, at the cost of triangle count.
/// A reasonable default is 24. Triangle count is `sectors` for an open cone, or
/// `2 * sectors` with the base cap.
///
/// Side normals point along the cone's outward slope (radial component + upward
/// tilt) for smooth (Gouraud) shading of the curved surface. Cap normals point
/// straight down (-Y) so the base/side seam stays sharp. The apex collapses to a
/// single 3D point but each side triangle still carries its own slope-aligned
/// normal at the apex to avoid lighting artefacts at the tip.
pub fn new_cone(mat: Material, sectors: usize, cap: bool) -> Model {
  let sectors = sectors.max(3);
  let radius: f32 = 0.5;
  let height: f32 = 1.0;
  let half_h: f32 = 0.5;
  let pi = std::f32::consts::PI;

  // Precompute the slope normal scaling. The cone's surface slope in the
  // radial-y plane is (-radius, height); the outward normal perpendicular to
  // that is (height, radius), normalised. So at angle theta:
  //   normal = (n_radial * cos(theta), n_y, n_radial * sin(theta))
  let slope_len = (radius * radius + height * height).sqrt();
  let n_radial = height / slope_len;
  let n_y = radius / slope_len;

  let mut mesh = Mesh::new_with_material(mat);

  // Vert buffer layout:
  //   [0 .. side_count)                          side: base ring then apex ring, each (sectors+1)
  //   [side_count .. side_count + cap_count)     base cap: centre vert + sectors rim verts (if cap)
  let side_count = 2 * (sectors + 1);
  let cap_count = if cap { 1 + sectors } else { 0 };
  let vert_count = side_count + cap_count;

  mesh.verts = Vec::with_capacity(vert_count);
  mesh.normals = Vec::with_capacity(vert_count);
  mesh.uvs = Vec::with_capacity(vert_count);

  // --- 1. Side. Two rings of (sectors+1) verts with a seam duplicate at j=sectors.
  // Base ring sits on the circle at y=-half_h; "apex ring" verts are all at the
  // same 3D point (0, +half_h, 0) but each carries its own slope normal so the
  // tip shades correctly under per-vertex lighting.
  for ring in 0..2 {
    let (y, v, r) = if ring == 0 {
      (-half_h, 1.0, radius) // base
    } else {
      (half_h, 0.0, 0.0) // apex, collapsed to a point
    };
    for j in 0..=sectors {
      let theta = 2.0 * pi * (j as f32) / (sectors as f32);
      let cx = theta.cos();
      let cz = theta.sin();
      mesh.verts.push(Vec3::new(cx * r, y, cz * r));
      mesh.normals.push(Vec3::new(n_radial * cx, n_y, n_radial * cz));
      mesh.uvs.push(Vec2::new(j as f32 / sectors as f32, v));
    }
  }

  // --- 2. Base cap (optional). Centre vert + sectors rim verts, all -Y normals.
  // UVs are a disc centred at (0.5, 0.5) in texture space.
  if cap {
    mesh.verts.push(Vec3::new(0.0, -half_h, 0.0));
    mesh.normals.push(Vec3::new(0.0, -1.0, 0.0));
    mesh.uvs.push(Vec2::new(0.5, 0.5));
    for j in 0..sectors {
      let theta = 2.0 * pi * (j as f32) / (sectors as f32);
      let cx = theta.cos();
      let cz = theta.sin();
      mesh.verts.push(Vec3::new(cx * radius, -half_h, cz * radius));
      mesh.normals.push(Vec3::new(0.0, -1.0, 0.0));
      mesh.uvs.push(Vec2::new(0.5 + 0.5 * cx, 0.5 - 0.5 * cz));
    }
  }

  // --- 3. Indices. CCW winding when viewed from outside.
  let tri_count = if cap { 2 * sectors } else { sectors } as u32;
  mesh.indices = Vec::with_capacity((tri_count * 3) as usize);

  // Side: one triangle per sector. The "second triangle" of a cylinder-style
  // quad would be degenerate here (both apex verts at the same 3D point) so
  // we skip it. Surviving triangle: (apex[j], base[j+1], base[j]) which is
  // CCW from outside, matching cylinder's (tl, br, bl) ordering.
  let base_base = 0_i32;
  let apex_base = (sectors + 1) as i32;
  for j in 0..sectors as i32 {
    let bl = base_base + j;
    let br = base_base + j + 1;
    let tl = apex_base + j;
    mesh.indices.extend_from_slice(&[tl, br, bl]);
  }

  if cap {
    // Base cap fan: (centre, rim[j], rim[j+1]) gives the -Y outward normal.
    let s = sectors as i32;
    let centre = side_count as i32;
    let rim_base = centre + 1;
    for j in 0..s {
      let a = rim_base + j;
      let b = rim_base + (j + 1) % s;
      mesh.indices.extend_from_slice(&[centre, a, b]);
    }
  }

  mesh.tri_count = tri_count;

  let name = if cap { format!("cone_{}_capped", sectors) } else { format!("cone_{}_open", sectors) };
  Model::from_mesh(mesh, name.as_str())
}
