#![allow(dead_code)]

use crate::widgets::base::{BuildContext, Widget};
use crate::shorts::utility::Rect;
use sdl2::render::WindowCanvas;
use std::borrow::BorrowMut;

fn render_vec(children: &mut Vec<Box<dyn Widget>>, canvas: &mut WindowCanvas) -> Result<(), String> {
    let mut errors = String::new();
    children.iter_mut().for_each(|e| {
        match e.render(canvas) {
            Ok(_) => {}
            Err(s) => { errors.push_str(s.as_str()) }
        }
    });
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn fmt_vec(children: &Vec<Box<dyn Widget>>) -> String {
    let mut s = String::new();
    children.iter()
        .for_each(|e| { s.push_str(e.str().as_str()) });
    s
}

pub struct RowWidget {
    children: Vec<Box<dyn Widget>>,

    context: Option<BuildContext>,
}

impl RowWidget {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Box<RowWidget> {
        Box::new(RowWidget {
            children,
            context: None,
        })
    }
}

impl Widget for RowWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let mut context = context;
        let mut flex_total = 0;
        let mut static_size = 0;
        self.children.iter_mut().for_each(|e| {
            let rect = e.update(context.clone()).unwrap();
            let flex = e.flex() as u32;
            flex_total += flex;
            if flex == 0 {
                static_size += rect.width();
            }
        });
        let dyn_available = context.rect.width() - static_size;
        let mut dyn_size = 0;
        let mut x = context.rect.x();
        let mut height = 0;
        let mut flex_filled = 0;
        self.children.iter_mut().for_each(|e| {
            let flex = e.flex() as u32;
            let mut rect = context.rect;
            rect.set_x(x);
            if flex == 0 {
                rect = e.update(context.with_rect(rect)).unwrap();
                x += rect.width() as i32;
                if height < rect.height() { height = rect.height() }
            } else {
                let width = if flex_filled + flex == flex_total {
                    // fill all available
                    dyn_available - dyn_size
                } else {
                    dyn_available * flex / flex_total
                };
                rect.set_width(width);
                rect = e.update(context.with_rect(rect)).unwrap();
                x += rect.width() as i32;
                dyn_size += rect.width();
                if height < rect.height() { height = rect.height() }
                flex_filled += flex;
            }
        });
        context.rect.resize(static_size + dyn_size, height);
        self.context.replace(context.clone());
        Ok(context.rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        render_vec(self.children.borrow_mut(), canvas)
    }

    fn rect(&self) -> Rect {
        self.context.as_ref().unwrap().rect
    }

    fn flex(&self) -> u8 {
        0
    }

    fn str(&self) -> String {
        String::from("RowWidget")
    }

    fn fmt(&self) -> String {
        format!("RowWidget({})", fmt_vec(self.children.as_ref()))
    }
}


pub struct ColumnWidget {
    children: Vec<Box<dyn Widget>>,

    context: Option<BuildContext>,
}

impl ColumnWidget {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Box<ColumnWidget> {
        Box::new(ColumnWidget {
            children,
            context: None,
        })
    }
}

impl Widget for ColumnWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let mut context = context;
        let mut flex_total = 0;
        let mut static_size = 0;
        self.children.iter_mut().for_each(|e| {
            let rect = e.update(context.clone()).unwrap();
            let flex = e.flex() as u32;
            flex_total += flex;
            if flex == 0 {
                static_size += rect.height();
            }
        });
        let dyn_available = context.rect.height() - static_size;
        let mut dyn_size = 0;
        let mut y = context.rect.y();
        let mut width = 0;
        let mut flex_filled = 0;
        self.children.iter_mut().for_each(|e| {
            let flex = e.flex() as u32;
            if flex == 0 {
                let mut rect = context.rect;
                rect.set_y(y);
                rect = e.update(context.with_rect(rect)).unwrap();
                y += rect.height() as i32;
                if width < rect.width() { width = rect.width() }
            } else {
                let height = if flex_filled + flex == flex_total {
                    dyn_available - dyn_size
                } else {
                    dyn_available * flex / flex_total
                };
                let mut rect = context.rect;
                rect.set_y(y);
                rect.set_height(height);
                rect = e.update(context.with_rect(rect)).unwrap();
                y += rect.height() as i32;
                dyn_size += rect.height();
                if width < rect.width() { width = rect.width() }
                flex_filled += flex;
            }
        });
        context.rect.resize(width, static_size + dyn_size);
        self.context.replace(context.clone());
        Ok(context.rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        render_vec(self.children.borrow_mut(), canvas)
    }

    fn rect(&self) -> Rect {
        self.context.as_ref().unwrap().rect
    }

    fn flex(&self) -> u8 {
        0
    }

    fn str(&self) -> String {
        String::from("ColumnWidget")
    }

    fn fmt(&self) -> String {
        format!("ColumnWidget({})", fmt_vec(self.children.as_ref()))
    }
}