use crate::game::components::PausedButtonAction;
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
