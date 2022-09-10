use bevy::prelude::*;

use crate::utils::*;

use super::*;
use super::spawn_game::*;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    saves: Res<Saves>,
    window: Res<Windows>,
    mut state: ResMut<State<MainState>>,
)
{
    let main_menu = spawn_main_menu::spawn(&mut commands, asset_server.as_ref());
    commands.insert_resource(MainMenuUI { entity_root: main_menu });

    let save = save_load::spawn_save(
        &mut commands, asset_server.as_ref(), saves.as_ref()
    );
    commands.insert_resource(save);

    let settings = spawn_settings(&mut commands, asset_server.as_ref());
    commands.insert_resource(SettingsUI { entity_root: settings });

    let game = spawn_game(&mut commands, asset_server.as_ref(), window.as_ref());
    commands.insert_resource(game);

    let game_menu = spawn_game_menu::spawn(&mut commands, asset_server.as_ref());
    commands.insert_resource(GameMenuUI { root: game_menu });

    state.set(MainState::Ready).unwrap_or_else(|e| warn!("{e:?}"));
}

fn make_ui_base(
    commands: &mut Commands,
    builder: impl FnOnce(&mut ChildBuilder),
) -> Entity
{
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::None,
                size: SIZE_ALL,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: TRANSPARENT.into(),
            ..Default::default()
        })
        .with_children(builder)
        .id()
}

fn spawn_settings(mut commands: &mut Commands, asset_server: &AssetServer) -> Entity
{
    let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");
    // let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    make_ui_base(&mut commands, |parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: SIZE_ALL,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: TRANSPARENT.into(),
                ..Default::default()
            })
            .with_children(|parent| {  // Header
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::FlexStart,
                            ..Default::default()
                        },
                        color: Color::DARK_GREEN.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    "Settings",
                                    TextStyle {
                                        font: text_font.clone(),
                                        font_size: 40.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                ..Default::default()
                            });
                    });
            })
            .with_children(|parent| {  // Body
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: Color::GRAY.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    "TODO: Settings",
                                    TextStyle {
                                        font: text_font.clone(),
                                        font_size: 60.0,
                                        color: Color::ANTIQUE_WHITE,
                                    },
                                ),
                                ..Default::default()
                            });
                    });
            });
    })
}

