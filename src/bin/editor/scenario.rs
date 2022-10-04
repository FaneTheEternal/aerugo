use bevy::prelude::*;
use bevy_egui::egui;
use crate::*;

pub fn edit_scenario(
    commands: &mut Commands,
    egui_ctx: &egui::Context,
    aerugo: &mut Aerugo,
)
{
    let mut targets = Vec::new();
    let mut narrator_names = Vec::new();
    let mut narrator_sprites = Vec::new();
    for step in &aerugo.steps {
        targets.push((step.id.clone(), step.name.clone()));
        match &step.inner {
            Steps::Text { author, .. } => {
                if !narrator_names.contains(author) {
                    narrator_names.push(author.clone());
                }
            }
            Steps::SpriteNarrator(cmd) => {
                match &cmd {
                    NarratorCommand::Set { name, .. } => {
                        if !narrator_sprites.contains(name) {
                            narrator_sprites.push(name.clone());
                        }
                    }
                    _ => {}
                }
            }
            Steps::Sprite(_) => {}
            Steps::None => {}
            _ => {}
        }
    }
    egui::CentralPanel::default().show(
        egui_ctx,
        |mut ui| {
            egui::ScrollArea::vertical().auto_shrink([false, true]).show(
                &mut ui,
                |mut ui| {
                    let mut insert = None;
                    let mut delete = None;
                    let mut mc = None;
                    let mut sc = None;
                    for (i, step) in aerugo.steps.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            if ui.button("INSERT").clicked() {
                                insert = Some(i);
                            }
                            if ui.button("DELETE").clicked() {
                                delete = Some(i);
                            }
                            if ui.button("PRE MC").clicked() {
                                mc = Some((i, true));
                            }
                            if ui.button("POST MC").clicked() {
                                mc = Some((i, false));
                            }
                            if ui.button("PRE SC").clicked() {
                                sc = Some((i, true));
                            }
                            if ui.button("POST SC").clicked() {
                                sc = Some((i, false));
                            }
                        });
                        step_widget(ui, step, &targets, &narrator_names, &narrator_sprites);
                    }
                    ui.horizontal(|ui| {
                        if ui.button("+").clicked() {
                            aerugo.steps.push(Step::new_text());
                        }
                        if ui.button("+MC").clicked() {
                            aerugo.steps.push(Step::new().with_inner(
                                Steps::SpriteNarrator(NarratorCommand::Set {
                                    name: "".to_string(),
                                    sprite: "textures\\main_char\\default.png".to_string(),
                                })
                            ));
                            aerugo.steps.push(Step::new().with_inner(
                                Steps::Text { author: "Ты".to_string(), texts: "".to_string() }
                            ));
                            aerugo.steps.push(Step::new().with_inner(
                                Steps::SpriteNarrator(NarratorCommand::Clean)
                            ));
                        }
                    });
                    if let Some(insert) = insert {
                        aerugo.steps.insert(insert, Step::new_text());
                    }
                    if let Some(delete) = delete {
                        aerugo.steps.remove(delete);
                    }
                    ui.allocate_space(egui::Vec2::new(100.0, 500.0));
                    if let Some(mc) = mc {
                        let mut step = Step::new();
                        if mc.1 {
                            step.inner = Steps::SpriteNarrator(NarratorCommand::Set {
                                name: "".to_string(),
                                sprite: "textures\\main_char\\default.png".to_string(),
                            });
                        } else {
                            step.inner = Steps::SpriteNarrator(NarratorCommand::Clean);
                        }
                        aerugo.steps.insert(
                            mc.0 + if mc.1 { 0 } else { 1 },
                            step,
                        )
                    }
                    if let Some(sc) = sc {
                        let mut step = Step::new();
                        if sc.1 {
                            step.inner = Steps::SpriteNarrator(NarratorCommand::Set {
                                name: "second".to_string(),
                                sprite: "textures\\char\\Медив.png".to_string(),
                            });
                        } else {
                            step.inner = Steps::SpriteNarrator(NarratorCommand::Clean);
                        }
                        aerugo.steps.insert(
                            sc.0 + if sc.1 { 0 } else { 1 },
                            step,
                        )
                    }
                },
            );
        },
    );
}

fn step_widget(
    ui: &mut egui::Ui,
    step: &mut Step,
    targets: &Vec<(Uuid, String)>,
    narrator_names: &Vec<String>,
    narrator_sprites: &Vec<String>,
)
{
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
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_label("Author:").width(200.0)
                                .show_ui(ui, |ui| {
                                    for narrator in narrator_names {
                                        ui.selectable_value(
                                            author,
                                            narrator.clone(),
                                            narrator,
                                        );
                                    }
                                });
                            ui.text_edit_singleline(author);
                        });
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
                                file_pick(ui, sprite);
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
                                file_pick(ui, sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::Remove { name } => {
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::FadeIn { sprite, name, position } => {
                                horizontal_text(ui, "Sprite:", sprite);
                                file_pick(ui, sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::FadeOut { name } => {
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::LeftIn { sprite, name, position } => {
                                horizontal_text(ui, "Sprite:", sprite);
                                file_pick(ui, sprite);
                                horizontal_text(ui, "Name:", name);
                                ui.add(egui::DragValue::new(position).speed(0.1));
                            }
                            SpriteCommand::LeftOut { name } => {
                                horizontal_text(ui, "Name:", name);
                            }
                            SpriteCommand::RightIn { sprite, name, position } => {
                                horizontal_text(ui, "Sprite:", sprite);
                                file_pick(ui, sprite);
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
                                file_pick(ui, new);
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
                                file_pick(ui, name);
                            }
                            SceneCommand::Remove => {}
                            SceneCommand::Play { name, is_loop, tile, columns, rows } => {
                                horizontal_text(ui, "Name:", name);
                                file_pick(ui, name);
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