use std::io::Read;
use substring::Substring;
use bevy::{
    prelude::*,
};

use aerugo::*;
use crate::saves::AerugoLoaded;

use super::*;
use crate::ui::{GameUI, OverlayButton, OverlayButtons, UiState};
use crate::utils::{BTN_HOVERED, BTN_NORMAL, BTN_PRESSED, load_aerugo, Y_SPRITE, Z_SPRITE};

pub fn setup_game(
    mut commands: Commands,
    aerugo: Res<Aerugo>,
    mut next_step_event: EventWriter<NextStepEvent>,
    aerugo_loaded: Option<Res<AerugoLoaded>>,
    mut game_state: ResMut<State<GameState>>,
    mut game_ui: ResMut<GameUI>,
    mut style_query: Query<&mut Style>,
    mut visibility_query: Query<&mut Visibility>,
    mut ui_image_query: Query<&mut UiImage>,
    mut image_query: Query<&mut Handle<Image>>,
)
{
    let aerugo_state = aerugo_loaded
        .map(|loaded| { loaded.0.to_owned() })
        .unwrap_or_else(|| { AerugoState::setup(aerugo.as_ref()) });
    commands.remove_resource::<AerugoLoaded>();

    commands.insert_resource(aerugo_state);
    commands.insert_resource(JustInit);

    next_step_event.send(NextStepEvent);
    game_state.set(GameState::Active).unwrap_or_else(|e| warn!("{e:?}"));

    game_ui.sprites.values().for_each(|&e| {
        commands.entity(e).despawn_recursive();
    });
    game_ui.background_visible = false;
    game_ui.scene_visible = false;
    *image_query.get_mut(game_ui.background).unwrap() = Default::default();
    *image_query.get_mut(game_ui.scene).unwrap() = Default::default();
    visibility_query.get_mut(game_ui.background).unwrap().is_visible = false;
    visibility_query.get_mut(game_ui.scene).unwrap().is_visible = false;
    game_ui.sprites = Default::default();
    style_query.get_mut(game_ui.text.narrator_sprite).unwrap()
        .display = Display::None;
    ui_image_query.get_mut(game_ui.text.narrator_sprite).unwrap()
        .0 = Default::default();
}

pub fn open_overlay(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
)
{
    if input.clear_just_released(KeyCode::Escape) {
        game_state.set(GameState::Paused).unwrap_or_else(|e| warn!("{e:?}"));
    }
}

pub fn next_step_listener(
    mut commands: Commands,
    mut events: EventReader<NextStepEvent>,
    mut aerugo_state: ResMut<AerugoState>,
    aerugo: Res<Aerugo>,
    mut new_narrator_event: EventWriter<NewNarratorEvent>,
    mut new_sprite_event: EventWriter<NewSpriteEvent>,
    mut new_background_event: EventWriter<NewBackgroundEvent>,
    mut new_scene_event: EventWriter<NewSceneEvent>,
    just_init: Option<Res<JustInit>>,
)
{
    if events.iter().count() > 0 {
        match just_init {
            None => {
                if aerugo_state.next(aerugo.as_ref()).is_none() {
                    return;
                }
            }
            Some(_) => {
                commands.remove_resource::<JustInit>();
            }
        }
        let steps = aerugo_state.collect(aerugo.as_ref());

        // send events to update graphic part
        for step in steps {
            match step {
                Steps::SpriteNarrator { sprite } => {
                    new_narrator_event.send(NewNarratorEvent(sprite));
                }
                Steps::Sprite(command) => {
                    new_sprite_event.send(NewSpriteEvent(command));
                }
                Steps::Background(command) => {
                    new_background_event.send(NewBackgroundEvent(command));
                }
                Steps::Scene(command) => {
                    new_scene_event.send(NewSceneEvent(command));
                }
                _ => {}
            }
        }

        let step = aerugo_state.step(aerugo.as_ref());
        commands.insert_resource(step);
    }
}

