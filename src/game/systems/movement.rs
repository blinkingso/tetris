//! Movement System
use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    components::{Block, CurrentTetromino, LockedDownBlock, UpdateBlock},
    global::HARD_DROP_SPEED,
    matrix::Matrix,
    resources::ScoreEvent,
    tetromino::Tetromino,
    timer::SoftDropTimer,
    GameState,
};

pub fn debug_minos(
    matrix: Res<Matrix>,
    mut commands: Commands,
    query: Query<(Entity, &mut Block), With<LockedDownBlock>>,
) {
    if matrix.create {
        println!("**********************************");
        for (entity, block) in query.iter() {
            println!(
                "pos:{}, {}, entity id: {:?}",
                block.position.x,
                block.position.y,
                commands.entity(entity).id()
            );
        }
        println!("=================================");
    }
}

pub fn movement_system(
    mut commands: Commands,
    mut matrix: ResMut<Matrix>,
    time: Res<Time>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
    mut current_minos: Query<(Entity, &mut Block), With<CurrentTetromino>>,
    mut current_tetromino: Query<(Entity, &mut Tetromino), With<CurrentTetromino>>,
    key_code: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut score_writer: EventWriter<ScoreEvent>,
) {
    if matrix.create {
        return;
    }
    soft_drop_timer.timer.tick(time.delta());

    if let Ok((entity, mut current_tetromino)) = current_tetromino.get_single_mut() {
        let mut can_move_x = true;
        let mut can_move_y = true;
        let mut desired_x = 0;
        let mut desired_y = 0;
        let mut desired_rot = 0;

        if key_code.just_pressed(KeyCode::Left) || key_code.just_pressed(KeyCode::J) {
            desired_x = -1;
        }
        if key_code.just_pressed(KeyCode::Right) || key_code.just_pressed(KeyCode::L) {
            desired_x = 1;
        }
        if key_code.just_pressed(KeyCode::Up) {
            desired_rot = 1;
        }
        if key_code.just_pressed(KeyCode::X) {
            desired_rot = -1;
        }
        if key_code.just_pressed(KeyCode::Down)
            || key_code.just_pressed(KeyCode::K)
            || soft_drop_timer.timer.just_finished()
        {
            desired_y = 1;
        }

        if key_code.just_pressed(KeyCode::Space) {
            desired_x = 0;
            desired_y = 1;
            soft_drop_timer
                .timer
                .set_duration(Duration::from_secs_f32(HARD_DROP_SPEED));
            matrix.hard_dropping = true;
        }

        #[cfg(debug_assertions)]
        if key_code.just_pressed(KeyCode::Slash) {
            matrix.print();
        }

        if desired_x == 0 && desired_y == 0 && desired_rot == 0 {
            return;
        }
        if desired_rot != 0 {
            if let Some(new_tetromino) = matrix.can_rotate(desired_rot, &current_tetromino) {
                let positions = new_tetromino.get_blocks_position();
                for (index, (entity, mut block)) in current_minos.iter_mut().enumerate() {
                    let pos = positions[index];
                    block.position = matrix.start_pos + pos;
                    commands.entity(entity).insert(UpdateBlock);
                }
                *current_tetromino = new_tetromino;
                desired_x = 0;
                desired_y = 0;
            }
        }

        // prefered to move y in prioty
        let max_y = current_minos
            .iter()
            .map(|(_, b)| b.position.y)
            .max()
            .unwrap_or(matrix.field_height as i32 - 1);
        // used when hard dropping
        let mut min_y = max_y;

        'piece: for (_entity, block) in current_minos.iter_mut() {
            if block.position.x + desired_x < 0
                || block.position.x + desired_x > matrix.field_width as i32 - 1
            {
                can_move_x = false;
            }
            if block.position.y + desired_y > matrix.field_height as i32 - 1 {
                can_move_y = false;
                break 'piece;
            }

            if can_move_y && desired_y != 0 {
                let index = matrix.field_width * (block.position.y + desired_y) as usize
                    + block.position.x as usize;
                if matrix.occupation[index] == 1 {
                    can_move_y = false;
                }
            }

            if can_move_x && desired_x != 0 {
                let index = matrix.field_width * block.position.y as usize
                    + (block.position.x + desired_x) as usize;
                if matrix.occupation[index] == 1 {
                    can_move_x = false;
                }
            }

            if !can_move_x && !can_move_y {
                break 'piece;
            }
        }

        if can_move_x || can_move_y {
            for (entity, mut block) in current_minos.iter_mut() {
                if can_move_x {
                    block.position.x += desired_x;
                }

                if can_move_y {
                    block.position.y += desired_y;
                }
                commands.entity(entity).insert(UpdateBlock);
            }

            if can_move_x {
                matrix.start_pos.x += desired_x;
            }
            if can_move_y {
                matrix.start_pos.y += desired_y;
                if !matrix.hard_dropping {
                    score_writer.send(ScoreEvent::soft_drop());
                } else {
                    min_y += 1;
                }
            }
        }

        if !can_move_y && desired_y != 0 {
            for (entity, block) in current_minos.iter_mut() {
                if block.position.y == 1 {
                    matrix.game_over = true;
                    matrix.create = false;
                    game_state.set(GameState::Over);
                }

                commands.entity(entity).remove::<CurrentTetromino>();
                commands.entity(entity).insert(LockedDownBlock);
                let index =
                    matrix.field_width * block.position.y as usize + block.position.x as usize;
                matrix.occupation[index] = 1;
            }

            if !matrix.game_over {
                score_writer.send(ScoreEvent::hard_drop(min_y - max_y));
                matrix.hard_dropping = false;
                matrix.create = true;
                matrix.reset_start_pos();
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
