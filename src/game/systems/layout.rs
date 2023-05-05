//! Layout in game page.
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::game::components::{GameArea, PausedButtonAction, PausedLayout};
use crate::game::global::{BLOCK_SIZE, BLOCK_SPACE, RIGHT_WIDTH, SEPARATE, WHITESPACE_WIDTH};
use crate::game::matrix::Matrix;
use crate::game::style::{
    get_game_label_text_style, get_game_text_style, PAUSED_LAYOUT_BACKGROUND_COLOR, TEXT_FONT_SIZE,
};
use crate::ui::style::{get_text_style, get_title_text_style, BUTTON_STYLE, NORMAL_BUTTON};

/// Spawn game background area.
pub fn spawn_board_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    matrix: Res<Matrix>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bg: Handle<Image> = asset_server.load("bg.png");
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(
                    matrix.width,
                    matrix.height,
                ))))
                .into(),
            material: materials
                .add(ColorMaterial {
                    texture: Some(bg),
                    ..Default::default()
                })
                .into(),
            transform: Transform::from_xyz(-(SEPARATE + RIGHT_WIDTH / 2.0), 0.0, 0.0),
            ..Default::default()
        },
        GameArea::Left,
    ));
}

pub fn spawn_bg_block_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    matrix: Res<Matrix>,
) {
    let block_bg = asset_server.load("black.png");
    let mut bundles = vec![];
    for i in 0..matrix.field_width {
        for j in 0..matrix.field_height {
            bundles.push((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                        ..default()
                    },
                    texture: block_bg.clone(),
                    transform: Transform::from_xyz(
                        -(SEPARATE + RIGHT_WIDTH / 2.0) - matrix.width / 2.0
                            + i as f32 * (BLOCK_SIZE + BLOCK_SPACE)
                            + BLOCK_SIZE / 2.0,
                        matrix.height / 2.0
                            - j as f32 * (BLOCK_SIZE + BLOCK_SPACE)
                            - BLOCK_SIZE / 2.0,
                        0.0,
                    ),
                    ..default()
                },
                GameArea::Block,
            ));
        }
    }
    commands.spawn_batch(bundles.into_iter());
}

/// Spawn game background area.
pub fn spawn_right_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    matrix: Res<Matrix>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bg: Handle<Image> = asset_server.load("bg.png");

    // 1. Hold on queue Area
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(
                    RIGHT_WIDTH,
                    RIGHT_WIDTH,
                ))))
                .into(),
            material: materials
                .add(ColorMaterial {
                    texture: Some(bg.clone()),
                    ..default()
                })
                .into(),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                matrix.height / 2.0 - RIGHT_WIDTH / 2.0,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::HoldOnQueue);

    // 2. Level
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(
                    RIGHT_WIDTH,
                    BLOCK_SIZE * 4.0,
                ))))
                .into(),
            material: materials
                .add(ColorMaterial {
                    texture: Some(bg.clone()),
                    ..default()
                })
                .into(),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0 + BLOCK_SIZE * 2.0 + SEPARATE * 2.0 + 8.0 * BLOCK_SIZE,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextArea);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Level", get_game_label_text_style(&asset_server))
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0
                    + BLOCK_SIZE * 2.0
                    + SEPARATE * 2.0
                    + 8.0 * BLOCK_SIZE
                    + TEXT_FONT_SIZE,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextLevelLabel);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("1", get_game_text_style(&asset_server))
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0 + BLOCK_SIZE * 2.0 + SEPARATE * 2.0 + 8.0 * BLOCK_SIZE,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextLevel);

    // 3. Score
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(
                    RIGHT_WIDTH,
                    BLOCK_SIZE * 4.0,
                ))))
                .into(),
            material: materials
                .add(ColorMaterial {
                    texture: Some(bg.clone()),
                    ..default()
                })
                .into(),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0 + BLOCK_SIZE * 2.0 + SEPARATE + 4.0 * BLOCK_SIZE,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextArea);
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Score", get_game_label_text_style(&asset_server))
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0
                    + BLOCK_SIZE * 2.0
                    + SEPARATE
                    + 4.0 * BLOCK_SIZE
                    + TEXT_FONT_SIZE,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextScoreLabel);
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(format!("{:07}", 0), get_game_text_style(&asset_server))
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0 + BLOCK_SIZE * 2.0 + SEPARATE + 4.0 * BLOCK_SIZE,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextScore);

    // 4. Lines cleared
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(
                    RIGHT_WIDTH,
                    BLOCK_SIZE * 4.0,
                ))))
                .into(),
            material: materials
                .add(ColorMaterial {
                    texture: Some(bg.clone()),
                    ..default()
                })
                .into(),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0 + BLOCK_SIZE * 2.0,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextArea);
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Lines Cleared", get_game_label_text_style(&asset_server))
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0 + BLOCK_SIZE * 2.0 + TEXT_FONT_SIZE,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextLinesLabel);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(format!("{:07}", 0), get_game_text_style(&asset_server))
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                SEPARATE + RIGHT_WIDTH / 2.0 + WHITESPACE_WIDTH,
                -matrix.height / 2.0 + BLOCK_SIZE * 2.0 - TEXT_FONT_SIZE,
                0.0,
            ),
            ..default()
        })
        .insert(GameArea::TextLines);
}

pub fn spawn_pause_layout_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                background_color: BackgroundColor::from(PAUSED_LAYOUT_BACKGROUND_COLOR),
                ..Default::default()
            },
            PausedLayout,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "You Paused The Game",
                        get_title_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    PausedButtonAction::Continue,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Continue",
                                get_text_style(&asset_server),
                            )],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    PausedButtonAction::Renew,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "New Game",
                                get_text_style(&asset_server),
                            )],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    PausedButtonAction::Exit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("Exit", get_text_style(&asset_server))],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}
