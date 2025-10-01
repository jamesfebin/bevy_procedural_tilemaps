use bevy::{
    app::{App, PluginGroup, Startup},
    asset::{AssetServer, Handle},
    ecs::system::{Commands, Res},
    image::{Image, ImagePlugin},
    math::Vec3,
    prelude::Camera2d,
    transform::components::Transform,
    DefaultPlugins,
};

use bevy_examples::utils::load_assets;
use bevy_procedural_tilemaps::prelude::*;

use crate::rules::rules_and_assets;

mod rules;

// -----------------  Configurable values ---------------------------
/// Modify these values to control the map size.
const GRID_X: u32 = 25;
const GRID_Y: u32 = 18;

const ASSETS_PATH: &str = "tile_layers";
/// Size of a block in world units (in Bevy 2d, 1 pixel is 1 world unit)
const TILE_SIZE: f32 = 32.;
/// Size of a grid node in world units
const NODE_SIZE: Vec3 = Vec3::new(TILE_SIZE, TILE_SIZE, 1.);

const ASSETS_SCALE: Vec3 = Vec3::ONE;

/// Number of z layers in the map, do not change without adapting the rules.
const GRID_Z: u32 = 5;

fn setup_scene(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}

fn setup_generator(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Get rules from rules.rs
    let (assets_definitions, models, socket_collection) = rules_and_assets();

    let rules = RulesBuilder::new_cartesian_3d(models, socket_collection)
        // Use ZForward as the up axis (rotation axis for models) since we are using Bevy in 2D
        .with_rotation_axis(Direction::ZForward)
        .build()
        .unwrap();
    let grid = CartesianGrid::new_cartesian_3d(GRID_X, GRID_Y, GRID_Z, false, false, false);
    let gen_builder = GeneratorBuilder::new()
        .with_rules(rules)
        .with_grid(grid.clone())
        .with_rng(RngMode::RandomSeed)
        .with_node_heuristic(NodeSelectionHeuristic::MinimumRemainingValue)
        .with_model_heuristic(ModelSelectionHeuristic::WeightedProbability);
    let generator = gen_builder.build().unwrap();

    let models_assets = load_assets::<Image>(&asset_server, assets_definitions, ASSETS_PATH, "png");

    commands.spawn((
        Transform::from_translation(Vec3 {
            x: -TILE_SIZE * grid.size_x() as f32 / 2.,
            y: -TILE_SIZE * grid.size_y() as f32 / 2.,
            z: 0.,
        }),
        grid,
        generator,
        NodesSpawner::new(models_assets, NODE_SIZE, ASSETS_SCALE).with_z_offset_from_y(true),
    ));
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(ProcGenSimplePlugin::<Cartesian3D, Handle<Image>>::default());
    app.add_systems(Startup, (setup_generator, setup_scene));
    app.run();
}
