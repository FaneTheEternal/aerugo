use std::io::Write;
use std::time::Duration;

use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::extract::*;

mod extract;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ExtractorPlugin)
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .add_startup_system(setup)
        .add_system(ui_system)
        .add_system(save_listener.exclusive_system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    make_default_ui(&mut commands);
}

fn ui_system(
    mut commands: Commands,
    mut e_ctx: ResMut<EguiContext>,
)
{
    egui::Window::new("Hello")
        .show(e_ctx.ctx_mut(), |ui| {
            ui.label("world");
            if ui.button("save").clicked() {
                commands.insert_resource(SaveSignal);
            }
        });
}

struct EditorState {
    file: String,
    root: Entity,
}

#[derive(Component)]
struct UIRoot;

const NAME: &'static str = "new_ui.ron";
const F_NAME: &'static str = const_format::formatcp!("./assets/{}", NAME);

fn make_default_ui(commands: &mut Commands) {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .insert(UIRoot)
        .id();
    commands.insert_resource(EditorState { file: F_NAME.to_string(), root })
}

struct SaveSignal;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(ignore)]
    pub _time_since_startup: Duration,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            _time_since_startup: time.time_since_startup(),
            value: "Default Value".to_string(),
        }
    }
}

fn save_listener(world: &mut World) {
    if world.remove_resource::<SaveSignal>().is_some() {
        let mut scene_world = extract_ui(world);

        // The TypeRegistry resource contains information about all registered types (including
        // components). This is used to construct scenes.
        let type_registry = world.resource::<TypeRegistry>();
        let scene = DynamicScene::from_world(&scene_world, type_registry);

        let saved_scene = scene.serialize_ron(type_registry).unwrap();
        let mut file = std::fs::File::options()
            .create(true).truncate(true).write(true)
            .open(F_NAME)
            .unwrap();
        file.write(saved_scene.as_bytes()).unwrap();
    }
}
