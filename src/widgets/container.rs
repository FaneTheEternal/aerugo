#![allow(dead_code)]

use crate::widgets::base::{BuildContext, Widget};
use crate::shorts::utility::*;
use crate::rect;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum CrossAxisX {
    Center,
    Left,
    Right,
    None,
}

impl std::fmt::Display for CrossAxisX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match *self {
            CrossAxisX::Center => { "Center" }
            CrossAxisX::Left => { "Left" }
            CrossAxisX::Right => { "Right" }
            CrossAxisX::None => { "None" }
        };
        write!(f, "{}", value)
    }
}

impl CrossAxisX {
    pub fn transform(&self, src: Rect, dest: Rect) -> Rect {
        match *self {
            CrossAxisX::Center => {
                rect!(
                    dest.x() + (dest.width() as i32) / 2 - (src.width() as i32) / 2,
                    src.y(),
                    src.width(),
                    src.height()
                )
            }
            CrossAxisX::Left => { rect!(dest.x(), src.y(), src.width(), src.height()) }
            CrossAxisX::Right => {
                rect!(dest.x() + (dest.width() as i32) - (src.width() as i32), src.y(),
                    src.width(), src.height())
            }
            CrossAxisX::None => { rect!(dest.x(), src.y(), src.width(), src.height()) }
        }
    }
}

#[derive(Clone)]
pub enum CrossAxisY {
    Center,
    Top,
    Down,
    None,
}

impl std::fmt::Display for CrossAxisY {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match *self {
            CrossAxisY::Center => { "Center" }
            CrossAxisY::Top => { "Top" }
            CrossAxisY::Down => { "Down" }
            CrossAxisY::None => { "None" }
        };
        write!(f, "{}", value)
    }
}

impl CrossAxisY {
    pub fn transform(&self, src: Rect, dest: Rect) -> Rect {
        match *self {
            CrossAxisY::Center => {
                rect!(
                    src.x(),
                    dest.y() + (dest.height() as i32) / 2 - (src.height() as i32) / 2,
                    src.width(),
                    src.height()
                )
            }
            CrossAxisY::Top => { rect!(src.x(), dest.y(), src.width(), src.height()) }
            CrossAxisY::Down => {
                rect!(src.x(), dest.y() + (dest.height() as i32) - (src.height() as i32),
                    src.width(), src.height())
            }
            CrossAxisY::None => { rect!(src.x(), dest.y(), src.width(), src.height()) }
        }
    }
}

#[derive(Clone)]
pub enum Indent {
    All(i32),
    X(i32),
    Y(i32),
    XY(i32, i32),
    Partial(i32, i32, i32, i32),
    None,
}

impl Indent {
    fn _transform_inside(src: Rect, top: i32, right: i32, down: i32, left: i32) -> Rect {
        let mut x = src.x();
        let mut y = src.y();
        let mut width = src.width();
        let mut height = src.height();
        if src.width() > (left + right) as u32 {
            x += left;
            width -= (left + right) as u32;
        }
        if src.height() > (top + down) as u32 {
            y += top;
            height -= (top + down) as u32;
        }
        rect!(x, y, width, height)
    }

    pub fn transform_inside(&self, src: Rect) -> Rect {
        match *self {
            Indent::All(n) => { Indent::_transform_inside(src, n, n, n, n) }
            Indent::X(x) => { Indent::_transform_inside(src, 0, x, 0, x) }
            Indent::Y(y) => { Indent::_transform_inside(src, y, 0, y, 0) }
            Indent::XY(x, y) => { Indent::_transform_inside(src, y, x, y, x) }
            Indent::Partial(top, right, down, left) => {
                Indent::_transform_inside(src, top, right, down, left)
            }
            Indent::None => { Indent::_transform_inside(src, 0, 0, 0, 0) }
        }
    }

    fn _transform_outer(src: Rect, bound: Rect, top: i32, right: i32, down: i32, left: i32) -> Rect {
        let mut width = src.width() + (left + right) as u32;
        if width > bound.width() { width = bound.width() }
        let mut height = src.height() + (top + down) as u32;
        if height > bound.height() { height = bound.height() }
        rect!(
            src.x(),
            src.y(),
            width,
            height
        )
    }

    pub fn transform_outer(&self, src: Rect, bound: Rect) -> Rect {
        match *self {
            Indent::All(n) => { Indent::_transform_outer(src, bound, n, n, n, n) }
            Indent::X(x) => { Indent::_transform_outer(src, bound, 0, x, 0, x) }
            Indent::Y(y) => { Indent::_transform_outer(src, bound, y, 0, y, 0) }
            Indent::XY(x, y) => { Indent::_transform_outer(src, bound, y, x, y, x) }
            Indent::Partial(top, right, down, left) => {
                Indent::_transform_outer(src, bound, top, right, down, left)
            }
            Indent::None => { Indent::_transform_outer(src, bound, 0, 0, 0, 0) }
        }
    }
}

pub struct ContainerWidget {
    child: Box<dyn Widget>,

    cross_axis_x: CrossAxisX,
    cross_axis_y: CrossAxisY,
    indent: Indent,
    color: Color,
    tight: bool,

    flex: u8,

    context: Option<BuildContext>,
}