pub fn new_narrator_listener(
    game_ui: Res<GameUI>,
    mut style_query: Query<&mut Style>,
    mut image_query: Query<&mut UiImage>,
    mut new_narrator_event: EventReader<NewNarratorEvent>,
    asset_server: Res<AssetServer>,
)
{
    for event in new_narrator_event.iter() {
        let narrator: &Option<String> = &event.0;

        match narrator {
            None => {
                style_query.get_mut(game_ui.text.narrator_sprite).unwrap()
                    .display = Display::None;
                image_query.get_mut(game_ui.text.narrator_sprite).unwrap()
                    .0 = Default::default();
            }
            Some(s) => {
                style_query.get_mut(game_ui.text.narrator_sprite).unwrap()
                    .display = Display::Flex;
                image_query.get_mut(game_ui.text.narrator_sprite).unwrap()
                    .0 = asset_server.load(s);
            }
        }
    }
}

pub fn new_background_listener(
    mut game_ui: ResMut<GameUI>,
    mut new_background_event: EventReader<NewBackgroundEvent>,
    mut background_query: Query<(&mut Handle<Image>, &mut Visibility)>,
    asset_server: Res<AssetServer>,
)
{
    for event in new_background_event.iter() {
        let (mut background, mut visibility): (Mut<Handle<Image>>, Mut<Visibility>) =
            background_query.get_mut(game_ui.background).unwrap();
        let cmd: &BackgroundCommand = &event.0;
        match cmd {
            BackgroundCommand::Change { new, .. } => {
                *background = asset_server.load(new);
                visibility.is_visible = true;
                game_ui.background_visible = true;
            }
            BackgroundCommand::Shake => {
                unimplemented!("Unimplemented 'Shake'")
            }
            BackgroundCommand::None => {
                *background = Default::default();
                visibility.is_visible = true;
                game_ui.background_visible = false;
            }
        }
    }
}

pub fn new_scene_listener(
    mut game_ui: ResMut<GameUI>,
    mut new_scene_event: EventReader<NewSceneEvent>,
    mut scene_query: Query<(&mut Handle<Image>, &mut Visibility)>,
    asset_server: Res<AssetServer>,
)
{
    for event in new_scene_event.iter() {
        let cmd: &SceneCommand = &event.0;
        let (mut scene, mut visibility): (Mut<Handle<Image>>, Mut<Visibility>) =
            scene_query.get_mut(game_ui.scene).unwrap();
        match cmd {
            SceneCommand::Set { name } => {
                *scene = asset_server.load(name);
                visibility.is_visible = true;
                game_ui.scene_visible = true;
            }
            SceneCommand::Remove => {
                *scene = Default::default();
                visibility.is_visible = false;
                game_ui.scene_visible = false;
            }
            SceneCommand::Play { .. } => { todo!("Play") }
            SceneCommand::Pause => { todo!("Pause") }
            SceneCommand::None => {}
        }
    }
}

