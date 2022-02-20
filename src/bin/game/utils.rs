use std::io::Read;
use std::ops::Not;
use bevy::ecs::schedule::{ShouldRun, StateError};
use bevy::prelude::*;
use aerugo::Aerugo;

pub fn make_button_closure<B>(
    text: &str,
    font: Handle<Font>,
    button: B,
    button_color: Color,
) -> impl FnOnce(&mut ChildBuilder) + '_
    where B: Component
{
    move |parent| {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(60.0)),
                    margin: Rect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: button_color.into(),
                ..Default::default()
            })
            .insert(button)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            text,
                            TextStyle {
                                font,
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
            });
    }
}

pub fn grow_z_index<'closure>(
    deep: u8,
    builder: &mut ChildBuilder,
    tree_style: Style,
    closure: impl FnOnce(&mut ChildBuilder) + 'closure,
) {
    builder
        .spawn_bundle(NodeBundle {
            style: tree_style.clone(),
            color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            if deep == 0 {
                parent
                    .spawn_bundle(NodeBundle {
                        style: tree_style.clone(),
                        color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
                        ..Default::default()
                    })
                    .with_children(closure);
            } else {
                grow_z_index(deep - 1, parent, tree_style, closure);
            }
        });
}

pub fn warn_state_err(err: StateError) -> () {
    match err {
        StateError::AlreadyInState => { warn!("AlreadyInState") }
        StateError::StateAlreadyQueued => {}
        StateError::StackEmpty => { error!("StackEmpty") }
    }
}

pub fn load_aerugo() -> Aerugo {
    const SCENARIO_PATH: &str = "scenario.ron";
    let mut file = std::fs::File::options()
        .read(true).write(true).create(true)
        .open(SCENARIO_PATH)
        .unwrap();
    let mut aerugo = String::new();
    file.read_to_string(&mut aerugo).unwrap();
    ron::from_str(&aerugo).unwrap()
}

pub fn should_run_once(mut run_once: Local<bool>) -> ShouldRun {
    if run_once.not() {
        *run_once = true;
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
