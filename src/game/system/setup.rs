use std::time::Duration;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::constants::*;
use crate::game::components::*;
use crate::global::component::GameObject;
use crate::global::component::Scoreboard;
use crate::global::constants::SCOREBOARD_FONT_SIZE;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                ..Default::default()
            },
            texture: asset_server.load("sprites/ship_sidesA.png"),
            ..Default::default()
        })
        .insert(Player::default() )
        .insert(GameObject);
}

pub fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/Excluded.ttf");
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle { font: font.clone(), font_size: SCOREBOARD_FONT_SIZE, color: Color::WHITE }
            ),
            TextSection::from_style(TextStyle { font: font.clone(), font_size: SCOREBOARD_FONT_SIZE, color: Color::WHITE })
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        })
    )
    .insert(GameObject);
    commands.insert_resource(Scoreboard { score: 0 })
}

pub fn setup_life_counter(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let life_sprite: Handle<Image> = asset_server.load("sprites/life.png");
    let window = windows.get_single().unwrap();

    for life in 1..PLAYER_LIVES + 1 {
        let offset = LIFE_PADDING * life as f32;
        commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new((window.width() / 2.0) - offset, (window.height() / 2.0) - LIFE_PADDING, 1.0),
                    ..Default::default()
                },
                texture: life_sprite.clone(),
                ..Default::default()
            })
            .insert(Life { counter: life })
            .insert(GameObject);
    }
}

pub fn setup_game_state(mut commands: Commands) {
    commands.insert_resource(GameState { 
        asteroid_rate_increase_timer: Timer::new(Duration::from_secs_f32(15.0), TimerMode::Repeating), 
        asteroid_spawn_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating), 
        paused: false 
    });
}