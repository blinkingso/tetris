//! Tetromino pieces data with `Matrix`

#[derive(Debug)]
pub struct Matrix {
    pieces_data: Vec<u8>,
    offset_index: usize,
}
