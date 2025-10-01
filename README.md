# Bevy Procedural Tilemaps

This repository is a streamlined fork of [Guillaume Henaux’s `ghx_proc_gen`](https://github.com/Henauxg/ghx_proc_gen), trimmed to focus on lightweight **2D tilemap generation** with Wave Function Collapse / Model Synthesis for Bevy. The work powers the procedural-generation chapters of my upcoming Bevy game-development book—The Impatient Programmer's Guide to Bevy and Rust (Chapter 1 is already available [here](https://aibodh.com/posts/bevy-rust-game-development-chapter-1/)).

Highlights:
- Updated for Bevy **0.17**.
- Small API surface: cartesian grid helpers, rule/model builders, and a simple Bevy runner.
- Designed for layered 2D tile maps (Z layers handled by a 3D grid).
- MIT/Apache dual-licensed, preserving full credit to the original authors.

## Quickstart

```sh
cargo add bevy_procedural_tilemaps
```

```rust
use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Handle<Image>>::default())
        // add your systems …
        .run();
}
```

The core building blocks come from `procedural_tilemaps_core::prelude` (re-exported through the Bevy prelude):

- `RulesBuilder`, `SocketCollection`, `ModelCollection` to describe adjacency rules.
- `GeneratorBuilder` / `Generator` to run Model Synthesis / WFC.
- `CartesianGrid`, `GridDelta`, `Direction` for cartesian grids.
- `NodesSpawner` and bundle inserters to spawn tiles in Bevy worlds.

## Example (tile layers)

The workspace ships with a single Bevy example demonstrating layered tile generation:

```
cargo run -p bevy_examples --example tile-layers
```

It stacks multiple Z layers to render a top-down map while using Bevy’s 2D camera. Assets come from the “16x16 Game Assets” pack by George Bailey ([OpenGameArt, CC-BY 4.0](https://opengameart.org/content/16x16-game-assets)).

## Feature flags

```
[dependencies]
bevy_procedural_tilemaps = { version = "0.1.1", default-features = false, features = [
    "simple-plugin",
    "default-bundle-inserters"
] }
```

- `simple-plugin` – registers the minimal generator runner used by the example.
- `default-bundle-inserters` – provides default `BundleInserter` impls for common Bevy asset handles.
- Enabling the `bevy` feature on the core crate (automatically applied by this crate) derives `Component` for relevant types and is required when targeting Bevy.

## Credits

- Original project design & implementation: [Guillaume Henaux (`Henauxg`)](https://github.com/Henauxg) and contributors.
- Lean tiles edition & book integration maintained by [AIBodh](https://aibodh.com/).

## License

Code is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option. Contributions are accepted under the same terms.

## Assets

- `bevy_examples/assets/tile_layers`: “16x16 Game Assets” by George Bailey, CC-BY 4.0.
