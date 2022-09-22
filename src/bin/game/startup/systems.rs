use std::path::PathBuf;

use bevy::prelude::*;

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
)
{
    // TODO: universal solution
    let current_dir = std::env::current_dir().unwrap();
    let assets_dir = current_dir.join("assets");
    let mut assets: HashMap<String, HandleUntyped> = default();

    fn _load(
        assets: &mut HashMap<String, HandleUntyped>,
        path: &PathBuf,
        base: &PathBuf,
        asset_server: &mut CachedAssetServer,
    )
    {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let metadata = std::fs::metadata(&path).unwrap();

            if metadata.is_file() {
                let asset_path = path.strip_prefix(base).unwrap();
                let asset_path = asset_path.to_string_lossy().to_string();
                let _ = asset_server.load_untyped(&asset_path);
            } else {
                _load(assets, &path, base, asset_server);
            }
        }
    }
    _load(&mut assets, &assets_dir, &assets_dir, &mut asset_server);
}
