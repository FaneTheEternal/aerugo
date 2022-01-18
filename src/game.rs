#![allow(unused_imports)]

mod components;
mod systems;

use bevy::prelude::*;

use crate::states::MainState;
use systems::*;
use components::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(MainState::InGame)
                    .with_system(setup_game)
            )
            .add_system_set(
                SystemSet::on_update(MainState::InGame)
                    .with_system(open_overlay)
                    .with_system(game_buttons)
            )
            .add_system_set(
                SystemSet::on_exit(MainState::InGame)
                    .with_system(cleanup)
            );
    }
}

pub struct GameData {
    ui_entity: Entity,
}
