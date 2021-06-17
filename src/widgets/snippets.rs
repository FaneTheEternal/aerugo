#![allow(dead_code)]

use crate::widgets::base::{BuildContext, Widget};
use crate::widgets::container::*;
use crate::widgets::flex::{RowWidget, ColumnWidget};
use crate::widgets::text::TextWidget;
use sdl2::pixels::Color;
use uuid::Uuid;
use crate::shorts::utility::*;
use crate::rect;
use sdl2::keyboard::Keycode;
use crate::widgets::actions::ActionWidget;

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
    let mut bounds = context.rect.clone();
    let a = 100u32;
    bounds.resize(bounds.width() - a, bounds.height() - a);

    let closure = move |context: BuildContext| {
        let context = context;
        let mut core = context.state_machine.borrow_mut();
        let bounds = bounds.clone();
        core.ini(bounds);
        let key = key.clone();
        for keycode in context.interactions.as_ref().unwrap().dump_keys() {
            context.widgets_states.borrow_mut().update(key);
            match keycode {
                Keycode::Up => { core.up(bounds) }
                Keycode::Down => { core.down(bounds) }
                Keycode::Left => { core.left(bounds) }
                Keycode::Right => { core.right(bounds) }
                Keycode::Num0 => { core.re_color(0) }
                Keycode::Num1 => { core.re_color(1) }
                Keycode::Num2 => { core.re_color(2) }
                Keycode::Num3 => { core.re_color(3) }
                Keycode::Num4 => { core.re_color(4) }
                Keycode::Num5 => { core.re_color(5) }
                Keycode::Num6 => { core.re_color(6) }
                Keycode::Num7 => { core.re_color(7) }
                Keycode::Num8 => { core.re_color(8) }
                Keycode::Num9 => { core.re_color(9) }
                _ => {}
            }
        }
    };

    // println!("{}, {}:{}", key, core.x, core.y);
    ContainerWidget::expand(
        ActionWidget::new(
            BoundWidget::new(
                ContainerWidget::expand(
                    TextWidget::simple(String::from("<cube>")),
                    CrossAxisX::Center,
                    CrossAxisY::Center,
                    None,
                    core.color,
                    None,
                ),
                rect!(a, a),
            ),
            Box::new(closure),
        ),
        None,
        None,
        Indent::Partial(core.y.clone() as i32, 0, 0, core.x.clone() as i32),
        None,
        None,
    )
};
