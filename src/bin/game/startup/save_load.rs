use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::utils::HashMap;
use aerugo::bevy_glue::SavePageButton;
use crate::saves::{LoadMark, Save, SaveMark};

use crate::utils::*;

use super::*;
use super::spawn_game::*;

pub fn spawn_save(
    commands: &mut Commands,
    asset_server: &PreloadedAssets,
    saves: &Saves,
) -> SaveLoadUI
{
    let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let background = asset_server
        .load("hud/save_back.png");
    let page_btn = asset_server
        .load("hud/save_page.png");
    let page_btn_hover = asset_server
        .load("hud/save_page_hover.png");
    let save_frame = asset_server
        .load("hud/save_frame.png");

    let mut page_header = Entity::from_raw(0);
    let mut save_frames = vec![];

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: SIZE_ALL,
                display: Display::None,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            image: background.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Save",
                    TextStyle {
                        font: button_font.clone(),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                ),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            page_header = parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Page 0",
                        TextStyle {
                            font: text_font.clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .id();
        })
        .with_children(|parent| {
            spawn_pages_row(parent, button_font.clone(), page_btn, page_btn_hover)
        })
        .with_children(|parent| {
            let mut saves = (0..20usize)
                .map(|i| saves.saves.get(&i))
                .collect::<Vec<_>>();
            let mut entity = parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0),
                            Val::Percent(80.0),
                        ),
                        flex_wrap: FlexWrap::Wrap,
                        flex_direction: FlexDirection::RowReverse,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: TRANSPARENT.into(),
                    ..default()
                });
            save_frames = spawn_frames(
                &mut entity,
                button_font.clone(),
                &saves,
                save_frame,
                asset_server,
            );
        })
        .id();

    SaveLoadUI {
        root,
        page_header,
        current: "0".into(),
        save_frames,
    }
}

fn spawn_pages_row(
    parent: &mut ChildBuilder,
    button_font: Handle<Font>,
    btn: Handle<Image>,
    btn_hover: Handle<Image>,
)
{
    let mut pages = parent.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100.0),
                Val::Percent(10.0),
            ),
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Percent(1.0)),
            ..default()
        },
        color: TRANSPARENT.into(),
        ..default()
    });
    for page in 0..10usize {
        pages.with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(3.5),
                            Val::Percent(90.0),
                        ),
                        flex_wrap: FlexWrap::Wrap,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect {
                            left: Val::Px(10.0),
                            right: Val::Px(10.0),
                            top: default(),
                            bottom: default(),
                        },
                        ..default()
                    },
                    image: btn.clone().into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: SIZE_ALL,
                                flex_wrap: FlexWrap::Wrap,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            color: TRANSPARENT.into(),
                            image: btn_hover.clone().into(),
                            ..default()
                        })
                        .insert(SavePageButton(format!("{}", page)))
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    format!("{}", page),
                                    TextStyle {
                                        font: button_font.clone(),
                                        font_size: 30.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                ..default()
                            });
                        });
                });
        });
    }
}


fn spawn_frames(
    parent: &mut EntityCommands,
    button_font: Handle<Font>,
    saves: &[Option<&Save>],
    frame_img: Handle<Image>,
    asset_server: &PreloadedAssets,
) -> Vec<SaveFrameUI>
{
    let mut save_frames = vec![];
    let mut saves = saves.iter().enumerate().collect::<Vec<_>>();
    saves.reverse();
    for (i, save) in saves {
        let mut ui = SaveFrameUI {
            root: Entity::from_raw(0),
            btn: Entity::from_raw(0),
            has_save: false,
            num: Entity::from_raw(0),
            hint: Entity::from_raw(0),
        };
        parent.with_children(|parent| {
            ui.root = parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(16.0),
                            Val::Percent(16.0),
                        ),
                        margin: UiRect::all(Val::Px(10.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    image: frame_img.clone().into(),
                    ..default()
                })
                .with_children(|parent| {
                    ui.has_save = save.is_some();
                    let back = save
                        .and_then(|save| save.state.inspector.background.as_ref());
                    let (back, color) = match back {
                        None => { (UiImage::default(), TRANSPARENT) }
                        Some(name) => { (asset_server.load(name).into(), Color::WHITE) }
                    };

                    ui.btn = parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: SIZE_ALL,
                                flex_wrap: FlexWrap::Wrap,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            image: back,
                            color: color.into(),
                            ..default()
                        })
                        .insert(SaveMark { to: i })
                        .insert(LoadMark(i))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(100.0),
                                            Val::Percent(50.0),
                                        ),
                                        flex_wrap: FlexWrap::Wrap,
                                        align_items: AlignItems::FlexStart,
                                        justify_content: JustifyContent::FlexEnd,
                                        ..default()
                                    },
                                    focus_policy: FocusPolicy::Pass,
                                    color: TRANSPARENT.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    ui.num = parent.spawn_bundle(TextBundle {
                                        text: Text::from_section(
                                            i.to_string(),
                                            TextStyle {
                                                font: button_font.clone(),
                                                font_size: 40.0,
                                                color: Color::BLACK,
                                            },
                                        ),
                                        focus_policy: FocusPolicy::Pass,
                                        ..default()
                                    }).id();
                                });
                        })
                        .with_children(|parent| {
                            let hint = save
                                .and_then(|save| {
                                    Some(save.timestamp.format("%d/%m/%Y %H:%M").to_string())
                                })
                                .unwrap_or_default();
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(100.0),
                                            Val::Percent(50.0),
                                        ),
                                        flex_wrap: FlexWrap::Wrap,
                                        align_items: AlignItems::FlexStart,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    focus_policy: FocusPolicy::Pass,
                                    color: TRANSPARENT.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    ui.hint = parent.spawn_bundle(TextBundle {
                                        text: Text::from_section(
                                            hint,
                                            TextStyle {
                                                font: button_font.clone(),
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                        ),
                                        focus_policy: FocusPolicy::Pass,
                                        ..default()
                                    }).id();
                                });
                        })
                        .id();
                })
                .id();
        });
        save_frames.push(ui);
    }
    save_frames.reverse();
    save_frames
}
