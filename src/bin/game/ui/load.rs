use bevy::prelude::*;
use crate::utils::BTN_NORMAL;

use super::*;


pub struct LoadUI {
    pub(crate) entity_root: Entity,
}

impl LoadUI {
    pub fn show(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::Flex;
    }

    pub fn hide(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::None;
    }
}

pub fn load_show(load: Res<LoadUI>, query: Query<&mut Style>) {
    load.show(query);
}

pub fn load_hide(load: Res<LoadUI>, query: Query<&mut Style>) {
    load.hide(query);
}

pub fn load_actions(
    mut commands: Commands,
    mut ui_state: ResMut<State<UiState>>,
    mut load_buttons_query: Query<
        (&Interaction, &mut UiColor, &LoadMark),
        (Changed<Interaction>, With<Button>)
    >,
)
{
    for (interaction, color, mark) in load_buttons_query.iter_mut() {
        let interaction: &Interaction = interaction;
        let mut color: Mut<UiColor> = color;
        let mark: &LoadMark = mark;
        match interaction {
            Interaction::Clicked => {
                *color = BTN_NORMAL.into();
                commands.insert_resource(mark.clone());
                ui_state.set(UiState::Game).unwrap_or_else(|e| warn!("{e:?}"));
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
