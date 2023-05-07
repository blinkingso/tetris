//! Movement System
use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    components::{Block, CurrentTetromino, HeapBlock, MatrixPosition, UpdateBlock},
    global::HARD_DROP_SPEED,
    matrix::Matrix,
    timer::SoftDropTimer,
    GameState,
};

pub fn move_horizontal_system(
    mut commands: Commands,
    mut current_blocks: Query<(Entity, &mut Block), With<CurrentTetromino>>,
    key_code: Res<Input<KeyCode>>,
    matrix: Res<Matrix>,
) {
    if matrix.active {
        return;
    }
    let mut desired_x = 0;
    let mut can_move_x = true;
    if key_code.just_pressed(KeyCode::Left) {
        desired_x = -1;
    }
    if key_code.just_pressed(KeyCode::Right) {
        desired_x = 1;
    }
    if desired_x != 0 {
        for (_, block) in current_blocks.iter() {
            let mut position = block.position;
            position.x += desired_x;
            if matrix.is_collision(&position) {
                can_move_x = false;
            }
        }
    }
    if can_move_x {
        for (entity, mut block) in current_blocks.iter_mut() {
            block.position.x += desired_x;
            commands.entity(entity).insert(UpdateBlock);
        }
    }
}

pub fn movement_vertical_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Block), With<CurrentTetromino>>,
    key_code: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
    mut matrix: ResMut<Matrix>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    soft_drop_timer.timer.tick(time.delta());

    let mut desired_y = 0;
    let mut can_move_y = true;

    if key_code.just_pressed(KeyCode::Space) {
        soft_drop_timer
            .timer
            .set_duration(Duration::from_secs_f32(HARD_DROP_SPEED));
    }

    if key_code.just_pressed(KeyCode::Down) || soft_drop_timer.timer.finished() {
        desired_y = 1;
    }

    if desired_y != 0 {
        for (_, block) in query.iter() {
            let mut position = block.position;
            position.y += desired_y;
            if matrix.is_collision(&position) {
                can_move_y = false;
            }
        }
    }

    if can_move_y {
        for (entity, mut block) in query.iter_mut() {
            block.position.y += desired_y;
            commands.entity(entity).insert(UpdateBlock);
        }
    }

    if !can_move_y {
        for (entity, block) in query.iter_mut() {
            let MatrixPosition { x, y } = block.position;
            let index = (matrix.field_width as i32 * y + x) as usize;
            matrix.occupation[index] = 1;
            commands
                .entity(entity)
                .remove::<CurrentTetromino>()
                .insert(HeapBlock);
            if block.position.y == 0 {
                // game over
                matrix.game_over = true;
                game_state.set(GameState::Over);
            }
        }
        matrix.active = true;
    }
}
