use std::ops::Add;

use bevy::prelude::*;

use super::{global::BLOCK_SIZE, matrix::Matrix};

#[derive(Component)]
pub struct PausedLayout;
#[derive(Component)]
pub struct GameOverLayout;

#[derive(Component)]
pub enum PausedButtonAction {
    Continue,
    Renew,
    Exit,
}

#[derive(Component)]
pub enum GameOverButtonAction {
    Renew,
    MainMenu,
    Exit,
}

#[derive(Component)]
pub struct GameDisplay;

#[derive(Component, PartialEq, Eq)]
pub enum GameArea {
    Block,
    HoldOnQueue,
    Left,
    LeftBorder,
    RightBorder,
    TopBorder,
    BottomBorder,
    TextArea,
    TextScore,
    TextLevel,
    TextLines,
    TextScoreLabel,
    TextLevelLabel,
    TextLinesLabel,
}
#[derive(Component)]
pub struct CurrentTetromino;

#[derive(Component)]
pub struct HoldQueueTetromino;

#[derive(Debug, Clone, Copy, Component)]
pub struct MatrixPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Block {
    pub position: MatrixPosition,
}

#[derive(Component)]
pub struct UpdateBlock;

#[derive(Component)]
pub struct TetrominoPosition(pub MatrixPosition);

#[derive(Component)]
pub struct LockedDownBlock;

#[derive(Resource)]
pub struct HeapCounter(pub usize);

#[derive(Bundle)]
pub struct BlockBundle {
    pub block: Block,

    #[bundle]
    sprite: SpriteSheetBundle,
}

impl Add<MatrixPosition> for MatrixPosition {
    type Output = MatrixPosition;
    fn add(self, rhs: MatrixPosition) -> Self::Output {
        MatrixPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl BlockBundle {
    ///
    /// Create a new BlockBundle
    /// * start_postion: Tetromino position at Matrix
    /// * rel_position: Block position relative to Tetromino position
    /// * matrix: Matrix resource of the global game.
    ///
    pub fn new(
        start_position: MatrixPosition,
        rel_position: MatrixPosition,
        matrix: &Matrix,
        path: &'static str,
        asset_server: &AssetServer,
        texture_atlas_res: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture = asset_server.load(path);
        let texture_atlas =
            TextureAtlas::from_grid(texture, Vec2::new(BLOCK_SIZE, BLOCK_SIZE), 1, 1, None, None);
        let position = start_position + rel_position;
        let (x, y) = matrix.get_translation(position);
        BlockBundle {
            block: Block { position },
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                texture_atlas: texture_atlas_res.add(texture_atlas),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
        }
    }
}
