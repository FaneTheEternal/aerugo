use bevy::prelude::*;

use crate::utils::*;

use super::*;
use super::spawn_game::*;

pub fn spawn(
    mut commands: Commands,
    mut asset_server: CachedAssetServer,
    saves: Res<Saves>,
    window: Res<Windows>,
)
{
    let main_menu = spawn_main_menu::spawn(&mut commands, &mut asset_server);
    commands.insert_resource(MainMenuUI { entity_root: main_menu });

    let notice = NoticeUI::spawn(&mut commands, &mut asset_server);
    commands.insert_resource(notice);

    let save = save_load::spawn_save(
        &mut commands, &mut asset_server, saves.as_ref(),
    );
    commands.insert_resource(save);

    let game = spawn_game(&mut commands, &mut asset_server, window.as_ref());
    commands.insert_resource(game);

    let game_menu = spawn_game_menu::spawn(&mut commands, &mut asset_server);
    commands.insert_resource(GameMenuUI { root: game_menu });
}
