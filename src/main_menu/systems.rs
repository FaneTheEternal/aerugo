use bevy::app::AppExit;
use bevy::prelude::*;

use crate::main_menu::{MainMenuButtons, MainMenuButton, MainMenuData};
use crate::states::MainState;
use crate::utils::make_button_closure;

const BTN_NORMAL: Color = Color::WHITE;
const BTN_HOVERED: Color = Color::GRAY;
const BTN_PRESSED: Color = Color::DARK_GRAY;

pub fn setup_menu(
    mut command: Commands,
    asset_server: Res<AssetServer>,
)
{
    command.spawn_bundle(UiCameraBundle::default());

    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let ui_entity = command
        .spawn_bundle(NodeBundle::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                        padding: Rect::all(Val::Percent(10.0)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_wrap: FlexWrap::Wrap,
                        ..Default::default()
                    },
                    color: Color::rgba(0.65, 0.65, 0.65, 0.5).into(),
                    ..Default::default()
                })
                .with_children(
                    make_button_closure(
                        "New game",
                        button_font.clone(),
                        MainMenuButton { target: MainMenuButtons::NewGame },
                        BTN_NORMAL,
                    )
                )
                .with_children(
                    make_button_closure(
                        "Load",
                        button_font.clone(),
                        MainMenuButton { target: MainMenuButtons::Load },
                        BTN_NORMAL,
                    )
                )
                .with_children(
                    make_button_closure(
                        "Settings",
                        button_font.clone(),
                        MainMenuButton { target: MainMenuButtons::Settings },
                        BTN_NORMAL,
                    )
                )
                .with_children(
                    make_button_closure(
                        "Exit",
                        button_font.clone(),
                        MainMenuButton { target: MainMenuButtons::Exit },
                        BTN_NORMAL,
                    )
                );
        })
        .id();

    command.insert_resource(crate::main_menu::MainMenuData { ui_entity });
}

pub fn menu(
    mut main_state: ResMut<State<MainState>>,
    // TODO: OverlayState
    mut interactions_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    buttons_query: Query<&MainMenuButton>,
    mut exit: EventWriter<AppExit>,
)
{
    for (interaction, mut color, children) in interactions_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = BTN_PRESSED.into();

                if let Ok(btn) = buttons_query.get(children[0]) {
                    match btn.target {
                        MainMenuButtons::NewGame => {
                            main_state.set(MainState::InGame).unwrap();
                        }
                        MainMenuButtons::Load => {}
                        MainMenuButtons::Settings => {}
                        MainMenuButtons::Exit => {
                            exit.send(AppExit);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = BTN_HOVERED.into();
            }
            Interaction::None => {
                *color = BTN_NORMAL.into();
            }
        }
    }
}

pub fn cleanup_menu(mut command: Commands, menu_data: Option<Res<MainMenuData>>) {
    if let Some(menu_data) = menu_data {
        command.entity(menu_data.ui_entity).despawn_recursive();
    }
}
