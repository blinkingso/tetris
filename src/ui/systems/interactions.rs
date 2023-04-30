use crate::game::GameState;
use crate::ui::components::*;
use crate::ui::style::*;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub(crate) fn main_menu_button_action(
    mut writer: EventWriter<AppExit>,
    query: Query<(&Interaction, &MainMenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match *action {
                MainMenuButtonAction::Play => {
                    app_state.set(AppState::Game);
                    game_state.set(GameState::New);
                }
                MainMenuButtonAction::Exit => writer.send(AppExit),
            }
        }
    }
}

pub fn button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut bg) in query.iter_mut() {
        *bg = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}
