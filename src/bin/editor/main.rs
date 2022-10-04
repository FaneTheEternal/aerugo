mod light;
mod scenario;
mod international;

use std::fmt::Debug;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use uuid::Uuid;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use aerugo::*;
use aerugo::international::{ImanityLangs, Internationale};
use crate::egui::Ui;
use crate::light::{BackgroundLight, LightInner, NarratorLight, SceneLight, SpriteLight};
use scenario::*;
use international::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(ui_system)
        .run();
}

const SCENARIO_PATH: &str = "scenario.ron";

fn setup(
    mut commands: Commands,
)
{
    let aerugo = {
        let mut file = std::fs::File::options()
            .read(true).write(true).create(true)
            .open(SCENARIO_PATH)
            .unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        ron::from_str::<Aerugo>(&buff).unwrap_or_default()
    };
    // TODO: save edit state
    let state = AerugoState::new(&aerugo);
    commands.insert_resource(aerugo);
    commands.insert_resource(state);
    commands.insert_resource(Internationale::load());
}

fn save(aerugo: &Aerugo, internationale: &Internationale) {
    let aerugo = ron::ser::to_string_pretty(aerugo, default())
        .unwrap();
    std::fs::File::options()
        .write(true).truncate(true)
        .open(SCENARIO_PATH)
        .unwrap()
        .write(aerugo.as_bytes())
        .unwrap();

    for local in &internationale.defs {
        let name = std::env::current_dir().unwrap()
            .join("assets")
            .join("lang")
            .join(format!("{:?}.imanity", local.lang).to_lowercase());
        let local = ron::ser::to_string_pretty(local, default()).unwrap();
        std::fs::write(name, local.as_bytes()).unwrap();
    }
}

#[derive(Debug, Default, Clone)]
enum EditorMode {
    #[default]
    Scenario,
    International,
}

fn ui_system(
    mut commands: Commands,
    mut e_ctx: ResMut<EguiContext>,
    mut aerugo: ResMut<Aerugo>,
    mut mode: Local<EditorMode>,
    mut internationale: ResMut<Internationale>,
    mut lang: Local<ImanityLangs>,
)
{
    egui::TopBottomPanel::top("my_panel")
        .show(e_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    save(aerugo.as_ref(), internationale.as_ref());
                }
                if ui.button("Scenario").clicked() {
                    *mode = EditorMode::Scenario;
                }
                if ui.button("International").clicked() {
                    *mode = EditorMode::International;
                }
            });
        });

    match mode.deref() {
        EditorMode::Scenario => {
            edit_scenario(&mut commands, e_ctx.ctx_mut(), &mut aerugo);
        }
        EditorMode::International => {
            edit_international(
                &mut commands,
                e_ctx.ctx_mut(),
                &mut aerugo,
                &mut internationale,
                lang.deref_mut()
            );
        }
    }
}


pub fn horizontal_text(ui: &mut egui::Ui, label: &str, v: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(v);
    });
}


fn file_pick(ui: &mut Ui, target: &mut String) {
    if ui.button("FILE").clicked() {
        let path = std::env::current_dir().unwrap().join("assets");

        let res = rfd::FileDialog::new()
            .add_filter("image", &["png"])
            .set_directory(&path)
            .pick_files();
        if let Some(file) = res {
            let file = file.first().unwrap();
            let file = file.strip_prefix(path);
            if let Ok(file) = file {
                *target = file.to_string_lossy().to_string();
            }
        }
    }
}
