//! Tetromino pieces data with `Matrix`

use bevy::prelude::Resource;

use crate::game::global::{BLOCK_SIZE, BLOCK_SPACE};

use super::{
    components::MatrixPosition,
    global::{get_matrix_size, FIELD_HEIGHT, FIELD_WIDTH, RIGHT_WIDTH, SEPARATE},
    tetromino::{get_offset_data, Tetromino},
};

#[derive(Resource)]
pub struct Matrix {
    pub field_width: usize,
    pub field_height: usize,
    pub width: f32,
    pub height: f32,
    pub occupation: Vec<u8>,
    pub create: bool,
    pub hard_dropping: bool,
    pub level: usize,
    pub game_over: bool,
    pub lines_cleared: usize,
    pub start_pos: MatrixPosition,
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
            create: true,
            hard_dropping: false,
            level: 1,
            game_over: false,
            lines_cleared: 0,
            start_pos: MatrixPosition { x: 3, y: 0 },
        }
    }
}

impl Matrix {
    pub fn reset_start_pos(&mut self) {
        self.start_pos = MatrixPosition { x: 3, y: 0 };
    }
    pub fn get_index(&self, pos: &MatrixPosition) -> usize {
        (pos.x + pos.y * self.field_width as i32) as usize
    }

    pub fn check_collision(&self, pos: &MatrixPosition) -> bool {
        if pos.x < 0 || pos.x > self.field_width as i32 - 1 || pos.y > self.field_height as i32 - 1
        {
            return true;
        }

        let index = self.get_index(pos);
        self.occupation[index] != 0
    }

    pub fn get_translation(&self, position: MatrixPosition) -> (f32, f32) {
        let x = position.x;
        let y = position.y;
        let x = -self.width / 2.0 + x as f32 * (BLOCK_SIZE + BLOCK_SPACE)
            - (SEPARATE + RIGHT_WIDTH / 2.0);
        let y = self.height / 2.0 - y as f32 * (BLOCK_SIZE + BLOCK_SPACE);
        (x, y)
    }

    pub fn renew(&mut self) {
        *self = Matrix::default();
    }

    /// Check if current tetromino can rotate or not, return New Tetromino if allowed.
    pub fn can_rotate(
        &mut self,
        direction: i32,
        current_tetromino: &Tetromino,
    ) -> Option<Tetromino> {
        let (new_rotation, clockwise) = if direction >= 0 {
            // clockwise
            (current_tetromino.rotation.clockwise(), true)
        } else {
            //counter-clockwise
            (current_tetromino.rotation.counter_clockwise(), false)
        };
        // rotate tetromino-matrix and get blocks's new position
        let new_pieces_data = self.rotate(&current_tetromino.pieces_data, clockwise);
        let new_tetromino = Tetromino {
            pieces_data: new_pieces_data,
            rotation: new_rotation,
            ty: current_tetromino.ty,
        };

        let mut collision = false;
        for pos in new_tetromino.get_blocks_position().iter() {
            let m_pos = MatrixPosition {
                x: self.start_pos.x + pos.x,
                y: self.start_pos.y + pos.y,
            };
            if self.check_collision(&m_pos) {
                collision = true;
            }
        }

        if !collision {
            return Some(new_tetromino);
        }

        // check and try to do rotation.
        let origin_offsets = get_offset_data(current_tetromino.rotation, current_tetromino.ty);
        let new_offsets = get_offset_data(new_rotation, current_tetromino.ty);

        for i in (0..origin_offsets.len()).step_by(2) {
            let o_x = origin_offsets[i];
            let o_y = origin_offsets[i + 1];
            let n_x = new_offsets[i];
            let n_y = new_offsets[i + 1];
            let (x, y) = (o_x - n_x, o_y - n_y);
            let new_start_pos = MatrixPosition {
                x: self.start_pos.x + x as i32,
                y: self.start_pos.y + y as i32,
            };

            let blocks = new_tetromino.get_blocks_position();
            let mut collision = false;
            for pos in blocks.iter() {
                let m_pos = MatrixPosition {
                    x: new_start_pos.x + pos.x,
                    y: new_start_pos.y + pos.y,
                };
                if self.check_collision(&m_pos) {
                    collision = true;
                }
            }

            if !collision {
                self.start_pos = new_start_pos;
                return Some(new_tetromino);
            }
        }
        None
    }

    /// Matrix Rotation
    pub(crate) fn rotate(&self, pieces_data: &[u8], clockwise: bool) -> Vec<u8> {
        let row = (pieces_data.len() as f32).sqrt() as usize;
        let mut new_arr = vec![0u8; pieces_data.len()];

        if clockwise {
            for i in 0..row {
                for j in 0..row {
                    let index = (row - 1 - j) * row + i;
                    let new_index = i * row + j;
                    new_arr[new_index] = pieces_data[index];
                }
            }
        } else {
            for i in 0..row {
                for j in 0..row {
                    let index = j * row + (row - 1 - i);
                    let new_index = i * row + j;
                    new_arr[new_index] = pieces_data[index];
                }
            }
        }

        new_arr
    }

    pub fn print(&self) {
        for i in 0..self.field_height {
            for j in 0..self.field_width {
                print!("{}, ", self.occupation[i * self.field_width + j]);
            }
            println!()
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_rotate_matrix() {
        let matrix = Matrix::default();
        let mut pieces_data =
            super::super::tetromino::get_pieces_data(crate::game::tetromino::TetrominoType::T);
        println!("old pieces: {:?}", pieces_data);
        for _ in 0..4 {
            let new_pices = matrix.rotate(&pieces_data, true);
            println!("new pieces: {:?}", new_pices);
            pieces_data = new_pices;
        }

        println!("{:5}", "=============");
        let mut pieces_data =
            super::super::tetromino::get_pieces_data(crate::game::tetromino::TetrominoType::T);
        println!("old pieces: {:?}", pieces_data);
        for _ in 0..4 {
            let new_pices = matrix.rotate(&pieces_data, false);
            println!("new pieces: {:?}", new_pices);
            pieces_data = new_pices;
        }
    }
}
