use bevy::{
    asset::Handle,
    ecs::system::EntityCommands,
    image::Image,
    math::{Quat, Vec3},
    sprite::{Anchor, Sprite},
    transform::components::Transform,
    utils::default,
};
use procedural_tilemaps_core::generator::model::ModelRotation;

use super::assets::BundleInserter;

/// **WARNING**: Assumes a rotation axis aligned with +Z (convenient for 2D sprites).
impl BundleInserter for Handle<Image> {
    fn insert_bundle(
        &self,
        commands: &mut EntityCommands,
        translation: Vec3,
        scale: Vec3,
        rotation: ModelRotation,
    ) {
        commands.insert((
            Transform::from_translation(translation)
                .with_scale(scale)
                .with_rotation(Quat::from_rotation_z(rotation.rad())),
            Sprite {
                image: self.clone(),
                ..default()
            },
            Anchor::CENTER,
        ));
    }
}
