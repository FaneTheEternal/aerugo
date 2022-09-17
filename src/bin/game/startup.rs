mod systems;
mod spawn;
mod spawn_game;
mod spawn_main_menu;
pub mod spawn_game_menu;
pub mod save_load;

use bevy::asset::{Asset, AssetPath};
use bevy::prelude::*;
use bevy::utils::HashMap;
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
                    .with_system(preload_assets)
                    .with_system(load.after(preload_assets))
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

pub struct PreloadedAssets {
    pub assets: HashMap<String, HandleUntyped>,
}

impl PreloadedAssets {
    #[must_use = "not using the returned strong handle may result in the unexpected release of the asset"]
    pub fn load<T: Asset>(&self, path: &str) -> Handle<T> {
        let asset = self.assets.get(path.into())
            .expect(&format!("Asset not found: {:?}", path));
        asset.clone().typed()
    }
}
