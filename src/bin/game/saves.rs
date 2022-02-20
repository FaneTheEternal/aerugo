use std::collections::HashMap;
use std::io::Write;
use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(save.exclusive_system());
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct SaveMark {
    pub(crate) to: u8,
}

pub struct Save;

pub struct Saves {
    pub saves: HashMap<u8, Save>,
}

pub fn save(
    world: &mut World,
) {
    let save_mark = world.remove_resource::<SaveMark>();
    if let Some(_save_mark) = save_mark {
        todo!("Save")
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
