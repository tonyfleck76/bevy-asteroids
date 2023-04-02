use bevy::prelude::*;

use crate::global::component::GameObject;
use crate::main_menu::constants::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle::from_section(
        "'Stroids", 
        TextStyle { font: asset_server.load("fonts/ExcludedItalic.ttf"), font_size: 60.0, color: Color::WHITE }
    ).with_style(Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(100.0),
            left: Val::Px(250.0),
            ..default()
        },
        ..default()
    }))
    .insert(GameObject);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/Excluded.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ))
                    .insert(GameObject);
                })
                .insert(GameObject);
        })
        .insert(GameObject);
}