pub fn new_sprite_listener(
    mut commands: Commands,
    mut game_ui: ResMut<GameUI>,
    mut new_sprite_event: EventReader<NewSpriteEvent>,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
)
{
    const FADE_IN_DURATION: f32 = 1.0;
    const FADE_OUT_DURATION: f32 = 1.0;
    const LEFT_IN_DURATION: f32 = 1.0;
    const LEFT_OUT_DURATION: f32 = 1.0;
    const RIGHT_IN_DURATION: f32 = 1.0;
    const RIGHT_OUT_DURATION: f32 = 1.0;
    const MOVE_DURATION: f32 = 1.0;

    let window = window.get_primary().unwrap();
    let w = window.width() / 2.0;
    // let h = window.height();

    for event in new_sprite_event.iter() {
        let cmd: &SpriteCommand = &event.0;
        match cmd {
            SpriteCommand::Set { sprite, name, position } => {
                let sprite: Handle<Image> = asset_server.load(sprite);
                let mut entity_cmd = match game_ui.sprites.get_mut(name) {
                    None => {
                        commands.spawn_bundle(SpriteBundle::default())
                    }
                    Some(entity) => {
                        commands.entity(*entity)
                    }
                };
                entity_cmd.insert(sprite);
                entity_cmd.insert(Transform::from_xyz(w * position, Y_SPRITE, Z_SPRITE));
                game_ui.sprites.insert(name.clone(), entity_cmd.id());
            }
            SpriteCommand::Remove { name } => {
                game_ui.sprites
                    .remove(name)
                    .or_else(|| {
                        warn!("Invalid sprite name: {}", name);
                        Option::<Entity>::None
                    })
                    .and_then(|e| {
                        commands.entity(e).despawn_recursive();
                        Option::<Entity>::None
                    });
            }
            SpriteCommand::FadeIn { sprite, name, position } => {
                let sprite: Handle<Image> = asset_server.load(sprite);
                let mut entity_cmd = match game_ui.sprites.get_mut(name) {
                    None => { commands.spawn_bundle(SpriteBundle::default()) }
                    Some(entity) => { commands.entity(*entity) }
                };
                let entity = entity_cmd
                    .insert(sprite)
                    .insert(Transform::from_xyz(w * position, Y_SPRITE, Z_SPRITE))
                    .insert(AnimateFadeSprite {
                        timer: Timer::from_seconds(FADE_IN_DURATION, false),
                        fade_in: true,
                        name: name.clone(),
                    })
                    .id();
                game_ui.sprites.insert(name.clone(), entity);
            }
            SpriteCommand::FadeOut { name } => {
                game_ui.sprites.get(name)
                    .and_then(|e| {
                        commands
                            .entity(e.clone())
                            .insert(AnimateFadeSprite {
                                timer: Timer::from_seconds(FADE_OUT_DURATION, false),
                                fade_in: false,
                                name: name.clone(),
                            });
                        Some(e)
                    });
            }
            SpriteCommand::LeftIn { sprite, name, position } => {
                let sprite: Handle<Image> = asset_server.load(sprite);
                let mut entity_cmd = match game_ui.sprites.get_mut(name) {
                    None => { commands.spawn_bundle(SpriteBundle::default()) }
                    Some(entity) => { commands.entity(*entity) }
                };
                let entity = entity_cmd
                    .insert(sprite)
                    .insert(Transform::from_xyz(w * -2.0, Y_SPRITE, Z_SPRITE))
                    .insert(AnimateMoveSprite {
                        timer: Timer::from_seconds(LEFT_IN_DURATION, false),
                        start_pos: f32::NEG_INFINITY,
                        end_pos: w * position,
                        name: name.clone(),
                        move_out: false,
                    })
                    .id();
                game_ui.sprites.insert(name.clone(), entity);
            }
            SpriteCommand::LeftOut { name } => {
                game_ui.sprites.get(name)
                    .and_then(|e| {
                        commands
                            .entity(e.clone())
                            .insert(AnimateMoveSprite {
                                timer: Timer::from_seconds(LEFT_OUT_DURATION, false),
                                start_pos: f32::NAN,
                                end_pos: f32::NEG_INFINITY,
                                name: name.clone(),
                                move_out: true,
                            });
                        Some(e)
                    });
            }
            SpriteCommand::RightIn { sprite, name, position } => {
                let sprite: Handle<Image> = asset_server.load(sprite);
                let mut entity_cmd = match game_ui.sprites.get_mut(name) {
                    None => { commands.spawn_bundle(SpriteBundle::default()) }
                    Some(entity) => { commands.entity(*entity) }
                };
                let entity = entity_cmd
                    .insert(sprite)
                    .insert(Transform::from_xyz(w * 2.0, Y_SPRITE, Z_SPRITE))
                    .insert(AnimateMoveSprite {
                        timer: Timer::from_seconds(RIGHT_IN_DURATION, false),
                        start_pos: f32::INFINITY,
                        end_pos: w * position,
                        name: name.clone(),
                        move_out: false,
                    })
                    .id();
                game_ui.sprites.insert(name.clone(), entity);
            }
            SpriteCommand::RightOut { name } => {
                game_ui.sprites.get(name)
                    .and_then(|e| {
                        commands
                            .entity(e.clone())
                            .insert(AnimateMoveSprite {
                                timer: Timer::from_seconds(RIGHT_OUT_DURATION, false),
                                start_pos: f32::NAN,
                                end_pos: f32::INFINITY,
                                name: name.clone(),
                                move_out: true,
                            });
                        Some(e)
                    });
            }
            SpriteCommand::Move { name, position } => {
                game_ui.sprites.get(name)
                    .and_then(|e| {
                        commands
                            .entity(e.clone())
                            .insert(AnimateMoveSprite {
                                timer: Timer::from_seconds(MOVE_DURATION, false),
                                start_pos: f32::NAN,
                                end_pos: w * position,
                                name: name.clone(),
                                move_out: false,
                            });
                        Some(e)
                    });
            }
            _ => {}
        }
    }
}

