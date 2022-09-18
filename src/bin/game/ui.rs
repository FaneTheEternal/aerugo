use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::tracing::span;
use bevy::log::Level;

pub use game::*;
pub use main_menu::*;
pub use pause::*;
pub use save_load::*;

use crate::game::GameState;

mod main_menu;
mod game;
mod pause;
mod save_load;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(UiState::None)
            .add_system_set(
                SystemSet::on_enter(UiState::MainMenu)
                    .with_system(main_menu_show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::MainMenu)
                    .with_system(main_menu_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::MainMenu)
                    .with_system(main_menu_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Settings)
                    .with_system(settings_show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Settings)
                    .with_system(generic_break)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Settings)
                    .with_system(settings_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Save)
                    .with_system(save_load::save_show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Save)
                    .with_system(generic_break)
                    .with_system(save_page_actions)
                    .with_system(new_page)
                    .with_system(save_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Save)
                    .with_system(save_load::save_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Load)
                    .with_system(save_load::save_show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Load)
                    .with_system(generic_break)
                    .with_system(save_page_actions)
                    .with_system(new_page)
                    .with_system(load_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Load)
                    .with_system(save_load::save_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Game)
                    .with_system(game_show)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Game)
                    .with_system(game_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Pause)
                    .with_system(show_game_menu)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Pause)
                    .with_system(game_menu_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Pause)
                    .with_system(hide_game_menu)
            )
        ;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UiState {
    None,
    MainMenu,
    Settings,
    Save,
    Load,
    Game,
    Pause,
}


pub fn generic_break(
    mut ui_state: ResMut<State<UiState>>,
    game_state: Res<State<GameState>>,
    mut input: ResMut<Input<KeyCode>>,
)
{
    let span = span!(Level::WARN, "generic_break");
    let _ = span.enter();

    if input.clear_just_released(KeyCode::Escape) {
        if game_state.current().eq(&GameState::None) {
            ui_state.set(UiState::MainMenu).unwrap_or_else(|e| warn!("{e:?}"));
        } else {
            ui_state.set(UiState::Pause).unwrap_or_else(|e| warn!("{e:?}"));
        }
    }
}

pub struct SettingsUI {
    pub(crate) entity_root: Entity,
}

impl SettingsUI {
    pub fn show(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::Flex;
    }

    pub fn hide(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::None;
    }
}

pub fn settings_show(settings: Res<SettingsUI>, query: Query<&mut Style>) {
    settings.show(query);
}

pub fn settings_hide(settings: Res<SettingsUI>, query: Query<&mut Style>) {
    settings.hide(query);
}

