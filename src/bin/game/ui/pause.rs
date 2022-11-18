use bevy::prelude::*;

use aerugo::*;
use aerugo::bevy_glue::GameMenuButtons;

use crate::saves::AerugoLoaded;
use crate::utils::*;

use super::*;

#[derive(Debug, Resource)]
pub struct GameMenuUI {
    pub(crate) root: Entity,
}

impl GameMenuUI {
    pub fn show(&self, query: &mut Query<&mut Style>) {
        query.get_mut(self.root).unwrap().display = Display::Flex;
    }

    pub fn hide(&self, query: &mut Query<&mut Style>) {
        query.get_mut(self.root).unwrap().display = Display::None;
    }
}

pub fn show_game_menu(
    game_menu_ui: Res<GameMenuUI>,
    mut query: Query<&mut Style>,
)
{
    game_menu_ui.show(&mut query)
}

fn back_to_game(
    commands: &mut Commands,
    game_state: &mut ResMut<State<GameState>>,
    state: Res<AerugoState>,
)
{
    game_state.set(GameState::Init)
        .unwrap_or_else(|e| warn!("{e:?}"));
    commands.insert_resource(AerugoLoaded(state.clone().reload()));
}

pub fn game_menu_actions(
    mut commands: Commands,
    state: Res<AerugoState>,
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &GameMenuButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut input: ResMut<Input<KeyCode>>,
)
{
    let span = span!(Level::WARN, "game_menu_actions");
    let _enter = span.enter();

    if input.clear_just_released(KeyCode::Escape) {
        back_to_game(&mut commands, &mut game_state, state);
        return;
    }

    for (interaction, mut color, btn) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = TRANSPARENT.into();

                match btn {
                    GameMenuButtons::Continue => {
                        back_to_game(&mut commands, &mut game_state, state);
                        return;
                    }
                    GameMenuButtons::Load => {
                        ui_state.set(UiState::Load)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    GameMenuButtons::Save => {
                        ui_state.set(UiState::Save)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    GameMenuButtons::Gallery => {}
                    GameMenuButtons::Settings => {
                        ui_state.set(UiState::Settings)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                    }
                    GameMenuButtons::MainMenu => {
                        game_state.set(GameState::None)
                            .unwrap_or_else(|e| warn!("{e:?}"));
                        ui_state.set(UiState::MainMenu)
                            .unwrap_or_else(|e| warn!("{e:?}"));
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

pub fn hide_game_menu(
    game_menu_ui: Res<GameMenuUI>,
    mut query: Query<&mut Style>,
)
{
    game_menu_ui.hide(&mut query)
}
