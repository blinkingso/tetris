//! Game Global Configuration.

use super::resources::ScoreAction;

pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 22;
pub const BLOCK_SIZE: f32 = 40.0;
pub const BORDER_SIZE: f32 = 16.0;
pub const MAX_LEVEL: usize = 10;

pub const WHITESPACE_WIDTH: f32 = 80.0;
pub const WHITESPACE_HEIGHT: f32 = 60.0;

pub const SEPARATE: f32 = BLOCK_SIZE;
pub const RIGHT_WIDTH: f32 = 5.0 * (BLOCK_SIZE + BLOCK_SPACE);

pub const HARD_DROP_SPEED: f32 = 0.0001;

#[cfg(debug_assertions)]
pub const BLOCK_SPACE: f32 = 1.0;

#[cfg(release)]
pub const BLOCK_SPACE: f32 = 0.0;

/// Calculate Game Window min-width and min-height
pub fn get_game_window_min_size() -> (f32, f32) {
    let mut width = (BLOCK_SIZE + BLOCK_SPACE) * FIELD_WIDTH as f32 - BLOCK_SPACE;
    let mut height = (BLOCK_SIZE + BLOCK_SPACE) * FIELD_HEIGHT as f32 - BLOCK_SPACE;
    width += 2.0 * (WHITESPACE_WIDTH + BORDER_SIZE);
    width += SEPARATE;
    width += RIGHT_WIDTH;
    height += 2.0 * (WHITESPACE_HEIGHT + BORDER_SIZE);
    (width, height)
}

pub fn get_matrix_size() -> (f32, f32) {
    (
        (BLOCK_SIZE + BLOCK_SPACE) * FIELD_WIDTH as f32 - BLOCK_SPACE,
        (BLOCK_SIZE + BLOCK_SPACE) * FIELD_HEIGHT as f32 - BLOCK_SPACE,
    )
}

pub fn get_falling_speed(level: usize) -> f32 {
    0.8 - ((level - 1) as f32 * 0.007)
}

/// Calculate scores
pub fn calculate_score(level: usize, action: ScoreAction) -> usize {
    match action {
        ScoreAction::Single => level * 100,
        ScoreAction::Double => level * 300,
        ScoreAction::Triple => level * 500,
        ScoreAction::Tetris => level * 800,
        ScoreAction::MiniTSpin => level * 100,
        ScoreAction::MiniTSpinSingle => level * 200,
        ScoreAction::TSpin => level * 400,
        ScoreAction::TSpinSingle => level * 800,
        ScoreAction::TSpinDouble => level * 1200,
        ScoreAction::TSpinTriple => level * 1600,
        ScoreAction::BackToBackBonus => level * 5400,
        ScoreAction::SoftDrop(n) => n,
        ScoreAction::HardDrop(m) => m * 2,
    }
}
