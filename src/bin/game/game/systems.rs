use std::io::Read;
use bevy::app::Events;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use bevy::window::WindowResized;

use aerugo::*;

use super::*;
use crate::states::OverlayState;

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

const Z_TEXT: f32 = 10.0;
const Z_NARRATOR: f32 = 20.0;
const Z_BACKGROUND: f32 = 5.0;

pub fn preload_aerugo(mut command: Commands) {
    const SCENARIO_PATH: &str = "scenario.json";
    let mut file = std::fs::File::options()
        .read(true).write(true).create(true)
        .open(SCENARIO_PATH)
        .unwrap();
    let mut aerugo = String::new();
    file.read_to_string(&mut aerugo).unwrap();
    let aerugo: Aerugo = serde_json::from_str(&aerugo).unwrap();

    command.insert_resource(GameData { aerugo });
}

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
    game_data: Res<GameData>,
    mut next_step_event: EventWriter<NextStepEvent>,
)
{
    let text_font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let button_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    let window = window.get_primary().unwrap();
    let w = window.width();
    let h = window.height();

    let aerugo_state = AerugoState::setup(&game_data.aerugo);

    // region spawn text flow
    let text_narrator_entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(w * 0.99, h * 0.09)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, h * -0.25, Z_TEXT),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .id();
    let text_background_entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(w * 0.99, h * 0.19)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, h * -0.4, Z_TEXT),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .id();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: Rect::all(Val::Px(10.0)),
                position_type: PositionType::Absolute,
                display: Display::None,
                ..Default::default()
            },
            color: TRANSPARENT.into(),
            ..Default::default()
        })
        .insert(TextFlowBase)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        align_items: AlignItems::FlexStart,
                        align_content: AlignContent::FlexStart,
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_wrap: FlexWrap::Wrap,
                        padding: Rect {
                            left: Default::default(),
                            right: Default::default(),
                            top: Val::Px(10.0),
                            bottom: Default::default(),
                        },
                        ..Default::default()
                    },
                    color: TRANSPARENT.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Some text",
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
                        .insert(TextFlowMark);
                });
        });
    // endregion

    // region spawn phrase
    let phrase_ui_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::None,
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
    // endregion

    // region spawn narrator
    let narrator_entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(w * 0.19, h * 0.19)),
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform::from_xyz(w * -0.4, h * -0.4, Z_NARRATOR),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(NarratorMark)
        .id();
    // endregion

    // region spawn background
    let background_entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
            ..Default::default()
        })
        .insert(BackgroundMark)
        .id();
    // endregion

    commands.insert_resource(GameState {
        aerugo_state,
        text_narrator_entity,
        text_background_entity,
        phrase_ui_entity,
        narrator_entity,
        background_entity,
    });

    next_step_event.send(NextStepEvent);
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

pub fn next_step_listener(
    mut events: EventReader<NextStepEvent>,
    mut game_state: ResMut<GameState>,
    game_data: Res<GameData>,
    mut new_narrator_event: EventWriter<NewNarratorEvent>,
    mut new_sprite_event: EventWriter<NewSpriteEvent>,
    mut new_background_event: EventWriter<NewBackgroundEvent>,
    mut new_scene_event: EventWriter<NewSceneEvent>,
)
{
    if events.iter().count() > 0 {
        let steps = game_state.aerugo_state.collect(&game_data.aerugo);

        // send events to update graphic part
        for step in steps {
            match step {
                Steps::SpriteNarrator { sprite } => {
                    new_narrator_event.send(NewNarratorEvent(sprite));
                }
                Steps::Sprite { name, sprite, animation } => {
                    new_sprite_event.send(NewSpriteEvent { name, sprite, animation });
                }
                Steps::Background { command } => {
                    new_background_event.send(NewBackgroundEvent(command));
                }
                Steps::Scene { command } => {
                    new_scene_event.send(NewSceneEvent(command));
                }
                _ => {}
            }
        }
    }
}
