#![feature(unboxed_closures)]
#![feature(adt_const_params)]
#![cfg_attr(release_max, windows_subsystem = "windows")]


use bevy::prelude::*;

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
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 1280.0,
                    height: 720.0,
                    title: include_str!("game_name").to_string(),
                    resizable: false,
                    ..Default::default()
                },
                ..default()
            }))
        .add_plugin(startup::StartupPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(saves::SavePlugin)
        // .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(dbg_states)
        .run();
}

fn setup(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
    // command.insert_resource(DGBStates {
    //     timer: Timer::new(
    //         std::time::Duration::from_secs(5),
    //         TimerMode::Repeating,
    //     )
    // })
}

#[allow(dead_code)]
#[derive(Debug, Resource)]
struct DGBStates {
    pub timer: Timer,
}

fn dbg_states(
    time: Res<Time>,
    dbg: Option<ResMut<DGBStates>>,
    main_state: Res<State<crate::startup::MainState>>,
    ui_state: Res<State<crate::ui::UiState>>,
    game_state: Res<State<crate::game::GameState>>,
    control_state: Res<State<crate::game::GameControlState>>,
)
{
    if let Some(mut dbg) = dbg {
        if dbg.timer.tick(time.delta()).just_finished() {
            info!(
            "MainState::{:?}, UiState::{:?}, GameState::{:?}, GameControlState::{:?}",
            main_state.current(), ui_state.current(), game_state.current(), control_state.current()
        )
        }
    }
}
