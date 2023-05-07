//! Used to setup game page.

use crate::game::{
    components::MatrixPosition,
    global::get_game_window_min_size,
    matrix::Matrix,
    resources::{HoldOnQueueResoure, StartPosition},
};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn setup_game(mut commands: Commands, mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.single_mut();
    let (window_width, window_height) = get_game_window_min_size();
    // window should resize to defined width and height
    window.resize_constraints = WindowResizeConstraints {
        min_height: window_height,
        min_width: window_width,
        ..Default::default()
    };
}