impl ContainerWidget {
    pub fn new<CrossAxisX_, CrossAxisY_, Indent_, Color_, Flex_>(
        child: Box<dyn Widget>,
        cross_axis_x: CrossAxisX_,
        cross_axis_y: CrossAxisY_,
        indent: Indent_,
        color: Color_,
        flex: Flex_) -> Box<ContainerWidget>
        where
            CrossAxisX_: Into<Option<CrossAxisX>>,
            CrossAxisY_: Into<Option<CrossAxisY>>,
            Indent_: Into<Option<Indent>>,
            Color_: Into<Option<Color>>,
            Flex_: Into<Option<u8>>,
    {
        let cross_axis_x = match cross_axis_x.into() {
            Some(a) => { a }
            None => { CrossAxisX::None }
        };
        let cross_axis_y = match cross_axis_y.into() {
            Some(a) => { a }
            None => { CrossAxisY::None }
        };
        let indent = match indent.into() {
            Some(i) => { i }
            None => Indent::None
        };
        let color = match color.into() {
            Some(c) => { c }
            None => Color::RGBA(0, 0, 0, 0)
        };
        let flex = match flex.into() {
            Some(f) => { f }
            None => { 0 }
        };
        let tight = match flex {
            0 => true,
            _ => false,
        };
        Box::new(ContainerWidget {
            child,
            cross_axis_x,
            cross_axis_y,
            indent,
            color,
            context: None,
            tight,
            flex,
        })
    }

    pub fn tight<Indent_, Color_>(
        child: Box<dyn Widget>,
        indent: Indent_,
        color: Color_, ) -> Box<ContainerWidget>
        where
            Indent_: Into<Option<Indent>>,
            Color_: Into<Option<Color>>,
    {
        let cross_axis_x = CrossAxisX::None;
        let cross_axis_y = CrossAxisY::None;
        let indent = match indent.into() {
            Some(i) => { i }
            None => Indent::None
        };
        let color = match color.into() {
            Some(c) => { c }
            None => Color::RGBA(0, 0, 0, 0)
        };
        let flex = 0;
        let tight = true;
        Box::new(ContainerWidget {
            child,
            cross_axis_x,
            cross_axis_y,
            indent,
            color,
            context: None,
            tight,
            flex,
        })
    }

    pub fn expand<CrossAxisX_, CrossAxisY_, Indent_, Color_, Flex_>(
        child: Box<dyn Widget>,
        cross_axis_x: CrossAxisX_,
        cross_axis_y: CrossAxisY_,
        indent: Indent_,
        color: Color_,
        flex: Flex_) -> Box<ContainerWidget>
        where
            CrossAxisX_: Into<Option<CrossAxisX>>,
            CrossAxisY_: Into<Option<CrossAxisY>>,
            Indent_: Into<Option<Indent>>,
            Color_: Into<Option<Color>>,
            Flex_: Into<Option<u8>>,
    {
        let cross_axis_x = match cross_axis_x.into() {
            Some(a) => { a }
            None => { CrossAxisX::None }
        };
        let cross_axis_y = match cross_axis_y.into() {
            Some(a) => { a }
            None => { CrossAxisY::None }
        };
        let indent = match indent.into() {
            Some(i) => { i }
            None => Indent::None
        };
        let color = match color.into() {
            Some(c) => { c }
            None => Color::RGBA(0, 0, 0, 0)
        };
        let flex = match flex.into() {
            Some(f) => { f }
            None => { 1 }
        };
        let tight = false;
        Box::new(ContainerWidget {
            child,
            cross_axis_x,
            cross_axis_y,
            indent,
            color,
            context: None,
            tight,
            flex,
        })
    }
}

impl Widget for ContainerWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        if self.tight {
            let mut child_rect = match self.child.update(context.clone()) {
                Ok(r) => { r }
                Err(e) => { return Err(e); }
            };
            let rect = self.indent.transform_outer(child_rect, context.rect);
            child_rect.center_on(rect.center());
            self.child.update(context.with_rect(child_rect))?;
            self.context.replace(context.with_rect(rect));
            Ok(rect)
        } else {
            let rect = self.indent.transform_inside(context.rect);
            let mut child_rect = match self.child.update(context.with_rect(rect)) {
                Ok(r) => { r }
                Err(e) => { return Err(e); }
            };
            child_rect = self.cross_axis_x.transform(child_rect, rect);
            child_rect = self.cross_axis_y.transform(child_rect, rect);
            self.child.update(context.with_rect(child_rect))?;
            // println!("{}: {:?}|{:?}", self.fmt(), context.rect, child_rect);
            self.context.replace(context.with_rect(rect));
            Ok(context.rect)
        }
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let rect = self.rect();
        let color = canvas.draw_color();
        canvas.set_draw_color(self.color);
        if self.tight {
            canvas.fill_rect(self.child.rect())?;
        } else {
            canvas.fill_rect(rect)?;
        }
        canvas.set_draw_color(color);
        self.child.render(canvas)
    }

    fn rect(&self) -> Rect {
        self.context.as_ref().unwrap().rect.clone()
    }

    fn flex(&self) -> u8 {
        self.flex
    }

    fn str(&self) -> String {
        String::from("ContainerWidget")
    }

    fn fmt(&self) -> String {
        format!("ContainerWidget({})", self.child.str())
    }
}

pub struct BoundWidget {
    child: Box<dyn Widget>,
    bound: Rect,

    context: Option<BuildContext>,
}

impl BoundWidget {
    pub fn new(child: Box<dyn Widget>, bound: Rect) -> Box<dyn Widget> {
        Box::new(BoundWidget { child, bound, context: None })
    }
}

impl Widget for BoundWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let mut context = context;
        context.rect.resize(self.bound.width(), self.bound.height());
        self.child.update(context.clone())?;
        Ok(context.rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.child.render(canvas)
    }

    fn rect(&self) -> Rect {
        self.context.as_ref().unwrap().rect
    }

    fn flex(&self) -> u8 {
        0
    }

    fn str(&self) -> String {
        format!("BoundWidget")
    }

    fn fmt(&self) -> String {
        format!("BoundWidget({})", self.child.str())
    }
}