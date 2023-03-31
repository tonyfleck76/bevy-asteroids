use std::f32::consts::PI;
use std::fmt;
use std::time::Duration;

use bevy::sprite::collide_aabb::collide;
use bevy::time::common_conditions::on_fixed_timer;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy::log;
use rand::Rng;

const PLAYER_LIVES: u8 = 3;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

const LASER_SPEED: f32 = 10.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const LIFE_PADDING: f32 = 25.0;

trait HitBox {
    fn get_box(&self) -> Vec2;
}

#[derive(Component)]
struct Player {
    lives: u8
}

impl HitBox for Player {
    fn get_box(&self) -> Vec2 {
        Vec2::new(32.0, 48.0)
    }
}

#[derive(Resource)]
struct Scoreboard {
    score: usize
}

#[derive(Component)]
struct Laser{
    velocity: Vec2
}

#[derive(Component)]
struct Asteroid {
    trajectory: Vec2,
    speed: f32,
    rotation: f32,
    width: f32,
    height: f32
}
impl HitBox for Asteroid {
    fn get_box(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

#[derive(Component)]
struct Life {
    counter: u8
}

#[derive(Component)]
struct GameObject;


struct FireEvent;

struct PlayerHitEvent;

struct GameOverEvent;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    Paused,
    GameOver
}

fn main() {
    App::new()
        // Default and window setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroids!".into(),
                resolution: (800., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // State
        .add_state::<AppState>()
        // Events
        .add_event::<FireEvent>()
        .add_event::<PlayerHitEvent>()
        .add_event::<GameOverEvent>()
        // Base systems
        .add_startup_system(setup_camera)
        // Setup new game
        .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
        .add_system(setup_scoreboard.in_schedule(OnEnter(AppState::InGame)))
        .add_system(setup_life_counter.in_schedule(OnEnter(AppState::InGame)))
        // Update in game
        .add_system(aiming_handler.in_set(OnUpdate(AppState::InGame)))
        .add_system(shooting_handler.in_set(OnUpdate(AppState::InGame)))
        .add_system(shoot.in_set(OnUpdate(AppState::InGame)))
        .add_system(laser_movement.in_set(OnUpdate(AppState::InGame)))
        .add_system(spawn_asteroid.in_set(OnUpdate(AppState::InGame)).run_if(on_fixed_timer(Duration::from_secs_f32(1.0))))
        .add_system(asteroid_movement.in_set(OnUpdate(AppState::InGame)))
        .add_system(check_player_collisions.in_set(OnUpdate(AppState::InGame)))
        .add_system(check_laser_collisions.in_set(OnUpdate(AppState::InGame)))
        .add_system(update_scoreboard.in_set(OnUpdate(AppState::InGame)))
        .add_system(update_life_counter.in_set(OnUpdate(AppState::InGame)))
        .add_system(game_over_listener.in_set(OnUpdate(AppState::InGame)))
        // Setup game over
        .add_system(show_game_over_screen.in_schedule(OnEnter(AppState::GameOver)))
        // Game Over Listener
        .add_system(play_again_listener.in_set(OnUpdate(AppState::GameOver)))
        // Clean up game over
        .add_system(clear_game_over.in_schedule(OnExit(AppState::GameOver)))
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
        .insert(Player { lives: PLAYER_LIVES })
        .insert(GameObject);
}

fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/Excluded.ttf");
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle { font: font.clone(), font_size: SCOREBOARD_FONT_SIZE, color: Color::WHITE }
            ),
            TextSection::from_style(TextStyle { font: font.clone(), font_size: SCOREBOARD_FONT_SIZE, color: Color::WHITE })
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        })
    )
    .insert(GameObject);
    commands.insert_resource(Scoreboard { score: 0 })
}

fn setup_life_counter(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let life_sprite: Handle<Image> = asset_server.load("sprites/life.png");
    let window = windows.get_single().unwrap();

    for life in 1..PLAYER_LIVES + 1 {
        let offset = LIFE_PADDING * life as f32;
        commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new((window.width() / 2.0) - offset, (window.height() / 2.0) - LIFE_PADDING, 1.0),
                    ..Default::default()
                },
                texture: life_sprite.clone(),
                ..Default::default()
            })
            .insert(Life { counter: life })
            .insert(GameObject);
    }
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
                .insert(Laser { velocity: Vec2::new(trajectory.x * LASER_SPEED, trajectory.y * LASER_SPEED) })
                .insert(GameObject);
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

    let asteroid_type = random.gen_range(1..5);
    let asteroid_size = match asteroid_type {
        1 | 3 => 48.0,
        2 | 4 => 32.0,
        _ => 0.0
    };

    commands.spawn(SpriteBundle {
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
        height: asteroid_size
    })
    .insert(GameObject);
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
            transform.translation += Vec3::new(asteroid.trajectory.x * asteroid.speed, asteroid.trajectory.y * asteroid.speed, 0.0);
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

