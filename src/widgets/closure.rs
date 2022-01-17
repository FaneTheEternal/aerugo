#![allow(dead_code)]

use crate::widgets::base::{BuildContext, _Widget, Widget};
use crate::shorts::utility::Rect;
use sdl2::render::WindowCanvas;
use uuid::Uuid;

pub type _Closure = fn(BuildContext);
pub type Closure = Box<dyn FnMut(BuildContext)>;

pub struct ClosureWidget {
    child: Widget,
    closure: Closure,

    context: Option<BuildContext>,
}

impl ClosureWidget {
    pub fn new(child: Widget, closure: Closure) -> Box<ClosureWidget> {
        Box::new(ClosureWidget {
            child,
            closure,
            context: None,
        })
    }
}

impl _Widget for ClosureWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        self.context.replace(context.clone());
        self.child.update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.child.render(canvas)
    }

    fn touch(self: &mut Self) {
        self.child.touch();
        if self.context.is_some() {
            (self.closure)(self.context.clone().unwrap())
        }
    }

    fn rect(&self) -> Rect {
        self.child.rect()
    }

    fn flex(&self) -> u8 {
        self.child.flex()
    }

    fn str(&self) -> String {
        format!("ClosuresWidget")
    }

    fn fmt(&self) -> String {
        format!("ClosuresWidget({})", self.child.str())
    }
}

/// Type for defining
pub type _SLClosure = fn(BuildContext) -> Widget;

pub type SLClosure = Box<dyn FnMut(BuildContext) -> Box<dyn _Widget>>;

/// Stateless widget
/// what build once time
pub struct SLClosureWidget {
    closure: SLClosure,
    calculated: Option<Box<dyn _Widget>>,

    context: Option<BuildContext>,
}

impl SLClosureWidget {
    pub fn new(closure: SLClosure) -> Widget {
        Box::new(SLClosureWidget {
            closure,
            calculated: None,
            context: None,
        })
    }
}

impl _Widget for SLClosureWidget {
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

    fn touch(self: &mut Self) {
        self.calculated.as_mut().unwrap().touch()
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
/// Type for defining
pub type _SFClosure = fn(BuildContext, SFKey) -> Widget;
pub type SFClosure = Box<dyn FnMut(BuildContext, SFKey) -> Box<dyn _Widget>>;

pub struct SFClosureWidget {
    closure: SFClosure,
    calculated: Option<Box<dyn _Widget>>,
    key: SFKey,

    context: Option<BuildContext>,
}

impl SFClosureWidget {
    pub fn new(closure: SFClosure) -> Box<SFClosureWidget> {
        Box::new(SFClosureWidget {
            closure,
            calculated: None,
            key: Uuid::new_v4(),
            context: None,
        })
    }
}

impl _Widget for SFClosureWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let context = context;
        let wanna_update = context.widgets_states.borrow().known_state(self.key);
        if self.calculated.is_none() || wanna_update {
            context.widgets_states.borrow_mut().register(self.key);
            let inner: Widget = (self.closure)(context.clone(), self.key);
            self.calculated.replace(inner);
        }
        self.calculated.as_mut().unwrap().update(context)
    }

    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.calculated.as_mut().unwrap().render(canvas)
    }

    fn touch(self: &mut Self) {
        self.calculated.as_mut().unwrap().touch()
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
