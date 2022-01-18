use bevy::prelude::*;
use crate::game::components::{GameButton, GameButtons};
use super::GameData;
use crate::states::OverlayState;
use crate::utils::grow_z_index;

const BTN_NORMAL: Color = Color::WHITE;
const BTN_HOVERED: Color = Color::GRAY;
const BTN_PRESSED: Color = Color::DARK_GRAY;

pub fn setup_game(
    mut command: Commands,
    asset_server: Res<AssetServer>,
)
{
    command.spawn_bundle(OrthographicCameraBundle::new_2d());

    let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    pub fn make_button_closure_arrow<B>(
        text: &str,
        font: Handle<Font>,
        button: B,
        button_color: Color,
    ) -> impl FnOnce(&mut ChildBuilder) + '_
        where B: Component
    {
        move |parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button_color.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                text,
                                TextStyle {
                                    font,
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(button);
                });
        }
    }
    pub fn make_button_closure_menu<B>(
        text: &str,
        font: Handle<Font>,
        button: B,
        button_color: Color,
    ) -> impl FnOnce(&mut ChildBuilder) + '_
        where B: Component
    {
        move |parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button_color.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                text,
                                TextStyle {
                                    font,
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(button);
                });
        }
    }

    let ui_entity = command
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Game",
                    TextStyle {
                        font: text_font.clone(),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        flex_wrap: FlexWrap::Wrap,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_wrap: FlexWrap::Wrap,
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            color: Color::rgba(0.5, 0.5, 0.7, 0.5).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(
                                    make_button_closure_arrow(
                                        "Back", button_font.clone(), GameButton { target: GameButtons::Back }, BTN_NORMAL,
                                    )
                                );
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::Center,
                                        flex_wrap: FlexWrap::Wrap,
                                        flex_direction: FlexDirection::ColumnReverse,
                                        ..Default::default()
                                    },
                                    color: Color::RED.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {  // top menu
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                                justify_content: JustifyContent::FlexStart,
                                                align_items: AlignItems::FlexStart,
                                                flex_wrap: FlexWrap::Wrap,
                                                flex_direction: FlexDirection::Row,
                                                ..Default::default()
                                            },
                                            color: Color::GREEN.into(),
                                            ..Default::default()
                                        })
                                        .with_children(
                                            make_button_closure_menu(
                                                "Menu", button_font.clone(), GameButton { target: GameButtons::Menu }, BTN_NORMAL,
                                            )
                                        );
                                })
                                .with_children(|parent| {  // bottom text
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                                padding: Rect::all(Val::Px(10.0)),
                                                justify_content: JustifyContent::FlexStart,
                                                align_items: AlignItems::FlexStart,
                                                flex_wrap: FlexWrap::Wrap,
                                                flex_direction: FlexDirection::ColumnReverse,
                                                ..Default::default()
                                            },
                                            color: Color::BISQUE.into(),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn_bundle(TextBundle {
                                                text: Text::with_section(
                                                    "Some text",
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
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                                        padding: Rect::all(Val::Px(10.0)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(
                                    make_button_closure_arrow(
                                        "Forward", button_font.clone(), GameButton { target: GameButtons::Forward }, BTN_NORMAL,
                                    )
                                );
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                ..Default::default()
                            },
                            color: Color::rgba(0.5, 0.8, 0.8, 0.8).into(),
                            ..Default::default()
                        });
                });
        })
        .id();

    command.insert_resource(GameData { ui_entity });
}

pub fn open_overlay(
    mut input: ResMut<Input<KeyCode>>,
    mut overlay_state: ResMut<State<OverlayState>>,
)
{
    match overlay_state.current() {
        OverlayState::Hidden => {
            if input.clear_just_released(KeyCode::Escape) {
                overlay_state.set(OverlayState::Menu).unwrap();
            }
        }
        _ => {}
    }
}

pub fn game_buttons(
    mut overlay_state: ResMut<State<OverlayState>>,
    mut interactions_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    buttons_query: Query<&GameButton>,
)
{
    for (interaction, mut color, children) in interactions_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = BTN_PRESSED.into();

                if let Ok(btn) = buttons_query.get(children[0]) {
                    match btn.target {
                        GameButtons::Back => {}
                        GameButtons::Forward => {}
                        GameButtons::Menu => {
                            overlay_state.set(OverlayState::Menu).unwrap();
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

pub fn cleanup(
    mut command: Commands,
    game_data: Option<Res<GameData>>,
)
{
    if let Some(game_data) = game_data {
        command.entity(game_data.ui_entity).despawn_recursive();
        command.remove_resource::<GameData>();
    }
}
