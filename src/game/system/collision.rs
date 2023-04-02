use bevy::prelude::*;

use crate::game::components::*;
use crate::game::event::*;
use crate::global::component::Scoreboard;

use bevy::sprite::collide_aabb::collide;

pub fn check_player_collisions(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform, &Asteroid), Without<Player>>,
    player_query: Query<(&Transform, &Player), Without<Asteroid>>,
    mut player_hit_writer: EventWriter<PlayerHitEvent>
) {
    let (player_transform, player) = player_query.single();

    for (asteroid_entity, asteroid_transform, asteroid) in asteroid_query.iter() {
        let player_collision = collide(player_transform.translation, player.get_box(), asteroid_transform.translation, asteroid.get_box());

        if player_collision.is_some() {
            commands.entity(asteroid_entity).despawn();
            player_hit_writer.send(PlayerHitEvent);
        }
    }
}

pub fn check_laser_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    asteroid_query: Query<(Entity, &Transform, &Asteroid), Without<Laser>>,
    laser_query: Query<(Entity, &Transform), (With<Laser>, Without<Asteroid>)>
) {
    for (asteroid_entity, asteroid_transform, asteroid) in asteroid_query.iter() {
        for (laser_entity, laser_transform) in laser_query.iter() {
            let collision = collide(laser_transform.translation, laser_transform.scale.truncate(), asteroid_transform.translation, asteroid.get_box());

            if collision.is_some() {
                scoreboard.score += asteroid.speed as usize;
                commands.entity(asteroid_entity).despawn();
                commands.entity(laser_entity).despawn();
            }
        }
    }
}