fn check_player_collisions(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform, &Asteroid), Without<Player>>,
    player_query: Query<(&Transform, &Player), Without<Asteroid>>,
    mut player_hit_writer: EventWriter<PlayerHitEvent>
) {
    let (player_transform, player) = player_query.single();

    for (asteroid_entity, asteroid_transform, asteroid) in asteroid_query.iter() {
        let player_collision = collide(player_transform.translation, player.get_box(), asteroid_transform.translation, asteroid.get_box());

        if player_collision.is_some() {
            commands.entity(asteroid_entity).despawn();
            player_hit_writer.send(PlayerHitEvent);
        }
    }
}

fn check_laser_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    asteroid_query: Query<(Entity, &Transform, &Asteroid), Without<Laser>>,
    laser_query: Query<(Entity, &Transform), (With<Laser>, Without<Asteroid>)>
) {
    for (asteroid_entity, asteroid_transform, asteroid) in asteroid_query.iter() {
        for (laser_entity, laser_transform) in laser_query.iter() {
            let collision = collide(laser_transform.translation, laser_transform.scale.truncate(), asteroid_transform.translation, asteroid.get_box());

            if collision.is_some() {
                scoreboard.score += asteroid.speed as usize;
                commands.entity(asteroid_entity).despawn();
                commands.entity(laser_entity).despawn();
            }
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn update_life_counter(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Player>,
    mut life_query: Query<(Entity, &Life)>,
    asset_server: Res<AssetServer>,
    mut player_hit_reader: EventReader<PlayerHitEvent>,
    mut game_over_writer: EventWriter<GameOverEvent>
) {
    let lost_life_image: Handle<Image> = asset_server.load("sprites/lost_life.png");
    let window = windows.get_single().unwrap();

    if player_hit_reader.iter().next().is_some() {
        let mut player = player_query.get_single_mut().unwrap();
        player.lives -= 1;

        if player.lives == 0 {
            game_over_writer.send(GameOverEvent);
        }

        for (entity, life) in life_query.iter_mut() {
            if life.counter > player.lives {
                commands.entity(entity).despawn();
                commands.spawn(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new((window.width() / 2.0) - (life.counter as f32 * LIFE_PADDING), (window.height() / 2.0) - LIFE_PADDING, 1.0),
                        scale: Vec3::new(0.6, 0.6, 1.0),
                        ..Default::default()
                    },
                    texture: lost_life_image.clone(),
                    ..Default::default()
                })
                .insert(GameObject);
            }
        }
    }
}

fn game_over_listener(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_over_reader: EventReader<GameOverEvent>
) {
    if game_over_reader.iter().next().is_some() {
        next_state.set(AppState::GameOver);
    }
}

fn show_game_over_screen(
    mut commands: Commands,
    scoreboard: Res<Scoreboard>,
    entities_query: Query<Entity, With<GameObject>>,
    asset_server: Res<AssetServer>
) {
    let score = scoreboard.score;

    for entity in entities_query.iter() {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<Scoreboard>();

    let font: Handle<Font> = asset_server.load("fonts/Excluded.ttf");
    let italic_font: Handle<Font> = asset_server.load("fonts/ExcludedItalic.ttf");
    commands.spawn(TextBundle::from_section(
        "Game Over", 
        TextStyle { font: italic_font.clone(), font_size: 60.0, color: Color::WHITE }
    ).with_style(Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(300.0),
            left: Val::Px(225.0),
            ..default()
        },
        ..default()
    }))
    .insert(GameObject);

    commands.spawn(TextBundle::from_section(
        format!("Final score: {}", score), TextStyle { font: font.clone(), font_size: SCOREBOARD_FONT_SIZE, color: Color::WHITE }
    ).with_style(Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(400.0),
            left: Val::Px(225.0),
            ..default()
        },
        ..default()
    }))
    .insert(GameObject);

    commands.spawn(TextBundle::from_section(
        "Click anywhere to play again.",
        TextStyle { font: font.clone(), font_size: 24.0, color: Color::WHITE }
    ).with_style(Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(450.0),
            left: Val::Px(175.0),
            ..default()
        },
        ..default()
    }))
    .insert(GameObject);
}

fn play_again_listener(
    buttons: Res<Input<MouseButton>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        next_state.set(AppState::InGame);
    }
}

fn clear_game_over (
    mut commands: Commands,
    entities_query: Query<Entity, With<GameObject>>
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn();
    }
}