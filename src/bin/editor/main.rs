extern crate core;

use std::fs::File;
use std::io::{Read, Write};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
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
        .add_system(update_ui_scale_factor)
        .add_system(ui)
        .add_event::<SaveEvent>()
        .add_system(save_hotkey)
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
    condition: ConditionString,
    target: Uuid,
}

struct SaveEvent;

#[derive(Clone, Debug)]
struct ConditionString(String);

type ConditionStringInner = Option<Condition>;

impl Default for ConditionString {
    fn default() -> Self {
        ConditionStringInner::default().into()
    }
}

impl ConditionString {
    fn is_valid(&self) -> bool {
        serde_json::from_str::<'_, ConditionStringInner>(self.0.as_str()).is_ok()
    }
}

impl Into<ConditionStringInner> for ConditionString {
    fn into(self) -> ConditionStringInner {
        serde_json::from_str(self.0.as_str())
            .or_else::<serde_json::Error, _>(|_| { Ok(None) })
            .unwrap()
    }
}

impl From<ConditionStringInner> for ConditionString {
    fn from(c: ConditionStringInner) -> Self {
        ConditionString(serde_json::to_string(&c).unwrap())
    }
}

impl AppState {
    fn from_step(step: Step) -> AppState {
        match step.inner.clone() {
            Steps::Text { author, texts } => {
                AppState {
                    current: step,
                    author: author,
                    text: texts,
                    condition: Default::default(),
                    target: Uuid::nil(),
                }
            }
            Steps::Jump { target, condition } => {
                AppState {
                    current: step,
                    author: Default::default(),
                    text: Default::default(),
                    condition: condition.into(),
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
    command.insert_resource(AppState::from_step(first_step));
}

fn ui(
    mut command: Commands,
    egui_ctx: Res<EguiContext>,
    mut app_data: ResMut<AppData>,
    mut app_state: ResMut<AppState>,
    mut save_event: EventWriter<SaveEvent>,
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
                ui.horizontal(|ui| {
                    if app_state.condition.is_valid() {
                        ui.label(egui::RichText::new("Valid").color(egui::Color32::GREEN));
                    } else {
                        ui.label(egui::RichText::new("Invalid").color(egui::Color32::RED));
                        ui.small("will be reset");
                    }
                });
                ui.text_edit_multiline(&mut app_state.condition.0);
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
            Steps::Text { .. } => {
                app_state.current.clone().with_inner(
                    Steps::Text { author: app_state.author.clone(), texts: app_state.text.clone() }
                )
            }
            Steps::Jump { .. } => {
                app_state.current.clone().with_inner(
                    Steps::Jump { condition: app_state.condition.clone().into(), target: app_state.target }
                )
            }
            Steps::None => {
                app_state.current.clone()
            }
        };
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
