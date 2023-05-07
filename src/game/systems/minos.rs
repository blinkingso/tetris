//! Spawn current Tetromino

use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};

use crate::game::{
    components::{
        Block, CurrentTetromino, GameArea, HoldQueueTetromino, MatrixPosition, UpdateBlock,
    },
    global::{get_falling_speed, BLOCK_SIZE, BLOCK_SPACE},
    matrix::Matrix,
    resources::{HoldOnQueueResoure, ImagePathResources},
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
) {
    if !matrix.active {
        return;
    }

    let falling_speed = get_falling_speed(matrix.level);

    soft_drop_timer
        .timer
        .set_duration(Duration::from_secs_f32(falling_speed));
    soft_drop_timer.timer.reset();

    soft_drop_timer
        .timer
        .set_elapsed(Duration::from_secs_f32(falling_speed));

    let tetromino = hold_on_queue.pop_push();
    let image = asset_server.load(image_resource.get_path(tetromino.ty));

    let ta = TextureAtlas::from_grid(image, Vec2::new(BLOCK_SIZE, BLOCK_SIZE), 1, 1, None, None);

    let blocks = tetromino.get_blocks();
    for block in blocks.into_iter() {
        let position = MatrixPosition {
            x: block.position.x + tetromino.position.x,
            y: block.position.y + tetromino.position.y,
        };
        let (x, y) = matrix.get_translation(position);
        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture_atlas: texture_atlas.add(ta.clone()).into(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(CurrentTetromino)
            .insert(Block { position });
    }

    for entity in query_hold_on_queue.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // spawn next tetromino
    let next_tetromino = hold_on_queue.first();
    if let Some(next_tetromino) = next_tetromino {
        let image = asset_server.load(image_resource.get_path(next_tetromino.ty));
        let ta =
            TextureAtlas::from_grid(image, Vec2::new(BLOCK_SIZE, BLOCK_SIZE), 1, 1, None, None);
        let blocks = next_tetromino.get_blocks();
        for block in blocks.iter() {
            let position = MatrixPosition {
                x: block.position.x + 12,
                y: block.position.y + 1,
            };
            let (x, y) = matrix.get_translation(position);
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    texture_atlas: texture_atlas.add(ta.clone()).into(),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                })
                .insert(HoldQueueTetromino)
                .insert(GameArea::HoldOnQueue);
        }
    }

    matrix.active = false;
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