pub fn step_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_control_state: ResMut<State<GameControlState>>,
    step: Option<Res<Step>>,
    mut style_query: Query<&mut Style>,
    mut game_ui: ResMut<GameUI>,
)
{
    if let Some(step) = step {
        let text_font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

        game_ui.text.force_hide(&mut style_query);
        game_ui.phrase.force_hide(&mut style_query);

        match &step.inner {
            Steps::Text { author, texts } => {
                game_ui.text.force_show(&mut style_query);

                commands
                    .entity(game_ui.text.narrator)
                    .insert(Text::with_section(
                        author.as_str(),
                        TextStyle {
                            font: text_font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                        Default::default(),
                    ));
                commands
                    .entity(game_ui.text.text)
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
                game_control_state.set(GameControlState::TextPass).unwrap_or_else(|e| warn!("{e:?}"));
            }
            Steps::Phrase { phrases } => {
                game_ui.phrase.force_show(&mut style_query);

                let phrase_options: Vec<Entity> = phrases.iter()
                    .map(|o| {
                        let (key, verbose) = o;
                        commands
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    margin: UiRect::all(Val::Percent(1.0)),
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
                            })
                            .id()
                    })
                    .collect();
                commands.entity(game_ui.phrase.root).despawn_descendants();
                commands.entity(game_ui.phrase.root).push_children(phrase_options.as_slice());

                game_control_state.set(GameControlState::Phrase).unwrap_or_else(|e| warn!("{e:?}"));
            }
            Steps::ImageSelect { .. } => {
                todo!("ImageSelect")
            }
            _ => {}
        }
        commands.remove_resource::<Step>();
    }
}

pub fn input_text_pass(
    mut game_control_state: ResMut<State<GameControlState>>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
)
{
    if key_input.clear_just_pressed(KeyCode::Space)
        || key_input.clear_just_pressed(KeyCode::Return)
        || mouse_button_input.clear_just_pressed(MouseButton::Left) {
        game_control_state.set(GameControlState::Text).unwrap_or_else(|e| warn!("{e:?}"));
    }
}

pub fn input_text_next(
    mut game_control_state: ResMut<State<GameControlState>>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    mut next_step_event: EventWriter<NextStepEvent>,
)
{
    if key_input.clear_just_pressed(KeyCode::Space)
        || key_input.clear_just_pressed(KeyCode::Return)
        || mouse_button_input.clear_just_pressed(MouseButton::Left) {
        game_control_state.set(GameControlState::None).unwrap_or_else(|e| warn!("{e:?}"));
        next_step_event.send(NextStepEvent);
    }
}

