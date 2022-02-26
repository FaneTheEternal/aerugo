#![allow(dead_code)]

use std::collections::HashMap;
use bevy::prelude::*;
use crate::overlay::saves_ui::{LoadItemsParentMark, make_load_items, make_save_items, save_load_base, SaveItemsParentMark};
use crate::saves::{LoadMark, SaveMark, Saves};
use super::*;
use crate::states::{MainState, OverlayState};
use crate::utils::{grow_z_index, make_button_closure};

const BTN_NORMAL: Color = Color::WHITE;
const BTN_HOVERED: Color = Color::GRAY;
const BTN_PRESSED: Color = Color::DARK_GRAY;

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const GLASS_WHITE: Color = Color::rgba(1.0, 1.0, 1.0, 0.2);
const GLASS_GRAY: Color = Color::rgba(0.5, 0.5, 0.5, 0.2);
const GLASS_DARK: Color = Color::rgba(0.0, 0.0, 0.0, 0.2);

const OVERLAY_Z_DEEP: u8 = 10;

pub fn init_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    saves: Res<Saves>,
)
{
    fn make_ui_base(
        commands: &mut Commands,
        mark: impl Component,
        builder: impl FnOnce(&mut ChildBuilder),
    ) -> Entity
    {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    display: Display::None,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                color: TRANSPARENT.into(),
                ..Default::default()
            })
            .insert(mark)
            .with_children(|parent| {
                grow_z_index(
                    OVERLAY_Z_DEEP, parent,
                    Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    builder,
                )
            })
            .id()
    }

    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let ui_menu = make_ui_base(&mut commands, OverlayMenu, |parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::rgba(0.0, 0.0, 0.0, 0.4).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::ColumnReverse,
                            flex_wrap: FlexWrap::Wrap,
                            ..Default::default()
                        },
                        color: Color::rgba(1.0, 0.5, 0.5, 0.8).into(),
                        ..Default::default()
                    })
                    .with_children(
                        make_button_closure(
                            "Close",
                            button_font.clone(),
                            OverlayButton { target: OverlayButtons::Close },
                            BTN_NORMAL,
                        )
                    )
                    .with_children(
                        make_button_closure(
                            "Settings",
                            button_font.clone(),
                            OverlayButton { target: OverlayButtons::Settings },
                            BTN_NORMAL,
                        )
                    )
                    .with_children(
                        make_button_closure(
                            "Save",
                            button_font.clone(),
                            OverlayButton { target: OverlayButtons::Save },
                            BTN_NORMAL,
                        )
                    )
                    .with_children(
                        make_button_closure(
                            "Load",
                            button_font.clone(),
                            OverlayButton { target: OverlayButtons::Load },
                            BTN_NORMAL,
                        )
                    )
                    .with_children(
                        make_button_closure(
                            "Main Menu",
                            button_font.clone(),
                            OverlayButton { target: OverlayButtons::MainMenu },
                            BTN_NORMAL,
                        )
                    );
            });
    });

    let ui_settings = make_ui_base(&mut commands, OverlaySettings, |parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                            padding: Rect::all(Val::Px(10.0)),
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
                                text: Text::with_section(
                                    "Settings",
                                    TextStyle {
                                        font: text_font.clone(),
                                        font_size: 40.0,
                                        color: Color::BLACK,
                                    },
                                    Default::default(),
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
                            padding: Rect::all(Val::Px(10.0)),
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
                                text: Text::with_section(
                                    "TODO: Settings",
                                    TextStyle {
                                        font: text_font.clone(),
                                        font_size: 60.0,
                                        color: Color::ANTIQUE_WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                    });
            });
    });

    let save_items = make_save_items(&mut commands, saves.as_ref(), button_font.clone(), text_font.clone());

    let ui_save = make_ui_base(
        &mut commands,
        OverlaySave,
        save_load_base(
            save_items, text_font.clone(), SaveItemsParentMark, "Save",
        ),
    );

    let load_items = make_load_items(&mut commands, saves.as_ref(), button_font.clone(), text_font.clone());

    let ui_load = make_ui_base(
        &mut commands,
        OverlaySave,
        save_load_base(
            load_items, text_font.clone(), LoadItemsParentMark, "Load",
        ),
    );

    commands.insert_resource(OverlayData {
        ui_menu,
        ui_settings,
        ui_save,
        ui_load,
    });
}

