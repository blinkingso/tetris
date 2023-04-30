pub(crate) mod components;
pub(crate) mod style;
pub(crate) mod systems;

use crate::ui::systems::interactions::*;
use crate::ui::systems::layout::spawn_main_menu_system;
use crate::{despawn_components, AppState};
use bevy::prelude::{IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin};

use self::components::MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_main_menu_system.in_schedule(OnEnter(AppState::MainMenu)));
        app.add_system(despawn_components::<MainMenu>.in_schedule(OnExit(AppState::MainMenu)));
        app.add_systems(
            (button_system, main_menu_button_action).in_set(OnUpdate(AppState::MainMenu)),
        );
    }
}
