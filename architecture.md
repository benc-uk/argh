# Argh architecture

High-level ownership & reference map of the main runtime components.

```
                  ┌─────────────────────────┐
                  │   App   (your code)     │
                  └──┬──────────┬──────────┬┘
            owns /   │          │          │
            drives   ▼          ▼          ▼
                ┌────────┐ ┌────────┐ ┌────────┐
                │ Engine │ │ Scene  │ │ Camera │
                └────────┘ └────────┘ └────────┘

Engine
  └── models : SlotMap<ModelHandle, Model>
        │
        └── Model
              ├── name
              ├── tri_count
              └── meshes : Vec<Mesh>
                    │
                    └── Mesh
                          ├── verts, normals, uvs, indices
                          └── material : Material
                                ├── diffuse, specular, hardness
                                └── texture : Option<Rc<Texture>>
                                              │
                                              └── Texture  (shared via Rc)

Scene
  ├── ambient : Colour
  ├── instances : SlotMap<InstanceHandle, Instance>
  │     │
  │     └── Instance
  │           ├── pos, rot, scale, smooth
  │           └── model_handle  ──► refs into Engine.models
  │
  ├── lights : SlotMap<LightHandle, Light>
  │     │
  │     └── Light  (pos, colour, brightness, atten, is_static)
  │
  └── baked : Vec<BakedMesh>     ──► derived from a Model at
        │                            add_static() time (world space,
        └── BakedMesh                lighting pre-baked into verts)
              ├── verts, normals  (WORLD space)
              ├── uvs, indices
              ├── material : Material
              └── baked_lighting : Vec<Colour>

Camera
  ├── pos, look_at
  ├── view_mat
  └── pers_mat
```

Handle types live in `engine/mod.rs` as `new_key_type!` slotmap keys:
`ModelHandle`, `InstanceHandle`, `LightHandle`.

## Notes

- `Instance` and `BakedMesh` both originate from `Model` but in opposite
  ways: `Instance` keeps a live handle and applies a transform each frame,
  `BakedMesh` snapshots the geometry into world space and forgets the
  handle.
- `Material` is the only thing currently shared between `Mesh` and
  `BakedMesh` (via `Clone`), and `Texture` is shared via `Rc` so cloning a
  material is cheap.
