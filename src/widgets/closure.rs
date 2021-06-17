#![allow(dead_code)]

use crate::widgets::base::{BuildContext, Widget};
use crate::shorts::utility::Rect;
use sdl2::render::WindowCanvas;
use uuid::Uuid;

pub type ClosureType = Box<fn(BuildContext) -> Box<dyn Widget>>;

pub struct ClosureWidget {
    closure: ClosureType,
    calculated: Option<Box<dyn Widget>>,

    context: Option<BuildContext>,
}

impl ClosureWidget {
    pub fn new(closure: ClosureType) -> Box<ClosureWidget> {
        Box::new(ClosureWidget{
            closure,
            calculated: None,
            context: None
        })
    }

    fn build(&self) -> Box<dyn Widget> {
        let context = self.context.as_ref().unwrap().clone();
        (self.closure)(context)
    }
}

impl Widget for ClosureWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        self.context.replace(context.clone());
        self.calculated.replace(self.build());
        self.calculated.as_mut().unwrap().update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.calculated.as_mut().unwrap().render(canvas)
    }

    fn rect(&self) -> Rect {
        self.calculated.as_ref().unwrap().rect()
    }

    fn flex(&self) -> u8 {
        self.calculated.as_ref().unwrap().flex()
    }

    fn str(&self) -> String {
        format!("ClosuresWidget")
    }

    fn fmt(&self) -> String {
        format!("ClosuresWidget{{{}}}", self.calculated.as_ref().unwrap().fmt())
    }
}

pub type SLClosure = Box<fn(BuildContext) -> Box<dyn Widget>>;

/// Stateless widget
/// what build once time
pub struct SLClosuresWidget {
    closure: SLClosure,
    calculated: Option<Box<dyn Widget>>,

    context: Option<BuildContext>,
}

impl SLClosuresWidget {
    pub fn new(closure: SLClosure) -> Box<dyn Widget> {
        Box::new(SLClosuresWidget{
            closure,
            calculated: None,
            context: None
        })
    }
}

impl Widget for SLClosuresWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        if self.calculated.is_none() {
            let inner = (self.closure)(context.clone());
            self.calculated.replace(inner);
        }
        self.calculated.as_mut().unwrap().update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.calculated.as_mut().unwrap().render(canvas)
    }

    fn rect(&self) -> Rect {
        self.calculated.as_ref().unwrap().rect()
    }

    fn flex(&self) -> u8 {
        self.calculated.as_ref().unwrap().flex()
    }

    fn str(&self) -> String {
        format!("SLClosuresWidget({})", self.calculated.as_ref().unwrap().str())
    }

    fn fmt(&self) -> String {
        format!("SLClosuresWidget({})", self.calculated.as_ref().unwrap().fmt())
    }
}

pub type SFKey = Uuid;
pub type SFClosure = Box<fn(BuildContext, SFKey) -> Box<dyn Widget>>;

pub struct SFClosureWidget{
    closure: SFClosure,
    calculated: Option<Box<dyn Widget>>,
    key: SFKey,

    context: Option<BuildContext>,
}

impl SFClosureWidget {
    pub fn new(closure: SFClosure) -> Box<SFClosureWidget> {
        Box::new(SFClosureWidget{
            closure,
            calculated: None,
            key: Uuid::new_v4(),
            context: None
        })
    }
}

impl Widget for SFClosureWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        let wanna_update = context.widgets_states.borrow().known_state(self.key);
        if self.calculated.is_none() || wanna_update {
            context.widgets_states.borrow_mut().register(self.key);
            let inner: Box<dyn Widget> = (self.closure)(context.clone(), self.key);
            self.calculated.replace(inner);
        }
        self.calculated.as_mut().unwrap().update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.calculated.as_mut().unwrap().render(canvas)
    }

    fn rect(&self) -> Rect {
        self.calculated.as_ref().unwrap().rect()
    }

    fn flex(&self) -> u8 {
        self.calculated.as_ref().unwrap().flex()
    }

    fn str(&self) -> String {
        format!("SFClosureWidget({})", self.calculated.as_ref().unwrap().str())
    }

    fn fmt(&self) -> String {
        format!("SFClosureWidget({})", self.calculated.as_ref().unwrap().fmt())
    }
}
