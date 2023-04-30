use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct MainMenu;

#[derive(Clone, Copy, PartialEq, Eq, Component)]
pub(crate) enum MainMenuButtonAction {
    Play,
    Exit,
}
