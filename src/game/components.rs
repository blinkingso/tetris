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

#[derive(Component)]
pub struct MatrixPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct MinoBlock;

#[derive(Component)]
pub struct UpdateMinoBlock;
