use bevy::{
    prelude::{Plugin, Resource},
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct SoftDropTimer(pub Timer);

pub struct DropTimerPlugin;

impl Plugin for DropTimerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let soft_drop_timer = SoftDropTimer(Timer::from_seconds(1.0, TimerMode::Once));
        app.insert_resource(soft_drop_timer);
    }
}
