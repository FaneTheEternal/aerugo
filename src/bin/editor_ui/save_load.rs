use bevy::prelude::*;

use crate::*;

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(save_listener.exclusive_system())
            .add_system(load_scene_system)
            .add_system(fix_root_index.after(load_scene_system))
        ;
    }
}


pub struct SaveSignal;

pub struct LoadSignal;

pub struct FixRootIndex;

fn save_listener(world: &mut World) {
    if world.remove_resource::<SaveSignal>().is_some() {
        let mut scene_world = extract_ui(world);
        let type_registry = world.resource::<TypeRegistry>();
        let scene = DynamicScene::from_world(&scene_world, type_registry);

        let saved_scene = scene.serialize_ron(type_registry).unwrap();
        let mut file = std::fs::File::options()
            .create(true).truncate(true).write(true)
            .open(F_NAME)
            .unwrap();
        file.write(saved_scene.as_bytes()).unwrap();
        println!("UI saved in {}", F_NAME);
    }
}

fn load_scene_system(
    mut commands: Commands,
    mut asset_server: CachedAssetServer,
    load_signal: Option<Res<LoadSignal>>,
    mut editor_state: ResMut<EditorState>,
)
{
    if load_signal.is_some() {
        commands.entity(editor_state.scene).despawn_recursive();
        editor_state.scene = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            })
            .insert_bundle(DynamicSceneBundle {
                scene: asset_server.load(NAME),
                ..default()
            }).id();
        commands.remove_resource::<LoadSignal>();
        commands.insert_resource(FixRootIndex);
        println!("Scene loaded from {}", NAME);
    }
}

fn fix_root_index(
    mut commands: Commands,
    query: Query<Entity, With<UIRoot>>,
    mut editor_state: ResMut<EditorState>,
    fix_root_index: Option<Res<FixRootIndex>>,
)
{
    if fix_root_index.is_some() {
        for entity in &query {
            editor_state.root = entity;
        }
        commands.remove_resource::<FixRootIndex>();
        println!("Fix root index");
    }
}