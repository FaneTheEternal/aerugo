use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::utils::{BTN_NORMAL, FLOW_DEFAULT, GLASS_RED, NARRATOR_DEFAULT, NARRATOR_FRAME, SIZE_ALL, TRANSPARENT, Z_BACKGROUND, Z_SCENE};

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
    let mut narrator_entity = ui_text;
    let mut text_ui = None;

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
            text_ui = Some(spawn_text_ui(
                parent,
                text_font.clone(),
                button_font.clone(),
                asset_server,
            ));
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
            ..Default::default()
        })
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_SCENE),
            visibility: Visibility { is_visible: false },
            ..default()
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

    let mut narrator_sprites = HashMap::new();
    narrator_sprites.insert("".to_string(), narrator_entity.clone());

    GameUI {
        ui_root: root,
        background,
        background_visible: false,
        scene,
        scene_visible: false,
        sprites: Default::default(),
        text: text_ui.unwrap(),
        phrase: PhraseUI { root: ui_phrase, is_visible: false },
        menu,
    }
}

fn spawn_text_ui(
    builder: &mut ChildBuilder,
    text_font: Handle<Font>,
    button_font: Handle<Font>,
    asset_server: &AssetServer,
) -> TextUI
{
    let mut root = builder
        .spawn_bundle(NodeBundle {
            style: Style {
                size: SIZE_ALL,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: TRANSPARENT.into(),
            ..default()
        });

    let mut flow = root.id();
    let mut narrator = root.id();
    let mut text = root.id();
    let mut narrator_base = root.id();
    let mut text_base = root.id();
    root.with_children(|parent| {
        flow = parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: SIZE_ALL,
                    position_type: PositionType::Absolute,
                    flex_wrap: FlexWrap::Wrap,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                color: TRANSPARENT.into(),
                ..default()
            })
            .with_children(|parent| {
                narrator_base = parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(
                                Val::Percent(25.0),
                                Val::Percent(5.0),
                            ),
                            align_items: AlignItems::Center,
                            align_content: AlignContent::Center,
                            flex_wrap: FlexWrap::Wrap,
                            flex_direction: FlexDirection::Row,
                            padding: UiRect::all(Val::Px(20.0)),
                            margin: NARRATOR_DEFAULT,
                            ..default()
                        },
                        image: asset_server
                            .load("hud/game_narrator_name.png").into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        narrator = parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "Narrator",
                                TextStyle {
                                    font: text_font.clone(),
                                    font_size: 20.0,
                                    color: Color::GREEN,
                                },
                            ),
                            ..default()
                        }).id();
                    })
                    .id();
            })
            .with_children(|parent| {
                text_base = parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(
                                Val::Percent(75.0),
                                Val::Percent(20.0),
                            ),
                            flex_wrap: FlexWrap::Wrap,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexEnd,
                            justify_content: JustifyContent::FlexStart,
                            padding: FLOW_DEFAULT,
                            ..default()
                        },
                        image: asset_server
                            .load("hud/game_text_flow.png").into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        text = parent
                            .spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    "Some text",
                                    TextStyle {
                                        font: text_font.clone(),
                                        font_size: 20.0,
                                        color: Color::GREEN,
                                    },
                                ),
                                ..Default::default()
                            })
                            .id();
                    })
                    .id();
            })
            .id();
    });

    let mut narrator_sprites: HashMap<String, NarratorUI> = default();
    root.with_children(|parent| {
        let mut narrator = NarratorUI {
            root: flow.clone(),
            img: flow.clone(),
        };
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: SIZE_ALL,
                    position_type: PositionType::Absolute,
                    flex_wrap: FlexWrap::Wrap,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                color: TRANSPARENT.into(),
                ..default()
            })
            .with_children(|parent| {
                let entity = parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: NARRATOR_FRAME,
                            ..default()
                        },
                        image: asset_server
                            .load("hud/game_narrator_first.png").into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        narrator.img = parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: SIZE_ALL,
                                    ..default()
                                },
                                ..default()
                            })
                            .id();
                    })
                    .id();
                narrator.root = entity;
            });
        narrator_sprites.insert("".into(), narrator.clone());
        narrator_sprites.insert("first".into(), narrator);
    });
    root.with_children(|parent| {
        let mut narrator = NarratorUI {
            root: flow.clone(),
            img: flow.clone(),
        };
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: SIZE_ALL,
                    position_type: PositionType::Absolute,
                    flex_wrap: FlexWrap::Wrap,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                color: TRANSPARENT.into(),
                ..default()
            })
            .with_children(|parent| {
                narrator.root = parent.
                    spawn_bundle(NodeBundle {
                        style: Style {
                            size: NARRATOR_FRAME,
                            ..default()
                        },
                        image: asset_server
                            .load("hud/game_narrator_second.png").into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        narrator.img = parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: SIZE_ALL,
                                    ..default()
                                },
                                ..default()
                            })
                            .id();
                    })
                    .id();
            });
        narrator_sprites.insert("second".into(), narrator);
    });

    TextUI {
        root: root.id(),
        is_visible: false,
        flow,
        narrator,
        text,
        narrator_sprites,
        narrator_base,
        text_base,
    }
}
