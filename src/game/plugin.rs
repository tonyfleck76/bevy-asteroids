use bevy::prelude::*;

use super::components::GameState;
use super::system::setup::*;
use super::system::input::*;
use super::system::movement::*;
use super::system::collision::*;
use super::system::player::*;
use super::system::asteroid::*;

use super::event::*;
use crate::global::state::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<FireEvent>()
            .add_event::<PlayerHitEvent>()
            // Setup new game
            .add_systems(
                (
                    setup_game_state,
                    spawn_player,
                    setup_scoreboard,
                    setup_life_counter
                )
                .in_schedule(OnEnter(AppState::InGame))
            )
            // Update in game
            .add_systems(
                (
                    aiming_handler,
                    shooting_handler,
                    shoot,
                    laser_movement,
                    spawn_asteroid.run_if(asteroid_spawn_timer),
                    update_asteroid_spawn_timer,
                    asteroid_movement,
                    check_player_collisions,
                    check_laser_collisions,
                    update_scoreboard,
                    update_life_counter,
                    game_over_listener
                )
                .distributive_run_if(is_running)
                .in_set(OnUpdate(AppState::InGame))
            )
            .add_system(pause_handler.in_set(OnUpdate(AppState::InGame)));
    }
}

fn is_running(game_state: Res<GameState>) -> bool {
    !game_state.paused
}