use crate::widgets::base::{Widget, BuildContext};
use crate::widgets::closure::*;
use crate::widgets::container::*;
use crate::widgets::flex::*;
use sdl2::pixels::Color;
use crate::widgets::text::TextWidget;
use crate::fabula::core::Replies;
use crate::widgets::actions::ButtonWidget;

use crate::shorts::utility::*;
use crate::rect;

pub const FABULA_TEST: _SFClosure = |context, key| {
    let context = context;
    let core = context.state_machine.borrow();

    let mut widgets = Vec::new();
    widgets.push(BACKGROUND(context.clone(), key.clone()));
    widgets.push(READ_MODE(context.clone(), key.clone()));
    if core.in_choices {
        widgets.push(DIALOG_MODE(context.clone(), key.clone()));
    }
    StackWidget::new(widgets, None)
};

pub const READ_FOOTER: _SFClosure = |context, key| {
    ContainerWidget::tight(
        RowWidget::new(vec![
            // left
            ContainerWidget::expand(
                ButtonWidget::text_button_simple(
                    String::from("<"),
                    Box::new(move |context| {
                        let context = context;
                        context.state_machine.borrow_mut().wanna_choice();
                        context.widgets_states.borrow_mut().update(key.clone());
                    }),
                ),
                None,
                None,
                Indent::All(10),
                Color::CYAN,
                None,
            ),
            // middle
            ContainerWidget::expand(
                ContainerWidget::tight(
                    TextWidget::simple(context.state_machine.borrow().verbose()),
                    Indent::All(10),
                    Color::MAGENTA,
                ),
                None,
                None,
                Indent::All(10),
                Color::CYAN,
                4,
            ),
            // right
            ContainerWidget::expand(
                ButtonWidget::text_button_simple(
                    String::from(">"),
                    Box::new(move |context| {
                        let context = context;
                        context.state_machine.borrow_mut().next();
                        context.widgets_states.borrow_mut().update(key.clone());
                    }),
                ),
                None,
                None,
                Indent::All(10),
                Color::CYAN,
                None,
            )
        ]),
        Indent::All(10),
        Color::BLACK,
    )
};

pub const READ_MODE: _SFClosure = |context, key| {
    let key = key;
    let context = context;
    let core = context.state_machine.borrow();

    ColumnWidget::new(
        vec![
            ContainerWidget::expand(
                TextWidget::simple(String::from(" ")),
                CrossAxisX::Center,
                CrossAxisY::Center,
                None,
                None,
                3,
            ),
            ContainerWidget::expand(
                READ_FOOTER(context.clone(), key.clone()),
                None,
                None,
                None,
                None,
                None,
            )
        ]
    )
};

pub const DIALOG_MODE: _SFClosure = |_context, key| {
    fn build_choices(key: SFKey) -> Vec<Box<dyn Widget>> {
        let choices = vec![Replies::Yo, Replies::Ohayo, Replies::Kawaii];
        let mut v = Vec::<Box<dyn Widget>>::new();
        choices.iter().for_each(|e| {
            let replica = e.clone();
            v.push(
                ContainerWidget::expand(
                    ButtonWidget::text_button_simple(
                        replica.verbose(),
                        Box::new(move |context| {
                            let context = context;
                            context.state_machine.borrow_mut().choice(replica);
                            context.widgets_states.borrow_mut().update(key.clone());
                        }),
                    ),
                    CrossAxisX::Center,
                    CrossAxisY::Center,
                    Indent::Y(20),
                    Color::RGBA(200, 200, 0, 150),
                    None,
                )
            )
        });
        v
    }

    ContainerWidget::expand(
        ColumnWidget::new(build_choices(key.clone())),
        CrossAxisX::Center,
        CrossAxisY::Center,
        Indent::Y(100),
        Color::RGBA(0, 0, 0, 150),
        None,
    )
};

pub const BACKGROUND: _SFClosure = |context, key| {
    ContainerWidget::center(
        TextWidget::simple(String::from("Some pic"))
    )
};
