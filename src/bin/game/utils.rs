use bevy::ecs::schedule::StateError;
use bevy::prelude::*;

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
                    })
                    .insert(button);
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
