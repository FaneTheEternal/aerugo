use std::io::Read;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use uuid::Uuid;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use aerugo::*;

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

#[derive(EnumIter, Debug, Clone, Default, Eq, PartialEq)]
enum LightInner {
    Text,
    Jump,
    Phrase,
    ImageSelect,
    SpriteNarrator,
    Sprite,
    Background,
    Scene,
    #[default]
    None,
}

impl From<Steps> for LightInner {
    fn from(steps: Steps) -> Self {
        match steps {
            Steps::Text { .. } => { LightInner::Text }
            Steps::Jump { .. } => { LightInner::Jump }
            Steps::Phrase { .. } => { LightInner::Phrase }
            Steps::ImageSelect { .. } => { LightInner::ImageSelect }
            Steps::SpriteNarrator { .. } => { LightInner::SpriteNarrator }
            Steps::Sprite(_) => { LightInner::Sprite }
            Steps::Background(_) => { LightInner::Background }
            Steps::Scene(_) => { LightInner::Scene }
            Steps::None => { LightInner::None }
        }
    }
}

impl Into<Steps> for LightInner {
    fn into(self) -> Steps {
        match self {
            LightInner::Text => {
                Steps::Text { author: "".to_string(), texts: "".to_string() }
            }
            LightInner::Jump => {
                Steps::Jump { condition: None, target: Default::default() }
            }
            LightInner::Phrase => {
                Steps::Phrase { phrases: vec![] }
            }
            LightInner::ImageSelect => {
                Steps::ImageSelect { background: "".to_string(), options: Default::default() }
            }
            LightInner::SpriteNarrator => {
                Steps::SpriteNarrator { sprite: None }
            }
            LightInner::Sprite => {
                Steps::Sprite(SpriteCommand::None)
            }
            LightInner::Background => {
                Steps::Background(BackgroundCommand::None)
            }
            LightInner::Scene => {
                Steps::Scene(SceneCommand::None)
            }
            LightInner::None => {
                Steps::None
            }
        }
    }
}

fn step_widget(ui: &mut egui::Ui, step: &mut Step, targets: &Vec<(Uuid, String)>) {
    let current_step: LightInner = step.inner.clone().into();
    let mut new_step = current_step.clone();

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
                egui::ComboBox::from_label("Type")
                    .selected_text(format!("{:?}", new_step))
                    .show_ui(ui, |ui| {
                        for kind in LightInner::iter() {
                            ui.selectable_value(
                                &mut new_step,
                                kind.clone(),
                                format!("{:?}", kind),
                            );
                        }
                    });
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
                    Steps::SpriteNarrator { sprite } => {
                        ui.heading("SpriteNarrator");
                        let mut temp = sprite.clone().unwrap_or_default();
                        ui.text_edit_singleline(&mut temp);
                        if !temp.is_empty() {
                            sprite.replace(temp);
                        }
                    }
                    Steps::Sprite(cmd) => {
                        ui.heading("Sprite");
                        match cmd {
                            SpriteCommand::None => {
                                ui.label("None");
                            }
                            SpriteCommand::Set { sprite, name, position } => {
                                ui.label("Set");
                                horizontal_text(ui, "Sprite:", sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::Remove { name } => {
                                ui.label("Remove");
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::FadeIn { sprite, name, position } => {
                                ui.label("FadeIn");
                                horizontal_text(ui, "Sprite:", sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::FadeOut { name } => {
                                ui.label("FadeOut");
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::LeftIn { sprite, name, position } => {
                                ui.label("LeftIn");
                                horizontal_text(ui, "Sprite:", sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::LeftOut { name } => {
                                ui.label("LeftOut");
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::RightIn { sprite, name, position } => {
                                ui.label("RightIn");
                                horizontal_text(ui, "Sprite:", sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::RightOut { name } => {
                                ui.label("RightOut");
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::Move { name, position } => {
                                ui.label("Move");
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                        }
                    }
                    Steps::Background(cmd) => {
                        ui.heading("Background");
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
                        match cmd {
                            SceneCommand::Set { name } => {
                                ui.label("Set");
                                horizontal_text(ui, "Name:", name);
                            }
                            SceneCommand::Remove => {
                                ui.label("Remove");
                            }
                            SceneCommand::Play { name, is_loop, tile, columns, rows } => {
                                ui.label("Play");
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
                            SceneCommand::Pause => {
                                ui.label("Pause");
                            }
                            SceneCommand::Resume => {
                                ui.label("Resume");
                            }
                            SceneCommand::Stop => {
                                ui.label("Stop");
                            }
                            SceneCommand::None => {}
                        }
                    }
                    Steps::None => {}
                }
                ui.label(format!("DBG: {:?}", step.inner));
            },
        );
    if new_step != current_step {
        step.inner = new_step.into();
    }
}
