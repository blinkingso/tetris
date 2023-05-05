//! Game Global Configuration.

pub const FIELD_WIDTH: u32 = 10;
pub const FIELD_HEIGHT: u32 = 22;
pub const BLOCK_SIZE: f32 = 40.0;

pub const WHITESPACE_WIDTH: f32 = 80.0;
pub const WHITESPACE_HEIGHT: f32 = 60.0;

pub const SEPARATE: f32 = BLOCK_SIZE;
pub const RIGHT_WIDTH: f32 = 5.0 * (BLOCK_SIZE + BLOCK_SPACE);


#[cfg(debug_assertions)]
pub const BLOCK_SPACE: f32 = 2.0;

#[cfg(release)]
pub const BLOCK_SPACE: f32 = 0.0;

/// Calculate Game Window min-width and min-height
pub fn get_game_window_min_size() -> (f32, f32) {
    let mut width = (BLOCK_SIZE + BLOCK_SPACE) * FIELD_WIDTH as f32 - BLOCK_SPACE;
    let mut height = (BLOCK_SIZE + BLOCK_SPACE) * FIELD_HEIGHT as f32 - BLOCK_SPACE;
    width += 2.0 * WHITESPACE_WIDTH;
    width += SEPARATE;
    width += RIGHT_WIDTH;
    height += 2.0 * WHITESPACE_HEIGHT;
    (width, height)
}

pub fn get_matrix_size() -> (f32, f32) {
    (
        (BLOCK_SIZE + BLOCK_SPACE) * FIELD_WIDTH as f32 - BLOCK_SPACE,
        (BLOCK_SIZE + BLOCK_SPACE) * FIELD_HEIGHT as f32 - BLOCK_SPACE,
    )
}
