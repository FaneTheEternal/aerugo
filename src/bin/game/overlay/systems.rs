#![allow(dead_code)]

use bevy::prelude::*;
use crate::overlay::components::{OverlayButton, OverlayButtons};
use crate::overlay::OverlayData;
use crate::states::{MainState, OverlayState};
use crate::utils::{grow_z_index, make_button_closure};

const BTN_NORMAL: Color = Color::WHITE;
const BTN_HOVERED: Color = Color::GRAY;
const BTN_PRESSED: Color = Color::DARK_GRAY;

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const GLASS_WHITE: Color = Color::rgba(1.0, 1.0, 1.0, 0.2);
const GLASS_GRAY: Color = Color::rgba(0.5, 0.5, 0.5, 0.2);
const GLASS_DARK: Color = Color::rgba(0.0, 0.0, 0.0, 0.2);

const OVERLAY_Z_DEEP: u8 = 10;

fn overlay_ui_base() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        color: TRANSPARENT.into(),
        ..Default::default()
    }
}

pub fn setup_overlay(
    mut command: Commands,
    asset_server: Res<AssetServer>,
)
{
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let ui_entity = command
        .spawn_bundle(overlay_ui_base())
        .with_children(|parent| {
            grow_z_index(
                OVERLAY_Z_DEEP, parent,
                Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                |parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgba(0.0, 0.0, 0.0, 0.4).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        flex_direction: FlexDirection::ColumnReverse,
                                        flex_wrap: FlexWrap::Wrap,
                                        ..Default::default()
                                    },
                                    color: Color::rgba(1.0, 0.5, 0.5, 0.8).into(),
                                    ..Default::default()
                                })
                                .with_children(
                                    make_button_closure(
                                        "Close",
                                        button_font.clone(),
                                        OverlayButton { target: OverlayButtons::Close },
                                        BTN_NORMAL,
                                    )
                                )
                                .with_children(
                                    make_button_closure(
                                        "Settings",
                                        button_font.clone(),
                                        OverlayButton { target: OverlayButtons::Settings },
                                        BTN_NORMAL,
                                    )
                                )
                                .with_children(
                                    make_button_closure(
                                        "Save",
                                        button_font.clone(),
                                        OverlayButton { target: OverlayButtons::Save },
                                        BTN_NORMAL,
                                    )
                                )
                                .with_children(
                                    make_button_closure(
                                        "Load",
                                        button_font.clone(),
                                        OverlayButton { target: OverlayButtons::Load },
                                        BTN_NORMAL,
                                    )
                                )
                                .with_children(
                                    make_button_closure(
                                        "Main Menu",
                                        button_font.clone(),
                                        OverlayButton { target: OverlayButtons::MainMenu },
                                        BTN_NORMAL,
                                    )
                                );
                        });
                });
        })
        .id();

    command.insert_resource(OverlayData { ui_entity });
}

pub fn overlay(
    mut main_state: ResMut<State<MainState>>,
    mut overlay_state: ResMut<State<OverlayState>>,
    mut interactions_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    buttons_query: Query<&OverlayButton>,
)
{
    for (interaction, mut color, children) in interactions_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = BTN_PRESSED.into();

                if let Ok(btn) = buttons_query.get(children[0]) {
                    match btn.target {
                        OverlayButtons::Close => {
                            overlay_state.set(OverlayState::Hidden).unwrap();
                        }
                        OverlayButtons::Settings => {
                            overlay_state.set(OverlayState::Settings).unwrap();
                        }
                        OverlayButtons::Save => {
                            overlay_state.set(OverlayState::Save).unwrap();
                        }
                        OverlayButtons::Load => {
                            overlay_state.set(OverlayState::Load).unwrap();
                        }
                        OverlayButtons::MainMenu => {
                            overlay_state.set(OverlayState::Hidden).unwrap();
                            main_state.set(MainState::MainMenu).unwrap();
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = BTN_HOVERED.into();
            }
            Interaction::None => {
                *color = BTN_NORMAL.into();
            }
        }
    }
}

