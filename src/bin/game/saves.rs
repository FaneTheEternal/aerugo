#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::io::Write;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use aerugo::*;

use crate::game::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(preload)
            .add_system(save.exclusive_system());
    }
}

#[derive(Clone, Debug)]
pub struct SaveMark {
    pub(crate) to: u8,
}

pub struct Save;

pub struct Saves {
    pub saves: HashMap<u8, Save>,
}

pub fn preload(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{}

pub fn save(
    world: &mut World,
) {
    let mut save_mark = world.remove_resource::<SaveMark>();
    if let Some(save_mark) = save_mark {
        let game_state = world.get_resource::<GameState>().unwrap();
        let sprites = world.get_resource::<SpriteEntities>().unwrap();
        let mut save_world = World::new();

        let type_registry = world.get_resource::<TypeRegistry>().unwrap();
        let scene = DynamicScene::from_world(&world, type_registry);

        let data = scene.serialize_ron(type_registry).unwrap();
        _save(format!("save{}.scn.ron", save_mark.to), data);
    }
}

fn _save(save_path: String, data: String) {
    let save_path = std::path::Path::new(save_path.as_str());
    let mut save_file = std::fs::File::options()
        .write(true).create(true).truncate(true)
        .open(save_path)
        .unwrap();

    save_file
        .write_all(data.as_bytes())
        .unwrap();
}

pub fn load() {}
