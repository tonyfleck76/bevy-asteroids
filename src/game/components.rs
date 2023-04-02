use bevy::prelude::*;

pub trait HitBox {
    fn get_box(&self) -> Vec2;
}

#[derive(Component)]
pub struct Player {
    pub lives: u8,
    pub invincible: bool,
    pub respawn_timer: Timer,
}
impl Default for Player {
    fn default() -> Player {
        Player {
            lives: 3,
            invincible: false,
            respawn_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

impl HitBox for Player {
    fn get_box(&self) -> Vec2 {
        Vec2::new(32.0, 48.0)
    }
}

#[derive(Resource)]
pub struct GameState {
    pub asteroid_rate_increase_timer: Timer,
    pub asteroid_spawn_timer: Timer,
    pub paused: bool,
}

#[derive(Component)]
pub struct Laser {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Asteroid {
    pub trajectory: Vec2,
    pub speed: f32,
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
}
impl HitBox for Asteroid {
    fn get_box(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

#[derive(Component)]
pub struct Life {
    pub counter: u8,
}

#[derive(Component)]
pub struct PausedText;
