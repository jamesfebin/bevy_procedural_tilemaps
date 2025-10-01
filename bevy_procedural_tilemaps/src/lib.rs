#![warn(missing_docs)]

//! Minimal Bevy helpers around the procedural tilemap generation engine.

/// Types to define and spawn assets tied to generated models.
pub mod assets;
/// Adds default bundle inserters for common Bevy asset handles.
#[cfg(feature = "default-bundle-inserters")]
pub mod default_bundles;
/// Convenience re-exports for Bevy integration.
pub mod prelude;
/// Lightweight Bevy plugin that runs generators and spawns their nodes.
#[cfg(feature = "simple-plugin")]
pub mod simple_plugin;
/// Components used to spawn generated nodes inside a Bevy world.
pub mod spawner;

/// Re-export of the core procedural generation crate so consumers can build rules & generators.
pub use procedural_tilemaps_core as proc_gen;
