use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::utils::normalize_coords_in_window;
use crate::game::components::*;
use crate::game::constants::*;
use crate::game::event::*;
use crate::global::component::*;
use crate::global::event::GameOverEvent;
use crate::global::state::AppState;

pub fn shoot(
    mut commands: Commands,
    mut fire_reader: EventReader<FireEvent>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if fire_reader.iter().next().is_some() {
        let window = windows.get_single().unwrap();
        if let Some(_position) = window.cursor_position() {
            let mut transform = *player_transform.get_single_mut().unwrap();
            transform.scale = Vec3::new(0.3, 0.3, 0.0);

            let trajectory =
                (_position - normalize_coords_in_window(window, transform.translation)).normalize();

            commands
                .spawn(SpriteBundle {
                    transform,
                    texture: asset_server.load("sprites/effect_yellow.png"),
                    ..Default::default()
                })
                .insert(Laser {
                    velocity: Vec2::new(trajectory.x * LASER_SPEED, trajectory.y * LASER_SPEED),
                })
                .insert(GameObject);
        }
    }
}

pub fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreBoardText>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

pub fn update_life_counter(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Player>,
    mut life_query: Query<(Entity, &Life)>,
    asset_server: Res<AssetServer>,
    mut player_hit_reader: EventReader<PlayerHitEvent>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    let lost_life_image: Handle<Image> = asset_server.load("sprites/lost_life.png");
    let window = windows.get_single().unwrap();

    if player_hit_reader.iter().next().is_some() {
        let mut player = player_query.get_single_mut().unwrap();
        player.lives -= 1;

        if player.lives == 0 {
            game_over_writer.send(GameOverEvent);
        }

        for (entity, life) in life_query.iter_mut() {
            if life.counter > player.lives {
                commands.entity(entity).despawn();
                commands
                    .spawn(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                (window.width() / 2.0) - (life.counter as f32 * LIFE_PADDING),
                                (window.height() / 2.0) - LIFE_PADDING,
                                1.0,
                            ),
                            scale: Vec3::new(0.6, 0.6, 1.0),
                            ..Default::default()
                        },
                        texture: lost_life_image.clone(),
                        ..Default::default()
                    })
                    .insert(GameObject);
            }
        }
    }
}

pub fn player_invincibility_listener(
    mut player_query: Query<(&mut Sprite, &mut Player)>,
    mut player_hit_reader: EventReader<PlayerHitEvent>,
) {
    if player_hit_reader.iter().next().is_some() {
        for (mut sprite, mut player) in player_query.iter_mut() {
            sprite.color.set_a(0.3);
            player.invincible = true;
        }
    }
}

pub fn player_respawn_timer(
    time: Res<FixedTime>,
    mut player_query: Query<(&mut Sprite, &mut Player)>,
) {
    for (mut sprite, mut player) in player_query.iter_mut() {
        player.respawn_timer.tick(time.period);
        if player.respawn_timer.just_finished() {
            sprite.color.set_a(1.0);
            player.invincible = false;
            player.respawn_timer = Timer::from_seconds(RESPAWN_DURATION, TimerMode::Once);
        }
    }
}

pub fn player_is_respawning(player_query: Query<&Player>) -> bool {
    if player_query.is_empty() {
        return false;
    }

    let player = player_query.get_single().unwrap();
    player.invincible
}

pub fn game_over_listener(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_over_reader: EventReader<GameOverEvent>,
) {
    if game_over_reader.iter().next().is_some() {
        next_state.set(AppState::GameOver);
    }
}
