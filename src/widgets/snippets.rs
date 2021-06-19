#![allow(dead_code, unused_imports)]

use crate::widgets::base::{BuildContext, Widget};
use crate::widgets::container::*;
use crate::widgets::flex::{RowWidget, ColumnWidget};
use crate::widgets::text::TextWidget;
use sdl2::pixels::Color;
use uuid::Uuid;
use crate::shorts::utility::*;
use crate::rect;
use crate::widgets::actions::{ActionWidget, ButtonWidget};
use crate::widgets::closure::*;

fn test3() -> Vec<Box<dyn Widget>> {
    vec![
        ContainerWidget::expand(
            TextWidget::simple(String::from("<1>")),
            CrossAxisX::Center,
            CrossAxisY::Center,
            None,
            Color::RGBA(200, 0, 0, 200),
            None,
        ),
        ContainerWidget::expand(
            TextWidget::simple(String::from("<2>")),
            CrossAxisX::Center,
            CrossAxisY::Center,
            None,
            Color::RGBA(0, 200, 0, 200),
            None,
        ),
        ContainerWidget::expand(
            TextWidget::simple(String::from("<3>")),
            CrossAxisX::Center,
            CrossAxisY::Center,
            None,
            Color::RGBA(0, 0, 200, 200),
            None,
        ),
    ]
}

pub const ROW3: fn(BuildContext) -> Box<dyn Widget> = |_context| {
    ContainerWidget::expand(
        RowWidget::new(test3()),
        None,
        None,
        None,
        None,
        None,
    )
};

pub const COLUMN3: fn(BuildContext) -> Box<dyn Widget> = |_context| {
    ContainerWidget::expand(
        ColumnWidget::new(test3()),
        None,
        None,
        None,
        None,
        None,
    )
};

pub const SIMPLE: fn(BuildContext, Uuid) -> Box<dyn Widget> = |context, key| {
    let context = context;
    let core = context.state_machine.borrow();

    let closure = Box::new(move |context: BuildContext| {
        let context = context;
        let mut core = context.state_machine.borrow_mut();

        core.counter += 1;
        context.widgets_states.borrow_mut().update(key.clone());
    });

    ColumnWidget::new(
        vec![
            ContainerWidget::expand(
                BoundWidget::new(
                    Some(ContainerWidget::colored(
                        TextWidget::simple(format!("Clicked {} times!", core.counter)),
                        Color::CYAN,
                    )),
                    rect!(200, 100),
                ),
                CrossAxisX::Center,
                CrossAxisY::Center,
                None,
                None,
                None,
            ),
            ContainerWidget::expand(
                BoundWidget::new(
                    Some(ContainerWidget::colored(
                        ButtonWidget::simple(
                            BoundWidget::new(None, rect!(200, 100)),
                            closure,
                        ),
                        Color::MAGENTA,
                    )),
                    rect!(200, 100),
                ),
                CrossAxisX::Center,
                CrossAxisY::Center,
                None,
                None,
                None,
            )
        ]
    )
};
