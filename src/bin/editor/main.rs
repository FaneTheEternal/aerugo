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
        .add_startup_system(configure_visuals)
        .add_startup_system(setup)
        .add_system(update_ui_scale_factor)
        .add_system(ui)
        .add_event::<SaveEvent>()
        .add_system(save_hotkey)
        .add_system(save)
        .run();
}

fn configure_visuals(egui_ctx: ResMut<EguiContext>)
{
    let ctx = egui_ctx.ctx();

    ctx.set_visuals(egui::Visuals {
        ..Default::default()
    });

    let mut fonts = egui::FontDefinitions::default();
    fonts.family_and_size.insert(
        egui::TextStyle::Small,
        (egui::FontFamily::Proportional, 20.0));
    fonts.family_and_size.insert(
        egui::TextStyle::Body,
        (egui::FontFamily::Proportional, 20.0));
    fonts.family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, 20.0));
    fonts.family_and_size.insert(
        egui::TextStyle::Heading,
        (egui::FontFamily::Proportional, 40.0));
    ctx.set_fonts(fonts);
}

fn update_ui_scale_factor(
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
)
{
    if toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Some(window) = windows.get_primary() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.0
            } else {
                1.0 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
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
    let first_step = aerugo.steps.iter().next().unwrap().clone();

    command.insert_resource(AppData {
        file: SCENARIO_PATH.to_string(),
        aerugo,
    });
    fill_default_steps(&mut command, &first_step.inner);
    command.insert_resource(AppState::from_step(first_step));
}

fn fill_default_steps(commands: &mut Commands, init: &Steps) {
    commands.insert_resource(TextStep::default());
    commands.insert_resource(JumpStep::default());
    commands.insert_resource(PhraseStep::default());
    commands.insert_resource(ImageSelectStep::default());
    commands.insert_resource(SpriteNarratorStep::default());
    commands.insert_resource(SpriteStep::default());
    commands.insert_resource(BackgroundStep::default());
    commands.insert_resource(SceneStep::default());
    match init {
        Steps::Text { author, texts } => {
            commands.insert_resource(TextStep { author: author.clone(), texts: texts.clone() })
        }
        Steps::Jump { condition, target } => {
            commands.insert_resource(JumpStep { condition: condition.clone().into(), target: target.clone() })
        }
        Steps::Phrase { phrases } => {
            commands.insert_resource(PhraseStep { phrases: phrases.clone() })
        }
        Steps::ImageSelect { background, options } => {
            commands.insert_resource(ImageSelectStep {
                background: background.clone(),
                options: options.clone(),
            })
        }
        Steps::SpriteNarrator { sprite } => {
            commands.insert_resource(SpriteNarratorStep {
                sprite: sprite.clone().or_else(|| Some("".to_string())).unwrap()
            })
        }
        Steps::Sprite { name, sprite, animation } => {
            commands.insert_resource(SpriteStep {
                name: name.clone(),
                sprite: sprite.clone(),
                animation: animation.clone(),
            })
        }
        Steps::Background { command } => {
            commands.insert_resource(BackgroundStep { command: command.clone() })
        }
        Steps::Scene { command } => {
            commands.insert_resource(SceneStep { command: command.clone() })
        }
        Steps::None => {}
    }
}

