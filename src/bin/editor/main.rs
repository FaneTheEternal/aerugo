extern crate core;

use std::fs::File;
use std::io::{Read, Write};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use uuid::Uuid;
use aerugo::*;

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
        .add_system(ui)
        .add_system(save)
        .run();
}

struct AppData {
    file: String,
    aerugo: Aerugo,
}

struct AppState {
    current: Step,
    author: String,
    text: String,
    condition: Option<Condition>,
    target: Uuid,
}

impl AppState {
    fn from_step(step: Step) -> AppState {
        match step.inner.clone() {
            Steps::Text { author, texts } => {
                AppState {
                    current: step,
                    author: author,
                    text: texts,
                    condition: None,
                    target: Uuid::nil(),
                }
            }
            Steps::Jump { target, condition } => {
                AppState {
                    current: step,
                    author: Default::default(),
                    text: Default::default(),
                    condition: condition,
                    target: target,
                }
            }
            Steps::None => {
                AppState {
                    current: step,
                    author: Default::default(),
                    text: Default::default(),
                    condition: Default::default(),
                    target: Uuid::nil(),
                }
            }
        }
    }
}

fn configure_visuals(egui_ctx: ResMut<EguiContext>)
{
    egui_ctx.ctx().set_visuals(egui::Visuals {
        ..Default::default()
    });
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
    command.insert_resource(AppState::from_step(first_step));
    println!(
        "{}",
        serde_json::to_string(
            &Condition::GTE(
                vec![Condition::True, Condition::False],
                10
            )
        ).unwrap()
    );
}

fn ui(
    mut command: Commands,
    egui_ctx: Res<EguiContext>,
    mut app_data: ResMut<AppData>,
    mut app_state: ResMut<AppState>,
)
{
    let mut add_new_step = false;
    let mut select_another = None;

    egui::SidePanel::left("steps_list")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Steps list");
            ui.separator();
            for step in &app_data.aerugo.steps {
                if ui.button(format!("{:?}", step)).clicked() {
                    select_another = Some(step.id);
                }
            }
            if ui.add(egui::Button::new("+")).clicked() {
                add_new_step = true;
            }
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
            if ui.button("None").clicked() {
                app_state.current.inner = Steps::None;
            }
        });
        ui.separator();
        match &app_state.current.inner {
            Steps::Text { .. } => {
                ui.heading("Text");
                ui.label("Author");
                ui.text_edit_singleline(&mut app_state.author);
                ui.label("Text");
                ui.text_edit_multiline(&mut app_state.text);
            }
            Steps::Jump { .. } => {
                ui.heading("Jump");
                ui.label("Target");
                egui::ComboBox::from_label("Select target")
                    .selected_text(format!("{:?}", app_state.target))
                    .show_ui(ui, |ui| {
                        for step in &app_data.aerugo.steps {
                            ui.selectable_value(
                                &mut app_state.target,
                                step.id,
                                format!("{:?}", step),
                            );
                        }
                    });
                ui.separator();
                ui.heading("Condition");
                ui.label(serde_json::to_string(&app_state.condition).unwrap());
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
        let curr_id = app_state.current.id;
        let mut last = app_data.aerugo.steps
            .iter_mut()
            .find(|s| { s.id == curr_id })
            .unwrap();
        last.name = app_state.current.name.clone();
        // last.inner = app_state.current.inner.clone();
        match &app_state.current.inner {
            Steps::Text { .. } => {
                last.inner = Steps::Text {
                    author: app_state.author.clone(),
                    texts: app_state.text.clone(),
                }
            }
            Steps::Jump { .. } => {
                last.inner = Steps::Jump {
                    condition: app_state.condition.clone(),
                    target: app_state.target.clone(),
                }
            }
            Steps::None => {
                last.inner = Steps::None
            }
        }
        command.insert_resource(
            AppState::from_step(
                app_data.aerugo.steps
                    .iter()
                    .find(|s| { s.id == id })
                    .unwrap()
                    .clone()
            )
        );
    }
}

fn save(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    app_data: ResMut<AppData>,
)
{
    if keyboard_input.pressed(KeyCode::LControl)
        && keyboard_input.clear_just_released(KeyCode::S) {
        let data = serde_json::to_string(&app_data.aerugo).unwrap();
        File::options().write(true).create(true).truncate(true)
            .open(app_data.file.as_str())
            .unwrap()
            .write_all(data.as_bytes())
            .unwrap();
    }
}
