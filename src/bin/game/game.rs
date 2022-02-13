#![allow(unused_imports)]

mod components;
mod systems;

use std::collections::HashMap;
use bevy::prelude::*;

use aerugo::*;

use crate::states::MainState;
use systems::*;
use components::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(MuteControl::Mute)
            .add_event::<NextStepEvent>()
            .add_event::<NewNarratorEvent>()
            .add_event::<NewSpriteEvent>()
            .add_event::<NewBackgroundEvent>()
            .add_event::<NewSceneEvent>()
            .add_event::<PassAnimateEvent>()
            .init_resource::<SpriteEntities>()
            .add_startup_system(preload_aerugo)
            .add_system_set(
                SystemSet::on_enter(MainState::InGame)
                    .with_system(setup_game)
            )
            .add_system_set(
                SystemSet::on_update(MainState::InGame)
                    .with_system(open_overlay)
                    .with_system(next_step_listener)
                    .with_system(step_init)
                    .with_system(input_listener)
                    .with_system(animate)
                    .with_system(new_background_listener)
                    .with_system(new_scene_listener)
                    .with_system(new_narrator_listener)
                    .with_system(new_sprite_listener)
                    .with_system(resize)
            )
            .add_system_set(
                SystemSet::on_exit(MainState::InGame)
                    .with_system(cleanup)
            );
    }
}

pub struct GameData {
    pub aerugo: Aerugo,
}

pub struct GameState {
    pub just_init: bool,
    pub aerugo_state: AerugoState,

    pub text_narrator_entity: Entity,
    pub text_background_entity: Entity,
    pub text_ui_root_entity: Entity,
    pub text_ui_entity: Entity,

    pub phrase_ui_entity: Entity,

    pub narrator_entity: Entity,

    pub background_entity: Entity,

    pub scene_entity: Entity,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MuteControl {
    Mute,
    Pass,
    None,
}

pub struct NextStepEvent;

pub struct NewNarratorEvent(pub Option<String>);

pub struct NewSpriteEvent(pub SpriteCommand);

pub struct NewBackgroundEvent(pub BackgroundCommand);

pub struct NewSceneEvent(pub SceneCommand);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum CurrentStep {
    Text,
    Phrase,
    ImageSelect,
}

pub struct PassAnimateEvent;

#[derive(Default)]
pub struct SpriteEntities {
    entities: HashMap<String, Entity>,
}
