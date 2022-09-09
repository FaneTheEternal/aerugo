use std::io::Read;
use substring::Substring;
use bevy::{
    prelude::*,
};
use bevy::ecs::schedule::IntoRunCriteria;
use bevy::log::Level;
use bevy::utils::tracing::span;

use aerugo::*;
use aerugo::bevy_glue::GameMenuButtons;
use crate::saves::AerugoLoaded;

use super::*;
use crate::ui::{GameUI, UiState};
use crate::utils::{BTN_HOVERED, BTN_NORMAL, BTN_PRESSED, load_aerugo, SIZE_ALL, TRANSPARENT, Y_SPRITE, Z_SPRITE};

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
    mut atlas_query: Query<&mut Handle<TextureAtlas>>,
    mut game_control_state: ResMut<State<GameControlState>>,
)
{
    let span = span!(Level::WARN, "setup_game");
    let _enter = span.enter();

    let aerugo_state = aerugo_loaded
        .map(|loaded| { loaded.0.to_owned() })
        .unwrap_or_else(|| { AerugoState::setup(aerugo.as_ref()) });
    commands.remove_resource::<AerugoLoaded>();

    commands.insert_resource(aerugo_state);
    commands.insert_resource(JustInit);

    next_step_event.send(NextStepEvent);
    game_state.set(GameState::Active)
        .unwrap_or_else(|e| warn!("{e:?}"));

    game_ui.sprites.values().for_each(|&e| {
        commands.entity(e).despawn_recursive();
    });
    game_ui.background_visible = false;
    game_ui.scene_visible = false;
    *image_query.get_mut(game_ui.background).unwrap() = Default::default();
    *image_query.get_mut(game_ui.scene).unwrap() = Default::default();
    *atlas_query.get_mut(game_ui.scene).unwrap() = Default::default();
    visibility_query.get_mut(game_ui.background).unwrap().is_visible = false;
    visibility_query.get_mut(game_ui.scene).unwrap().is_visible = false;
    game_ui.sprites = Default::default();
    game_ui.text.clean_narrators(
        &mut style_query,
        &mut ui_image_query,
    );
    commands.entity(game_ui.scene).remove::<AnimateScene>();
    if !game_control_state.current().eq(&GameControlState::None) {
        game_control_state.set(GameControlState::None)
            .unwrap_or_else(|e| warn!("{:?}", e));
    }
}

