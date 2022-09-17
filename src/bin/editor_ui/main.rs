use std::io::Write;

use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{Inspectable, InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use bevy_inspector_egui::widgets::InspectorQuery;

use aerugo::bevy_glue::{ImageTip, MainMenuButtons};
use save_load::*;

use crate::extract::*;

mod extract;
mod save_load;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<InspectUIRoot>::new())
        .add_plugin(ExtractorPlugin)
        .add_plugin(SaveLoadPlugin)
        .add_startup_system(setup)
        .add_system(ui_system)
        .add_system(reopen_inspector)
        .add_system(fix_images)
        .add_system(fix_ui_images)
        .run();
}

#[derive(Component, Reflect, Default, Clone, Inspectable)]
#[reflect(Component)]
struct UIRoot;

type InspectUIRoot = InspectorQuery<Entity, With<UIRoot>>;

pub struct EditorState {
    file: String,
    root: Entity,
    scene: Entity,
}

const NAME: &'static str = "new_ui.scn.ron";
const F_NAME: &'static str = const_format::formatcp!("./assets/{}", NAME);

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    make_default_ui(&mut commands);
}

fn reopen_inspector(
    mut inspector: ResMut<bevy_inspector_egui::WorldInspectorParams>,
    keys: Res<Input<KeyCode>>,
)
{
    if keys.just_released(KeyCode::I) {
        inspector.enabled = true;
    }
}

fn ui_system(
    mut commands: Commands,
    mut e_ctx: ResMut<EguiContext>,
    // mut editor_state: ResMut<EditorState>,
)
{
    egui::Window::new("CMD")
        .show(e_ctx.ctx_mut(), |ui| {
            if ui.button("save").clicked() {
                commands.insert_resource(SaveSignal);
            }
            if ui.button("load").clicked() {
                commands.insert_resource(LoadSignal);
            }
        });
}

fn make_default_ui(commands: &mut Commands) {
    let mut root = Entity::from_raw(0);
    let scene = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            root = builder
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(UIRoot)
                .with_children(|builder| {
                    builder
                        .spawn_bundle(ButtonBundle::default())
                        .insert(MainMenuButtons::NewGame);
                })
                .with_children(|builder| {
                    builder
                        .spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(ImageTip::new("icon.png"));
                })
                .id();
        })
        .id();
    commands.insert_resource(EditorState {
        file: F_NAME.to_string(),
        root,
        scene,
    })
}

fn fix_images(
    mut image_query: Query<(&mut ImageTip, &mut Handle<Image>)>,
    asset_server: Res<PreloadedAssets>,
)
{
    for (tip, handle) in image_query.iter_mut() {
        let mut tip: Mut<ImageTip> = tip;
        let mut handle: Mut<Handle<Image>> = handle;
        if !tip.loaded {
            *handle = asset_server.load(&tip.name);
            tip.loaded = true;
        }
    }
}

fn fix_ui_images(
    mut ui_image_query: Query<(&mut ImageTip, &mut UiImage)>,
    asset_server: Res<PreloadedAssets>,
)
{
    for (tip, handle) in ui_image_query.iter_mut() {
        let mut tip: Mut<ImageTip> = tip;
        let mut handle: Mut<UiImage> = handle;
        if !tip.loaded {
            *handle = asset_server.load(&tip.name).into();
            tip.loaded = true;
        }
    }
}