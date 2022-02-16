mod resources;

extern crate core;

use std::fs::File;
use std::io::{Read, Write};
use bevy::prelude::*;
use aerugo::*;

use resources::*;

const SCENARIO_PATH: &str = "scenario.ron";

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Aerugo editor".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup)
        .add_event::<SaveEvent>()
        .add_system(ui)
        .add_system(save_hotkey)
        .add_system(save)
        .run();
}

pub struct SaveEvent;

fn setup(mut command: Commands)
{
    let mut file = File::options()
        .read(true).write(true).create(true)
        .open(SCENARIO_PATH)
        .unwrap();
    let mut aerugo = String::new();
    file.read_to_string(&mut aerugo).unwrap();
    let aerugo: Aerugo = ron::from_str(&aerugo)
        .or_else::<ron::Error, _>(|_| { Ok(Aerugo::default()) })
        .unwrap();

    command.insert_resource(AppData { aerugo });
}

fn ui() {}

fn save_hotkey(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut save_event: EventWriter<SaveEvent>,
)
{
    if keyboard_input.pressed(KeyCode::LControl)
        && keyboard_input.clear_just_released(KeyCode::S) {
        save_event.send(SaveEvent);
    }
}

fn save(app_data: Res<AppData>, mut events: EventReader<SaveEvent>) {
    for _ in events.iter() {
        let data = ron::ser::to_string_pretty(&app_data.aerugo, Default::default()).unwrap();
        let save_path = std::path::Path::new(SCENARIO_PATH);
        let mut save_file = File::options()
            .write(true).create(true).truncate(true)
            .open(save_path)
            .unwrap();

        save_file
            .write_all(data.as_bytes())
            .unwrap();

        println!("Scenario saved successfully to {}", save_path.to_str().unwrap());
    }
}
