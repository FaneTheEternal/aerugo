use std::path::PathBuf;

use bevy::prelude::*;
use aerugo::{Aerugo, BackgroundCommand, NarratorCommand, SceneCommand, SpriteCommand, Steps};

use crate::utils::{CachedAssetServer, SIZE_ALL};

use super::*;

const SPLASH_FONT: &str = "fonts/Orbitron Black.ttf";

pub fn spawn_splash_screen(
    mut commands: Commands,
    mut asset_server: CachedAssetServer,
)
{
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: SIZE_ALL,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Aerugo",
                        TextStyle {
                            font: asset_server.load(SPLASH_FONT),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                });
        })
        .id();

    commands.insert_resource(SplashScreen {
        timer: Timer::from_seconds(1.0, false),
        root,
    });

    // simple preload big picture
    let _ = asset_server.load::<Image>("logo.png");
}

pub fn update_splash_screen<const S: MainState>(
    asset_server: CachedAssetServer,
    mut main_state: ResMut<State<MainState>>,
    mut splash_screen: ResMut<SplashScreen>,
    time: Res<Time>,
)
{
    splash_screen.timer.tick(time.delta());
    if asset_server.all_loaded() & splash_screen.timer.finished() {
        main_state.set(S)
            .unwrap_or_else(|e| warn!("{e:?}"));
        splash_screen.timer.reset();
    }
}

pub fn game_splash_screen(
    mut commands: Commands,
    mut asset_server: CachedAssetServer,
    splash_screen: Res<SplashScreen>,
)
{
    commands.entity(splash_screen.root).despawn_recursive();
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: SIZE_ALL,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            image: asset_server.load("logo.png").into(),
            ..default()
        })
        .id();
    commands.insert_resource(SplashScreen {
        timer: Timer::from_seconds(1.0, false),
        root,
    });
}

pub fn remove_splash_screen(
    mut commands: Commands,
    mut ui_state: ResMut<State<UiState>>,
    splash_screen: Res<SplashScreen>,
)
{
    commands.entity(splash_screen.root).despawn_recursive();
    ui_state.set(UiState::MainMenu)
        .unwrap_or_else(|e| warn!("{e:?}"));
}

pub fn load(
    mut commands: Commands,
)
{
    let aerugo = crate::utils::load_aerugo();
    aerugo.validate().expect("Invalid aerugo scenario");
    let saves = pre_load_saves(&aerugo);
    commands.insert_resource(aerugo);
    commands.insert_resource(saves);
}

pub fn preload_assets(
    mut asset_server: CachedAssetServer,
    aerugo: Res<Aerugo>,
)
{
    for step in &aerugo.steps {
        match &step.inner {
            Steps::ImageSelect { .. } => {}
            Steps::SpriteNarrator(cmd) => {
                match cmd {
                    NarratorCommand::Set { name, sprite } => {
                        let _ = asset_server.load_untyped(sprite);
                    }
                    _ => {}
                }
            }
            Steps::Sprite(cmd) => {
                match cmd {
                    SpriteCommand::Set { sprite, .. } => {
                        let _ = asset_server.load_untyped(sprite);
                    }
                    SpriteCommand::FadeIn { sprite, .. } => {
                        let _ = asset_server.load_untyped(sprite);
                    }
                    SpriteCommand::LeftIn { sprite, .. } => {
                        let _ = asset_server.load_untyped(sprite);
                    }
                    SpriteCommand::RightIn { sprite, .. } => {
                        let _ = asset_server.load_untyped(sprite);
                    }
                    _ => {}
                }
            }
            Steps::Background(cmd) => {
                match cmd {
                    BackgroundCommand::Change { new, .. } => {
                        let _ = asset_server.load_untyped(new);
                    }
                    _ => {}
                }
            }
            Steps::Scene(cmd) => {
                match cmd {
                    SceneCommand::Set { name } => {
                        let _ = asset_server.load_untyped(name);
                    }
                    SceneCommand::Play { name, .. } => {
                        let _ = asset_server.load_untyped(name);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
