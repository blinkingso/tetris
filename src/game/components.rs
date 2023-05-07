use bevy::prelude::Component;

#[derive(Component)]
pub struct PausedLayout;

#[derive(Component)]
pub enum PausedButtonAction {
    Continue,
    Renew,
    Exit,
}

#[derive(Component)]
pub struct GameDisplay;

#[derive(Component)]
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

#[derive(Clone, Copy, Component)]
pub struct MatrixPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct MinoBlock {
    pub position: MatrixPosition,
}

#[derive(Component)]
pub struct UpdateMinoBlock;

#[derive(Component)]
pub struct TetrominoPosition(pub MatrixPosition);
