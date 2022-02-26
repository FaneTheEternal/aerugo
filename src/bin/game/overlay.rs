#![allow(unused_imports)]

mod components;
mod systems;
mod saves_ui;

use bevy::prelude::*;

use crate::states::OverlayState;
use systems::*;
use components::*;
use crate::utils::should_run_once;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CleanseSavesEvent>()
            .add_event::<RespawnSavesEvent>()
            .add_state(OverlayState::Hidden)
            .add_system(cleanse_saves_listener)
            .add_system(respawn_saves_listener)
            .add_system_set(
                SystemSet::new().with_run_criteria(should_run_once)
                    .with_system(init_overlay)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Hidden)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Hidden)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Hidden)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Menu)
                    .with_system(show_menu)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Menu)
                    .with_system(overlay_break)
                    .with_system(overlay_menu)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Menu)
                    .with_system(hide_menu)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Settings)
                    .with_system(show_settings)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Settings)
                    .with_system(overlay_break)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Settings)
                    .with_system(hide_settings)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Save)
                    .with_system(show_save)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Save)
                    .with_system(overlay_break)
                    .with_system(save_buttons)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Save)
                    .with_system(hide_save)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Load)
                    .with_system(show_load)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Load)
                    .with_system(overlay_break)
                    .with_system(load_buttons)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Load)
                    .with_system(hide_load)
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OverlayData {
    ui_menu: Entity,
    ui_settings: Entity,
    ui_save: Entity,
    ui_load: Entity,
}

pub struct CleanseSavesEvent;

pub struct RespawnSavesEvent;