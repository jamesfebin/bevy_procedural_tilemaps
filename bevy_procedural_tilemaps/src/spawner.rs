use std::sync::Arc;

use bevy::{
    ecs::{component::Component, system::Commands},
    math::Vec3,
    prelude::Entity,
};
use procedural_tilemaps_core::{
    generator::model::ModelInstance,
    grid::cartesian::{coordinates::CartesianCoordinates, grid::CartesianGrid},
    NodeIndex,
};

use crate::assets::{BundleInserter, ModelsAssets};

/// Stores the information needed to spawn assets for generated nodes.
#[derive(Component, Clone, Debug)]
pub struct NodesSpawner<A: BundleInserter> {
    /// Link a model index to its spawnable assets.
    pub assets: Arc<ModelsAssets<A>>,
    /// Size of a node in world units.
    pub node_size: Vec3,
    /// Scale applied to spawned assets.
    pub spawn_scale: Vec3,
    /// Whether to offset the z coordinate based on the y position (useful for 2D layering).
    pub z_offset_from_y: bool,
}

impl<A: BundleInserter> NodesSpawner<A> {
    /// Creates a new [`NodesSpawner`]. The `z_offset_from_y` flag defaults to `false`.
    pub fn new(models_assets: ModelsAssets<A>, node_size: Vec3, spawn_scale: Vec3) -> Self {
        Self {
            assets: Arc::new(models_assets),
            node_size,
            spawn_scale,
            z_offset_from_y: false,
        }
    }

    /// Enables or disables the z offset based on the node's y coordinate.
    pub fn with_z_offset_from_y(mut self, z_offset_from_y: bool) -> Self {
        self.z_offset_from_y = z_offset_from_y;
        self
    }
}

/// Spawns the assets for a generated node as children of the generator entity.
pub fn spawn_node<C: CartesianCoordinates, A: BundleInserter>(
    commands: &mut Commands,
    parent: Entity,
    grid: &CartesianGrid<C>,
    spawner: &NodesSpawner<A>,
    instance: &ModelInstance,
    node_index: NodeIndex,
) {
    let Some(node_assets) = spawner.assets.get(&instance.model_index) else {
        return;
    };

    let position = grid.pos_from_index(node_index);
    for asset in node_assets.iter() {
        // Center the entity within the node while applying optional offsets.
        let mut translation = Vec3::new(
            asset.world_offset.x
                + spawner.node_size.x * (position.x as f32 + asset.grid_offset.dx as f32 + 0.5),
            asset.world_offset.y
                + spawner.node_size.y * (position.y as f32 + asset.grid_offset.dy as f32 + 0.5),
            asset.world_offset.z
                + spawner.node_size.z * (position.z as f32 + asset.grid_offset.dz as f32 + 0.5),
        );

        if spawner.z_offset_from_y {
            translation.z += spawner.node_size.z * (1. - position.y as f32 / grid.size_y() as f32);
        }

        let entity = commands.spawn_empty().id();
        let entity_commands = &mut commands.entity(entity);
        asset.assets_bundle.insert_bundle(
            entity_commands,
            translation,
            spawner.spawn_scale,
            instance.rotation,
        );
        (asset.spawn_commands)(entity_commands);

        commands.entity(parent).add_child(entity);
    }
}
