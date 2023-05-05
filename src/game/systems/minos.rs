//! Spawn current Tetromino
use bevy::prelude::*;

use crate::game::{matrix::Matrix, resources::ImagePathResources, timer::SoftDropTimer, GameState};

/// A function to spawn current tetromino shape.
pub fn spawn_current_tetromino(
    mut commands: Commands,
    image_resource: Res<ImagePathResources>,
    mut matrix: ResMut<Matrix>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
) {
    if !matrix.active {
        return;
    }
}
