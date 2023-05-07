//! Spawn current Tetromino

use bevy::{
    prelude::*,
    sprite::{Anchor},
};

use crate::game::{
    components::{CurrentTetromino, MatrixPosition},
    global::{get_falling_speed, BLOCK_SIZE},
    matrix::Matrix,
    resources::{HoldOnQueueResoure, ImagePathResources, StartPosition},
    timer::SoftDropTimer,
};

/// A function to spawn current tetromino shape.
pub fn spawn_current_tetromino(
    mut commands: Commands,
    image_resource: Res<ImagePathResources>,
    mut matrix: ResMut<Matrix>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
    _start_pos: Res<StartPosition>,
    mut hold_on_queue: ResMut<HoldOnQueueResoure>,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    if !matrix.active {
        return;
    }

    let _falling_speed = get_falling_speed(matrix.level);

    soft_drop_timer.timer.reset();
    // soft_drop_timer
    //     .timer
    //     .set_elapsed(Duration::from_secs_f32(falling_speed));
    // soft_drop_timer
    //     .timer
    //     .set_duration(Duration::from_secs_f32(falling_speed));

    let tetromino = hold_on_queue.pop_push();
    let image = asset_server.load(image_resource.get_path(tetromino.ty));

    let ta = TextureAtlas::from_grid(image, Vec2::new(BLOCK_SIZE, BLOCK_SIZE), 1, 1, None, None);

    let blocks = tetromino.get_blocks();
    for block in blocks.into_iter() {
        let position = MatrixPosition {
            x: block.position.x + tetromino.position.x,
            y: block.position.y + tetromino.position.y,
        };
        let (x, y) = matrix.get_grid_position(position);
        // commands
        //     .spawn(SpriteBundle {
        //         sprite: Sprite {
        //             custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
        //             // color: get_block_color(i.into()),
        //             anchor: Anchor::TopLeft,
        //             ..default()
        //         },
        //         texture: image.clone(),
        //         transform: Transform::from_xyz(x, y, 0.0),
        //         ..default()
        //     })
        //     .insert(CurrentTetromino);
        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    // color: get_block_color(i.into()),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture_atlas: texture_atlas.add(ta.clone()).into(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(CurrentTetromino);
        // commands
        //     .spawn(MaterialMesh2dBundle {
        //         mesh: meshes
        //             .add(Mesh::from(shape::Quad::new(Vec2::new(
        //                 BLOCK_SIZE, BLOCK_SIZE,
        //             ))))
        //             .into(),
        //         material: materials
        //             .add(ColorMaterial {
        //                 texture: Some(image.clone()),
        //                 ..default()
        //             })
        //             .into(),
        //         transform: Transform::from_xyz(x, y, 0.0),
        //         ..default()
        //     })
        //     .insert(CurrentTetromino);
    }

    matrix.active = false;
}
