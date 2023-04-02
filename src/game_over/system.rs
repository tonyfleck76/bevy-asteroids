use bevy::prelude::*;

use crate::global::{component::{GameObject, Scoreboard}, state::AppState, constants::SCOREBOARD_FONT_SIZE};

pub fn show_game_over_screen(
    mut commands: Commands,
    scoreboard: Res<Scoreboard>,
    asset_server: Res<AssetServer>
) {
    let score = scoreboard.score;

    commands.remove_resource::<Scoreboard>();

    let font: Handle<Font> = asset_server.load("fonts/Excluded.ttf");
    let italic_font: Handle<Font> = asset_server.load("fonts/ExcludedItalic.ttf");
    commands.spawn(TextBundle::from_section(
        "Game Over", 
        TextStyle { font: italic_font, font_size: 60.0, color: Color::WHITE }
    ).with_style(Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(300.0),
            left: Val::Px(225.0),
            ..default()
        },
        ..default()
    }))
    .insert(GameObject);

    commands.spawn(TextBundle::from_section(
        format!("Final score: {}", score), TextStyle { font: font.clone(), font_size: SCOREBOARD_FONT_SIZE, color: Color::WHITE }
    ).with_style(Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(400.0),
            left: Val::Px(225.0),
            ..default()
        },
        ..default()
    }))
    .insert(GameObject);

    commands.spawn(TextBundle::from_section(
        "Click anywhere to play again.",
        TextStyle { font, font_size: 24.0, color: Color::WHITE }
    ).with_style(Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(450.0),
            left: Val::Px(175.0),
            ..default()
        },
        ..default()
    }))
    .insert(GameObject);
}

pub fn play_again_listener(
    buttons: Res<Input<MouseButton>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        next_state.set(AppState::InGame);
    }
}