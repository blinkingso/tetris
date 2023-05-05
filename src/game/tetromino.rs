//! Tetrominos Definition

use super::components::MatrixPosition;

const SHAPE_I: [[u8; 4]; 4] = [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]];

type Tt = [[u8; 3]; 3];

const SHAPE_J: Tt = [[0, 0, 1], [0, 0, 1], [0, 1, 1]];

const SHAPE_L: Tt = [[1, 0, 0], [1, 0, 0], [1, 1, 0]];

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

pub fn get_offset_data(_rotation: Rotation, _ty: TetrominoType) -> Vec<(i8, i8)> {
    todo!()
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

pub struct Tetromino {
    pub position: MatrixPosition,
    pub ty: TetrominoType,
    pub rotation: Rotation,
    pub pieces_data: Vec<u8>,
    pub offset_data: Vec<u8>,
}
