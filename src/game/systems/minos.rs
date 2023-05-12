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
    let mut full_rows = 0;
    while y > 0 {
        let mut full_row = true;
        for i in 0..matrix.field_width {
            let index = y * matrix.field_width + i;
            if matrix.occupation[index] == 0 {
                full_row = false;
            }
        }

        if full_row {
            matrix.print();
            full_rows += 1;
            // clear line
            for (entity, mut block) in heap_blocks.iter_mut() {
                match (y as i32).cmp(&block.position.y) {
                    // top -1
                    std::cmp::Ordering::Less => {}
                    // clear
                    std::cmp::Ordering::Equal => {
                        block.position.x = -1;
                        commands.entity(entity).despawn_recursive();
                    }
                    std::cmp::Ordering::Greater => {
                        block.position.y += 1;
                        commands.entity(entity).insert(UpdateBlock);
                    }
                }
            }
            // clear occupation data
            for j in (1..=y).rev() {
                for i in 0..matrix.field_width {
                    let down_row_index = j * matrix.field_width + i;
                    let up_row_index = (j - 1) * matrix.field_width + i;
                    matrix.occupation[down_row_index] = matrix.occupation[up_row_index];
                }
            }

            // make sure the first line is cleared.
            for i in 0..matrix.field_width {
                if matrix.occupation[i] == 1 {
                    matrix.occupation[i] = 0;
                }
            }
        } else {
            y -= 1;
        }
    }

    if full_rows > 0 {
        score_writer.send(ScoreEvent {
            cleared_lines: full_rows,
            action: ScoreAction::from(full_rows),
        });
        matrix.lines_cleared += full_rows;
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
