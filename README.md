# Bevy Procedural Tilemaps

Lightweight **2D procedural tilemap generation for Bevy 0.20**, using Wave Function Collapse / Model Synthesis. Define tile models and adjacency rules, generate a grid, and spawn the result as Bevy entities.

This repository is a streamlined fork of [Gilles Henaux’s `ghx_proc_gen`](https://github.com/Henauxg/ghx_proc_gen). It powers the procedural-generation chapters of [The Impatient Programmer's Guide to Bevy and Rust](https://aibodh.com/books/the-impatient-programmers-guide-to-bevy-and-rust/).

Highlights:
- Updated for Bevy **0.20** (Rust **1.97.1** or newer).
- Small API surface: cartesian grid helpers, rule/model builders, and a simple Bevy runner.
- Designed for layered 2D tile maps (Z layers handled by a 3D grid).
- MIT/Apache dual-licensed, preserving full credit to the original authors.

## Bevy compatibility

| `bevy_procedural_tilemaps` | Bevy | `procedural_tilemaps_core` |
| --- | --- | --- |
| 0.5.x | 0.20 | 0.5.x |
| 0.4.x | 0.19 | 0.4.x |
| 0.3.x | 0.18 | 0.3.x |
| 0.2.x | 0.18 | 0.2.x |
| 0.1.x | 0.17 | 0.1.x |

Bevy 0.20 requires **Rust 1.97.1 or newer**. Version 0.5.0 updated the integration and the core crate's optional Bevy support together; no public tilemap API changes were required. Version 0.5.1 uses this shared README for the published integration package and keeps the core dependency at 0.5.0.

## Quickstart

In your Bevy project, install and select the supported Rust toolchain, then add the dependencies:

```sh
rustup toolchain install 1.97.1
rustup override set 1.97.1
cargo add bevy@0.20
cargo add bevy_procedural_tilemaps@0.5.1
```

Or add these entries to `Cargo.toml`:

```toml
[dependencies]
bevy = "0.20"
bevy_procedural_tilemaps = "0.5.1"
```

Register the generator runner with your app:

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

This registers the runner. To generate a map, your setup systems must also define the rules and spawn an entity with a grid, generator, and `NodesSpawner`. The [complete tile-layers example](https://github.com/jamesfebin/bevy_procedural_tilemaps/tree/main/bevy_examples/tile-layers) shows how these pieces fit together.

The core building blocks come from `procedural_tilemaps_core::prelude`, re-exported through `bevy_procedural_tilemaps::prelude`:

- `RulesBuilder`, `SocketCollection`, `ModelCollection` to describe adjacency rules.
- `GeneratorBuilder` / `Generator` to run Model Synthesis / WFC.
- `CartesianGrid`, `GridDelta`, `Direction` for cartesian grids.

The integration crate adds `NodesSpawner` and bundle inserters to spawn tiles in Bevy worlds. You do not need to add the core crate separately when using the integration crate's prelude. For generation without Bevy, use `procedural_tilemaps_core = "0.5.0"` directly; its default features do not depend on Bevy.

## Example (tile layers)

From a checkout of this repository, run the layered tile generation example:

```sh
cargo +1.97.1 run -p bevy_examples --example tile-layers
```

It stacks multiple Z layers to render a top-down map while using Bevy’s 2D camera. Assets come from the “16x16 Game Assets” pack by George Bailey ([OpenGameArt, CC-BY 4.0](https://opengameart.org/content/16x16-game-assets)).

## Feature flags

```toml
[dependencies]
bevy_procedural_tilemaps = { version = "0.5.1", default-features = false }
```

Both integration features are enabled by default. The configuration above disables them so you can choose your own runner and inserters:

- `simple-plugin` – registers the minimal generator runner used by the example. Enabled by default; opt out with `default-features = false` if you want to wire systems manually.
- `default-bundle-inserters` – provides default `BundleInserter` impls for common Bevy asset handles (`Handle<Image>` and `Sprite`). Enabled by default; disable alongside the default features if you supply your own inserters.
- Enabling the `bevy` feature on the core crate (automatically applied by this crate) derives `Component` for relevant types and is required when targeting Bevy.

The integration always enables its required Bevy rendering and asset features. Disabling its default features only disables the runner and default inserters; use the core crate if you need generation without rendering.

## Credits

- Original project design & implementation: [Gilles Henaux ](https://github.com/Henauxg) and contributors.
- Lean tiles edition & book integration maintained by [AIBodh](https://aibodh.com/).

## License

Code is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option. Contributions are accepted under the same terms.

## Assets

- `bevy_examples/assets/tile_layers`: “16x16 Game Assets” by George Bailey, CC-BY 4.0.
