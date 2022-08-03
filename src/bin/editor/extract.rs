use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use bevy::ui::FocusPolicy;
use serde::{Deserialize, Serialize};

use crate::{BuildWorldChildren, Children, default, EditorState, Entity, Mut, World};

pub struct ExtractorPlugin;

impl Plugin for ExtractorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EntityCloneSystems>()
            // base
            .register_cloneable::<Transform>()
            .register_cloneable::<GlobalTransform>()
            .register_cloneable::<ComputedVisibility>()
            .register_cloneable::<Visibility>()
            // NodeBundle
            .register_cloneable::<Node>()
            .register_cloneable::<Style>()
            .register_cloneable::<UiColor>()
            .register_cloneable::<UiImage>()
            .register_cloneable::<FocusPolicy>()
            // some wiping snot
            .register_serde::<Option<f32>>()
        ;
    }
}

trait ExtractableApp {
    fn register_cloneable<T: Clone + Component>(&mut self) -> &mut Self;
    fn register_serde<T: Reflect + for<'a> Deserialize<'a> + GetTypeRegistration + Serialize + 'static>(&mut self);
}

impl ExtractableApp for App {
    fn register_cloneable<T: Clone + Component>(&mut self) -> &mut Self {
        self.world
            .get_resource_mut::<EntityCloneSystems>()
            .unwrap()
            .push(clone_system::<T>);
        self
    }

    fn register_serde<T: Reflect + for<'a> Deserialize<'a> + GetTypeRegistration + Serialize + 'static>(&mut self) {
        self.register_type::<T>()
            .register_type_data::<T, ReflectSerialize>()
            .register_type_data::<T, ReflectDeserialize>()
        ;
    }
}

fn clone_system<T>(src: &mut World, dest: &mut World, result: &Entity, target: &Entity)
    where T: Clone + Component
{
    if let Some(component) = src.entity(target.clone()).get::<T>() {
        dest.entity_mut(result.clone()).insert(component.clone());
    }
}

#[derive(Default, Deref, DerefMut)]
struct EntityCloneSystems(Vec<fn(&mut World, &mut World, &Entity, &Entity)>);

fn clone_entity(src: &mut World, dest: &mut World, target: &Entity) -> Entity {
    let result = dest.spawn().id();
    src.resource_scope(|src, systems: Mut<EntityCloneSystems>| {
        for s in systems.0.iter() {
            s(src, dest, &result, target);
        }
    });
    result
}

pub fn extract_ui(world: &mut World) -> World {
    let mut scene_world = World::new();

    let state = world.remove_resource::<EditorState>().unwrap();
    let root = state.root;
    let new_root = clone_entity(world, &mut scene_world, &root);

    fn _clone_recursive(root: Entity, new_root: Entity, src: &mut World, dest: &mut World) {
        let mut entities: Vec<Entity> = default();
        if let Some(children) = src.get::<Children>(root) {
            entities.extend(children);
        }
        let new_children = entities.into_iter()
            .map(|child| {
                let new = clone_entity(src, dest, &child);
                _clone_recursive(child, new, src, dest);
                new
            })
            .collect::<Vec<_>>();
        dest.entity_mut(new_root).push_children(&new_children);
    }

    _clone_recursive(root, new_root, world, &mut scene_world);
    scene_world
}
