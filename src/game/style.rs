use bevy::prelude::*;

pub(crate) const PAUSED_LAYOUT_BACKGROUND_COLOR: Color = Color::rgba(0.6, 0.6, 0.6, 0.8);

pub const TEXT_FONT_SIZE: f32 = 25.0;
pub const TEXT_LABEL_FONT_SIZE: f32 = 32.0;

pub fn get_game_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: TEXT_FONT_SIZE,
        color: Color::WHITE,
    }
}

pub fn get_game_label_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: TEXT_LABEL_FONT_SIZE,
        color: Color::WHITE,
    }
}
