use bevy::prelude::*;

use aerugo::bevy_glue::MainMenuButtons;

use crate::utils::*;

use super::*;

pub struct MainMenuUI {
    pub(crate) entity_root: Entity,
}

impl MainMenuUI {
    pub fn show(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::Flex;
    }

    pub fn hide(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::None;
    }
}

pub fn main_menu_show(main_menu: ResMut<MainMenuUI>, query: Query<&mut Style>) {
    main_menu.show(query);
}

pub fn main_menu_hide(main_menu: ResMut<MainMenuUI>, query: Query<&mut Style>) {
    main_menu.hide(query);
}

pub fn main_menu_actions(
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<
        (&Interaction, &mut UiColor, &MainMenuButtons),
        (Changed<Interaction>, With<Button>)
    >,
    mut exit: EventWriter<AppExit>,
)
{
    for (interaction, mut color, btn) in query.iter_mut() {
        let btn: &MainMenuButtons = btn;
        match *interaction {
            Interaction::Clicked => {
                *color = TRANSPARENT.into();

                match btn {
                    MainMenuButtons::NewGame => {
                        ui_state.set(UiState::Game).unwrap_or_else(|e| warn!("{e:?}"));
                        game_state.set(GameState::Init).unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    MainMenuButtons::Load => {
                        ui_state.set(UiState::Load).unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    MainMenuButtons::Gallery => {}
                    MainMenuButtons::Settings => {}
                    MainMenuButtons::About => {}
                    MainMenuButtons::Exit => {
                        exit.send(AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
            }
            Interaction::None => {
                *color = TRANSPARENT.into();
            }
        }
    }
}

pub struct NoticeUI {
    root: Entity,
}

impl NoticeUI {
    pub fn spawn(
        commands: &mut Commands,
        asset_server: &mut CachedAssetServer,
    ) -> NoticeUI
    {
        let root = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: SIZE_ALL,
                    display: Display::None,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                color: TRANSPARENT.into(),
                ..default()
            })
            .with_children(|parent| {
                grow_z_index(
                    10,
                    parent,
                    Style {
                        size: SIZE_ALL,
                        flex_wrap: FlexWrap::Wrap,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    |parent| {
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(400.0),
                                        Val::Px(200.0),
                                    ),
                                    flex_wrap: FlexWrap::Wrap,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    align_content: AlignContent::Center,
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..default()
                                },
                                color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            size: Size::new(
                                                Val::Percent(90.0),
                                                Val::Percent(50.0),
                                            ),
                                            flex_wrap: FlexWrap::Wrap,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        color: TRANSPARENT.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn_bundle(TextBundle {
                                            text: Text::from_section(
                                                "I am 18 years of age or older",
                                                TextStyle {
                                                    font: asset_server.load("fonts/CormorantGaramond-BoldItalic.ttf"),
                                                    font_size: 40.0,
                                                    color: Color::PURPLE,
                                                },
                                            ).with_alignment(TextAlignment::CENTER),
                                            ..default()
                                        });
                                    });
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            size: Size::new(
                                                Val::Percent(60.0),
                                                Val::Percent(25.0),
                                            ),
                                            flex_wrap: FlexWrap::Wrap,
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            align_content: AlignContent::Center,
                                            ..default()
                                        },
                                        color: TRANSPARENT.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn_bundle(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(40.0),
                                                        Val::Percent(100.0),
                                                    ),
                                                    margin: UiRect::all(Val::Px(10.0)),
                                                    ..default()
                                                },
                                                image: asset_server.load("hud/save_return.png").into(),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn_bundle(ButtonBundle {
                                                        style: Style {
                                                            size: SIZE_ALL,
                                                            flex_wrap: FlexWrap::Wrap,
                                                            align_items: AlignItems::Center,
                                                            justify_content: JustifyContent::Center,
                                                            ..default()
                                                        },
                                                        image: asset_server.load("hud/save_return_hover.png").into(),
                                                        ..default()
                                                    })
                                                    .insert(NoticeAccept::Yes)
                                                    .with_children(|parent| {
                                                        parent.spawn_bundle(TextBundle {
                                                            text: Text::from_section(
                                                                "Yes",
                                                                TextStyle {
                                                                    font: asset_server.load("fonts/CormorantGaramond-Italic.ttf"),
                                                                    font_size: 30.0,
                                                                    color: Color::PURPLE,
                                                                },
                                                            ),
                                                            ..default()
                                                        });
                                                    });
                                            });
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn_bundle(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(40.0),
                                                        Val::Percent(100.0),
                                                    ),
                                                    margin: UiRect::all(Val::Px(10.0)),
                                                    ..default()
                                                },
                                                image: asset_server.load("hud/save_return.png").into(),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn_bundle(ButtonBundle {
                                                        style: Style {
                                                            size: SIZE_ALL,
                                                            flex_wrap: FlexWrap::Wrap,
                                                            align_items: AlignItems::Center,
                                                            justify_content: JustifyContent::Center,
                                                            ..default()
                                                        },
                                                        image: asset_server.load("hud/save_return_hover.png").into(),
                                                        ..default()
                                                    })
                                                    .insert(NoticeAccept::No)
                                                    .with_children(|parent| {
                                                        parent.spawn_bundle(TextBundle {
                                                            text: Text::from_section(
                                                                "No",
                                                                TextStyle {
                                                                    font: asset_server.load("fonts/CormorantGaramond-Italic.ttf"),
                                                                    font_size: 30.0,
                                                                    color: Color::PURPLE,
                                                                },
                                                            ),
                                                            ..default()
                                                        });
                                                    });
                                            });
                                    });
                            });
                    },
                )
            })
            .id();
        NoticeUI {
            root
        }
    }

    pub fn show(ui: Res<NoticeUI>, mut style_query: Query<&mut Style>) {
        style_query.get_mut(ui.root).unwrap().display = Display::Flex;
    }

    pub fn actions(
        mut ui_state: ResMut<State<UiState>>,
        mut asset_server: CachedAssetServer,
        mut query: Query<
            (&Interaction, &mut UiColor, &mut UiImage, &NoticeAccept),
            (Changed<Interaction>, With<Button>)
        >,
        mut exit: EventWriter<AppExit>,
    )
    {
        for (interaction, mut color, mut img, btn) in query.iter_mut() {
            let btn: &NoticeAccept = btn;
            match *interaction {
                Interaction::Clicked => {
                    *color = TRANSPARENT.into();
                    *img = default();
                    match btn {
                        NoticeAccept::Yes => {
                            ui_state.set(UiState::MainMenu)
                                .unwrap_or_else(|e| warn!("{e:?}"));
                        }
                        NoticeAccept::No => {
                            exit.send(AppExit);
                        }
                    }
                }
                Interaction::Hovered => {
                    *color = Color::WHITE.into();
                    *img = asset_server.load("hud/save_return_hover.png").into();
                }
                Interaction::None => {
                    *color = TRANSPARENT.into();
                    *img = default();
                }
            }
        }
    }

    pub fn exit(ui: Res<NoticeUI>, mut commands: Commands) {
        commands.entity(ui.root).despawn_recursive();
    }
}

#[derive(Debug, Clone, Component)]
pub enum NoticeAccept {
    Yes,
    No,
}
