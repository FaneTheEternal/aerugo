#![allow(dead_code)]

use crate::widgets::base::{BuildContext, Widget};
use crate::shorts::utility::Rect;
use crate::widgets::utility::fmt_vec;

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

struct ChildrenCache {
    base_rect: Rect,
    self_rect: Rect,
    children_rects: Vec<Rect>,
}

pub struct RowWidget {
    children: Vec<Box<dyn Widget>>,

    context: Option<BuildContext>,
    cache: Option<ChildrenCache>,
}

impl RowWidget {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Box<RowWidget> {
        Box::new(RowWidget {
            children,
            context: None,
            cache: None,
        })
    }
}

impl Widget for RowWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;

        {  // try pass with cache
            if self.cache.is_some() {
                let cache = self.cache.as_ref().unwrap();
                if cache.base_rect == context.rect {
                    let mut check = true;
                    self.children.iter_mut().enumerate().for_each(|(i, e)| {
                        let cached = cache.children_rects[i].clone();
                        let child = e.update(context.with_rect(cached.clone())).unwrap();
                        check &= cached == child;
                    });
                    if check {
                        return Ok(cache.self_rect);
                    }
                }
            }
        }

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
        let mut children_rects = Vec::new();
        self.children.iter_mut().for_each(|e| {
            let flex = e.flex() as u32;
            let mut rect = context.rect;
            rect.set_x(x);
            if flex == 0 {
                rect = e.update(context.with_rect(rect)).unwrap();
                x += rect.width() as i32;
                if height < rect.height() { height = rect.height() }
                children_rects.push(rect);
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
                children_rects.push(rect);
            }
        });
        let mut rect = context.rect.clone();
        rect.resize(static_size + dyn_size, height);
        self.context.replace(context.with_rect(rect.clone()));
        self.cache.replace(ChildrenCache {
            base_rect: context.rect,
            self_rect: rect,
            children_rects,
        });
        Ok(rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        render_vec(self.children.borrow_mut(), canvas)
    }

    fn touch(self: &mut Self) {
        self.children.iter_mut().for_each(|e| {
            e.touch();
        });
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
    cache: Option<ChildrenCache>,
}

impl ColumnWidget {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Box<ColumnWidget> {
        Box::new(ColumnWidget {
            children,
            context: None,
            cache: None,
        })
    }
}

impl Widget for ColumnWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        {  // try pass with cache
            if self.cache.is_some() {
                let cache = self.cache.as_ref().unwrap();
                if cache.base_rect == context.rect {
                    let mut check = true;
                    for (i, child) in self.children.iter_mut().enumerate() {
                        let cached = cache.children_rects.get(i).unwrap().clone();
                        let child_rect = child.update(context.with_rect(cached.clone()))?;
                        check &= cached == child_rect;
                    }
                    if check {
                        return Ok(cache.self_rect);
                    }
                }
            }
        }
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
        let mut children_rects = Vec::new();
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
            children_rects.push(e.rect());
        });
        let mut rect = context.rect;
        rect.resize(width, static_size + dyn_size);
        // println!("##{}##", self.fmt());
        // println!("{:?}", context.rect);
        // println!("{:?}", rect);
        // println!("{:?}", children_rects);
        self.cache.replace(ChildrenCache {
            base_rect: context.rect,
            self_rect: rect,
            children_rects,
        });
        self.context.replace(context.with_rect(rect));
        Ok(rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        render_vec(self.children.borrow_mut(), canvas)
    }

    fn touch(self: &mut Self) {
        self.children.iter_mut().for_each(|e| {
            e.touch();
        })
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


pub struct StackWidget {
    children: Vec<Box<dyn Widget>>,
    flex: u8,

    context: Option<BuildContext>,
}

impl StackWidget {
    pub fn new<Flex>(children: Vec<Box<dyn Widget>>, flex: Flex) -> Box<StackWidget>
        where Flex: Into<Option<u8>>,
    {
        let flex = match flex.into() {
            None => { 1 }
            Some(n) => { n }
        };
        Box::new(StackWidget {
            children,
            flex,
            context: None,
        })
    }
}

impl Widget for StackWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        let mut errors = Vec::new();
        self.children.iter_mut().for_each(|e| {
            match e.update(context.clone()) {
                Ok(_) => {}
                Err(e) => { errors.push(e) }
            }
        });
        if errors.is_empty() {
            Ok(context.rect)
        } else {
            Err(errors.join("; "))
        }
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        render_vec(self.children.as_mut(), canvas)
    }

    fn touch(self: &mut Self) {
        self.children.iter_mut().for_each(|e| {
            e.touch();
        })
    }

    fn rect(&self) -> Rect {
        self.context.as_ref().unwrap().rect
    }

    fn flex(&self) -> u8 {
        self.flex
    }

    fn str(&self) -> String {
        format!("StackWidget")
    }

    fn fmt(&self) -> String {
        format!("StackWidget({})", fmt_vec(self.children.as_ref()))
    }
}
