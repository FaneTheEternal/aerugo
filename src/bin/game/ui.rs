use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use game::*;
pub use load::*;
pub use main_menu::*;
pub use save::*;

use crate::game::GameState;
use crate::saves::{LoadMark, SaveMark, Saves};
use crate::saves_ui::{LoadItemsParentMark, make_load_items, make_save_items, SaveItemsParentMark};

mod main_menu;
mod save;
mod game;
mod load;

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
                    .with_system(save_show)
            )
            .add_event::<CleanseSavesEvent>()
            .add_event::<RespawnSavesEvent>()
            .add_system_set(
                SystemSet::on_update(UiState::Save)
                    .with_system(generic_break)
                    .with_system(save_actions)
                    .with_system(cleanse_saves_listener)
                    .with_system(respawn_saves_listener)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Save)
                    .with_system(save_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Load)
                    .with_system(load_show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Load)
                    .with_system(generic_break)
                    .with_system(load_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Load)
                    .with_system(load_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Game)
                    .with_system(game_show)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Game)
                    .with_system(game_hide)
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
}


pub fn generic_break(
    mut ui_state: ResMut<State<UiState>>,
    game_state: Res<State<GameState>>,
    mut input: ResMut<Input<KeyCode>>,
)
{
    if input.clear_just_released(KeyCode::Escape) {
        if game_state.current().eq(&GameState::None) {
            ui_state.set(UiState::MainMenu).unwrap_or_else(|e| warn!("{e:?}"));
        } else {
            ui_state.set(UiState::Game).unwrap_or_else(|e| warn!("{e:?}"));
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

