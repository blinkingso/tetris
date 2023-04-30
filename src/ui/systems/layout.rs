use bevy::prelude::*;

use crate::ui::{
    components::{MainMenu, MainMenuButtonAction},
    style::{get_text_style, get_title_text_style, BUTTON_STYLE, MAIN_MENU_STYLE, NORMAL_BUTTON},
};

pub(crate) fn spawn_main_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = build_main_menu(&mut commands, &asset_server);
    commands.entity(entity).insert(MainMenu);
}

fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn(NodeBundle {
            style: MAIN_MENU_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            // spawn text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Tetris Game in Bevy",
                        get_title_text_style(asset_server),
                    )],
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
                    MainMenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("Play", get_text_style(asset_server))],
                            alignment: TextAlignment::Center,
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
                    MainMenuButtonAction::Exit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("Exit", get_text_style(asset_server))],
                            alignment: TextAlignment::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .id()
}
