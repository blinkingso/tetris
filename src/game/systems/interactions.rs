use crate::game::components::{GameOverButtonAction, PausedButtonAction};
use crate::game::matrix::Matrix;

use crate::{AppState, GameState};
use bevy::app::AppExit;
use bevy::prelude::*;


pub fn paused_button_actions(
    query: Query<(&Interaction, &PausedButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut writer: EventWriter<AppExit>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match *action {
                PausedButtonAction::Continue => {
                    game_state.set(GameState::Resume);
                }
                PausedButtonAction::Renew => {
                    app_state.set(AppState::MainMenu);
                    game_state.set(GameState::None);
                }
                PausedButtonAction::Exit => writer.send(AppExit),
            }
        }
    }
}

pub fn game_over_button_actions(
    query: Query<(&Interaction, &GameOverButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut writer: EventWriter<AppExit>,
    mut matrix: ResMut<Matrix>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match *action {
                GameOverButtonAction::Renew => {
                    matrix.renew();
                    game_state.set(GameState::New);
                }
                GameOverButtonAction::MainMenu => {
                    matrix.renew();
                    app_state.set(AppState::MainMenu);
                    game_state.set(GameState::None);
                }
                GameOverButtonAction::Exit => writer.send(AppExit),
            }
        }
    }
}
