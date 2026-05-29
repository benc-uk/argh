# Scene refactor: moving world state out of `Engine`

Design notes for splitting `Engine` into a runtime (window, buffer, assets,
renderer) and a `Scene` that owns its world state (instances, lights).

> Scope: this doc covers only the data-ownership split. It deliberately does
> not cover scene swapping (transitions between scenes). If we want that
> later, it gets its own design doc and shouldn't constrain the choices here.

## Motivation

Today, `Engine` owns everything: meshes, materials, instances, lights, window,
buffer, timing. `Scene` is just a trait with `update()` that the user
implements, typically storing a `Vec<InstanceHandle>` and reaching back into
the engine to mutate things.

The current model conflates two unrelated jobs:

- **Runtime**: window, buffer, input, timing, renderer.
- **World state**: lights, instances, ambient light.

Both live in one struct, which makes the engine API surface heavier than it
needs to be, and ties the lifetime of world state to the engine rather than
to the scene that owns it conceptually.

This refactor moves the world-state half into a `SceneCore` struct that the
user holds as a field on their scene. The engine keeps the runtime half plus
the shared assets (meshes, materials).

Honest caveat: the user-experience change is small. The big handle-pain wins
come from wrapping handles in domain types (see "Hiding handles" below), not
from moving the slotmaps around. This refactor is mostly architectural
cleanliness.

## Why "subclassing" doesn't apply

Rust has no `class extends BaseScene`. There is no virtual override, no
`super`. The replacement pattern is **composition + a trait**, which is
essentially how Go does it with struct embedding + interfaces:

```go
type SceneCore struct { /* lights, instances... */ }
type MyScene struct {
    SceneCore         // embedded
    camera Camera
}
```

In Rust the user writes the embedding explicitly as a field (`core: SceneCore`)
and uses it via `self.core.add_light(...)`. There is no field-name promotion;
Rust does not have the Go shorthand. That's the only ergonomic difference,
and it's tiny.

There is no Rust equivalent to JS `class MyScene extends Scene`. Don't try to
fake it with traits-of-traits or `Deref` to a parent. Composition is the
answer.

## The architectural split

Not everything moves. The split that maps to "what is an asset" vs "what is
the world":

| Stays on `Engine`                              | Moves to `SceneCore`                       |
| ---------------------------------------------- | ------------------------------------------ |
| `meshes` (vertex data, heavy)                  | `instances` (mesh + material + transform)  |
| `materials` (textures, heavy)                  | `lights`                                   |
| `Buffer`, window, input, timing                | `instance_keys`, `light_keys` (iteration)  |
| renderer (`render_all`, `render_instance`)     | `ambient_light`                            |

Reasoning:

- **Meshes and materials are assets.** A cube mesh is a cube mesh regardless
  of which scene draws it.
- **Instances and lights are the world.** They are the scene.

`Instance` keeps its `MeshHandle` and `MaterialHandle` fields unchanged. At
render time the engine resolves those handles against its own asset slotmaps;
the core's instance slotmap and the engine's mesh slotmap coexist without
conflict.

## On handles

This refactor does **not** eliminate handles. Handles exist because:

1. Rust forbids two mutable references to the same value, so you can't both
   own an `Instance` in a central pool and hand out long-lived `&mut Instance`
   references that live elsewhere.
2. `Vec` / `SlotMap` reallocate when they grow, so raw references would
   dangle.

Handles sidestep both rules: they are plain lookup tokens, not active borrows.
This is fundamental to how Rust differs from Go and JS, where every object is
its own heap allocation with a stable address and the GC handles aliasing.

The UX win for handle-heavy code comes from **wrapping handles in domain
types**, not from changing where the slotmap lives. See "Hiding handles in
domain types" below.

## API shape

### `SceneCore`

```rust
pub struct SceneCore {
    pub(crate) instances:     SlotMap<InstanceHandle, Instance>,
    pub(crate) lights:        SlotMap<LightHandle, Light>,
    pub(crate) instance_keys: Vec<InstanceHandle>,
    pub(crate) light_keys:    Vec<LightHandle>,
    pub ambient_light: Colour,
}

impl SceneCore {
    pub fn new() -> Self { /* ... */ }
    pub fn add_light(&mut self, l: Light) -> LightHandle { /* ... */ }
    pub fn add_instance(&mut self, m: MeshHandle, mat: MaterialHandle) -> InstanceHandle { /* ... */ }
    pub fn instance(&self, h: InstanceHandle) -> &Instance { /* ... */ }
    pub fn instance_mut(&mut self, h: InstanceHandle) -> &mut Instance { /* ... */ }
    // ...all the existing per-instance / per-light methods from engine/resources.rs
}
```

### `Scene` trait: unchanged

```rust
pub trait Scene {
    fn update(&mut self, engine: &mut Engine, dt: f64, t: f64);
    fn new(e: &mut Engine) -> Self;
}
```

No accessor methods, no `on_enter`. The trait keeps its current shape. The
engine never needs to reach into the scene generically to find the core,
because rendering takes the core as an explicit parameter (see below). The
user provides their own `&self.core` when calling render.

### Renderer

`render_all` and `render_instance` gain a `&SceneCore` parameter:

