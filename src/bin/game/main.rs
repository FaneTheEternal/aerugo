#![feature(unboxed_closures)]

mod states;
mod main_menu;
mod overlay;
mod utils;
mod game;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(overlay::OverlayPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut command: Commands) {
    command.spawn_bundle(UiCameraBundle::default());
    command.spawn_bundle(OrthographicCameraBundle::new_2d());
}