#![feature(unboxed_closures)]
#![feature(adt_const_params)]

// #![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;

mod utils;
mod game;
mod saves;
mod startup;
mod ui;
mod saves_ui;
mod settings;
pub mod translator;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: include_str!("game_name").to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(startup::StartupPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(saves::SavePlugin)
        // .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_startup_system(setup)
        // .add_system(dbg_states)
        .run();
}

fn setup(mut command: Commands) {
    command.spawn_bundle(Camera2dBundle::default());
    // command.insert_resource(DGBStates {
    //     timer: Timer::new(
    //         std::time::Duration::from_secs(5),
    //         true
    //     )
    // })
}

#[allow(dead_code)]
struct DGBStates {
    pub timer: Timer,
}

#[allow(dead_code)]
fn dbg_states(
    time: Res<Time>,
    mut dbg: ResMut<DGBStates>,
    main_state: Res<State<crate::startup::MainState>>,
    ui_state: Res<State<crate::ui::UiState>>,
    game_state: Res<State<crate::game::GameState>>,
    control_state: Res<State<crate::game::GameControlState>>,
)
{
    if dbg.timer.tick(time.delta()).just_finished() {
        info!(
            "MainState::{:?}, UiState::{:?}, GameState::{:?}, GameControlState::{:?}",
            main_state.current(), ui_state.current(), game_state.current(), control_state.current()
        )
    }
}
