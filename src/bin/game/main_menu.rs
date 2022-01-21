mod systems;
mod components;

use bevy::prelude::*;

use crate::states::MainState;
use systems::*;
use components::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(MainState::MainMenu)
            .add_system_set(
                SystemSet::on_enter(MainState::MainMenu)
                    .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_update(MainState::MainMenu)
                    .with_system(menu)
            )
            .add_system_set(
                SystemSet::on_exit(MainState::MainMenu)
                    .with_system(cleanup_menu)
            );
    }
}

pub struct MainMenuData {
    ui_entity: Entity,
}
