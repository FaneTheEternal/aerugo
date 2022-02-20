use bevy::prelude::*;
use crate::saves::{LoadMark, Save, Saves};
use super::*;

const TRANSPARENT: Color = Color::rgba(1.0, 1.0, 1.0, 0.0);

pub fn make_load_items(
    commands: &mut Commands,
    saves: Res<Saves>,
    button_font: Handle<Font>,
    text_font: Handle<Font>,
) -> Vec<Entity>
{
    (0..6)
        .map(|n: u8| {
            commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(33.3), Val::Percent(50.0)),
                        padding: Rect::all(Val::Percent(5.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    color: TRANSPARENT.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    match saves.saves.get(&n) {
                        None => {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    color: Color::WHITE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Empty save",
                                                TextStyle {
                                                    font: text_font.clone(),
                                                    font_size: 20.0,
                                                    color: Color::BLACK,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });
                                });
                        }
                        Some(save) => {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        flex_wrap: FlexWrap::Wrap,
                                        flex_direction: FlexDirection::ColumnReverse,
                                        ..Default::default()
                                    },
                                    color: Color::WHITE.into(),
                                    ..Default::default()
                                })
                                .insert(LoadMark(n))
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            format!("Some save {}", n),
                                            TextStyle {
                                                font: text_font.clone(),
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
                                                font: text_font.clone(),
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
