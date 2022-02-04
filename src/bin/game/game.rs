#![allow(unused_imports)]

mod components;
mod systems;

use bevy::prelude::*;

use aerugo::*;

use crate::states::MainState;
use systems::*;
use components::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(MuteControl::Mute)
            .add_startup_system(preload_aerugo)
            .add_system_set(
                SystemSet::on_enter(MainState::InGame)
                    .with_system(setup_game)
            )
            .add_system_set(
                SystemSet::on_update(MainState::InGame)
                    .with_system(open_overlay)
            )
            .add_system_set(
                SystemSet::on_exit(MainState::InGame)
            );
    }
}

pub struct GameData {
    pub aerugo: Aerugo,
}

pub struct GameState {
    pub aerugo_state: AerugoState,

    pub text_narrator_entity: Entity,
    pub text_background_entity: Entity,

    pub phrase_ui_entity: Entity,

    pub narrator_entity: Entity,

    pub background_entity: Entity,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MuteControl {
    Mute,
    None,
}