pub fn overlay_menu(
    mut main_state: ResMut<State<MainState>>,
    mut overlay_state: ResMut<State<OverlayState>>,
    mut interactions_query: Query<
        (&Interaction, &mut UiColor, &OverlayButton),
        (Changed<Interaction>, With<Button>),
    >,
)
{
    for (interaction, mut color, btn) in interactions_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = BTN_PRESSED.into();

                match btn.target {
                    OverlayButtons::Close => {
                        overlay_state.set(OverlayState::Hidden).unwrap();
                    }
                    OverlayButtons::Settings => {
                        overlay_state.set(OverlayState::Settings).unwrap();
                    }
                    OverlayButtons::Save => {
                        overlay_state.set(OverlayState::Save).unwrap();
                    }
                    OverlayButtons::Load => {
                        overlay_state.set(OverlayState::Load).unwrap();
                    }
                    OverlayButtons::MainMenu => {
                        overlay_state.set(OverlayState::Hidden).unwrap();
                        main_state.set(MainState::MainMenu).unwrap();
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

pub fn overlay_break(
    main_state: Res<State<MainState>>,
    mut overlay_state: ResMut<State<OverlayState>>,
    mut input: ResMut<Input<KeyCode>>,
)
{
    if input.clear_just_released(KeyCode::Escape) {
        match overlay_state.current() {
            OverlayState::Hidden => {}
            OverlayState::Menu => {
                overlay_state.set(OverlayState::Hidden).unwrap();
            }
            OverlayState::Settings | OverlayState::Load => {
                match main_state.current() {
                    MainState::MainMenu => {
                        overlay_state.set(OverlayState::Hidden).unwrap();
                    }
                    MainState::InGame => {
                        overlay_state.set(OverlayState::Menu).unwrap();
                    }
                    _ => {}
                }
            }
            OverlayState::Save => {
                overlay_state.set(OverlayState::Menu).unwrap();
            }
        }
    }
}

// region show_hide

fn show(entity: Entity, mut style_query: Query<&mut Style>) {
    if let Ok(mut style) = style_query.get_mut(entity) {
        style.display = Display::Flex;
    }
}

fn hide(entity: Entity, mut style_query: Query<&mut Style>) {
    if let Ok(mut style) = style_query.get_mut(entity) {
        style.display = Display::None;
    }
}

pub fn show_menu(overlay_data: Res<OverlayData>, style_query: Query<&mut Style>)
{
    show(overlay_data.ui_menu, style_query);
}

pub fn hide_menu(overlay_data: Res<OverlayData>, style_query: Query<&mut Style>)
{
    hide(overlay_data.ui_menu, style_query);
}

pub fn show_settings(overlay_data: Res<OverlayData>, style_query: Query<&mut Style>)
{
    show(overlay_data.ui_settings, style_query);
}

pub fn hide_settings(overlay_data: Res<OverlayData>, style_query: Query<&mut Style>)
{
    hide(overlay_data.ui_settings, style_query);
}

pub fn show_save(overlay_data: Res<OverlayData>, style_query: Query<&mut Style>)
{
    show(overlay_data.ui_save, style_query);
}

pub fn hide_save(overlay_data: Res<OverlayData>, style_query: Query<&mut Style>)
{
    hide(overlay_data.ui_save, style_query);
}

pub fn show_load(overlay_data: Res<OverlayData>, style_query: Query<&mut Style>)
{
    show(overlay_data.ui_load, style_query);
}

pub fn hide_load(overlay_data: Res<OverlayData>, style_query: Query<&mut Style>)
{
    hide(overlay_data.ui_load, style_query);
}

// endregion

pub fn load_buttons(
    mut commands: Commands,
    mut overlay_state: ResMut<State<OverlayState>>,
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
                *color = Color::WHITE.into();
                commands.insert_resource(mark.clone());
                overlay_state.set(OverlayState::Hidden);
            }
            Interaction::Hovered => {
                *color = Color::ANTIQUE_WHITE.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}

pub fn cleanse_saves_listener(
    mut commands: Commands,
    mut events: EventReader<CleanseSavesEvent>,
    mut send: EventWriter<RespawnSavesEvent>,
    mut save_query: Query<Entity, (With<SaveItemsParentMark>, Without<LoadItemsParentMark>)>,
    mut load_query: Query<Entity, (With<LoadItemsParentMark>, Without<SaveItemsParentMark>)>,
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
    mut save_query: Query<Entity, (With<SaveItemsParentMark>, Without<LoadItemsParentMark>)>,
    mut load_query: Query<Entity, (With<LoadItemsParentMark>, Without<SaveItemsParentMark>)>,
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

pub fn save_buttons(
    mut commands: Commands,
    mut overlay_state: ResMut<State<OverlayState>>,
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
                *color = Color::WHITE.into();
                commands.insert_resource(mark.clone());
                save_events.send(CleanseSavesEvent);
            }
            Interaction::Hovered => {
                *color = Color::ANTIQUE_WHITE.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}
