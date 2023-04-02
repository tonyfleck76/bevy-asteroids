use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::components::Asteroid;
use crate::game::components::Laser;

use super::utils::normalize_coords_in_window;

pub fn laser_movement(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut laser_transforms: Query<(Entity, &mut Transform, &Laser)>,
) {
    let window = windows.get_single().unwrap();

    for (entity, mut transform, laser) in laser_transforms.iter_mut() {
        let coords = normalize_coords_in_window(window, transform.translation);
        if coords.x > window.width() + 50.0
            || coords.y > window.height() + 50.0
            || coords.x < -50.0
            || coords.y < -50.0
        {
            commands.entity(entity).despawn();
        } else {
            transform.translation += Vec3::new(laser.velocity.x, laser.velocity.y, 0.0);
        }
    }
}

pub fn asteroid_movement(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut asteroid_transforms: Query<(Entity, &mut Transform, &Asteroid)>,
) {
    let window = windows.get_single().unwrap();

    for (entity, mut transform, asteroid) in asteroid_transforms.iter_mut() {
        let coords = normalize_coords_in_window(window, transform.translation);
        if coords.x > window.width() + 50.0
            || coords.y > window.height() + 50.0
            || coords.x < -50.0
            || coords.y < -50.0
        {
            commands.entity(entity).despawn();
        } else {
            transform.translation += Vec3::new(
                asteroid.trajectory.x * asteroid.speed,
                asteroid.trajectory.y * asteroid.speed,
                0.0,
            );
            transform.rotate_z(asteroid.rotation);
        }
    }
}
