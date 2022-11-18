use std::collections::HashMap;
use std::io::{Read, Write};

use bevy::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use aerugo::{Aerugo, AerugoState};

use crate::game::GameState;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(save)
            .add_system(load);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Resource)]
pub struct SaveMark {
    pub(crate) to: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Resource)]
pub struct LoadMark(pub usize);

#[derive(Debug, Resource)]
pub struct AerugoLoaded(pub AerugoState);

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Save {
    pub state: AerugoState,
    pub timestamp: DateTime<chrono::Local>,
}

#[derive(Default, Debug, Resource)]
pub struct Saves {
    pub saves: HashMap<usize, Save>,
}

pub fn pre_load_saves(aerugo: &Aerugo) -> Saves
{
    let mut saves: HashMap<usize, Save> = Default::default();
    let path = std::env::current_dir().unwrap()
        .join("saves");
    std::fs::create_dir_all(&path).unwrap();
    for n in 0..200 {
        let save_name = format!("save{n}.ron");
        let save_path = path.join(&save_name);
        if let Ok(mut save) = std::fs::File::open(save_path) {
            let mut save_data = String::new();
            save.read_to_string(&mut save_data).unwrap();
            if let Ok(save) = ron::from_str::<Save>(&save_data) {
                if AerugoState::validate(&aerugo, &save.state).is_some() {
                    saves.insert(n, save);
                }
            }
        }
    }
    Saves { saves }
}

pub fn save(world: &mut World) {
    let save_mark = world.remove_resource::<SaveMark>();
    if let Some(save_mark) = save_mark {
        let aerugo_state = world.get_resource::<AerugoState>().unwrap().clone();
        let save = Save { state: aerugo_state, timestamp: chrono::Local::now() };
        _save(
            format!("save{}.ron", save_mark.to),
            ron::ser::to_string_pretty(&save, Default::default()).unwrap(),
        );
        world.get_resource_mut::<Saves>()
            .and_then(|mut s| {
                s.saves.insert(save_mark.to, save)
            });
    }
}

fn _save(save_path: String, data: String) {
    let path = std::env::current_dir().unwrap()
        .join("saves");
    let save_path = path.join(&save_path);
    let mut save_file = std::fs::File::options()
        .write(true).create(true).truncate(true)
        .open(save_path)
        .unwrap();

    save_file
        .write_all(data.as_bytes())
        .unwrap();
}

pub fn load(world: &mut World)
{
    if let Some(mark) = world.remove_resource::<LoadMark>() {
        if let Some(save) = world.remove_resource::<Saves>() {
            if let Some(save) = save.saves.get(&mark.0) {
                world.insert_resource(AerugoLoaded(save.state.clone().reload()));
                world.get_resource_mut::<State<GameState>>()
                    .and_then::<(), _>(|mut s| {
                        s.set(GameState::Init)
                            .unwrap_or_else(|e| { warn!("{e:?}") });
                        None
                    });
            }
            world.insert_resource(save);
        }
    }
}
