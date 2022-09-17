use std::path::PathBuf;
use bevy::prelude::*;
use crate::utils::SIZE_ALL;
use super::*;

pub fn spawn_splash_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<MainState>>,
)
{
    commands
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
        .insert(SplashScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Aerugo",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                });
        });
    state.set(MainState::Load).unwrap_or_else(|e| warn!("{e:?}"));
}

pub fn remove_splash_screen(
    mut commands: Commands,
    query: Query<Entity, With<SplashScreen>>,
    mut ui_state: ResMut<State<UiState>>,
)
{
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    ui_state.set(UiState::MainMenu).unwrap_or_else(|e| warn!("{e:?}"));
}

pub fn load(
    mut commands: Commands,
    mut state: ResMut<State<MainState>>,
)
{
    let aerugo = crate::utils::load_aerugo();
    let saves = pre_load_saves(&aerugo);
    commands.insert_resource(aerugo);
    commands.insert_resource(saves);
    state.set(MainState::Spawn).unwrap_or_else(|e| warn!("{e:?}"));
}

pub fn preload_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
    // TODO: universal solution
    let current_dir = std::env::current_dir().unwrap();
    let assets_dir = current_dir.join("assets");
    let mut assets: HashMap<String, HandleUntyped> = default();

    fn _load(assets: &mut HashMap<String, HandleUntyped>, path: &PathBuf, base: &PathBuf, asset_server: &AssetServer) {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let metadata = std::fs::metadata(&path).unwrap();

            if metadata.is_file() {
                let asset_path = path.strip_prefix(base).unwrap();
                let asset = asset_server.load_untyped(asset_path);
                let asset_path = asset_path.to_string_lossy().to_string();
                let same = asset_path.replace(r"\", r"/");
                assets.insert(asset_path, asset.clone());
                assets.insert(same, asset);
            } else {
                _load(assets, &path, base, asset_server);
            }
        }
    }
    _load(&mut assets, &assets_dir, &assets_dir, asset_server.as_ref());
    commands.insert_resource(PreloadedAssets { assets });
}
