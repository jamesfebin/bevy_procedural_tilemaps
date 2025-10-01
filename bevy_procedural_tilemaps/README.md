# bevy_procedural_tilemaps

`bevy_procedural_tilemaps` is the Bevy integration layer for the lean tile-oriented fork of [Guillaume Henaux’s `ghx_proc_gen`](https://github.com/Henauxg/ghx_proc_gen). It is maintained by [AIBodh](https://aibodh.com/) for use in the upcoming Bevy game-development book.

- Updated for Bevy **0.17**.
- Original project design & implementation: **Guillaume Henaux (Henauxg)** and contributors.
- Dual-licensed under MIT/Apache like the upstream project.

## Quickstart

```
cargo add bevy_procedural_tilemaps
```

```rust
use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Handle<Image>>::default())
        .run();
}
```

## Features

- `simple-plugin` – minimal "run the generator and spawn tiles" plugin.
- `default-bundle-inserters` – provides default `BundleInserter` implementations for common asset handles.

For more details see the top-level README.
