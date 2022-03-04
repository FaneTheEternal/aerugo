use bevy::prelude::*;
use crate::utils::BTN_NORMAL;

use super::*;


pub struct SaveUI {
    pub(crate) entity_root: Entity,
}

impl SaveUI {
    pub fn show(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::Flex;
    }

    pub fn hide(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::None;
    }
}

pub fn save_show(save: Res<SaveUI>, query: Query<&mut Style>) {
    save.show(query);
}

pub fn save_hide(save: Res<SaveUI>, query: Query<&mut Style>) {
    save.hide(query);
}

pub fn save_actions(
    mut commands: Commands,
    mut save_events: EventWriter<CleanseSavesEvent>,
    mut save_buttons_query: Query<
        (&Interaction, &mut UiColor, &SaveMark),
        (Changed<Interaction>, With<Button>)
    >,
)
{
    for (interaction, color, mark) in save_buttons_query.iter_mut() {
        let interaction: &Interaction = interaction;
        let mut color: Mut<UiColor> = color;
        let mark: &SaveMark = mark;
        match interaction {
            Interaction::Clicked => {
                *color = BTN_NORMAL.into();
                commands.insert_resource(mark.clone());
                save_events.send(CleanseSavesEvent);
            }
            Interaction::Hovered => {
                *color = Color::ANTIQUE_WHITE.into();
            }
            Interaction::None => {
                *color = BTN_NORMAL.into();
            }
        }
    }
}

pub struct CleanseSavesEvent;

pub struct RespawnSavesEvent;

pub fn cleanse_saves_listener(
    mut commands: Commands,
    mut events: EventReader<CleanseSavesEvent>,
    mut send: EventWriter<RespawnSavesEvent>,
    save_query: Query<Entity, (With<SaveItemsParentMark>, Without<LoadItemsParentMark>)>,
    load_query: Query<Entity, (With<LoadItemsParentMark>, Without<SaveItemsParentMark>)>,
)
{
    if events.iter().count() > 0 {
        if let Some(save_entity) = save_query.iter().next() {
            commands.entity(save_entity).despawn_descendants();
        }
        if let Some(load_entity) = load_query.iter().next() {
            commands.entity(load_entity).despawn_descendants();
        }
        send.send(RespawnSavesEvent);
    }
}

pub fn respawn_saves_listener(
    mut commands: Commands,
    mut events: EventReader<RespawnSavesEvent>,
    saves: Res<Saves>,
    asset_server: Res<AssetServer>,
    save_query: Query<Entity, (With<SaveItemsParentMark>, Without<LoadItemsParentMark>)>,
    load_query: Query<Entity, (With<LoadItemsParentMark>, Without<SaveItemsParentMark>)>,
)
{
    if events.iter().count() > 0 {
        let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");

        if let Some(save_entity) = save_query.iter().next() {
            let save_items = make_save_items(
                &mut commands, saves.as_ref(),
                button_font.clone(), text_font.clone(),
            );
            commands.entity(save_entity).push_children(save_items.as_slice());
        }

        if let Some(load_entity) = load_query.iter().next() {
            let load_items = make_load_items(
                &mut commands, saves.as_ref(),
                button_font.clone(), text_font.clone(),
            );
            commands.entity(load_entity).push_children(load_items.as_slice());
        }
    }
}
