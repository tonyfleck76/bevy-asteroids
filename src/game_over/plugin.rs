use bevy::prelude::*;
use crate::{global::state::AppState, clear_game_objects};

use super::system::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup game over
            .add_system(show_game_over_screen.in_schedule(OnEnter(AppState::GameOver)))
            // Game Over Listener
            .add_system(play_again_listener.in_set(OnUpdate(AppState::GameOver)))
            // Clean up game over
            .add_system(clear_game_objects.in_schedule(OnExit(AppState::GameOver)));
    }
}