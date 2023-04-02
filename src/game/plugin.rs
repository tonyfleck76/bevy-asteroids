use bevy::prelude::*;

use super::components::GameState;
use super::system::setup::*;
use super::system::input::*;
use super::system::movement::*;
use super::system::collision::*;
use super::system::player::*;
use super::system::asteroid::*;

use super::event::*;
use crate::clear_game_objects;
use crate::global::state::AppState;

pub struct GamePlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameSet {
    Input,
    Spawning,
    Movement,
    Collision,
    Updates,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<FireEvent>()
            .add_event::<PlayerHitEvent>()
            // Setup new game
            .add_startup_system(setup_game_state)
            .add_systems(
                (
                    spawn_player,
                    setup_scoreboard,
                    setup_life_counter
                )
                .in_schedule(OnEnter(AppState::InGame))
            )
            // Update in game
            .add_systems(
                (check_laser_collisions, check_player_collisions)
                .chain()
                .distributive_run_if(is_running)
                .in_set(OnUpdate(AppState::InGame))
                .in_set(GameSet::Collision)
            )
            .add_systems(
                (laser_movement, asteroid_movement)
                .chain()
                .distributive_run_if(is_running)
                .in_set(OnUpdate(AppState::InGame))
                .in_set(GameSet::Movement)
            )
            .add_systems(
                (aiming_handler, shooting_handler.run_if(player_is_not_respawning))
                .chain()
                .distributive_run_if(is_running)
                .in_set(OnUpdate(AppState::InGame))
                .in_set(GameSet::Input)
            )
            .add_systems(
                (shoot, spawn_asteroid.run_if(asteroid_spawn_timer))
                .chain()
                .distributive_run_if(is_running)
                .in_set(OnUpdate(AppState::InGame))
                .in_set(GameSet::Spawning)
            )
            .add_systems(
                (
                    update_asteroid_spawn_timer,
                    update_scoreboard,
                    update_life_counter,
                    game_over_listener,
                    player_invincibility_listener,
                    player_respawn_timer.run_if(player_is_respawning)
                )
                .chain()
                .in_set(OnUpdate(AppState::InGame))
                .in_set(GameSet::Updates)
                .distributive_run_if(is_running)
            )
            .add_system(pause_handler.in_set(OnUpdate(AppState::InGame)))
            .add_system(clear_game_objects.in_schedule(OnExit(AppState::InGame)))
            .configure_set(
                // Run systems in the Movement set before systems in the CollisionDetection set
                GameSet::Movement.before(GameSet::Collision)
            );
    }
}

fn is_running(game_state: Res<GameState>) -> bool {
    !game_state.paused
}