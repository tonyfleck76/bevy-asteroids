use std::f32::consts::PI;
use std::time::Duration;

use bevy::time::common_conditions::on_fixed_timer;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy::log;
use rand::Rng;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

const LASER_SPEED: f32 = 10.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Laser{
    velocity: Vec2
}

#[derive(Component)]
struct Asteroid {
    velocity: Vec2,
    rotation: f32
}

struct FireEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroids!".into(),
                resolution: (800., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<FireEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(aiming_handler)
        .add_system(shooting_handler)
        .add_system(shoot)
        .add_system(laser_movement)
        .add_system(spawn_asteroid.run_if(on_fixed_timer(Duration::from_secs_f32(1.0))))
        .add_system(asteroid_movement)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                ..Default::default()
            },
            texture: asset_server.load("sprites/ship_sidesA.png"),
            ..Default::default()
        })
        .insert(Player);
}

fn aiming_handler(windows: Query<&Window, With<PrimaryWindow>>, mut player_sprite: Query<&mut Transform, With<Player>>) {
    let window = windows.get_single().unwrap();

    if let Some(_position) = window.cursor_position() {
        for mut transform in player_sprite.iter_mut() {
            let player_pos = normalize_coords_in_window(window, transform.translation);
            let angle = calculate_angle(_position, player_pos);

            transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle + (PI / 2.0));
        }
    }
}

fn shooting_handler(buttons: Res<Input<MouseButton>>, mut fire_writer: EventWriter<FireEvent>) {
    if buttons.just_pressed(MouseButton::Left) {
        fire_writer.send(FireEvent);
    }
}

fn shoot(mut commands: Commands, mut fire_reader: EventReader<FireEvent>, player_transform: Query<&mut Transform, With<Player>>, windows: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    if fire_reader.iter().next().is_some() {
        let window = windows.get_single().unwrap();
        if let Some(_position) = window.cursor_position() {
            let mut transform = player_transform.get_single().unwrap().clone();
            transform.scale = Vec3::new(0.3, 0.3, 0.0);

            let trajectory = (_position - normalize_coords_in_window(window, transform.translation)).normalize();

            commands
                .spawn(SpriteBundle {
                    transform: transform,
                    texture: asset_server.load("sprites/effect_yellow.png"),
                    ..Default::default()
                })
                .insert(Laser { velocity: Vec2::new(trajectory.x * LASER_SPEED, trajectory.y * LASER_SPEED) });
        }
    }
}

fn laser_movement(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>, mut laser_transforms: Query<(Entity, &mut Transform, &Laser)>) {
    let window = windows.get_single().unwrap();

    for (entity, mut transform, laser) in laser_transforms.iter_mut() {
        let coords = normalize_coords_in_window(window, transform.translation);
        if coords.x > window.width() + 50.0 || 
            coords.y > window.height() + 50.0 || 
            coords.x < -50.0 || 
            coords.y < -50.0 {
                commands.entity(entity).despawn();
        } else {
            transform.translation += Vec3::new(laser.velocity.x, laser.velocity.y, 0.0);
        }
    }
}

fn spawn_asteroid(mut commands: Commands,  windows: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = windows.get_single().unwrap();
    let mut random = rand::thread_rng();

    let side = random.gen_range(0..4);
    let spawn_x = match side {
        1 => -25.0,
        3 => window.width() + 25.0,
        _ => random.gen_range(-25.0..825.0)
    };
    let spawn_y = match side {
        0 => -25.0,
        2 => window.height() + 25.0,
        _ => random.gen_range(-25.0..825.0)
    };
    let spawn_coords = normalize_coords_in_window(window, Vec3::new(spawn_x, spawn_y, 0.0));

    let target_x = random.gen_range(0.0..window.width());
    let target_y = random.gen_range(0.0..window.height());
    let speed = random.gen_range(1.0..8.0);

    let trajectory = (Vec2::new(spawn_x, spawn_y) - Vec2::new(target_x, target_y)).normalize();

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(spawn_coords.x, spawn_coords.y, 0.0),
            ..Default::default()
        },
        texture: asset_server.load(format!("sprites/meteor/{}.png", random.gen_range(1..5))),
        ..Default::default()
    })
    .insert(Asteroid { 
        velocity: Vec2::new(trajectory.x * speed, trajectory.y * speed),
        rotation: random.gen_range(-0.1..0.1)
    });
}

fn asteroid_movement(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>, mut asteroid_transforms: Query<(Entity, &mut Transform, &Asteroid)>) {
    let window = windows.get_single().unwrap();

    for (entity, mut transform, asteroid) in asteroid_transforms.iter_mut() {
        let coords = normalize_coords_in_window(window, transform.translation);
        if coords.x > window.width() + 50.0 || 
           coords.y > window.height() + 50.0 || 
           coords.x < -50.0 || 
           coords.y < -50.0 {
                commands.entity(entity).despawn();
        } else {
            transform.translation += Vec3::new(asteroid.velocity.x, asteroid.velocity.y, 0.0);
            transform.rotate_z(asteroid.rotation);
        }
    }
}

fn calculate_angle(pos1: Vec2, pos2: Vec2) -> f32 {
    let diff = pos2 - pos1;
    diff.y.atan2(diff.x)
}

fn normalize_coords_in_window(window: &Window, coords: Vec3) -> Vec2 {
    Vec2::new((window.width() / 2.) - coords.x, (window.height() / 2.0) - coords.y)
}