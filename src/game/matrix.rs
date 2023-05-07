//! Tetromino pieces data with `Matrix`

use bevy::prelude::Resource;

use crate::game::global::{BLOCK_SIZE, BLOCK_SPACE};

use super::{
    components::MatrixPosition,
    global::{get_matrix_size, FIELD_HEIGHT, FIELD_WIDTH, RIGHT_WIDTH, SEPARATE},
};

#[derive(Resource)]
pub struct Matrix {
    pub field_width: u32,
    pub field_height: u32,
    pub width: f32,
    pub height: f32,
    pub occupation: Vec<u8>,
    pub active: bool,
    pub hard_dropping: bool,
    pub level: usize,
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
            level: 1,
        }
    }
}

impl Matrix {
    pub fn get_index(&self, pos: &MatrixPosition) -> usize {
        (pos.x + pos.y * self.field_width as i32) as usize
    }
    pub fn is_x_collision(&self, pos: &MatrixPosition) -> bool {
        let index = self.get_index(pos);
        pos.x < 0 || pos.x > self.field_width as i32 - 1 || self.occupation[index] != 0
    }

    pub fn is_y_collision(&self, pos: &MatrixPosition) -> bool {
        let index = self.get_index(pos);
        pos.y > self.field_height as i32 - 1 || self.occupation[index] != 0
    }

    pub fn get_grid_position(&self, position: MatrixPosition) -> (f32, f32) {
        let x = position.x;
        let y = position.y;
        let x = -self.width / 2.0 + x as f32 * (BLOCK_SIZE + BLOCK_SPACE)
            - (SEPARATE + RIGHT_WIDTH / 2.0);
        let y = self.height / 2.0 - y as f32 * (BLOCK_SIZE + BLOCK_SPACE);
        (x, y)
    }
}