pub fn open_overlay(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
)
{
    let span = span!(Level::WARN, "open_overlay");
    let _enter = span.enter();

    if input.clear_just_released(KeyCode::Escape) {
        game_state.set(GameState::Paused)
            .unwrap_or_else(|e| warn!("{e:?}"));
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
                Steps::SpriteNarrator(cmd) => {
                    new_narrator_event.send(NewNarratorEvent(cmd));
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
        let cmd: &NarratorCommand = &event.0;

        match cmd {
            NarratorCommand::Set { name, sprite } => {
                game_ui.text.set_narrator(
                    &mut style_query,
                    &mut image_query,
                    name,
                    Some(sprite.clone()),
                    asset_server.as_ref(),
                );
            }
            NarratorCommand::Remove { name } => {
                game_ui.text.set_narrator(
                    &mut style_query,
                    &mut image_query,
                    name,
                    None,
                    asset_server.as_ref(),
                );
            }
            NarratorCommand::Clean => {
                game_ui.text.clean_narrators(
                    &mut style_query,
                    &mut image_query,
                )
            }
            NarratorCommand::None => {}
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
    mut commands: Commands,
    mut game_ui: ResMut<GameUI>,
    mut new_scene_event: EventReader<NewSceneEvent>,
    mut scene_query: Query<(&mut Handle<Image>, &mut Handle<TextureAtlas>, &mut Visibility)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animate_query: Query<&mut AnimateScene>,
)
{
    for event in new_scene_event.iter() {
        let cmd: &SceneCommand = &event.0;
        let (mut scene, mut texture_atlas, mut visibility): (Mut<Handle<Image>>, Mut<Handle<TextureAtlas>>, Mut<Visibility>) =
            scene_query.get_mut(game_ui.scene).unwrap();
        match cmd {
            SceneCommand::Set { name } => {
                *scene = asset_server.load(name);
                visibility.is_visible = true;
                game_ui.scene_visible = true;
            }
            SceneCommand::Remove => {
                *scene = Default::default();
                *texture_atlas = default();
                visibility.is_visible = false;
                game_ui.scene_visible = false;
                commands.entity(game_ui.scene).remove::<AnimateScene>();
            }
            SceneCommand::Play { name, is_loop, tile, columns, rows } => {
                let texture_handle = asset_server.load(name);
                let new_texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(tile.0 as f32, tile.1 as f32),
                    *columns, *rows,
                );
                *texture_atlas = texture_atlases.add(new_texture_atlas);
                visibility.is_visible = true;
                game_ui.scene_visible = true;
                commands
                    .entity(game_ui.scene)
                    .insert(AnimateScene {
                        timer: Timer::from_seconds(0.042, true),
                        is_loop: *is_loop,
                        is_paused: false,
                    });
            }
            SceneCommand::Pause => {
                if let Ok(mut animate) = animate_query.get_mut(game_ui.scene) {
                    animate.is_paused = true;
                }
            }
            SceneCommand::Resume => {
                if let Ok(mut animate) = animate_query.get_mut(game_ui.scene) {
                    animate.is_paused = false;
                }
            }
            SceneCommand::Stop => {
                if let Ok(mut animate) = animate_query.get_mut(game_ui.scene) {
                    animate.is_loop = false;
                }
            }
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
    let span = span!(Level::WARN, "step_init");
    let _enter = span.enter();

    if let Some(step) = step {
        let text_font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

        // game_ui.text.force_hide(&mut style_query);
        game_ui.phrase.force_hide(&mut style_query);

        match &step.inner {
            Steps::Text { author, texts } => {
                game_ui.text.force_show(&mut style_query);

                if author.is_empty() {
                    style_query.get_mut(game_ui.text.narrator_base).unwrap()
                        .display = Display::None;
                } else {
                    style_query.get_mut(game_ui.text.narrator_base).unwrap()
                        .display = Display::Flex;
                    commands
                        .entity(game_ui.text.narrator)
                        .insert(Text::from_section(
                            author.as_str(),
                            TextStyle {
                                font: text_font.clone(),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        ));
                }
                commands
                    .entity(game_ui.text.text)
                    .insert(AnimateText {
                        text: texts.clone(),
                        timer: Timer::from_seconds(0.1, true),
                        style: TextStyle {
                            font: text_font.clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                        chars: 0,
                    });
                game_control_state.overwrite_set(GameControlState::TextPass)
                    .unwrap_or_else(|e| warn!("{e:?}"));
            }
            Steps::Phrase { phrases } => {
                game_ui.phrase.force_show(&mut style_query);

                let phrase_options: Vec<Entity> = phrases.iter()
                    .map(|o| {
                        let (key, verbose) = o;
                        commands
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Percent(40.0),
                                        Val::Auto,
                                    ),
                                    margin: UiRect::all(Val::Percent(1.0)),
                                    ..default()
                                },
                                image: asset_server.load("hud/game_option.png").into(),
                                ..default()
                            })
                            // .insert(PhraseValue(key.clone()))
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(ButtonBundle {
                                        style: Style {
                                            size: SIZE_ALL,
                                            padding: UiRect::all(Val::Px(10.0)),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .insert(PhraseValue(key.clone()))
                                    .with_children(|parent| {
                                        parent.spawn_bundle(TextBundle {
                                            text: Text::from_section(
                                                verbose.as_str(),
                                                TextStyle {
                                                    font: text_font.clone(),
                                                    font_size: 30.0,
                                                    color: Color::BLACK,
                                                },
                                            ).with_alignment(TextAlignment {
                                                vertical: VerticalAlign::Center,
                                                horizontal: HorizontalAlign::Center,
                                            }),
                                            ..Default::default()
                                        });
                                    });
                            })
                            .id()
                    })
                    .collect();
                commands.entity(game_ui.phrase.root).despawn_descendants();
                commands.entity(game_ui.phrase.root).push_children(phrase_options.as_slice());

                game_control_state.overwrite_set(GameControlState::Phrase)
                    .unwrap_or_else(|e| warn!("{e:?}"));
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
    let span = span!(Level::WARN, "input_text_pass");
    let _enter = span.enter();

    if key_input.clear_just_pressed(KeyCode::Space)
        || key_input.clear_just_pressed(KeyCode::Return)
        || mouse_button_input.clear_just_pressed(MouseButton::Left) {
        game_control_state.set(GameControlState::Text)
            .unwrap_or_else(|e| warn!("{e:?}"));
    }
}

pub fn input_text_next(
    mut game_control_state: ResMut<State<GameControlState>>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    mut next_step_event: EventWriter<NextStepEvent>,
)
{
    let span = span!(Level::WARN, "input_text_next");
    let _enter = span.enter();

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
    let span = span!(Level::WARN, "input_phrase");
    let _enter = span.enter();

    for (interaction, phrase, color) in phrase_query.iter_mut() {
        let interaction: &Interaction = interaction;
        let phrase: &PhraseValue = phrase;
        let mut color: Mut<UiColor> = color;
        match interaction {
            Interaction::Clicked => {
                *color = TRANSPARENT.into();

                let step = aerugo_state.step(aerugo.as_ref());
                aerugo_state.select_unique(step.id, phrase.0.clone());
                game_control_state.set(GameControlState::None)
                    .unwrap_or_else(|e| warn!("{e:?}"));
                next_step_event.send(NextStepEvent);
            }
            Interaction::Hovered => {
                *color = Color::rgba(1.0, 1.0, 1.0, 0.1).into();
            }
            Interaction::None => {
                *color = TRANSPARENT.into();
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
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut scene_query: Query<(
        Entity,
        &mut AnimateScene,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
)
{
    let span = span!(Level::WARN, "animate");
    let _enter = span.enter();

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

    for (entity, animate, sprite, texture) in scene_query.iter_mut() {
        let mut animate: Mut<AnimateScene> = animate;
        let mut sprite: Mut<TextureAtlasSprite> = sprite;
        let texture: &Handle<TextureAtlas> = texture;

        if animate.is_paused { continue; }
        animate.timer.tick(time.delta());
        if animate.timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture).unwrap();
            if !animate.is_loop && sprite.index + 1 == texture_atlas.textures.len() {
                commands.entity(entity).remove::<AnimateScene>();
            } else {
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
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
    mut commands: Commands,
    state: Res<AerugoState>,
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<
        (&Interaction, &mut UiColor, &GameMenuButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut input: ResMut<Input<KeyCode>>,
)
{
    let span = span!(Level::WARN, "input_menu");
    let _enter = span.enter();

    if input.clear_just_released(KeyCode::Escape) {
        game_state.set(GameState::Init)
            .unwrap_or_else(|e| warn!("{e:?}"));
        commands.insert_resource(AerugoLoaded(state.clone().reload()));
        return;
    }

    for (interaction, mut color, btn) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = TRANSPARENT.into();

                match btn {
                    GameMenuButtons::Continue => {
                        game_state.set(GameState::Init)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                        commands.insert_resource(AerugoLoaded(state.clone().reload()));
                    }
                    GameMenuButtons::Load => {
                        ui_state.set(UiState::Load)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    GameMenuButtons::Save => {
                        ui_state.set(UiState::Save)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    GameMenuButtons::Gallery => {}
                    GameMenuButtons::Settings => {}
                    GameMenuButtons::MainMenu => {
                        game_state.set(GameState::None)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                        ui_state.set(UiState::MainMenu)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
            }
            Interaction::None => {
                *color = TRANSPARENT.into();
            }
        }
    }
}

pub fn enable_game_input(mut state: ResMut<State<GameControlState>>)
{
    let span = span!(Level::WARN, "enable_game_input");
    let _enter = span.enter();

    state.pop().unwrap_or_else(|e| warn!("{e:?}"));
}

pub fn disable_game_input(mut state: ResMut<State<GameControlState>>)
{
    state.push(GameControlState::None).unwrap_or_else(|e| warn!("{e:?}"));
}
