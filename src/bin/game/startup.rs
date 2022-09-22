use bevy::prelude::*;
use bevy::utils::HashMap;

use spawn::*;
use systems::*;

use crate::saves::{pre_load_saves, Saves};
use crate::ui::*;

mod systems;
mod spawn;
mod spawn_game;
mod spawn_main_menu;
pub mod spawn_game_menu;
pub mod save_load;

pub struct StartupPlugin;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MainState {
    Init,
    Load,
    Spawn,
    Ready,
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(MainState::Init)
            .init_resource::<AssetCache>()
            .add_system_set(
                SystemSet::on_enter(MainState::Init)
                    .with_system(spawn_splash_screen)
            )
            .add_system_set(
                SystemSet::on_update(MainState::Init)
                    .with_system(update_splash_screen::<{ MainState::Load }>)
            )
            .add_system_set(
                SystemSet::on_enter(MainState::Load)
                    .with_system(game_splash_screen)
                    .with_system(load)
            )
            .add_system_set(
                SystemSet::on_update(MainState::Load)
                    .with_system(update_splash_screen::<{ MainState::Spawn }>)
            )
            .add_system_set(
                SystemSet::on_enter(MainState::Spawn)
                    .with_system(spawn)
                    .with_system(preload_assets)
            )
            .add_system_set(
                SystemSet::on_update(MainState::Spawn)
                    .with_system(update_splash_screen::<{ MainState::Ready }>)
            )
            .add_system_set(
                SystemSet::on_enter(MainState::Ready)
                    .with_system(remove_splash_screen)
            )
        ;
    }
}

#[derive(Debug, Default)]
pub struct AssetCache {
    pub assets: HashMap<String, HandleUntyped>,
}

pub struct SplashScreen {
    timer: Timer,
    root: Entity,
}
