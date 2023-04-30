use bevy::prelude::*;
use bevy::ui::{AlignItems, JustifyContent, Style};

pub(crate) const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub(crate) const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub(crate) const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub(crate) const MAIN_MENU_STYLE: Style = Style {
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    flex_direction: FlexDirection::Column,
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(180.0), Val::Px(80.0)),
    margin: UiRect {
        top: Val::Px(20.0),
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};

pub(crate) fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font_size: 32.0,
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        color: Color::WHITE,
    }
}

pub(crate) fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font_size: 64.0,
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        color: Color::WHITE,
    }
}
