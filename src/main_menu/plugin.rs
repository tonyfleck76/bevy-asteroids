use bevy::prelude::*;

use crate::global::state::AppState;
use crate::global::system::clear_game_objects;

use super::system::input::*;
use super::system::render::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(input.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(clear_game_objects.in_schedule(OnExit(AppState::MainMenu)));
    }
}
