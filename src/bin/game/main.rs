#![feature(unboxed_closures)]

mod utils;
mod game;
mod saves;
mod startup;
mod ui;
mod saves_ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(startup::StartupPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(saves::SavePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut command: Commands) {
    command.spawn_bundle(Camera2dBundle::default());
}