```rust
impl Engine {
    pub fn render_all(&mut self, core: &SceneCore, cam: &Camera) {
        for &h in &core.instance_keys {
            self.render_instance(core, cam, h);
        }
    }

    pub fn render_instance(&mut self, core: &SceneCore, cam: &Camera, h: InstanceHandle) {
        let inst = &core.instances[h];
        let mesh = &self.meshes[inst.mesh_handle];
        let mat  = &self.materials[inst.material_handle];
        // ... existing rasterisation, but reads lights from core.lights ...
    }
}
```

Engine and core are separately borrowed in the call signature, no conflict.

### `Engine::tick` and `Engine::start_window`: unchanged

Both stay generic over `S: Scene` exactly as today. No signature changes.
The user still calls `engine.tick(&mut scene, dt)` (WASM) or
`engine.start_window(scene, "title", scale)` (desktop). The WASM glue
doesn't need any structural change.

## User code shape

```rust
pub struct MyScene {
    core: SceneCore,        // just a regular field, like camera
    camera: Camera,
    player: InstanceHandle,
}

impl Scene for MyScene {
    fn new(e: &mut Engine) -> Self {
        let mut core = SceneCore::new();
        core.add_light(Light::new(Vec3::new(3.0, 7.0, 5.0), 1.0, WHITE));

        let cube = e.add_mesh(primitives::new_cube());
        let mat  = e.add_material(Material::new(Texture::solid(RED)));
        let player = core.add_instance(cube, mat);

        Self {
            core,
            camera: Camera::new_perspective(/* ... */).unwrap(),
            player,
        }
    }

    fn update(&mut self, e: &mut Engine, dt: f64, _t: f64) {
        e.clear(BLACK);
        self.core.instance_mut(self.player).rot_y(0.5 * dt);
        e.render_all(&self.core, &self.camera);
    }
}
```

No accessor traits. No boilerplate. `self.core` is just a field, the same as
`self.camera`. The user freely mixes `core` operations and `engine`
operations because they are independent mutable borrows (the engine is a
parameter, not a field of self).

### Hiding handles in domain types

A separate but related improvement. Replace `Vec<InstanceHandle>` indexed by
magic numbers with named fields, and wrap entities behind structs:

```rust
struct Player {
    instance: InstanceHandle,
    pub hp: u32,
    pub velocity: Vec3,
}

impl Player {
    fn tick(&mut self, core: &mut SceneCore, dt: f64) {
        core.instance_mut(self.instance)
            .set_pos(/* compute from velocity */);
    }
}
```

This is orthogonal to the scene-state refactor and could be done
independently. It pairs well: gameplay code stops looking like dictionary
lookups.

## Borrow-checker sanity

Inside `update`, the typical patterns all check out:

```rust
self.core.instance_mut(self.player).rot_y(0.5 * dt);
// self.core borrowed mutably (temporary); self.player is Copy. Fine.

engine.render_all(&self.core, &self.camera);
// self.core borrowed immutably, self.camera borrowed immutably.
// engine is a separate &mut Engine parameter, no overlap with self. Fine.
```

The key reason it works: the `engine` reference lives on the stack as a
parameter, not as a field of `MyScene`. The borrow checker can split mutable
access to `self.core` from immutable access to `engine` cleanly.

## Trade-offs

| Cost                                                     | Why it's acceptable                          |
| -------------------------------------------------------- | -------------------------------------------- |
| `render_all` / `render_instance` gain a `&SceneCore` parameter | Minor; explicit is fine               |
| User code changes from `engine.add_light` etc. to `self.core.add_light` | Same word count; mechanical rename |
| All existing examples need updating                      | Mechanical search-and-replace                |

No vtable cost, no boilerplate accessors, no `Box<dyn>`, no API churn beyond
the rename.

## Migration plan

1. Add `SceneCore` struct with `instances`, `lights`, key vectors,
   `ambient_light`. Port all per-instance and per-light methods from
   `engine/resources.rs` to it.
2. Remove those fields and methods from `Engine`. Keep `meshes` and
   `materials` (and their methods) where they are.
3. Change `Engine::render_all` and `Engine::render_instance` to take
   `&SceneCore`.
4. Update each example in `examples/` to:
   - Add a `core: SceneCore` field to the scene struct.
   - Move `engine.add_light` -> `core.add_light`,
     `engine.add_instance` -> `core.add_instance`, etc.
   - Change render calls to pass `&self.core`.

`Scene` trait and `Engine::tick` / `Engine::start_window` signatures don't
change.

Doc updates needed:

- `argh/src/lib.rs` top-of-crate example.
- `argh/src/engine/mod.rs` module doc.

## What we are explicitly not doing

- **No scene swapping / transitions.** Out of scope; gets its own design doc
  when needed. The current shape (user holds `SceneCore` as a field) does
  not block adding swap later; it just isn't required yet.
- **No `Rc<RefCell<Instance>>`** to imitate Go pointers. Handles are cheaper,
  safer at compile time, and idiomatic for this kind of system.
- **No ECS.** Overkill for a software renderer at this scale.
- **No per-scene mesh/material storage.** Shared assets stay shared on the
  engine.
- **No accessor trait methods (`fn core() -> &SceneCore`).** An earlier draft
  had these to let the engine reach into the scene generically; we don't
  actually need that, because rendering takes the core as an explicit
  parameter from the user.
