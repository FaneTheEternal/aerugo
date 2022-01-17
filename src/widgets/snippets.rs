#![allow(dead_code, unused_imports)]

use super::prelude::*;
use sdl2::pixels::Color;
use uuid::Uuid;
use crate::shorts::utility::*;
use crate::{rect, set, svg};
use sdl2::mouse::MouseButton;

fn test3() -> Vec<Widget> {
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

pub const ROW3: fn(BuildContext) -> Widget = |_context| {
    ContainerWidget::expand(
        RowWidget::new(test3()),
        None,
        None,
        None,
        None,
        None,
    )
};

pub const COLUMN3: fn(BuildContext) -> Widget = |_context| {
    ContainerWidget::expand(
        ColumnWidget::new(test3()),
        None,
        None,
        None,
        None,
        None,
    )
};

pub const SVG_TEST: _SFClosure = |_, _| {
    let svg: SVG = ("regular".into(), "arrow-alt-circle-left.svg".into());
    SVGWidget::new(svg)
};

pub const SVG_BUTTON: _SLClosure = |_| {
    let default: SVG = svg!("regular", "arrow-alt-circle-left.svg");
    let hover: SVG = svg!("regular", "arrow-alt-circle-right.svg");
    let click: SVG = svg!("regular", "arrow-alt-circle-down.svg");
    ContainerWidget::expand_indent(
        ContainerWidget::center(
            RebuildingButtonWidget::svg(
                default, hover, click,
                Box::new(move |_context| { println!("UP!") }),
                set![MouseButton::Left],
            ),
        ),
        Indent::All(100),
    )
};