pub fn overlay_break(
    main_state: Res<State<MainState>>,
    mut overlay_state: ResMut<State<OverlayState>>,
    mut input: ResMut<Input<KeyCode>>,
)
{
    if input.clear_just_released(KeyCode::Escape) {
        match overlay_state.current() {
            OverlayState::Hidden => {}
            OverlayState::Menu => {
                overlay_state.set(OverlayState::Hidden).unwrap();
            }
            OverlayState::Settings | OverlayState::Load => {
                match main_state.current() {
                    MainState::MainMenu => {
                        overlay_state.set(OverlayState::Hidden).unwrap();
                    }
                    MainState::InGame => {
                        overlay_state.set(OverlayState::Menu).unwrap();
                    }
                }
            }
            OverlayState::Save => {
                overlay_state.set(OverlayState::Menu).unwrap();
            }
        }
    }
}

pub fn cleanup(mut command: Commands, overlay_data: Option<Res<OverlayData>>) {
    if let Some(overlay_data) = overlay_data {
        command.entity(overlay_data.ui_entity).despawn_recursive();
        command.remove_resource::<OverlayData>();
    }
}

pub fn setup_settings(
    mut command: Commands,
    asset_server: Res<AssetServer>,
)
{
    let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");
    // let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let ui_entity = command
        .spawn_bundle(overlay_ui_base())
        .with_children(|parent| {
            grow_z_index(
                OVERLAY_Z_DEEP, parent,
                Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                |parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_wrap: FlexWrap::Wrap,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..Default::default()
                            },
                            color: TRANSPARENT.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {  // Header
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::FlexStart,
                                        ..Default::default()
                                    },
                                    color: Color::DARK_GREEN.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Settings",
                                                TextStyle {
                                                    font: text_font.clone(),
                                                    font_size: 40.0,
                                                    color: Color::BLACK,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });
                                });
                        })
                        .with_children(|parent| {  // Body
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::GRAY.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "TODO: Settings",
                                                TextStyle {
                                                    font: text_font,
                                                    font_size: 60.0,
                                                    color: Color::ANTIQUE_WHITE,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });
                                });
                        });
                },
            )
        })
        .id();

    command.insert_resource(OverlayData { ui_entity });
}

pub fn setup_save(
    mut command: Commands,
    asset_server: Res<AssetServer>,
)
{
    let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");
    // let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let ui_entity = command
        .spawn_bundle(overlay_ui_base())
        .with_children(|parent| {
            grow_z_index(
                OVERLAY_Z_DEEP, parent,
                Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                |parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_wrap: FlexWrap::Wrap,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..Default::default()
                            },
                            color: Color::GRAY.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {  // Header
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::FlexStart,
                                        ..Default::default()
                                    },
                                    color: Color::DARK_GREEN.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Save",
                                                TextStyle {
                                                    font: text_font.clone(),
                                                    font_size: 40.0,
                                                    color: Color::BLACK,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });
                                });
                        })
                        .with_children(|parent| {  // Body
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::GRAY.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "TODO: Save grid",
                                                TextStyle {
                                                    font: text_font,
                                                    font_size: 60.0,
                                                    color: Color::ANTIQUE_WHITE,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });
                                });
                        });
                },
            )
        })
        .id();

    command.insert_resource(OverlayData { ui_entity });
}

pub fn setup_load(
    mut command: Commands,
    asset_server: Res<AssetServer>,
)
{
    let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");
    // let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let ui_entity = command
        .spawn_bundle(overlay_ui_base())
        .with_children(|parent| {
            grow_z_index(
                OVERLAY_Z_DEEP, parent,
                Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                |parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_wrap: FlexWrap::Wrap,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..Default::default()
                            },
                            color: Color::GRAY.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {  // Header
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::FlexStart,
                                        ..Default::default()
                                    },
                                    color: Color::DARK_GREEN.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Load",
                                                TextStyle {
                                                    font: text_font.clone(),
                                                    font_size: 40.0,
                                                    color: Color::BLACK,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });
                                });
                        })
                        .with_children(|parent| {  // Body
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::GRAY.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "TODO: Load grid",
                                                TextStyle {
                                                    font: text_font,
                                                    font_size: 60.0,
                                                    color: Color::ANTIQUE_WHITE,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });
                                });
                        });
                },
            )
        })
        .id();

    command.insert_resource(OverlayData { ui_entity });
}
