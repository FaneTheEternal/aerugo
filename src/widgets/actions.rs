#![allow(dead_code)]

use crate::widgets::base::{BuildContext, Widget};
use crate::shorts::utility::Rect;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseButton;

pub type ActionClosure = Box<dyn FnMut(BuildContext)>;

pub struct ActionWidget {
    child: Box<dyn Widget>,
    closure: ActionClosure,

    context: Option<BuildContext>,
}

impl ActionWidget {
    pub fn new(child: Box<dyn Widget>, closure: ActionClosure) -> Box<ActionWidget> {
        Box::new(ActionWidget {
            child,
            closure,
            context: None,
        })
    }
}

impl Widget for ActionWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        (self.closure)(context.clone());
        self.child.update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.child.render(canvas)
    }

    fn rect(&self) -> Rect {
        self.child.rect()
    }

    fn flex(&self) -> u8 {
        self.child.flex()
    }

    fn str(&self) -> String {
        format!("ActionWidget")
    }

    fn fmt(&self) -> String {
        format!("ActionWidget({})", self.child.str())
    }
}

pub type ButtonClosure = Box<dyn FnMut(BuildContext)>;

pub struct ButtonWidget {
    child: Box<dyn Widget>,

    on_hover: ButtonClosure,
    be_hovered: bool,
    is_hover: bool,
    on_free: ButtonClosure,
    is_free: bool,

    on_click: ButtonClosure,
    is_click: bool,
    on_clicked: ButtonClosure,
    is_clicked: bool,

    context: Option<BuildContext>,
}

impl ButtonWidget {
    pub fn new(child: Box<dyn Widget>,
               on_hover: ButtonClosure,
               on_click: ButtonClosure,
               on_clicked: ButtonClosure,
               on_free: ButtonClosure,
    ) -> Box<ButtonWidget> {
        Box::new(ButtonWidget {
            child,
            on_hover,
            on_click,
            is_click: false,
            on_clicked,
            on_free,
            context: None,
            is_hover: false,
            is_free: false,
            is_clicked: false,
            be_hovered: false,
        })
    }

    fn stub() -> ButtonClosure { Box::new(|_context| {}) }

    pub fn simple(child: Box<dyn Widget>,
                  on_clicked: ButtonClosure,
    ) -> Box<ButtonWidget> {
        ButtonWidget::new(
            child,
            ButtonWidget::stub(),
            ButtonWidget::stub(),
            on_clicked,
            ButtonWidget::stub(),
        )
    }
}

impl Widget for ButtonWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        let interactions = context.interactions.as_ref().unwrap();
        let now_hovered = interactions.mouse.is_in(context.rect);
        if now_hovered && !self.be_hovered {
            self.is_hover = true;
        } else if !now_hovered && self.be_hovered {
            self.is_free = true;
        }
        if now_hovered {
            match interactions.mouse.is_pushed(MouseButton::Left) {
                None => {}
                Some(_) => { self.is_click = true }
            }
            match interactions.mouse.is_released(MouseButton::Left) {
                None => {}
                Some(((x1, y1), (x2, y2))) => {
                    if context.rect.contains_point((x1, y1)) &&
                        context.rect.contains_point((x2, y2)) {
                        self.is_clicked = true;
                    }
                }
            }
        }
        let child_rect = self.child.update(context.clone())?;
        self.context.replace(context.with_rect(child_rect));
        Ok(child_rect)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let res = self.child.render(canvas);
        if self.is_hover { (self.on_hover)(self.context.as_ref().unwrap().clone()) }
        if self.is_free { (self.on_free)(self.context.as_ref().unwrap().clone()) }
        if self.is_click { (self.on_click)(self.context.as_ref().unwrap().clone()) }
        if self.is_clicked { (self.on_clicked)(self.context.as_ref().unwrap().clone()) }
        res
    }

    fn rect(&self) -> Rect {
        self.child.rect()
    }

    fn flex(&self) -> u8 {
        self.child.flex()
    }

    fn str(&self) -> String {
        format!("ButtonClosure")
    }

    fn fmt(&self) -> String {
        format!("ButtonClosure({})", self.child.str())
    }
}
