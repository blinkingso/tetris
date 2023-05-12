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
    components::{
        Block, GameArea, GameDisplay, GameOverLayout, HeapCounter, MatrixPosition, PausedLayout,
    },
    matrix::Matrix,
    resources::{HoldOnQueueResoure, Score, ScoreEvent, StartPosition},
    systems::{
        interactions::{game_over_button_actions, paused_button_actions},
        minos::{spawn_current_tetromino, update_block_system},
        movement::contain_cleared_lines,
        paused::{is_game_resumed_or_new, is_game_resumed_or_new_or_paused},
    },
};
use crate::{despawn_components, ui::systems::interactions::button_system, AppState};
use bevy::prelude::*;
use systems::layout::spawn_pause_layout_system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let matrix = Matrix::default();
        let hold_on_queue = HoldOnQueueResoure::new(matrix.start_pos);
        app.insert_resource(Score::default());
        app.insert_resource(hold_on_queue);
        app.insert_resource(matrix);
        app.insert_resource(HeapCounter(0));
        app.add_event::<ScoreEvent>();
        // init game page
        app.add_system(setup_game::setup_game.in_schedule(OnEnter(GameState::New)));
        // enter game over page
        app.add_system(layout::spawn_game_over_layout_system.in_schedule(OnEnter(GameState::Over)));
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

        app.add_system(
            score::update_score
                .in_set(OnUpdate(AppState::Game))
                .run_if(is_game_resumed_or_new),
        );
        app.add_system(
            score::update_level
                .in_set(OnUpdate(AppState::Game))
                .run_if(is_game_resumed_or_new),
        );
        // update tetromino blocks
        app.add_system(
            update_block_system
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
        app.add_system(game_over_button_actions.in_set(OnUpdate(GameState::Over)));

        // hover buttons system
        app.add_system(button_system.run_if(is_paused_or_over));

        // despawn paused layout when exit `GameState::paused`
        app.add_system(despawn_components::<PausedLayout>.in_schedule(OnExit(GameState::Paused)));
        // despawn all entity in GameState on exit GameOver State
        app.add_system(despawn_components::<GameDisplay>.in_schedule(OnExit(GameState::Over)));
        app.add_system(despawn_components::<Block>.in_schedule(OnExit(GameState::Over)));
        app.add_system(despawn_components::<GameOverLayout>.in_schedule(OnExit(GameState::Over)));
        app.add_system(despawn_components::<GameArea>.in_schedule(OnExit(GameState::Over)));
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

pub fn is_paused_or_over(game_state: Res<State<GameState>>) -> bool {
    game_state.0 == GameState::Paused || game_state.0 == GameState::Over
}
