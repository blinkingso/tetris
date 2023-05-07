//! Game logic and structs definitions
mod components;
mod matrix;
pub mod resources;
mod systems;
mod tetromino;
pub mod timer;
use systems::*;
mod global;
mod style;

use self::{
    components::{GameArea, GameDisplay, MatrixPosition, PausedLayout},
    matrix::Matrix,
    resources::{HoldOnQueueResoure, StartPosition},
    systems::{
        interactions::paused_button_actions,
        minos::spawn_current_tetromino,
        paused::{is_game_resumed_or_new, is_game_resumed_or_new_or_paused},
    },
};
use crate::{despawn_components, ui::systems::interactions::button_system, AppState};
use bevy::prelude::*;
use systems::layout::spawn_pause_layout_system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // init matrix;
        let matrix = Matrix::default();
        let start_pos = StartPosition(MatrixPosition { x: 4, y: 0 });
        let hold_on_queue = HoldOnQueueResoure::new(&start_pos);
        app.insert_resource(start_pos);
        app.insert_resource(hold_on_queue);
        app.insert_resource(matrix);
        // init game page
        app.add_system(setup_game::setup_game.in_schedule(OnEnter(GameState::New)));
        // init board area
        // init right area (Score, Next shape...)
        app.add_systems(
            (
                layout::spawn_board_system,
                layout::spawn_bg_block_system,
                layout::spawn_right_system,
            )
                .chain()
                .in_schedule(OnEnter(GameState::New)),
        );

        // despawn game page when exit
        app.add_system(despawn_components::<GameDisplay>.in_schedule(OnExit(AppState::Game)));
        app.add_system(despawn_components::<GameArea>.in_schedule(OnExit(AppState::Game)));

        // spawn current_tetromino
        app.add_system(
            spawn_current_tetromino
                .in_set(OnUpdate(AppState::Game))
                .run_if(is_game_resumed_or_new),
        );

        // movement in game state with new or resumed
        app.add_system(
            movement::movement_system
                .in_set(OnUpdate(AppState::Game))
                .run_if(is_game_resumed_or_new),
        );

        // change game state in AppState::Game state
        // press key P/Esc to change to paused state
        app.add_system(
            systems::paused::pause_resume_system
                .in_set(OnUpdate(AppState::Game))
                .run_if(is_game_resumed_or_new_or_paused),
        );

        // when in game state and new or resumed
        app.add_system(spawn_pause_layout_system.in_schedule(OnEnter(GameState::Paused)));

        // Paused interactions running when paused
        app.add_system(paused_button_actions.in_set(OnUpdate(GameState::Paused)));

        // hover buttons system
        app.add_system(button_system.in_set(OnUpdate(GameState::Paused)));

        // despawn paused layout when exit `GameState::paused`
        app.add_system(despawn_components::<PausedLayout>.in_schedule(OnExit(GameState::Paused)));
    }
}

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub enum GameState {
    None,
    #[default]
    New,
    Resume,
    Paused,
    Over,
}
