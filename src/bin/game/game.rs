#![allow(unused_imports)]

use std::collections::HashMap;

use bevy::prelude::*;

use aerugo::*;
use components::*;
use systems::*;

use crate::ui::game_show;

mod components;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NextStepEvent>()
            .add_event::<NewNarratorEvent>()
            .add_event::<NewSpriteEvent>()
            .add_event::<NewBackgroundEvent>()
            .add_event::<NewSceneEvent>()
            .add_state(GameState::None)
            .add_system_set(
                SystemSet::on_enter(GameState::None)
                    .with_system(hide_game)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Init)
                    .with_system(setup_game)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Init)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Active)
                    .with_system(open_overlay)
                    .with_system(next_step_listener)
                    .with_system(step_init.after(next_step_listener))
                    .with_system(new_narrator_listener.after(step_init))
                    .with_system(new_background_listener.after(step_init))
                    .with_system(new_scene_listener.after(step_init))
                    .with_system(new_sprite_listener.after(step_init))
                    .with_system(animate
                        .after(new_narrator_listener)
                        .after(new_background_listener)
                        .after(new_scene_listener)
                        .after(new_sprite_listener))
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Paused)
                    .with_system(disable_game_input)
                    .with_system(hide_game)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Paused)
                    .with_system(enable_game_input)
                    .with_system(show_game)
            )
            .add_plugin(GameControlPlugin)
        ;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GameState {
    None,
    Init,
    Active,
    Paused,
}

pub struct NextStepEvent;

pub struct JustInit;

pub struct NewNarratorEvent(pub NarratorCommand);

pub struct NewSpriteEvent(pub SpriteCommand);

pub struct NewBackgroundEvent(pub BackgroundCommand);

pub struct NewSceneEvent(pub SceneCommand);

pub struct GameControlPlugin;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GameControlState {
    None,
    TextPass,
    Text,
    Phrase,
}

impl Plugin for GameControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(GameControlState::None)
            .add_system_set(
                SystemSet::on_update(GameControlState::TextPass)
                    .with_system(input_text_pass)
            )
            .add_system_set(
                SystemSet::on_update(GameControlState::Text)
                    .with_system(input_text_next)
            )
            .add_system_set(
                SystemSet::on_update(GameControlState::Phrase)
                    .with_system(input_phrase)
            )
        ;
    }
}