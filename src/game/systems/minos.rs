//! Spawn current Tetromino

use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    components::{
        Block, BlockBundle, CurrentTetromino, GameArea, HoldQueueTetromino, LockedDownBlock,
        MatrixPosition, UpdateBlock,
    },
    global::get_falling_speed,
    matrix::Matrix,
    resources::{HoldOnQueueResoure, ImagePathResources, ScoreAction, ScoreEvent},
    timer::SoftDropTimer,
};

/// A function to spawn current tetromino shape.
pub fn spawn_current_tetromino(
    mut commands: Commands,
    image_resource: Res<ImagePathResources>,
    mut matrix: ResMut<Matrix>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
    mut hold_on_queue: ResMut<HoldOnQueueResoure>,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    query_hold_on_queue: Query<Entity, With<HoldQueueTetromino>>,
    mut heap_blocks: Query<(Entity, &mut Block), With<LockedDownBlock>>,
    mut score_writer: EventWriter<ScoreEvent>,
) {
    if !matrix.create || matrix.game_over {
        return;
    }
    matrix.create = false;

    // From last line to first
    let mut y = matrix.field_height - 1;
    let mut full_rows = vec![];
    while y > 0 {
        let mut full_row = true;
        for i in 0..matrix.field_width {
            let index = y * matrix.field_width + i;
            if matrix.occupation[index] == 0 {
                full_row = false;
            }
        }

        if full_row {
            full_rows.push(y);
        }
        y -= 1;
    }

    if full_rows.len() > 0 {
        // clear line
        let max_row = full_rows.iter().max().unwrap();
        for _ in full_rows.iter() {
            for (entity, mut block) in heap_blocks.iter_mut() {
                match (*max_row as i32).cmp(&block.position.y) {
                    // top -1
                    std::cmp::Ordering::Less => {}
                    // clear
                    std::cmp::Ordering::Equal => {
                        if block.position.x != -1 {
                            block.position.x = -1;
                            #[cfg(debug_assertions)]
                            {
                                println!("-------------------------");
                                println!(
                                    "pos:{}, {}, entity id: {:?}",
                                    block.position.x,
                                    block.position.y,
                                    commands.entity(entity).id()
                                );
                                println!("-------------------------");
                            }
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        block.position.y += 1;
                        commands.entity(entity).insert(UpdateBlock);
                    }
                }
            }
        }
        let min_y = full_rows.iter().min().unwrap() - 1;
        // clear occupation data
        for j in (1..=min_y).rev() {
            for i in 0..matrix.field_width {
                let down_row_index = (j + full_rows.len()) * matrix.field_width + i;
                let up_row_index = j * matrix.field_width + i;
                matrix.occupation[down_row_index] = matrix.occupation[up_row_index];
            }
        }

        // make sure the first line is cleared.
        for j in 0..full_rows.len() {
            for i in 0..matrix.field_width {
                let index = j * matrix.field_width + i;
                if matrix.occupation[index] == 1 {
                    matrix.occupation[index] = 0;
                }
            }
        }

        score_writer.send(ScoreEvent {
            cleared_lines: full_rows.len(),
            action: ScoreAction::from(full_rows.len()),
        });
        matrix.lines_cleared += full_rows.len();
    }
    if matrix.hard_dropping {
        score_writer.send(ScoreEvent::hard_drop(1));
    }

    let falling_speed = get_falling_speed(matrix.level);

    soft_drop_timer
        .timer
        .set_duration(Duration::from_secs_f32(falling_speed));
    soft_drop_timer.timer.reset();
    soft_drop_timer
        .timer
        .set_elapsed(Duration::from_secs_f32(falling_speed));

    // spawn current tetromino
    let tetromino = hold_on_queue.pop_push();

    for mp in tetromino.get_blocks_position().into_iter() {
        commands
            .spawn(BlockBundle::new(
                matrix.start_pos,
                mp,
                &matrix,
                image_resource.get_path(tetromino.ty),
                asset_server.as_ref(),
                texture_atlas.as_mut(),
            ))
            .insert(CurrentTetromino);
    }
    // spawn current tetromino
    commands.spawn(tetromino).insert(CurrentTetromino);

    // despawn poped tetromino in the hold_on_queue.
    for entity in query_hold_on_queue.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // spawn next tetromino
    let next_tetromino = hold_on_queue.first();
    if let Some(next_tetromino) = next_tetromino {
        for mp in next_tetromino.get_blocks_position().iter() {
            commands
                .spawn(BlockBundle::new(
                    MatrixPosition { x: 12, y: 1 },
                    *mp,
                    matrix.as_ref(),
                    image_resource.get_path(next_tetromino.ty),
                    asset_server.as_ref(),
                    texture_atlas.as_mut(),
                ))
                .insert(HoldQueueTetromino)
                .insert(GameArea::HoldOnQueue);
        }
    }
}

pub fn update_block_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Block), With<UpdateBlock>>,
    matrix: Res<Matrix>,
) {
    for (entity, mut transform, block) in query.iter_mut() {
        let (x, y) = matrix.get_translation(block.position);
        transform.translation = Vec3::new(x, y, 0.0);
        commands.entity(entity).remove::<UpdateBlock>();
    }
}
