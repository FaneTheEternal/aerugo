use std::io::Read;
use bevy::app::Events;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use bevy::window::WindowResized;

use aerugo::*;

use crate::game::components::{GameButton, GameButtons, SpriteMark};
use super::GameData;
use crate::states::OverlayState;

const BTN_NORMAL: Color = Color::WHITE;
const BTN_HOVERED: Color = Color::GRAY;
const BTN_PRESSED: Color = Color::DARK_GRAY;

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

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
    mut command: Commands,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
    game_data: Res<GameData>,
)
{
    let text_font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let button_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    let window = window.get_primary().unwrap();
    let w = window.width();
    let h = window.height();

    let game_state = AerugoState::setup(&game_data.aerugo);
    command.insert_resource(game_state);

    let text_narrator_entity = command
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(w * 0.99, h * 0.09)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, h * -0.25, 10.0),
            ..Default::default()
        })
        .id();
    let text_background_entity = command
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(w * 0.99, h * 0.19)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, h * -0.4, 10.0),
            ..Default::default()
        })
        .id();
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
