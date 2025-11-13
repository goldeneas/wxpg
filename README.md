# wxpg
`wxpg` is an experimental, custom-tailored 2D/3D game engine built in Rust. It serves as a foundational layer, providing abstractions over low-level graphics, UI, and entity-component-system (ECS) libraries.

This project appears to be a continuation of [vox](https://github.com/goldeneas/vox), shifting from a pure-voxel focus to a more general-purpose rendering engine architecture.

## ⚠️ Project Status: Experimental

This engine is in the early stages of development. The APIs are not stable and are subject to change. It provides a set of abstractions over its core dependencies and is not a "batteries-included" engine. The main `src/main.rs` file serves as the primary example of its current capabilities.

## Core Features

* **Rendering:** A `wgpu`-based render pipeline (`DefaultPipeline`) with a `RenderStorage` system for managing meshes, models, materials, and instances.
* **ECS:** Uses `bevy_ecs` for its core architecture.
* **Screen Management:** A `ScreenServer` state machine to manage game states (e.g., `GameState::Menu`, `GameState::Game`).
* **Asset Loading:** An `AssetServer` for loading and managing assets like textures (`.png`, `.jpg`) and 3D models (`.obj`).
* **UI:** Integrated `egui` support via a dedicated `EguiRenderer` module.
* **Text:** Integrated `glyphon` support for high-performance text rendering.
* **Input:** A simple `InputServer` for handling key states (Pressed, Released, JustPressed) and mouse delta.

## Technology Stack

`wxpg` is built on top of several powerful Rust libraries:

* **Graphics:** [wgpu](https://github.com/gfx-rs/wgpu)
* **ECS:** [bevy_ecs](https://github.com/bevyengine/bevy/tree/main/crates/bevy_ecs)
* **UI:** [egui](https://github.com/emilk/egui) (with `egui_plot` for graphing)
* **Text:** [glyphon](https://github.com/grovesNL/glyphon)
* **Windowing:** [winit](https://github.com/rust-windowing/winit)
* **Math:** [cgmath](https://github.com/rustgd/cgmath)
* **Model Loading:** [tobj](https://github.com/Twinklebear/tobj)

## How to Run

The project is configured as a library with a runnable example in `src/main.rs`.

1.  Clone the repository.
2.  The engine's `AssetServer` expects resource files (like `debug.png`) to be in a `res/` directory at the root of the project.
3.  Run the main example:
    ```sh
    cargo run
    ```
