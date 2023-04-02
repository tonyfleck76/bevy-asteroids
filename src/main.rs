mod game;
mod game_over;
mod global;
mod main_menu;

use game::event::*;
use game::plugin::GamePlugin;
use game_over::plugin::GameOverPlugin;
use global::component::GameObject;
use global::event::*;
use global::state::AppState;
use main_menu::plugin::MainMenuPlugin;

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
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn clear_game_objects(mut commands: Commands, entities_query: Query<Entity, With<GameObject>>) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn();
    }
}
