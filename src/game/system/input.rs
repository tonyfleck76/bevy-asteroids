use bevy::{prelude::*, window::PrimaryWindow};

use std::f32::consts::PI;

use crate::game::{event::FireEvent, components::*};

use super::utils::{normalize_coords_in_window, calculate_angle};

pub fn aiming_handler(windows: Query<&Window, With<PrimaryWindow>>, mut player_transform_query: Query<&mut Transform, With<Player>>) {
    let window = windows.get_single().unwrap();

    if let Some(_position) = window.cursor_position() {
        for mut transform in player_transform_query.iter_mut() {
            let player_pos = normalize_coords_in_window(window, transform.translation);
            let angle = calculate_angle(_position, player_pos);

            transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle + (PI / 2.0));
        }
    }
}

pub fn shooting_handler(buttons: Res<Input<MouseButton>>, mut fire_writer: EventWriter<FireEvent>) {
    if buttons.just_pressed(MouseButton::Left) {
        fire_writer.send(FireEvent);
    }
}

pub fn pause_handler(
    mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>, 
    mut game_state: ResMut<GameState>,
    paused_text_query: Query<Entity, With<PausedText>>,
    asset_server: Res<AssetServer>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if !game_state.paused {
            commands.spawn(TextBundle::from_section(
                "Paused", 
                TextStyle { font: asset_server.load("fonts/ExcludedItalic.ttf"), font_size: 60.0, color: Color::WHITE }
            ).with_style(Style {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(200.0),
                    left: Val::Px(275.0),
                    ..default()
                },
                ..default()
            }))
            .insert(PausedText);
    
            game_state.paused = true;
        } else {
            let entity = paused_text_query.get_single().unwrap();
            commands.entity(entity).despawn();
            game_state.paused = false;
        }
    }
}