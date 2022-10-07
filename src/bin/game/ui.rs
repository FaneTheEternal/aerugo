use bevy::app::AppExit;
use bevy::log::Level;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::tracing::span;
use bevy::window::WindowResized;
use bevy_egui::{egui, EguiPlugin};

pub use game::*;
pub use main_menu::*;
pub use pause::*;
pub use save_load::*;

use crate::game::GameState;
use crate::settings::{Resolution, Settings};
use crate::translator::{Lang, NewLang, Translator};

mod main_menu;
mod game;
mod pause;
mod save_load;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_state(UiState::None)
            .add_system_set(
                SystemSet::on_enter(UiState::Notice)
                    .with_system(main_menu_show)
                    .with_system(NoticeUI::show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Notice)
                    .with_system(NoticeUI::actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Notice)
                    .with_system(NoticeUI::exit)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::MainMenu)
                    .with_system(main_menu_show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::MainMenu)
                    .with_system(main_menu_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::MainMenu)
                    .with_system(main_menu_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Settings)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Settings)
                    .with_system(settings_ui)
                    .with_system(generic_break)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Settings)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Save)
                    .with_system(save_load::save_show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Save)
                    .with_system(generic_break)
                    .with_system(save_page_actions)
                    .with_system(new_page)
                    .with_system(save_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Save)
                    .with_system(save_load::save_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Load)
                    .with_system(save_load::save_show)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Load)
                    .with_system(generic_break)
                    .with_system(save_page_actions)
                    .with_system(new_page)
                    .with_system(load_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Load)
                    .with_system(save_load::save_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Game)
                    .with_system(game_show)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Game)
                    .with_system(game_hide)
            )
            .add_system_set(
                SystemSet::on_enter(UiState::Pause)
                    .with_system(show_game_menu)
            )
            .add_system_set(
                SystemSet::on_update(UiState::Pause)
                    .with_system(game_menu_actions)
            )
            .add_system_set(
                SystemSet::on_exit(UiState::Pause)
                    .with_system(hide_game_menu)
            )
        ;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UiState {
    None,
    Notice,
    MainMenu,
    Settings,
    Save,
    Load,
    Game,
    Pause,
}


pub fn generic_break(
    mut ui_state: ResMut<State<UiState>>,
    game_state: Res<State<GameState>>,
    mut input: ResMut<Input<KeyCode>>,
)
{
    let span = span!(Level::WARN, "generic_break");
    let _ = span.enter();

    if input.clear_just_released(KeyCode::Escape) {
        if game_state.current().eq(&GameState::None) {
            ui_state.set(UiState::MainMenu).unwrap_or_else(|e| warn!("{e:?}"));
        } else {
            ui_state.set(UiState::Pause).unwrap_or_else(|e| warn!("{e:?}"));
        }
    }
}

pub fn relative(
    game_ui: Res<GameUI>,
    mut style_query: Query<&mut Style>,
    mut sprite_query: Query<&mut Sprite>,
    mut atlas_query: Query<&mut TextureAtlasSprite>,
    mut resize_event: EventReader<WindowResized>,
)
{
    for event in resize_event.iter() {
        game_ui.resize_relative(&mut sprite_query, &mut atlas_query, event.width, event.height);
        game_ui.text.resize_relative(&mut style_query, event.width, event.height);
    }
}

pub fn settings_ui(
    mut e_ctx: ResMut<bevy_egui::EguiContext>,
    mut settings: ResMut<Settings>,
    mut translator: ResMut<Translator>,
    mut new_lang: EventWriter<NewLang>,
    mut windows: ResMut<Windows>,
)
{
    egui::TopBottomPanel::top("my_panel")
        .show(e_ctx.ctx_mut(), |ui| {
            ui.heading(translator.get(&settings.lang, "Settings"));
        });
    egui::CentralPanel::default().show(
        e_ctx.ctx_mut(),
        |ui| {
            ui.horizontal(|ui| {
                ui.label(translator.get(&settings.lang, "Selected"));
                let curr_lang = settings.lang.clone();
                egui::ComboBox::from_label(
                    translator.get(&settings.lang, "language")
                )
                    .selected_text(format!("{:?}", settings.lang))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut settings.lang,
                            Lang::En,
                            format!("{:?}", Lang::En),
                        );
                        ui.selectable_value(
                            &mut settings.lang,
                            Lang::Ru,
                            format!("{:?}", Lang::Ru),
                        );
                    });
                if curr_lang != settings.lang {
                    new_lang.send(NewLang(settings.lang.clone()));
                    settings.dump();
                }
            });
            ui.horizontal(|ui| {
                ui.label(translator.get(&settings.lang, "Resolution"));
                let current = settings.resolution.clone();
                egui::ComboBox::from_label("")
                    .selected_text(settings.resolution.verbose())
                    .width(200.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut settings.resolution,
                            Resolution::HD,
                            Resolution::HD.verbose(),
                        );
                        ui.selectable_value(
                            &mut settings.resolution,
                            Resolution::FHD,
                            Resolution::FHD.verbose(),
                        );
                        ui.selectable_value(
                            &mut settings.resolution,
                            Resolution::QHD,
                            Resolution::QHD.verbose(),
                        );
                    });
                if current != settings.resolution {
                    let window = windows.get_primary_mut().unwrap();
                    let (w, h) = settings.resolution.get();
                    window.set_resolution(w, h);
                }
            });
            ui.horizontal(|ui| {
                ui.label(translator.get(&settings.lang, "NarratorSize"));
                ui.add(egui::DragValue::new(&mut settings.narrator_size)
                    .clamp_range(1.0..=100.0)
                );
            });
            ui.horizontal(|ui| {
                ui.label(translator.get(&settings.lang, "FlowSize"));
                ui.add(egui::DragValue::new(&mut settings.flow_size)
                    .clamp_range(1.0..=100.0)
                );
            });
            ui.horizontal(|ui| {
                ui.label(translator.get(&settings.lang, "FlowSpeed"));
                ui.add(
                    egui::DragValue::new(&mut settings.flow_speed)
                        .clamp_range(0.01..=1.0)
                        .fixed_decimals(2)
                        .speed(0.01)
                );
                ui.label(translator.get(&settings.lang, "Sec/Char"));
            });
        },
    );
    egui::TopBottomPanel::bottom("my_bottom_panel")
        .show(
            e_ctx.ctx_mut(),
            |ui| {
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    ui.hyperlink_to("GUI by egui", "https://github.com/emilk/egui");
                });
            },
        );
}
