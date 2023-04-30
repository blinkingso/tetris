mod game;
mod ui;

use crate::game::{resources::ImageLoadPlugin, GamePlugin, GameState};
use crate::ui::*;
use bevy::prelude::*;
use game::timer::DropTimerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_state::<AppState>()
        .add_state::<GameState>()
        .add_plugin(DropTimerPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ImageLoadPlugin)
        // .insert_resource(ClearColor(Color::WHITE))
        .add_startup_system(setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn despawn_components<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Game State
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
