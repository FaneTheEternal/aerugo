mod resources;

extern crate core;

use std::fs::File;
use std::io::{Read, Write};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use uuid::Uuid;
use aerugo::*;

use resources::*;

const SCENARIO_PATH: &str = "scenario.json";

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Aerugo editor".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_event::<SaveEvent>()
        .add_system(save_hotkey)
        .add_system(save)
        .run();
}

fn setup(mut command: Commands)
{
    let mut file = File::options()
        .read(true).write(true).create(true)
        .open(SCENARIO_PATH)
        .unwrap();
    let mut aerugo = String::new();
    file.read_to_string(&mut aerugo).unwrap();
    let aerugo: Aerugo = serde_json::from_str(&aerugo)
        .or_else::<serde_json::Error, _>(|_| { Ok(Aerugo::default()) })
        .unwrap();

    todo!("Insert aerugo");
}

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
        let data = serde_json::to_string(&app_data.aerugo).unwrap();
        let save_path = std::path::Path::new(app_data.file.as_str());
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
