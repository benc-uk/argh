# Argh Taxonomy

| Concept | Purpose | Composition / Relationships |
|---------|---------|------------------------------|
| **Scene** | Top-level container of everything the renderer draws in a frame. Holds instances, lights and an ambient colour. | Owns a `SlotMap` of `Instance`s and a `SlotMap` of `Light`s (accessed via handles). |
| **Instance** | A placed occurrence of a `Model` in world space, with its own position, rotation, scale and shading flag. Lets one `Model` appear many times without duplicating geometry. | Stores a `ModelHandle` referencing a `Model` registered with the engine; lives inside a `Scene`. |
| **Model** | A named, renderable 3D object that groups one or more meshes together. Used as the unit you load, register with the engine, and instance from. | Contains a `Vec<Mesh>`; referenced by `Instance`s via `ModelHandle`. |
| **Mesh** | The actual triangle geometry: vertices, normals, UVs and indices. Each mesh carries exactly one material so different parts of a model can be shaded differently. | Owns a `Material`; lives inside a `Model`. |
| **Material** | Surface description used during shading: diffuse colour, specular colour and hardness, plus an optional texture. Determines how a mesh reacts to light. | Optionally owns a `Texture`; embedded in a `Mesh`. |
| **Texture** | Raw image pixel data (packed 0RGB) with width, height and an alpha-cutout flag. Sampled in UV space to colour fragments during rasterisation. | Owned by a `Material`; sampled using a mesh's UVs. |
| **Light** | Point light defined by position, brightness, colour and attenuation factors. Contributes to per-pixel/per-vertex lighting calculations across all instances. | Stored in a `Scene` and addressed via `LightHandle`. |
| **Camera** | Holds the view and perspective matrices and provides position / look-at controls. Defines from where and how the scene is projected to the screen. | Independent of `Scene`; passed to the engine when rendering. |

**Flow:** `Engine` registers `Model`s (which own `Mesh`es → `Material`s → `Texture`s) and hands back `ModelHandle`s. A `Scene` then holds `Instance`s (referencing those models) plus `Light`s, and a `Camera` defines the viewpoint used to render it.
