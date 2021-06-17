#![allow(dead_code)]

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use std::rc::Rc;
use crate::control::user_actions::UserActions;
use sdl2::ttf::Sdl2TtfContext;
use std::cell::RefCell;
use crate::types::state_machine::SimpleCore;
use std::collections::HashMap;
use uuid::Uuid;

pub trait Widget {
    fn update(self: &mut Self, context: BuildContext) -> Result<Rect, String>;
    fn render(self: &mut Self, canvas: &mut WindowCanvas) -> Result<(), String>;
    fn rect(&self) -> Rect;
    fn flex(&self) -> u8;
    fn str(&self) -> String;
    fn fmt(&self) -> String;
}

#[derive(Clone)]
pub struct BuildContext {
    pub interactions: Option<UserActions>,
    pub creator: Rc<TextureCreator<WindowContext>>,
    pub ttf_context: Rc<Sdl2TtfContext>,
    pub rect: Rect,
    pub abs_rect: Rc<Rect>,

    pub widgets_states: Rc<RefCell<WidgetsStatesInspector>>,

    pub state_machine: Rc<RefCell<SimpleCore>>,
}

impl BuildContext {
    pub fn ini(creator: Rc<TextureCreator<WindowContext>>,
               ttf_context: Rc<Sdl2TtfContext>,
               rect: Rect,
    ) -> BuildContext {
        BuildContext {
            interactions: None,
            creator,
            ttf_context,
            rect,
            abs_rect: Rc::new(rect),
            widgets_states: Rc::new(RefCell::new(WidgetsStatesInspector::new())),
            state_machine: SimpleCore::new(),
        }
    }

    pub fn update(&mut self, interactions: UserActions) {
        self.interactions = Some(interactions)
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
