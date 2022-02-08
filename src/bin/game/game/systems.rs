use std::io::Read;
use substring::Substring;
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
    // let button_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

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
    let mut text_ui_entity = None;
    let text_ui_root_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: Rect::all(Val::Px(10.0)),
                position_type: PositionType::Absolute,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Column,
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
                    let entity = parent
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
                        .insert(TextFlowMark)
                        .id();
                    text_ui_entity = Some(entity);
                });
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
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
                        .insert(NarratorFlowMark);
                });
        })
        .id();
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
        just_init: true,
        aerugo_state,
        text_narrator_entity,
        text_background_entity,
        text_ui_root_entity,
        text_ui_entity: text_ui_entity.unwrap(),
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
    mut commands: Commands,
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
        if game_state.just_init {
            game_state.just_init = false;
        } else {
            if game_state.aerugo_state.next(&game_data.aerugo).is_none() {
                return;
            }
        }
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

        let step = game_state.aerugo_state.step(&game_data.aerugo);
        commands.insert_resource(step);
    }
}

pub fn new_background_listener(
    game_state: Res<GameState>,
    mut new_background_event: EventReader<NewBackgroundEvent>,
    mut background_query: Query<&mut Handle<Image>, With<BackgroundMark>>,
    asset_server: Res<AssetServer>,
)
{
    for event in new_background_event.iter() {
        let mut background = background_query.get_mut(game_state.background_entity).unwrap();
        let cmd: &BackgroundCommand = &event.0;
        match cmd {
            BackgroundCommand::Change { new, .. } => {
                *background = asset_server.load(new);
            }
            BackgroundCommand::Shake => {}
            BackgroundCommand::None => {}
        }
    }
}

pub fn step_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
    step: Option<Res<Step>>,
    mut text_base_query: Query<&mut Style, With<TextFlowBase>>,
    mut narrator_flow_query: Query<&mut Text, With<NarratorFlowMark>>,
    mut text_sprite_query: Query<&mut Visibility, With<Sprite>>,
    mut style_query: Query<&mut Style, Without<TextFlowBase>>,
    mut mute_control_state: ResMut<State<MuteControl>>,
)
{
    if let Some(step) = step {
        let text_font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

        text_base_query.for_each_mut(|mut e| { e.display = Display::None });
        text_sprite_query.get_mut(game_state.text_narrator_entity).unwrap().is_visible = false;
        text_sprite_query.get_mut(game_state.text_background_entity).unwrap().is_visible = false;
        commands.entity(game_state.phrase_ui_entity).despawn_descendants();
        style_query.get_mut(game_state.phrase_ui_entity).unwrap().display = Display::None;

        match &step.inner {
            Steps::Text { author, texts } => {
                text_base_query.for_each_mut(|mut e| { e.display = Display::Flex });
                text_sprite_query.get_mut(game_state.text_narrator_entity).unwrap().is_visible = true;
                text_sprite_query.get_mut(game_state.text_background_entity).unwrap().is_visible = true;

                narrator_flow_query.for_each_mut(|mut text| {
                    text.sections = vec![TextSection {
                        value: author.clone(),
                        style: TextStyle {
                            font: text_font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    }];
                });
                commands
                    .entity(game_state.text_ui_entity)
                    .insert(AnimateText {
                        text: texts.clone(),
                        timer: Timer::from_seconds(0.1, true),
                        style: TextStyle {
                            font: text_font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                        chars: 0,
                    });
                commands.insert_resource(CurrentStep::Text);
            }
            Steps::Phrase { phrases } => {
                style_query.get_mut(game_state.phrase_ui_entity).unwrap().display = Display::Flex;
                let mut ui = commands.entity(game_state.phrase_ui_entity);
                for phrase in phrases {
                    let (key, verbose) = phrase;
                    ui.with_children(|parent| {
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    margin: Rect::all(Val::Percent(1.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PhraseValue(key.clone()))
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        verbose.as_str(),
                                        TextStyle {
                                            font: text_font.clone(),
                                            font_size: 40.0,
                                            color: Color::BLACK,
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    ..Default::default()
                                });
                            });
                    });
                }
                commands.insert_resource(CurrentStep::Phrase);
            }
            Steps::ImageSelect { .. } => {}
            _ => {}
        }
        commands.remove_resource::<Step>();
        mute_control_state.set(MuteControl::Pass);
    }
}

