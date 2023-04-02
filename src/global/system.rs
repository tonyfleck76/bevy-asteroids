use bevy::prelude::*;
use super::component::GameObject;

pub fn clear_game_objects (
    mut commands: Commands,
    entities_query: Query<Entity, With<GameObject>>
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn();
    }
}