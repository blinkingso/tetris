use bevy::{prelude::*, transform::commands};

use crate::game::{
    components::{Block, CurrentTetromino, MatrixPosition, UpdateBlock},
    matrix::Matrix,
    tetromino::Tetromino,
};
pub fn rotate_system(
    key_code: Res<Input<KeyCode>>,
    mut query_current_tetromino: Query<&mut Tetromino, With<CurrentTetromino>>,
    mut query_blocks: Query<(Entity, &mut Block), With<CurrentTetromino>>,
    matrix: Res<Matrix>,
    mut commands: Commands,
) {
    if let Ok(mut current_tetromino) = query_current_tetromino.get_single_mut() {
        // clockwise rotation
        let mut new_tetromino = None;
        if key_code.just_pressed(KeyCode::Up) {
            new_tetromino = matrix.can_rotate(1, &current_tetromino);
        }
        // anti-clockwise rotation
        if key_code.just_pressed(KeyCode::X) {
            new_tetromino = matrix.can_rotate(-1, &current_tetromino);
        }

        if let Some(new_tetromino) = new_tetromino {
            let positions = new_tetromino.get_blocks_position();
            let start_pos = new_tetromino.position;
            for (index, (entity, mut block)) in query_blocks.iter_mut().enumerate() {
                let pos = positions[index];
                block.position = MatrixPosition {
                    x: pos.x + start_pos.x,
                    y: pos.y + start_pos.y,
                };
                commands.entity(entity).insert(UpdateBlock);
            }
            *current_tetromino = new_tetromino;
        }
    }
}
