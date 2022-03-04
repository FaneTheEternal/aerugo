use bevy::prelude::*;
use crate::saves::{LoadMark, SaveMark, Saves};
use crate::utils::{SIZE_ALL, TRANSPARENT};

#[derive(Component)]
pub struct SaveItemsParentMark;

#[derive(Component)]
pub struct LoadItemsParentMark;

pub fn save_load_base(
    items: Vec<Entity>,
    text_font: Handle<Font>,
    items_parent_mark: impl Component,
    header: &str,
) -> impl FnOnce(&mut ChildBuilder) + '_
{
    move |parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: SIZE_ALL,
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
                                    header,
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
                            flex_wrap: FlexWrap::WrapReverse,
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        color: Color::GRAY.into(),
                        ..Default::default()
                    })
                    .insert(items_parent_mark)
                    .push_children(items.as_slice());
            });
    }
}

fn _make_card_base() -> impl Bundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(33.3), Val::Percent(50.0)),
            padding: Rect::all(Val::Percent(5.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        color: TRANSPARENT.into(),
        ..Default::default()
    }
}

fn _card_style() -> Style {
    Style {
        size: SIZE_ALL,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_wrap: FlexWrap::Wrap,
        flex_direction: FlexDirection::ColumnReverse,
        ..Default::default()
    }
}

fn _empty_card(text_font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
    move |parent| {
        parent
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Empty save",
                    TextStyle {
                        font: text_font,
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
    }
}

pub fn make_load_items(
    commands: &mut Commands,
    saves: &Saves,
    save_font: Handle<Font>,
    empty_save_font: Handle<Font>,
) -> Vec<Entity>
{
    (0..6)
        .map(|n: u8| {
            commands
                .spawn_bundle(_make_card_base())
                .with_children(|parent| {
                    match saves.saves.get(&n) {
                        None => {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: _card_style(),
                                    color: Color::WHITE.into(),
                                    ..Default::default()
                                })
                                .with_children(_empty_card(empty_save_font.clone()));
                        }
                        Some(save) => {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: _card_style(),
                                    color: Color::WHITE.into(),
                                    ..Default::default()
                                })
                                .insert(LoadMark(n))
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            format!("Some save {}", n),
                                            TextStyle {
                                                font: save_font.clone(),
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    });
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            format!("{}", save.0.current),
                                            TextStyle {
                                                font: save_font.clone(),
                                                font_size: 15.0,
                                                color: Color::BLACK,
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    });
                                });
                        }
                    }
                })
                .id()
        })
        .collect()
}


pub fn make_save_items(
    commands: &mut Commands,
    saves: &Saves,
    save_font: Handle<Font>,
    empty_save_font: Handle<Font>,
) -> Vec<Entity>
{
    (0..6)
        .map(|n: u8| {
            commands
                .spawn_bundle(_make_card_base())
                .with_children(|parent| {
                    let mut card = parent
                        .spawn_bundle(ButtonBundle {
                            style: _card_style(),
                            ..Default::default()
                        });
                    card.insert(SaveMark { to: n });
                    match saves.saves.get(&n) {
                        None => {
                            card.with_children(_empty_card(empty_save_font.clone()));
                        }
                        Some(save) => {
                            card
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            format!("Some save {}", n),
                                            TextStyle {
                                                font: save_font.clone(),
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    });
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            format!("{}", save.0.current),
                                            TextStyle {
                                                font: save_font.clone(),
                                                font_size: 15.0,
                                                color: Color::BLACK,
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    });
                                });
                        }
                    }
                })
                .id()
        })
        .collect()
}
