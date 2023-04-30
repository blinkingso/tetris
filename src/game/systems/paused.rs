use bevy::prelude::*;

use crate::{game::GameState, AppState};

pub fn pause_resume_system(
    key_code: Res<Input<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if key_code.just_pressed(KeyCode::Escape) || key_code.just_pressed(KeyCode::P) {
        match current_game_state.0 {
            GameState::Paused => game_state.set(GameState::Resume),
            GameState::New => game_state.set(GameState::Paused),
            GameState::Resume => game_state.set(GameState::Paused),
            _ => {
                // we can do nothing here.
            }
        }
    }
}

pub fn is_game_resumed_or_new(
    current_app_state: Res<State<AppState>>,
    current_game_state: Res<State<GameState>>,
) -> bool {
    current_app_state.0 == AppState::Game
        && (current_game_state.0 == GameState::Resume || current_game_state.0 == GameState::New)
}

pub fn is_game_resumed_or_new_or_paused(
    current_app_state: Res<State<AppState>>,
    current_game_state: Res<State<GameState>>,
) -> bool {
    current_app_state.0 == AppState::Game
        && (current_game_state.0 == GameState::Resume
            || current_game_state.0 == GameState::New
            || current_game_state.0 == GameState::Paused)
}
