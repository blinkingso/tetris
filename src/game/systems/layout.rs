//! paused layout and other layout in game model
use bevy::prelude::*;

use crate::game::components::{PausedButtonAction, PausedLayout};
use crate::game::style::PAUSED_LAYOUT_BACKGROUND_COLOR;
use crate::ui::style::{get_text_style, get_title_text_style, BUTTON_STYLE, NORMAL_BUTTON};

pub(crate) fn spawn_pause_layout_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                background_color: BackgroundColor::from(PAUSED_LAYOUT_BACKGROUND_COLOR),
                ..Default::default()
            },
            PausedLayout,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "You Paused The Game",
                        get_title_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    PausedButtonAction::Continue,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Continue",
                                get_text_style(&asset_server),
                            )],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    PausedButtonAction::Renew,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "New Game",
                                get_text_style(&asset_server),
                            )],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    PausedButtonAction::Exit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("Exit", get_text_style(&asset_server))],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}
