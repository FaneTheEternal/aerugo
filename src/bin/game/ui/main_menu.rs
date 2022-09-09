use bevy::prelude::*;

use aerugo::bevy_glue::MainMenuButtons;

use crate::utils::*;

use super::*;

pub struct MainMenuUI {
    pub(crate) entity_root: Entity,
}

impl MainMenuUI {
    pub fn show(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::Flex;
    }

    pub fn hide(&self, mut query: Query<&mut Style>) {
        query.get_mut(self.entity_root).unwrap().display = Display::None;
    }
}

pub fn main_menu_show(main_menu: ResMut<MainMenuUI>, query: Query<&mut Style>) {
    main_menu.show(query);
}

pub fn main_menu_hide(main_menu: ResMut<MainMenuUI>, query: Query<&mut Style>) {
    main_menu.hide(query);
}

pub fn main_menu_actions(
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<
        (&Interaction, &mut UiColor, &MainMenuButtons),
        (Changed<Interaction>, With<Button>)
    >,
    mut exit: EventWriter<AppExit>,
)
{
    for (interaction, mut color, btn) in query.iter_mut() {
        let btn: &MainMenuButtons = btn;
        match *interaction {
            Interaction::Clicked => {
                *color = TRANSPARENT.into();

                match btn {
                    MainMenuButtons::NewGame => {
                        ui_state.set(UiState::Game).unwrap_or_else(|e| warn!("{e:?}"));
                        game_state.set(GameState::Init).unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    MainMenuButtons::Load => {
                        ui_state.set(UiState::Load).unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    MainMenuButtons::Gallery => {}
                    MainMenuButtons::Settings => {}
                    MainMenuButtons::About => {}
                    MainMenuButtons::Exit => {
                        exit.send(AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
            }
            Interaction::None => {
                *color = TRANSPARENT.into();
            }
        }
    }
}
