use std::time::Duration;

use bevy::{
    prelude::{in_state, IntoSystemConfig, Plugin, Res, ResMut, Resource},
    time::{Time, Timer, TimerMode},
};

use crate::AppState;

use super::systems::paused::is_game_resumed_or_new;

#[derive(Resource)]
pub struct SoftDropTimer {
    pub timer: Timer,
}

impl Default for SoftDropTimer {
    fn default() -> Self {
        SoftDropTimer {
            timer: Timer::new(Duration::from_secs_f32(0.8), TimerMode::Repeating),
        }
    }
}

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SoftDropTimer::default());
        app.add_system(
            tick.run_if(in_state(AppState::Game))
                .run_if(is_game_resumed_or_new),
        );
    }
}

pub fn tick(time: Res<Time>, mut soft_drop_timer: ResMut<SoftDropTimer>) {
    soft_drop_timer.timer.tick(time.delta());
}
