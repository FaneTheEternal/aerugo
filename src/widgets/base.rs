#![allow(dead_code)]

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use std::rc::Rc;
use crate::control::user_actions::UserActions;
use sdl2::ttf::Sdl2TtfContext;
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;
use crate::fabula::core::CoreMachine;
use crate::wrapper::main_wrapper::SVGPreload;
use crate::wrapper::aerugo_wrapper::AerugoCore;

pub trait _Widget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String>;
    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String>;
    fn touch(self: &mut Self);
    fn rect(&self) -> Rect;
    fn flex(&self) -> u8;
    fn str(&self) -> String;
    fn fmt(&self) -> String;
}

/// type of typical widget
pub type Widget = Box<dyn _Widget>;

#[derive(Clone)]
pub struct BuildContext {
    pub interactions: Option<Rc<UserActions>>,
    pub creator: Rc<TextureCreator<WindowContext>>,
    pub ttf_context: Rc<Sdl2TtfContext>,
    pub rect: Rect,
    pub abs_rect: Rc<Rect>,

    pub widgets_states: Rc<RefCell<WidgetsStatesInspector>>,

    pub state_machine: Rc<RefCell<CoreMachine>>,
    pub aerugo: Rc<RefCell<AerugoCore>>,

    pub svgs: Rc<SVGPreload>,
}

impl BuildContext {
    pub fn ini(creator: Rc<TextureCreator<WindowContext>>,
               rect: Rect,
               svgs: SVGPreload,
               aerugo: Rc<RefCell<AerugoCore>>,
    ) -> BuildContext {
        let aerugo = aerugo;
        let ttf_context: Rc<Sdl2TtfContext> = aerugo.borrow_mut().ttf_context.clone();
        BuildContext {
            interactions: None,
            creator,
            ttf_context,
            rect,
            abs_rect: Rc::new(rect),
            widgets_states: Rc::new(RefCell::new(WidgetsStatesInspector::new())),
            state_machine: Rc::new(RefCell::new(CoreMachine::new())),
            aerugo,
            svgs: Rc::new(svgs),
        }
    }

    pub fn update(&mut self, interactions: UserActions) {
        self.interactions.replace(Rc::from(interactions));
    }

    pub fn with_rect(&self, rect: Rect) -> BuildContext {
        let mut context = self.clone();
        context.rect = rect;
        context
    }
}

pub struct WidgetsStatesInspector {
    states: HashMap<Uuid, bool>,
}

impl WidgetsStatesInspector {
    pub fn new() -> WidgetsStatesInspector {
        WidgetsStatesInspector { states: Default::default() }
    }

    pub fn register(&mut self, key: Uuid) {
        self.states.insert(key, false);
    }

    pub fn known_state(&self, key: Uuid) -> bool {
        match self.states.get(&key) {
            None => { false }
            Some(b) => { b.clone() }
        }
    }

    pub fn update(&mut self, key: Uuid) {
        if self.states.contains_key(&key) {
            self.states.insert(key, true);
        }
    }
}

/// Simple widget stub
/// with size (0, 0)
pub struct StubWidget {
    context: Option<BuildContext>,
}

impl StubWidget {
    pub fn new() -> Widget {
        Box::new(StubWidget { context: None })
    }
}

impl _Widget for StubWidget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String> {
        let mut context = context;
        context.rect.resize(0, 0);
        self.context.replace(context);
        Ok(self.rect())
    }

    fn render(self: &mut Self, _canvas: &mut WindowCanvas) -> Result<(), String> {
        Ok(())
    }

    fn touch(self: &mut Self) {}

    fn rect(&self) -> Rect {
        self.context.as_ref().unwrap().rect
    }

    fn flex(&self) -> u8 {
        0
    }

    fn str(&self) -> String {
        format!("StubWidget")
    }

    fn fmt(&self) -> String {
        format!("StubWidget")
    }
}