pub fn input_listener(
    mut game_state: ResMut<GameState>,
    game_data: Res<GameData>,
    mut mute_control_state: ResMut<State<MuteControl>>,
    mut key_input: ResMut<Input<KeyCode>>,
    current_step: Option<Res<CurrentStep>>,
    mut next_step_event: EventWriter<NextStepEvent>,
    mut phrase_query: Query<(&Interaction, &mut UiColor, &PhraseValue)>,
    mut pass_animate_event: EventWriter<PassAnimateEvent>,
)
{
    let current = mute_control_state.current();
    if current.eq(&MuteControl::Mute) || current_step.is_none() {
        key_input.clear();
        return;
    }
    let current_step = current_step.unwrap();

    let any = current.eq(&MuteControl::None);
    let pass = current.eq(&MuteControl::Pass);

    if pass {
        if key_input.clear_just_released(KeyCode::Space)
            || key_input.clear_just_released(KeyCode::Return) {
            mute_control_state.set(MuteControl::Mute);
            pass_animate_event.send(PassAnimateEvent);
        }
    }

    if any && current_step.eq(&CurrentStep::Text) {
        if key_input.clear_just_released(KeyCode::Space)
            || key_input.clear_just_released(KeyCode::Return) {
            mute_control_state.set(MuteControl::Mute);
            next_step_event.send(NextStepEvent);
        }
    }

    if any && current_step.eq(&CurrentStep::Phrase) {
        for (interaction, color, phrase) in phrase_query.iter_mut() {
            let interaction: &Interaction = interaction;
            let mut color: Mut<UiColor> = color;
            let phrase: &PhraseValue = phrase;
            match interaction {
                Interaction::Clicked => {
                    *color = Color::DARK_GRAY.into();

                    let step = game_state.aerugo_state.step(&game_data.aerugo);
                    game_state.aerugo_state.select_unique(step.id, phrase.0.clone());
                    mute_control_state.set(MuteControl::Mute);
                    next_step_event.send(NextStepEvent);
                }
                Interaction::Hovered => {
                    *color = Color::GRAY.into();
                }
                Interaction::None => {
                    *color = Color::WHITE.into();
                }
            }
        }
    }
}

pub fn animate(
    mut commands: Commands,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut mute_control_state: ResMut<State<MuteControl>>,
    mut pass: EventReader<PassAnimateEvent>,
    mut text_query: Query<(&mut Text, &mut AnimateText), With<TextFlowMark>>,
)
{
    let mut unmute_control = true;
    let pass = pass.iter().count() > 0;

    let text_animate = text_query.get_mut(game_state.text_ui_entity);
    if let Ok((text, animate)) = text_animate {
        unmute_control = false;

        let mut text: Mut<Text> = text;
        let mut animate: Mut<AnimateText> = animate;

        if pass {
            animate.chars = animate.text.chars().count();
        } else if animate.timer.tick(time.delta()).just_finished() {
            animate.chars += 1;
        }
        text.sections = vec![TextSection {
            value: animate.text.substring(0, animate.chars).to_string(),
            style: animate.style.clone(),
        }];
        if animate.text.chars().count() <= animate.chars {
            commands.entity(game_state.text_ui_entity).remove::<AnimateText>();
        }
    }

    if unmute_control {
        mute_control_state.set(MuteControl::None);
    }
}

pub fn cleanup(mut commands: Commands, game_state: Res<GameState>) {
    commands.entity(game_state.text_narrator_entity).despawn_recursive();
    commands.entity(game_state.text_background_entity).despawn_recursive();
    commands.entity(game_state.text_ui_root_entity).despawn_recursive();
    commands.entity(game_state.phrase_ui_entity).despawn_recursive();
    commands.entity(game_state.narrator_entity).despawn_recursive();
    commands.entity(game_state.background_entity).despawn_recursive();
}
