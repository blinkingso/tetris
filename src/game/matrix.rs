//! Tetromino pieces data with `Matrix`

use bevy::prelude::Resource;

use super::{
    components::MatrixPosition,
    global::{get_matrix_size, FIELD_HEIGHT, FIELD_WIDTH},
};

#[derive(Debug, Resource)]
pub struct Matrix {
    pub field_width: u32,
    pub field_height: u32,
    pub width: f32,
    pub height: f32,
    pub occupation: Vec<u8>,
    pub active: bool,
    pub hard_dropping: bool,
}

impl Default for Matrix {
    fn default() -> Self {
        let (width, height) = get_matrix_size();
        let mut occupation = Vec::with_capacity(FIELD_WIDTH as usize * FIELD_HEIGHT as usize);
        for _i in 0..occupation.capacity() {
            occupation.push(0u8);
        }
        Matrix {
            field_width: FIELD_WIDTH,
            field_height: FIELD_HEIGHT,
            width,
            height,
            occupation,
            active: true,
            hard_dropping: false,
        }
    }
}

impl Matrix {
    pub fn check_collision(pos: &MatrixPosition) {}
}
