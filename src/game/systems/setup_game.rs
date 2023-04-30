//! Used to setup game page.

use std::time::Duration;

use crate::game::{components::GameDisplay, resources::ImagePathResources, timer::SoftDropTimer};
use bevy::prelude::*;

pub(crate) fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_resource: Res<ImagePathResources>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
) {
    let time_speed = Duration::from_secs_f32(1.0);
    soft_drop_timer.0.set_duration(time_speed);
    soft_drop_timer.0.reset();
    soft_drop_timer.0.set_elapsed(time_speed);

    for (index, handle) in image_resource.hgetall().iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(80., 80.)),
                    ..Default::default()
                },
                texture: asset_server.load(handle.1.to_string()),
                transform: Transform::from_xyz(-260.0 + index as f32 * 80.0 + 2.0, 0.0, 0.0),
                ..Default::default()
            },
            GameDisplay,
        ));
    }
}
