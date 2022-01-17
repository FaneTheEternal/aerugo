#![allow(unused_imports)]

use super::prelude::*;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::control::user_actions::UserActions;
use sdl2::pixels::Color;
use crate::widgets::snippets::*;

pub struct RootWidget {
    context: BuildContext,
    child: Widget,
}

impl _Widget for RootWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        self.context = context.clone();
        self.child.update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.child.render(canvas)
    }

    fn touch(self: &mut Self) {
        self.child.touch()
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
                // SFClosureWidget::new(Box::new(FABULA_TEST)),
                // SFClosureWidget::new(Box::new(SVG_TEST)),
                SLClosureWidget::new(Box::new(SVG_BUTTON)),
                None,
                None,
                Indent::All(0),
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