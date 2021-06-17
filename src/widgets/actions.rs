#![allow(dead_code)]

use crate::widgets::base::{BuildContext, Widget};
use crate::shorts::utility::Rect;
use sdl2::render::WindowCanvas;

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
            context: None
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