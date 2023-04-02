mod global;
mod game;
mod game_over;

use game::plugin::GamePlugin;
use game_over::plugin::GameOverPlugin;
use global::state::AppState;
use global::event::*;
use game::event::*;

use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        // Default and window setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroids!".into(),
                resolution: (800., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // State
        .add_state::<AppState>()
        // Events
        .add_event::<FireEvent>()
        .add_event::<PlayerHitEvent>()
        .add_event::<GameOverEvent>()
        // Base systems
        .add_startup_system(setup_camera)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}