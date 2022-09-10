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

struct DGBStates {
    pub timer: Timer,
}

fn dbg_states(
    time: Res<Time>,
    mut dbg: ResMut<DGBStates>,
    mut main_state: ResMut<State<crate::startup::MainState>>,
    mut ui_state: ResMut<State<crate::ui::UiState>>,
    mut game_state: ResMut<State<crate::game::GameState>>,
    mut control_state: ResMut<State<crate::game::GameControlState>>,
)
{
    if dbg.timer.tick(time.delta()).just_finished() {
        info!(
            "MainState::{:?}, UiState::{:?}, GameState::{:?}, GameControlState::{:?}",
            main_state.current(), ui_state.current(), game_state.current(), control_state.current()
        )
    }
}
