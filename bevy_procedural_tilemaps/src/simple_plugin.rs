use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        entity::Entity,
        query::Added,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, ResMut},
    },
    prelude::{Children, Resource},
};
use procedural_tilemaps_core::{
    generator::Generator,
    grid::cartesian::{coordinates::CartesianCoordinates, grid::CartesianGrid},
    GeneratorError,
};

use crate::{
    assets::BundleInserter,
    spawner::{spawn_node, NodesSpawner},
};

/// Plugin that runs every generator once per frame until a valid grid is produced,
/// then spawns the generated nodes using the attached [`NodesSpawner`].
pub struct ProcGenSimplePlugin<C: CartesianCoordinates, A: BundleInserter> {
    typestate: PhantomData<(C, A)>,
}

impl<C: CartesianCoordinates, A: BundleInserter> Default for ProcGenSimplePlugin<C, A> {
    fn default() -> Self {
        Self {
            typestate: PhantomData,
        }
    }
}

impl<C: CartesianCoordinates, A: BundleInserter> Plugin for ProcGenSimplePlugin<C, A> {
    fn build(&self, app: &mut App) {
        app.insert_resource(PendingGenerations::default());
        app.add_systems(
            Update,
            (register_new_generations::<C>, generate_and_spawn::<C, A>).chain(),
        );
    }
}

/// Resource used to track generators that still need to complete.
#[derive(Resource, Default)]
pub struct PendingGenerations {
    pendings: Vec<Entity>,
}

/// Registers entities that just gained a [`Generator`] component.
pub fn register_new_generations<C: CartesianCoordinates>(
    mut pending_generations: ResMut<PendingGenerations>,
    mut new_generations: Query<Entity, Added<Generator<C, CartesianGrid<C>>>>,
) {
    pending_generations
        .pendings
        .extend(new_generations.iter_mut());
}

/// Attempts to generate a grid for every pending generator and spawns nodes on success.
pub fn generate_and_spawn<C: CartesianCoordinates, A: BundleInserter>(
    mut commands: Commands,
    mut pending_generations: ResMut<PendingGenerations>,
    mut generations: Query<(&mut Generator<C, CartesianGrid<C>>, &NodesSpawner<A>)>,
    children: Query<&Children>,
) {
    let mut still_pending = Vec::new();

    for entity in pending_generations.pendings.drain(..) {
        let Ok((mut generation, spawner)) = generations.get_mut(entity) else {
            continue;
        };

        match generation.generate_grid() {
            Ok((_info, grid_data)) => {
                // Remove previously spawned nodes before inserting the new set.
                if let Ok(existing_children) = children.get(entity) {
                    for &child in existing_children.iter() {
                        commands.entity(child).despawn();
                    }
                }

                for (node_index, model_instance) in grid_data.iter().enumerate() {
                    spawn_node(
                        &mut commands,
                        entity,
                        &generation.grid(),
                        spawner,
                        model_instance,
                        node_index,
                    );
                }
            }
            Err(GeneratorError { .. }) => {
                // Generation failed for now, retry on the next frame.
                still_pending.push(entity);
            }
        }
    }

    pending_generations.pendings = still_pending;
}
