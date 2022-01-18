#![allow(unused_imports)]

mod components;
mod systems;

use bevy::prelude::*;

use crate::states::OverlayState;
use systems::*;
use components::*;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(OverlayState::Hidden)
            .add_system_set(
                SystemSet::on_enter(OverlayState::Hidden)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Hidden)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Hidden)
                    .with_system(cleanup)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Menu)
                    .with_system(setup_overlay)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Menu)
                    .with_system(overlay)
                    .with_system(overlay_break)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Menu)
                    .with_system(cleanup)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Settings)
                    .with_system(setup_settings)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Settings)
                    .with_system(overlay_break)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Settings)
                    .with_system(cleanup)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Save)
                    .with_system(setup_save)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Save)
                    .with_system(overlay_break)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Save)
                    .with_system(cleanup)
            )
            .add_system_set(
                SystemSet::on_enter(OverlayState::Load)
                    .with_system(setup_load)
            )
            .add_system_set(
                SystemSet::on_update(OverlayState::Load)
                    .with_system(overlay_break)
            )
            .add_system_set(
                SystemSet::on_exit(OverlayState::Load)
                    .with_system(cleanup)
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OverlayData {
    ui_entity: Entity,
}
