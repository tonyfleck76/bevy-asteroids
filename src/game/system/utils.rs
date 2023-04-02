use bevy::prelude::*;

pub fn calculate_angle(pos1: Vec2, pos2: Vec2) -> f32 {
    let diff = pos2 - pos1;
    diff.y.atan2(diff.x)
}

pub fn normalize_coords_in_window(window: &Window, coords: Vec3) -> Vec2 {
    Vec2::new(
        (window.width() / 2.) - coords.x,
        (window.height() / 2.0) - coords.y,
    )
}
