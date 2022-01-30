use bevy::app::Events;
use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::game::components::{GameButton, GameButtons, SpriteMark};
use super::GameData;
use crate::states::OverlayState;
use crate::utils::grow_z_index;

const BTN_NORMAL: Color = Color::WHITE;
const BTN_HOVERED: Color = Color::GRAY;
const BTN_PRESSED: Color = Color::DARK_GRAY;

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

pub fn setup_game(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
)
{
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
            color: TRANSPARENT.into(),
            ..Default::default()
        })
        // .with_children(|parent| {
        //     parent.spawn_bundle(TextBundle {
        //         text: Text::with_section(
        //             "Game",
        //             TextStyle {
        //                 font: text_font.clone(),
        //                 font_size: 40.0,
        //                 color: Color::BLACK,
        //             },
        //             Default::default(),
        //         ),
        //         ..Default::default()
        //     });
        // })
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

    // Background
    command.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(10_000.0, 10_000.0)),
            ..Default::default()
        },
        ..Default::default()
    });

    let window = window.get_primary().unwrap();
    let x_peace = window.width() / 5.0;
    command.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        transform: Transform::from_xyz(
            -(x_peace * 1.5),
            0.0,
            1.0,
        ),
        ..Default::default()
    }).insert(SpriteMark::new("left"));
    command.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        transform: Transform::from_xyz(
            -(x_peace * 0.5),
            0.0,
            1.0,
        ),
        ..Default::default()
    }).insert(SpriteMark::new("jump"));
    command.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        transform: Transform::from_xyz(
            x_peace * 0.5,
            0.0,
            1.0,
        ),
        ..Default::default()
    }).insert(SpriteMark::new("emergence"));
    command.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        transform: Transform::from_xyz(
            x_peace * 1.5,
            0.0,
            1.0,
        ),
        ..Default::default()
    }).insert(SpriteMark::new("right"));
}

pub fn update_sleep_sprite(
    resize_event: Res<Events<WindowResized>>,
    mut sprite_transform_query: Query<(&mut Transform, &SpriteMark)>,
)
{
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        let x_peace = e.width / 5.0;
        for (transform, mark) in sprite_transform_query.iter_mut() {
            let mut transform: Mut<Transform> = transform;
            let mark: &SpriteMark = mark;
            if mark.is_await {
                transform.translation.x = match mark.name.as_str() {
                    "left" => {
                        -(x_peace * 1.5)
                    }
                    "jump" => {
                        -(x_peace * 0.5)
                    }
                    "emergence" => {
                        x_peace * 0.5
                    }
                    "right" => {
                        x_peace * 1.5
                    }
                    _ => { transform.translation.y }
                }
            }
        }
    }
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

pub fn game_sprite_animate(
    time: Res<Time>,
    window: Res<Windows>,
    mut sprite_transform_query: Query<(&mut Transform, &mut Sprite, &mut SpriteMark)>,
)
{
    const SPRITE_X: f32 = 256.0;

    let window = window.get_primary().unwrap();
    let x_peace = window.width() / 5.0;

    let max_x = window.width() / 2.0 + SPRITE_X;
    let min_x = x_peace * 1.5;

    let max_y = 100.0;
    let min_y = 0.0;

    for (transform, sprite, mark) in sprite_transform_query.iter_mut() {
        let mut transform: Mut<Transform> = transform;
        let mut mark: Mut<SpriteMark> = mark;
        let mut sprite: Mut<Sprite> = sprite;
        mark.timer.tick(time.delta());
        if !mark.is_await {
            let animation_k = mark.timer.elapsed_secs() / mark.timer.duration().as_secs_f32();
            match mark.name.as_str() {
                "left" => {
                    if mark.is_rev {
                        transform.translation.x = -(max_x - (max_x - min_x) * animation_k);
                    } else {
                        transform.translation.x = -(min_x + (max_x - min_x) * animation_k);
                    }
                    if mark.timer.just_finished() {
                        mark.is_rev = !mark.is_rev;
                    }
                }
                "jump" => {
                    transform.translation.y = if animation_k >= 0.5 {
                        max_y - (max_y - min_y) * (animation_k - 0.5) * 2.0
                    } else {
                        min_y + (max_y - min_y) * animation_k * 2.0
                    }
                }
                "emergence" => {
                    let a = if mark.is_rev {
                        animation_k
                    } else {
                        1.0 - animation_k
                    };
                    sprite.color.set_a(a);
                    if mark.timer.just_finished() {
                        mark.is_rev = !mark.is_rev;
                    }
                }
                "right" => {
                    if mark.is_rev {
                        transform.translation.x = max_x - (max_x - min_x) * animation_k;
                    } else {
                        transform.translation.x = min_x + (max_x - min_x) * animation_k;
                    }
                    if mark.timer.just_finished() {
                        mark.is_rev = !mark.is_rev;
                    }
                }
                _ => {}
            }
        }
        if mark.timer.just_finished() {
            mark.is_await = !mark.is_await;
            mark.timer.reset();
        }
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
