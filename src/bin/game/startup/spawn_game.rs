use bevy::prelude::*;
use crate::utils::{BTN_NORMAL, GLASS_RED, SIZE_ALL, TRANSPARENT, Z_BACKGROUND, Z_SCENE};

use super::*;


pub(crate) fn spawn_game(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Windows,
) -> GameUI
{
    let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let window = window.get_primary().unwrap();
    let w = window.width();
    let h = window.height();

    let mut ui_text = Entity::from_raw(0);
    let mut text_flow_entity = ui_text;
    let mut text_narrator_entity = ui_text;
    let mut narrator_entity = ui_text;
    let mut text_items = ui_text;

    let mut ui_phrase = Entity::from_raw(0);

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::None,
                size: SIZE_ALL,
                position_type: PositionType::Absolute,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            color: TRANSPARENT.into(),
            ..Default::default()
        })
        // TextUI
        .with_children(|parent| {
            let mut entity = parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: SIZE_ALL,
                        padding: Rect::all(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    color: TRANSPARENT.into(),
                    ..Default::default()
                });
            ui_text = entity
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                                flex_wrap: FlexWrap::Wrap,
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            color: TRANSPARENT.into(),
                            ..Default::default()
                        })
                        // narrator
                        .with_children(|parent| {
                            narrator_entity = parent
                                .spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                                        display: Display::None,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .id();
                        })
                        // text items
                        .with_children(|parent| {
                            text_items = parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        flex_direction: FlexDirection::Column,
                                        flex_grow: 1.0,
                                        flex_shrink: 0.0,
                                        ..Default::default()
                                    },
                                    color: TRANSPARENT.into(),
                                    ..Default::default()
                                })
                                // flow
                                .with_children(|parent| {
                                    parent
                                        // wrapper
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Percent(67.0)),
                                                padding: Rect::all(Val::Px(10.0)),
                                                ..Default::default()
                                            },
                                            color: TRANSPARENT.into(),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn_bundle(NodeBundle {
                                                    style: Style {
                                                        size: SIZE_ALL,
                                                        align_items: AlignItems::FlexStart,
                                                        align_content: AlignContent::FlexStart,
                                                        flex_direction: FlexDirection::ColumnReverse,
                                                        flex_wrap: FlexWrap::Wrap,
                                                        ..Default::default()
                                                    },
                                                    color: GLASS_RED.into(),
                                                    ..Default::default()
                                                })
                                                .with_children(|parent| {
                                                    text_flow_entity = parent
                                                        .spawn_bundle(TextBundle {
                                                            text: Text::with_section(
                                                                "Some text",
                                                                TextStyle {
                                                                    font: text_font.clone(),
                                                                    font_size: 20.0,
                                                                    color: Color::GREEN,
                                                                },
                                                                TextAlignment::default(),
                                                            ),
                                                            ..Default::default()
                                                        })
                                                        .id();
                                                });
                                        });
                                })
                                // narrator
                                .with_children(|parent| {
                                    parent
                                        // wrapper
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Percent(33.0)),
                                                padding: Rect::all(Val::Px(10.0)),
                                                ..Default::default()
                                            },
                                            color: TRANSPARENT.into(),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn_bundle(NodeBundle {
                                                    style: Style {
                                                        size: SIZE_ALL,
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::FlexStart,
                                                        flex_direction: FlexDirection::ColumnReverse,
                                                        flex_wrap: FlexWrap::Wrap,
                                                        ..Default::default()
                                                    },
                                                    color: GLASS_RED.into(),
                                                    ..Default::default()
                                                })
                                                .with_children(|parent| {
                                                    text_narrator_entity = parent
                                                        .spawn_bundle(TextBundle {
                                                            text: Text::with_section(
                                                                "Narrator",
                                                                TextStyle {
                                                                    font: text_font.clone(),
                                                                    font_size: 20.0,
                                                                    color: Color::GREEN,
                                                                },
                                                                TextAlignment {
                                                                    vertical: VerticalAlign::Top,
                                                                    horizontal: HorizontalAlign::Left,
                                                                },
                                                            ),
                                                            ..Default::default()
                                                        })
                                                        .id();
                                                });
                                        });
                                })
                                .id();
                        });
                })
                .id();
        })
        // PhraseUI
        .with_children(|parent| {
            ui_phrase = parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: SIZE_ALL,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        flex_wrap: FlexWrap::Wrap,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::rgba(0.5, 0.5, 0.5, 0.5).into(),
                    ..Default::default()
                })
                .id();
        })
        .id();

    let background = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .id();

    let scene = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_SCENE),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .id();

    let menu = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::None,
                size: SIZE_ALL,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: TRANSPARENT.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            grow_z_index(
                10, parent,
                Style {
                    size: SIZE_ALL,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                |parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: SIZE_ALL,
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
                },
            )
        })
        .id();

    GameUI {
        ui_root: root,
        background,
        background_visible: false,
        scene,
        scene_visible: false,
        sprites: Default::default(),
        text: TextUI {
            root: ui_text,
            is_visible: false,
            narrator: text_narrator_entity,
            text: text_flow_entity,
            narrator_sprite: narrator_entity,
        },
        phrase: PhraseUI { root: ui_phrase, is_visible: false },
        menu,
    }
}
