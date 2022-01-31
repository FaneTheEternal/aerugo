use bevy::app::Events;
use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::game::components::{GameButton, GameButtons, SpriteMark};
use super::GameData;
use crate::states::OverlayState;

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
    let text_font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let button_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    let window = window.get_primary().unwrap();
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