fn ui(
    mut command: Commands,
    egui_ctx: Res<EguiContext>,
    mut app_data: ResMut<AppData>,
    mut app_state: ResMut<AppState>,
    mut save_event: EventWriter<SaveEvent>,
    mut text_step: ResMut<TextStep>,
    mut jump_step: ResMut<JumpStep>,
    mut phrase_step: ResMut<PhraseStep>,
    mut image_select_step: ResMut<ImageSelectStep>,
    mut sprite_narrator_step: ResMut<SpriteNarratorStep>,
    mut sprite_step: ResMut<SpriteStep>,
    mut background_step: ResMut<BackgroundStep>,
    mut scene_step: ResMut<SceneStep>,
)
{
    let ctx = egui_ctx.ctx();

    let mut add_new_step = false;
    let mut select_another = None;

    egui::TopBottomPanel::top("top_panel")
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Save").clicked() {
                    save_event.send(SaveEvent);
                }
            });
        });

    egui::SidePanel::left("steps_list")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Steps list");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for step in &app_data.aerugo.steps {
                    let step_label = if step.name.is_empty() {
                        format!("{}", step.id)
                    } else {
                        format!("{}\n~{}", step.id, step.name)
                    };
                    if ui.button(step_label).clicked() {
                        select_another = Some(step.id);
                    }
                    ui.separator();
                }
                if ui.add(egui::Button::new("+")).clicked() {
                    add_new_step = true;
                }
            });
        });

    egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        ui.heading("Scenario edit");
        ui.horizontal(|ui| {
            ui.label(format!("ID: {}", app_state.current.id));
            ui.separator();
            ui.label("Name: ");
            ui.text_edit_singleline(&mut app_state.current.name);
            if !app_state.current.id.is_nil() {
                ui.separator();
                if ui.button("Remove").clicked() {
                    app_state.current = app_data.aerugo.remove(&app_state.current).clone();
                }
            }
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Text").clicked() {
                app_state.current.inner = Steps::Text { author: "".to_string(), texts: "".to_string() };
            }
            if ui.button("Jump").clicked() {
                app_state.current.inner = Steps::Jump { condition: None, target: Uuid::nil() };
            }
            if ui.button("Phrase").clicked() {
                app_state.current.inner = Steps::Phrase { phrases: vec![] };
            }
            if ui.button("ImageSelect").clicked() {
                app_state.current.inner = Steps::ImageSelect { background: Default::default(), options: Default::default() };
            }
            if ui.button("SpriteNarrator").clicked() {
                app_state.current.inner = Steps::SpriteNarrator { sprite: None };
            }
            if ui.button("Sprite").clicked() {
                app_state.current.inner = Steps::Sprite {
                    name: Default::default(),
                    sprite: Default::default(),
                    animation: CommonAnimation::None,
                };
            }
            if ui.button("Background").clicked() {
                app_state.current.inner = Steps::Background { command: BackgroundCommand::None };
            }
            if ui.button("Scene").clicked() {
                app_state.current.inner = Steps::Scene { command: SceneCommand::None };
            }
            if ui.button("None").clicked() {
                app_state.current.inner = Steps::None;
            }
        });
        ui.separator();
        match &app_state.current.inner {
            Steps::Text { .. } => {
                ui.heading("Text");
                ui.label("Author");
                ui.text_edit_singleline(&mut text_step.author);
                ui.label("Text");
                ui.text_edit_multiline(&mut text_step.texts);
            }
            Steps::Jump { .. } => {
                ui.heading("Jump");
                ui.label("Target");
                egui::ComboBox::from_label("Select target")
                    .selected_text(format!("{:?}", jump_step.target))
                    .show_ui(ui, |ui| {
                        for step in &app_data.aerugo.steps {
                            ui.selectable_value(
                                &mut jump_step.target,
                                step.id,
                                format!("{:?}", step),
                            );
                        }
                    });
                ui.separator();
                ui.heading("Condition");
                ui.horizontal(|ui| {
                    if jump_step.condition.is_valid() {
                        ui.label(egui::RichText::new("Valid").color(egui::Color32::GREEN));
                    } else {
                        ui.label(egui::RichText::new("Invalid").color(egui::Color32::RED));
                        ui.small("will be reset");
                    }
                });
                ui.text_edit_multiline(&mut jump_step.condition.0);
            }
            Steps::Phrase { .. } => {
                ui.heading("Phrase");
            }
            Steps::ImageSelect { .. } => {
                ui.heading("ImageSelect");
            }
            Steps::SpriteNarrator { .. } => {
                ui.heading("SpriteNarrator");
                ui.label("Sprite");
                ui.text_edit_singleline(&mut sprite_narrator_step.sprite);
            }
            Steps::Sprite { .. } => {
                ui.heading("Sprite");
                ui.label("Name");
                ui.text_edit_singleline(&mut sprite_step.name);
                ui.label("Sprite");
                ui.text_edit_singleline(&mut sprite_step.sprite);
            }
            Steps::Background { .. } => {
                ui.heading("Background");
            }
            Steps::Scene { .. } => {
                ui.heading("Scene");
            }
            Steps::None => {
                ui.heading("None");
            }
        }
    });

    if add_new_step {
        app_data.aerugo.steps.push(Step::new());
    }

    if let Some(id) = select_another {
        let last = app_data.aerugo.steps.iter()
            .position(|s| { s.id == app_state.current.id })
            .unwrap();
        app_data.aerugo.steps[last] = match &app_state.current.inner {
            Steps::Text { .. } => { app_state.current.clone().with_inner(text_step.as_origin()) }
            Steps::Jump { .. } => { app_state.current.clone().with_inner(jump_step.as_origin()) }
            Steps::Phrase { .. } => { app_state.current.clone().with_inner(phrase_step.as_origin()) }
            Steps::ImageSelect { .. } => { app_state.current.clone().with_inner(image_select_step.as_origin()) }
            Steps::SpriteNarrator { .. } => { app_state.current.clone().with_inner(sprite_narrator_step.as_origin()) }
            Steps::Sprite { .. } => { app_state.current.clone().with_inner(sprite_step.as_origin()) }
            Steps::Background { .. } => { app_state.current.clone().with_inner(background_step.as_origin()) }
            Steps::Scene { .. } => { app_state.current.clone().with_inner(scene_step.as_origin()) }
            Steps::None => { app_state.current.clone() }
        };
        let current = app_data.aerugo.steps
            .iter()
            .find(|s| { s.id == id })
            .unwrap()
            .clone();
        fill_default_steps(&mut command, &current.inner);
        command.insert_resource(
            AppState::from_step(current)
        );
    }
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
