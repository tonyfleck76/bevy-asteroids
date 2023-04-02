use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    game::components::{Asteroid, GameState},
    global::component::GameObject,
};

use super::utils::normalize_coords_in_window;

pub fn spawn_asteroid(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_single().unwrap();
    let mut random = rand::thread_rng();

    let side = random.gen_range(0..4);
    let spawn_x = match side {
        1 => -25.0,
        3 => window.width() + 25.0,
        _ => random.gen_range(-25.0..825.0),
    };
    let spawn_y = match side {
        0 => -25.0,
        2 => window.height() + 25.0,
        _ => random.gen_range(-25.0..825.0),
    };
    let spawn_coords = normalize_coords_in_window(window, Vec3::new(spawn_x, spawn_y, 0.0));

    let target_x = random.gen_range(0.0..window.width());
    let target_y = random.gen_range(0.0..window.height());
    let speed = random.gen_range(1.0..8.0);

    let trajectory = (Vec2::new(spawn_x, spawn_y) - Vec2::new(target_x, target_y)).normalize();

    let asteroid_type = random.gen_range(1..5);
    let asteroid_size = match asteroid_type {
        1 | 3 => 48.0,
        2 | 4 => 32.0,
        _ => 0.0,
    };

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(spawn_coords.x, spawn_coords.y, 0.0),
                ..Default::default()
            },
            texture: asset_server.load(format!("sprites/meteor/{}.png", asteroid_type)),
            ..Default::default()
        })
        .insert(Asteroid {
            trajectory,
            speed,
            rotation: random.gen_range(-0.1..0.1),
            width: asteroid_size,
            height: asteroid_size,
        })
        .insert(GameObject);
}

pub fn update_asteroid_spawn_timer(time: Res<FixedTime>, mut game_state: ResMut<GameState>) {
    game_state.asteroid_spawn_timer.tick(time.period);
    game_state.asteroid_rate_increase_timer.tick(time.period);

    if game_state.asteroid_rate_increase_timer.just_finished() {
        game_state.asteroid_spawn_timer = Timer::new(
            game_state.asteroid_spawn_timer.duration().mul_f32(0.8),
            TimerMode::Repeating,
        );
    }
}

pub fn asteroid_spawn_timer(game_state: Res<GameState>) -> bool {
    game_state.asteroid_spawn_timer.just_finished()
}
