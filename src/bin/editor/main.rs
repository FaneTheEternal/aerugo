mod light;

use std::fmt::Debug;
use std::io::{Read, Write};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use uuid::Uuid;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use aerugo::*;
use crate::light::{BackgroundLight, LightInner, NarratorLight, SceneLight, SpriteLight};

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
}

fn save(aerugo: &Aerugo) {
    let aerugo = ron::ser::to_string_pretty(aerugo, default())
        .unwrap();
    std::fs::File::options()
        .write(true).truncate(true)
        .open(SCENARIO_PATH)
        .unwrap()
        .write(aerugo.as_bytes())
        .unwrap();
}

fn ui_system(
    mut commands: Commands,
    mut e_ctx: ResMut<EguiContext>,
    mut aerugo: ResMut<Aerugo>,
)
{
    let targets = aerugo.steps.iter()
        .map(|step| {
            (step.id.clone(), step.name.clone())
        })
        .collect::<Vec<_>>();

    egui::TopBottomPanel::top("my_panel")
        .show(e_ctx.ctx_mut(), |ui| {
            if ui.button("Save").clicked() {
                save(aerugo.as_ref());
            }
        });

    egui::CentralPanel::default().show(
        e_ctx.ctx_mut(),
        |mut ui| {
            egui::ScrollArea::vertical().auto_shrink([false, true]).show(
                &mut ui,
                |mut ui| {
                    let mut insert = None;
                    let mut delete = None;
                    for (i, step) in aerugo.steps.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            if ui.button("INSERT").clicked() {
                                insert = Some(i);
                            }
                            if ui.button("DELETE").clicked() {
                                delete = Some(i);
                            }
                        });
                        step_widget(ui, step, &targets);
                    }
                    if let Some(insert) = insert {
                        aerugo.steps.insert(insert, Step::new());
                    }
                    if ui.button("+").clicked() {
                        aerugo.steps.push(Step::new());
                    }
                    if let Some(delete) = delete {
                        aerugo.steps.remove(delete);
                    }
                },
            );
        },
    );
}

fn horizontal_text(ui: &mut egui::Ui, label: &str, v: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(v);
    });
}


pub fn light_edit<O, L>(ui: &mut egui::Ui, origin: &mut O, label: &str)
    where
        O: Clone,
        L: Clone + From<O> + Into<O> + IntoEnumIterator + Debug + Eq
{
    let current: L = origin.clone().into();
    let mut new: L = current.clone();
    egui::ComboBox::from_label(label)
        .selected_text(format!("{:?}", new))
        .show_ui(ui, |ui| {
            for option in L::iter() {
                ui.selectable_value(
                    &mut new,
                    option.clone(),
                    format!("{:?}", option),
                );
            }
        });
    if current != new {
        *origin = new.into();
    }
}

fn step_widget(ui: &mut egui::Ui, step: &mut Step, targets: &Vec<(Uuid, String)>) {
    egui::CollapsingHeader::new(format!("{} - {:?}", step.id, step.name))
        .default_open(true)
        .show(
            ui,
            |ui| {
                ui.horizontal(|ui| {
                    if ui.button("🆔").clicked() {
                        ui.output().copied_text = format!("{}", step.id);
                    }
                    ui.text_edit_singleline(&mut step.name);
                });
                light_edit::<_, LightInner>(ui, &mut step.inner, "Type");
                match &mut step.inner {
                    Steps::Text { author, texts } => {
                        ui.heading("Text");
                        horizontal_text(ui, "Author:", author);
                        ui.horizontal(|ui| {
                            ui.label("Text:");
                            ui.text_edit_multiline(texts);
                        });
                    }
                    Steps::Jump { condition, target } => {
                        ui.heading("Jump");
                        egui::ComboBox::from_label("Target")
                            .selected_text(format!("{}", target))
                            .show_ui(ui, |ui| {
                                for (option, verbose) in targets {
                                    ui.selectable_value(
                                        target,
                                        *option,
                                        format!("{} - {}", option, verbose),
                                    );
                                }
                            });
                        ui.label(format!("Condition: {:?}", condition));
                    }
                    Steps::Phrase { phrases } => {
                        ui.heading("Phrase");
                        ui.horizontal(|ui| {
                            ui.label("Phrase");
                            ui.separator();
                            ui.label("Verbose");
                        });
                        for (phrase, verbose) in phrases.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(phrase);
                                ui.separator();
                                ui.text_edit_singleline(verbose);
                            });
                        }
                        if ui.button("+").clicked() {
                            phrases.push(default());
                        }
                    }
                    Steps::ImageSelect { .. } => {}
                    Steps::SpriteNarrator(cmd) => {
                        ui.heading("SpriteNarrator");
                        light_edit::<_, NarratorLight>(ui, cmd, "Kind");
                        match cmd {
                            NarratorCommand::Set { name, sprite } => {
                                horizontal_text(ui, "Name: ", name);
                                horizontal_text(ui, "Sprite: ", sprite);
                            }
                            NarratorCommand::Remove { name } => {
                                horizontal_text(ui, "Name: ", name);
                            }
                            NarratorCommand::Clean => {}
                            NarratorCommand::None => {}
                        }
                    }
                    Steps::Sprite(cmd) => {
                        ui.heading("Sprite");
                        light_edit::<_, SpriteLight>(ui, cmd, "Kind");
                        match cmd {
                            SpriteCommand::None => {}
                            SpriteCommand::Set { sprite, name, position } => {
                                horizontal_text(ui, "Sprite:", sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::Remove { name } => {
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::FadeIn { sprite, name, position } => {
                                horizontal_text(ui, "Sprite:", sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::FadeOut { name } => {
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::LeftIn { sprite, name, position } => {
                                horizontal_text(ui, "Sprite:", sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::LeftOut { name } => {
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::RightIn { sprite, name, position } => {
                                horizontal_text(ui, "Sprite:", sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::RightOut { name } => {
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::Move { name, position } => {
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                        }
                    }
                    Steps::Background(cmd) => {
                        ui.heading("Background");
                        light_edit::<_, BackgroundLight>(ui, cmd, "Kind");
                        match cmd {
                            BackgroundCommand::Change { new, animation } => {
                                horizontal_text(ui, "Name:", new);
                                ui.label(format!("Animation: {:?}", animation));  // TODO
                            }
                            BackgroundCommand::Shake => {}
                            BackgroundCommand::None => {}
                        }
                    }
                    Steps::Scene(cmd) => {
                        ui.heading("Scene");
                        light_edit::<_, SceneLight>(ui, cmd, "Kind");
                        match cmd {
                            SceneCommand::Set { name } => {
                                horizontal_text(ui, "Name:", name);
                            }
                            SceneCommand::Remove => {}
                            SceneCommand::Play { name, is_loop, tile, columns, rows } => {
                                horizontal_text(ui, "Name:", name);
                                ui.checkbox(is_loop, "Is loop");
                                ui.horizontal(|ui| {
                                    ui.label("Tile:");
                                    ui.add(egui::DragValue::new(&mut tile.0));
                                    ui.add(egui::DragValue::new(&mut tile.1));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Columns:");
                                    ui.add(egui::DragValue::new(columns));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Rows:");
                                    ui.add(egui::DragValue::new(rows));
                                });
                            }
                            SceneCommand::Pause => {}
                            SceneCommand::Resume => {}
                            SceneCommand::Stop => {}
                            SceneCommand::None => {}
                        }
                    }
                    Steps::None => {}
                }
                ui.label(format!("DBG: {:?}", step.inner));
            },
        );
}
