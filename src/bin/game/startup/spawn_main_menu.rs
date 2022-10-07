use bevy::prelude::*;

use aerugo::bevy_glue::MainMenuButtons;
use crate::translator::{TranslatableText};
use crate::ui::PatreonBTN;

use crate::utils::*;

pub fn spawn(
    commands: &mut Commands,
    asset_server: &mut CachedAssetServer,
) -> Entity
{
    let button_font = asset_server
        .load(FONT_MAIN_MENU);
    let background = asset_server
        .load(MAIN_BACK);
    let btn_background = asset_server
        .load(MAIN_BTN_BACK);
    let btn_hover = asset_server
        .load(MAIN_BTN_HOVER);

    let entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: SIZE_ALL,
                display: Display::None,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                align_content: AlignContent::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            image: background.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(33.3), Val::Percent(100.0)),
                        flex_wrap: FlexWrap::Wrap,
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexEnd,
                        padding: UiRect::new(
                            Val::Percent(8.0),
                            Val::Undefined,
                            Val::Undefined,
                            Val::Percent(4.5),
                        ),
                        ..default()
                    },
                    image: btn_background.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(84.0),
                                    Val::Percent(15.0),
                                ),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::new(
                                    Val::Percent(25.0),
                                    Val::Undefined,
                                    Val::Undefined,
                                    Val::Percent(1.0),
                                ),
                                ..default()
                            },
                            color: TRANSPARENT.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(60.0),
                                            Val::Percent(100.0),
                                        ),
                                        ..default()
                                    },
                                    color: Color::WHITE.into(),
                                    image: asset_server.load("Patreon.png").into(),
                                    ..default()
                                })
                                .insert(PatreonBTN);
                        });
                })
                .with_children(
                    make_btn(
                        "New game",
                        button_font.clone(),
                        MainMenuButtons::NewGame,
                        asset_server.load(BTN1),
                        btn_hover.clone(),
                    )
                )
                .with_children(
                    make_btn(
                        "Load",
                        button_font.clone(),
                        MainMenuButtons::Load,
                        asset_server.load(BTN2),
                        btn_hover.clone(),
                    )
                )
                .with_children(
                    make_btn(
                        "Gallery",
                        button_font.clone(),
                        MainMenuButtons::Gallery,
                        asset_server.load(BTN3),
                        btn_hover.clone(),
                    )
                )
                .with_children(
                    make_btn(
                        "Settings",
                        button_font.clone(),
                        MainMenuButtons::Settings,
                        asset_server.load(BTN4),
                        btn_hover.clone(),
                    )
                )
                .with_children(
                    make_btn(
                        "About",
                        button_font.clone(),
                        MainMenuButtons::About,
                        asset_server.load(BTN5),
                        btn_hover.clone(),
                    )
                )
                .with_children(
                    make_btn(
                        "Exit",
                        button_font.clone(),
                        MainMenuButtons::Exit,
                        asset_server.load(BTN6),
                        btn_hover.clone(),
                    )
                );
        })
        .id();

    entity
}

fn make_btn(
    text: &str,
    font: Handle<Font>,
    btn: MainMenuButtons,
    img: Handle<Image>,
    hover: Handle<Image>,
) -> impl FnOnce(&mut ChildBuilder) + '_
{
    move |parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(
                        Val::Percent(84.0),
                        Val::Percent(13.0),
                    ),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: img.clone().into(),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: SIZE_ALL,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        image: hover.into(),
                        color: TRANSPARENT.into(),
                        ..default()
                    })
                    .insert(btn)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    text,
                                    TextStyle {
                                        font,
                                        font_size: 40.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                ..Default::default()
                            })
                            .insert(TranslatableText);
                    });
            });
    }
}