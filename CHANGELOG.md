# Changelog

## 0.2.0
- **BREAKING**: Updated to Bevy 0.18 (from 0.17)
- Updated all dependencies to be compatible with Bevy 0.18

## 0.1.3
- Added a `BundleInserter` implementation for `Sprite`, so preconfigured sprites (for example from an atlas) can be spawned when using the `default-bundle-inserters` feature.

## 0.1.2
- Enabled the `simple-plugin` and `default-bundle-inserters` features by default in `bevy_procedural_tilemaps`, simplifying the tile layers example dependency stanza.
- Updated documentation and example manifests to rely on the new default feature set.

## 0.1.1
- Renamed crates to `procedural_tilemaps_core` and `bevy_procedural_tilemaps`, with the Bevy crate re-exporting the core prelude for a single import path.
- Added `prelude` modules for both the core and Bevy crates to make common imports easier.

## 0.1.0
- Initial lean fork prepared for Bevy 0.17 (trimmed debug tooling, internal cartesian grid, tile-layers example).
