#![allow(unused_imports)]

use crate::widgets::base::{BuildContext, Widget};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::control::user_actions::UserActions;
use crate::widgets::text::TextWidget;
use crate::widgets::container::*;
use sdl2::pixels::Color;
use crate::widgets::flex::*;
use crate::widgets::closure::{ClosureWidget, SFClosureWidget, SLClosuresWidget};
use crate::widgets::snippets::*;

pub struct RootWidget {
    context: BuildContext,
    child: Box<dyn Widget>,
}

impl Widget for RootWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        self.context = context.clone();
        self.child.update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.child.render(canvas)
    }

    fn rect(&self) -> Rect {
        self.context.rect
    }

    fn flex(&self) -> u8 { 0 }

    fn str(&self) -> String {
        String::from("RootWidget")
    }

    fn fmt(&self) -> String {
        format!("RootWidget({})", self.child.str())
    }
}

impl RootWidget {
    pub fn new(context: BuildContext) -> RootWidget {
        RootWidget {
            context,
            child: ContainerWidget::expand(
                // SLClosuresWidget::new(Box::new(COLUMN3)),
                SFClosureWidget::new(Box::new(SIMPLE)),
                None,
                None,
                Indent::All(10),
                Color::GRAY,
                None,
            ),
        }
    }

    pub fn update(&mut self, actions: UserActions) {
        self.context.update(actions);
        self.child.update(self.context.clone()).expect("Cant update root");
    }
}