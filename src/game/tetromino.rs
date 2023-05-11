//! Tetrominos Definition

use bevy::prelude::{Color, Component};
use rand::{distributions::Standard, prelude::Distribution, Rng};

use super::components::{Block, MatrixPosition};

const SHAPE_I: [[u8; 4]; 4] = [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

type Tt = [[u8; 3]; 3];

const SHAPE_J: Tt = [[1, 0, 0], [1, 1, 1], [0, 0, 0]];

const SHAPE_L: Tt = [[0, 0, 1], [1, 1, 1], [0, 0, 0]];

const SHAPE_T: Tt = [[0, 1, 0], [1, 1, 1], [0, 0, 0]];

const SHAPE_S: Tt = [[0, 1, 1], [1, 1, 0], [0, 0, 0]];

const SHAPE_Z: Tt = [[1, 1, 0], [0, 1, 1], [0, 0, 0]];

const SHAPE_O: Tt = [[0, 1, 1], [0, 1, 1], [0, 0, 0]];

// SHAPE-J,L,S,T,Z rotate offset data
const OFFSET_DATA_JLSTZ: [i8; 40] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, -1, 0, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, -1, 0, -1, -1, 0, 2, -1, 2,
];

// Shape-I rotate offset data
const OFFSET_DATA_I: [i8; 40] = [
    0, 0, -1, 0, 2, 0, -1, 0, 2, 0, -1, 0, 0, 0, 0, 0, 0, 1, 0, -2, -1, 1, 1, 1, -2, 1, 1, 0, -2,
    0, 0, 1, 0, 1, 0, 1, 0, -1, 0, 2,
];

// Shape-O rotate offset data
const OFFSET_DATA_O: [i8; 8] = [0, 0, 0, -1, -1, -1, -1, 0];

pub fn array_to_vec<const R: usize>(array: [[u8; R]; R]) -> Vec<u8> {
    array.into_iter().flatten().collect()
}

pub fn vec_to_array(vec: Vec<u8>) -> Vec<Vec<u8>> {
    let len = vec.len();
    let row_len = (len as f32).sqrt() as usize;

    let chunks = vec.as_slice().chunks_exact(row_len);
    chunks.into_iter().map(|slice| slice.to_vec()).collect()
}

pub fn get_pieces_data(ty: TetrominoType) -> Vec<u8> {
    match ty {
        TetrominoType::I => array_to_vec(SHAPE_I),
        TetrominoType::J => array_to_vec(SHAPE_J),
        TetrominoType::L => array_to_vec(SHAPE_L),
        TetrominoType::S => array_to_vec(SHAPE_S),
        TetrominoType::Z => array_to_vec(SHAPE_Z),
        TetrominoType::T => array_to_vec(SHAPE_T),
        TetrominoType::O => array_to_vec(SHAPE_O),
    }
}

pub fn get_offset_data(rotation: Rotation, ty: TetrominoType) -> Vec<i8> {
    let rotation = rotation as usize;
    let pieces_offset: Vec<i8> = match ty {
        TetrominoType::I => OFFSET_DATA_I
            .into_iter()
            .enumerate()
            .filter_map(|(index, value)| {
                if index >= 10 * rotation && index < 10 * (rotation + 1) {
                    Some(value)
                } else {
                    None
                }
            })
            .collect(),
        TetrominoType::O => OFFSET_DATA_O
            .into_iter()
            .enumerate()
            .filter_map(|(index, value)| {
                if index >= 2 * rotation && index < 2 * (rotation + 1) {
                    Some(value)
                } else {
                    None
                }
            })
            .collect(),
        _ => OFFSET_DATA_JLSTZ
            .into_iter()
            .enumerate()
            .filter_map(|(index, value)| {
                if index >= 10 * rotation && index < 10 * (rotation + 1) {
                    Some(value)
                } else {
                    None
                }
            })
            .collect(),
    };
    pieces_offset
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TetrominoType {
    I = 0,
    J = 1,
    L = 2,
    S = 3,
    Z = 4,
    T = 5,
    O = 6,
}

impl From<i32> for TetrominoType {
    fn from(value: i32) -> Self {
        match value {
            0 => TetrominoType::I,
            1 => TetrominoType::J,
            2 => TetrominoType::L,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::T,
            _ => TetrominoType::O,
        }
    }
}

pub const BLOCK_COLORS: [Color; 7] = [
    Color::RED,
    Color::ORANGE,
    Color::YELLOW,
    Color::GREEN,
    Color::CYAN,
    Color::BLUE,
    Color::PURPLE,
];

pub fn get_block_color(ty: TetrominoType) -> Color {
    BLOCK_COLORS[ty as usize]
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        match rng.gen_range(0..7) {
            0 => TetrominoType::I,
            1 => TetrominoType::J,
            2 => TetrominoType::L,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::T,
            _ => TetrominoType::O,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
}

impl Rotation {
    pub fn clockwise(&self) -> Self {
        match *self {
            Rotation::R0 => Rotation::R1,
            Rotation::R1 => Rotation::R2,
            Rotation::R2 => Rotation::R3,
            Rotation::R3 => Rotation::R0,
        }
    }

    pub fn counter_clockwise(&self) -> Self {
        match *self {
            Rotation::R0 => Rotation::R3,
            Rotation::R1 => Rotation::R0,
            Rotation::R2 => Rotation::R1,
            Rotation::R3 => Rotation::R2,
        }
    }
}

impl Distribution<Rotation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
        match rng.gen_range(0..4) {
            0 => Rotation::R0,
            1 => Rotation::R1,
            2 => Rotation::R2,
            _ => Rotation::R3,
        }
    }
}

#[derive(Clone, Component)]
pub struct Tetromino {
    pub position: MatrixPosition,
    pub ty: TetrominoType,
    pub rotation: Rotation,
    pub pieces_data: Vec<u8>,
}

impl Tetromino {
    /// Create a new Tetromino
    pub fn new(position: MatrixPosition) -> Tetromino {
        let ty: TetrominoType = rand::random();
        let offset_data = match ty {
            TetrominoType::I => OFFSET_DATA_I.to_vec(),
            TetrominoType::O => OFFSET_DATA_O.to_vec(),
            _ => OFFSET_DATA_JLSTZ.to_vec(),
        };
        Tetromino {
            position,
            ty,
            rotation: Rotation::R0,
            pieces_data: get_pieces_data(ty),
        }
    }

    /// Get all tetromino blocks in separate model.
    pub fn get_blocks_position(&self) -> Vec<MatrixPosition> {
        let row = (self.pieces_data.len() as f32).sqrt() as usize;
        self.pieces_data
            .iter()
            .enumerate()
            .filter_map(|(index, val)| if *val != 0 { Some(index) } else { None })
            .map(|index| MatrixPosition {
                x: (index % row) as i32,
                y: (index / row) as i32,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::{get_offset_data, Rotation, TetrominoType};

    #[test]
    fn test_get_pieces_offset() {
        for _ in 0..8 {
            let rotation: Rotation = rand::random();
            let ty: TetrominoType = rand::random();
            let offsets = get_offset_data(rotation, ty);
            println!(
                "rotation: {:?}, ty: {:?}, offsets: {:?}",
                rotation, ty, offsets
            );
        }
    }
}
