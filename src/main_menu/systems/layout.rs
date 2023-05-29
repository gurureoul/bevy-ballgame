use crate::main_menu::components::*;
use crate::main_menu::styles::*;
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..Default::default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites\\ball_blue_large.png").into(),
                        ..Default::default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Bevy Ball Game",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites\\ball_red_large.png").into(),
                        ..Default::default()
                    });
                });
            // Play
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..Default::default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            // Quit
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..Default::default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .id();
    main_menu_entity
}
