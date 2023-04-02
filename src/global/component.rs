use bevy::prelude::*;

#[derive(Component)]
pub struct GameObject;

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize
}