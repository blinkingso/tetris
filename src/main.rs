mod game;
mod ui;

use crate::game::{resources::ImageLoadPlugin, GamePlugin, GameState};
use crate::ui::*;
use bevy::prelude::*;
use bevy::window::exit_on_primary_closed;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::GRAY))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_state::<GameState>()
        .add_plugin(game::timer::TimerPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ImageLoadPlugin)
        .add_startup_system(setup)
        .add_system(exit_on_primary_closed)
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