pub fn input_phrase(
    mut aerugo_state: ResMut<AerugoState>,
    aerugo: Res<Aerugo>,
    mut game_control_state: ResMut<State<GameControlState>>,
    mut phrase_query: Query<(&Interaction, &PhraseValue, &mut UiColor), Changed<Interaction>>,
    mut next_step_event: EventWriter<NextStepEvent>,
)
{
    for (interaction, phrase, color) in phrase_query.iter_mut() {
        let interaction: &Interaction = interaction;
        let phrase: &PhraseValue = phrase;
        let mut color: Mut<UiColor> = color;
        match interaction {
            Interaction::Clicked => {
                *color = BTN_PRESSED.into();

                let step = aerugo_state.step(aerugo.as_ref());
                aerugo_state.select_unique(step.id, phrase.0.clone());
                game_control_state.set(GameControlState::None).unwrap_or_else(|e| warn!("{e:?}"));
                next_step_event.send(NextStepEvent);
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

pub fn animate(
    mut commands: Commands,
    time: Res<Time>,
    mut game_ui: ResMut<GameUI>,
    mut game_control_state: ResMut<State<GameControlState>>,
    window: Res<Windows>,
    mut text_query: Query<(&mut Text, &mut AnimateText)>,
    mut sprite_fade_query: Query<(&mut Sprite, &mut AnimateFadeSprite)>,
    mut sprite_move_query: Query<(&mut Transform, &mut AnimateMoveSprite)>,
)
{
    let mut unmute_control = true;
    let pass = game_control_state.current().eq(&GameControlState::Text);

    let window = window.get_primary().unwrap();
    let w = window.width() / 2.0;
    // let h = window.height();

    let text_animate = text_query.get_mut(game_ui.text.text);
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
            commands.entity(game_ui.text.text).remove::<AnimateText>();
        }
    }

    for sprite in sprite_fade_query.iter_mut() {
        let (mut sprite, mut animate_fade): (Mut<Sprite>, Mut<AnimateFadeSprite>) = sprite;
        unmute_control = false;

        animate_fade.timer.tick(time.delta());

        let alfa = if animate_fade.fade_in {
            animate_fade.timer.percent()
        } else {
            1.0 - animate_fade.timer.percent()
        };
        sprite.color.set_a(alfa);
        if animate_fade.timer.just_finished() {
            if animate_fade.fade_in {
                commands.entity(*game_ui.sprites.get(&animate_fade.name).unwrap())
            } else {
                commands.entity(game_ui.sprites.remove(&animate_fade.name).unwrap())
            }.remove::<AnimateFadeSprite>();
        }
    }

    for sprite in sprite_move_query.iter_mut() {
        let (mut transform, mut animate_move): (Mut<Transform>, Mut<AnimateMoveSprite>) = sprite;
        unmute_control = false;

        if !animate_move.start_pos.is_finite() {
            animate_move.start_pos = transform.translation.x;
        }

        let end_pos = if animate_move.end_pos.is_finite() {
            animate_move.end_pos
        } else {
            if animate_move.end_pos.is_sign_negative() {
                w * -2.0
            } else {
                w * 2.0
            }
        };

        animate_move.timer.tick(time.delta());

        transform.translation.x = animate_move.start_pos
            + (end_pos - animate_move.start_pos) * animate_move.timer.percent();

        if animate_move.timer.just_finished() {
            if !animate_move.move_out {
                commands.entity(*game_ui.sprites.get(&animate_move.name).unwrap())
            } else {
                commands.entity(game_ui.sprites.remove(&animate_move.name).unwrap())
            }.remove::<AnimateMoveSprite>();
        }
    }

    if unmute_control && game_control_state.current().eq(&GameControlState::TextPass) {
        game_control_state.set(GameControlState::Text).unwrap_or_else(|e| warn!("{e:?}"));
    }
}

pub fn show_game(
    game_ui: Res<GameUI>,
    query: Query<&mut Style>,
    query_2d: Query<&mut Visibility>,
)
{
    game_ui.show(query, query_2d);
}

pub fn hide_game(
    game_ui: Option<Res<GameUI>>,
    query: Query<&mut Style>,
    query_2d: Query<&mut Visibility>,
)
{
    if let Some(game_ui) = game_ui {
        game_ui.hide(query, query_2d);
    }
}

pub fn show_menu(
    game_ui: Res<GameUI>,
    query: Query<&mut Style>,
)
{
    game_ui.show_menu(query);
}

pub fn hide_menu(
    game_ui: Res<GameUI>,
    query: Query<&mut Style>,
)
{
    game_ui.hide_menu(query);
}

pub fn input_menu(
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<
        (&Interaction, &mut UiColor, &OverlayButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut input: ResMut<Input<KeyCode>>,
)
{
    if input.clear_just_released(KeyCode::Escape) {
        game_state.set(GameState::Active).unwrap_or_else(|e| warn!("{e:?}"));
        return;
    }

    for (interaction, mut color, btn) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = BTN_PRESSED.into();

                match btn.target {
                    OverlayButtons::Close => {
                        game_state.set(GameState::Active).unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    OverlayButtons::Settings => {
                        ui_state.set(UiState::Settings).unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    OverlayButtons::Save => {
                        ui_state.set(UiState::Save).unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    OverlayButtons::Load => {
                        ui_state.set(UiState::Load).unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    OverlayButtons::MainMenu => {
                        game_state.set(GameState::None).unwrap_or_else(|e| warn!("{e:?}"));
                        ui_state.set(UiState::MainMenu).unwrap_or_else(|e| warn!("{e:?}"));
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

pub fn enable_game_input(mut state: ResMut<State<GameControlState>>)
{
    state.pop().unwrap_or_else(|e| warn!("{e:?}"));
}

pub fn disable_game_input(mut state: ResMut<State<GameControlState>>)
{
    state.push(GameControlState::None).unwrap_or_else(|e| warn!("{e:?}"));
}
