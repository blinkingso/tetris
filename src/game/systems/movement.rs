//! Movement System
use std::time::Duration;

use bevy::prelude::*;

use crate::game::{components::GameDisplay, timer::SoftDropTimer};
pub(crate) fn movement_system(
    mut query: Query<(Entity, &mut Transform), With<GameDisplay>>,
    _key_code: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
) {
    soft_drop_timer
        .0
        .tick(Duration::from_secs_f32(time.delta_seconds()));
    if soft_drop_timer.0.just_finished() {
        for (_entity, mut transform) in query.iter_mut() {
            let old_translation = transform.translation;
            transform.translation = old_translation + Vec3::new(0.0, 2.0, 0.0);
        }
        soft_drop_timer.0.reset();
    } else {
        for (_entity, mut transform) in query.iter_mut() {
            let old_translation = transform.translation;
            transform.translation = old_translation + Vec3::new(0.0, -2.0, 0.0);
        }
    }
}
