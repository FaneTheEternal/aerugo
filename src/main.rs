#![feature(unboxed_closures)]

mod states;
mod main_menu;
mod overlay;
mod utils;
mod game;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(overlay::OverlayPlugin)
        .run();
}
