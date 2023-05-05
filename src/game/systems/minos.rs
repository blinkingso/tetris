//! Spawn current Tetromino
use bevy::prelude::*;

use crate::game::{matrix::Matrix, resources::ImagePathResources, timer::SoftDropTimer};

/// A function to spawn current tetromino shape.
pub fn spawn_current_tetromino(
    _commands: Commands,
    _image_resource: Res<ImagePathResources>,
    matrix: ResMut<Matrix>,
    _soft_drop_timer: ResMut<SoftDropTimer>,
) {
    if !matrix.active {
        return;
    }
}
