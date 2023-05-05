//! Movement System
use bevy::prelude::*;

use crate::game::{components::GameDisplay, timer::SoftDropTimer};
pub(crate) fn movement_system(
    _query: Query<(Entity, &mut Transform), With<GameDisplay>>,
    _key_code: Res<Input<KeyCode>>,
    _soft_drop_timer: Res<SoftDropTimer>,
) {
}
