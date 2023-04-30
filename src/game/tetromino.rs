//! Tetrominos Definition

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TetrominoType {
    I = 0,
    J = 1,
    L = 2,
    S = 3,
    Z = 4,
    T = 5,
    O = 6,
}
