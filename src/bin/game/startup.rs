mod systems;
mod spawn;
mod spawn_game;
mod spawn_main_menu;
pub mod spawn_game_menu;
pub mod save_load;

use bevy::prelude::*;
use crate::saves::{pre_load_saves, Saves};
use crate::saves_ui::{LoadItemsParentMark, SaveItemsParentMark};
use crate::ui::*;

use systems::*;
use spawn::*;


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
            .add_system_set(
                SystemSet::on_update(MainState::Init)
                    .with_system(spawn_splash_screen)
            )
            .add_system_set(
                SystemSet::on_update(MainState::Load)
                    .with_system(load)
            )
            .add_system_set(
                SystemSet::on_update(MainState::Spawn)
                    .with_system(spawn)
            )
            .add_system_set(
                SystemSet::on_enter(MainState::Ready)
                    .with_system(remove_splash_screen)
            )
        ;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct SplashScreen;
