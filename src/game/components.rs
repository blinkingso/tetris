use bevy::prelude::Component;

#[derive(Component)]
pub struct PausedLayout;

#[derive(Component)]
pub enum PausedButtonAction {
    Continue,
    Renew,
    Exit,
}

#[derive(Component)]
pub struct GameDisplay;
