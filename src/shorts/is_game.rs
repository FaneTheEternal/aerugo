#![allow(dead_code, unused_imports)]

extern crate sdl2;

use sdl2::keyboard::Keycode;
use crate::control::user_actions::UserActions;
use crate::types::utility::GameState;
use sdl2::surface::Surface;
use std::path::Path;
use std::sync::Mutex;
use std::rc::Rc;
use sdl2::render::WindowCanvas;
use crate::widgets::root::RootWidget;
use crate::widgets::base::{BuildContext, Widget};

use crate::shorts::utility::*;
use crate::rect;
use self::sdl2::pixels::Color;
use std::time::Instant;


pub trait IsGame {
    fn new() -> Self where Self: Sized;
    fn init(&mut self, canvas: WindowCanvas);
    fn get_title(self: &Self) -> String;
    fn tick_actions(self: &mut Self, _actions: &UserActions) -> GameState;
    fn tick(self: &mut Self) -> GameState;
    fn render(self: &mut Self);
}

pub struct Game {
    canvas: Mutex<Option<WindowCanvas>>,
}

impl Game {
    fn init(&mut self, canvas: WindowCanvas) {
        self.canvas.get_mut().unwrap().replace(canvas);
    }

    fn borrow_canvas(&mut self) -> WindowCanvas {
        self.canvas.lock().unwrap().take().expect("Haven't canvas")
    }

    fn release(&mut self, canvas: WindowCanvas) {
        let mut _canvas = self.canvas.lock().unwrap();
        (*_canvas).replace(canvas);
        (*_canvas).as_mut().unwrap().present()
    }
}

pub struct _SimpleGame<'a> {
    wink: Surface<'a>,
    wink_path: &'a Path,

    context: Option<BuildContext>,
    interface: Option<RootWidget>,
}

pub struct SimpleGame<'a>(_SimpleGame<'a>, Game);

impl<'a> IsGame for SimpleGame<'a> {
    fn new() -> SimpleGame<'a> {
        let wink_path = Path::new("assets/wink.png");
        let simple_game = SimpleGame {
            0: _SimpleGame {
                wink: Surface::load_bmp(Path::new("bpm/wink_clean.bmp")).expect("Cant load wink file"),
                wink_path,
                context: None,
                interface: None,
            },
            1: Game {
                canvas: Mutex::new(None),
            },
        };
        simple_game
    }

    fn init(&mut self, canvas: WindowCanvas) {
        self.1.init(canvas);

        let mut canvas = self.1.borrow_canvas();
        let texture_creator = Rc::from(canvas.texture_creator());

        let ttf_context = Rc::from(sdl2::ttf::init().expect("Cant init ttf"));

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(100, 100, 100, 255));
        canvas.clear();

        let (width, height) = canvas.output_size().unwrap();
        let context = BuildContext::ini(texture_creator, ttf_context, rect!(width, height));
        self.0.context.replace(context.clone());
        self.0.interface = Some(RootWidget::new(context));
        self.1.release(canvas);
    }

    fn get_title(&self) -> String {
        "Simple Game".to_string()
    }

    fn tick_actions(&mut self, _actions: &UserActions) -> GameState {
        let mut state = GameState::NOP;
        for key in _actions.dump_keys() {
            match key {
                Keycode::Escape => { state = GameState::Exit }
                _ => {}
            }
        }
        if _actions.old_keyboard.contains(&Keycode::N) | true {
            const DEBUG_FRAME: bool = false;
            const DEBUG_FRAME_PARTIAL: bool = false;
            if DEBUG_FRAME | DEBUG_FRAME_PARTIAL { println!("##############################") }
            let mut start_time = Instant::now();
            let time = Instant::now();
            self.0.interface.as_mut().unwrap().update(_actions.clone());
            if DEBUG_FRAME_PARTIAL {
                println!("Update: {}ms", start_time.elapsed().as_millis());
                start_time = Instant::now();
            }
            self.render();
            if DEBUG_FRAME_PARTIAL {
                println!("Render: {}ms", start_time.elapsed().as_millis());
                start_time = Instant::now();
            }
            self.0.interface.as_mut().unwrap().touch();
            if DEBUG_FRAME_PARTIAL { println!("Touch: {}ms", start_time.elapsed().as_millis()) }
            if DEBUG_FRAME { println!("Frame time: {}ms", time.elapsed().as_millis()) }
            if state == GameState::NOP {
                state = self.0.context.as_ref().unwrap().state_machine.borrow().extract_state();
            }
        }
        state
    }

    fn tick(self: &mut Self) -> GameState {
        let state = GameState::NOP;
        self.render();
        state
    }

    fn render(self: &mut Self) {
        let mut canvas = self.1.borrow_canvas();

        canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        canvas.clear();

        match self.0.interface.as_mut().unwrap().render(&mut canvas) {
            Err(e) => println!("{}", e),
            _ => {}
        }

        self.1.release(canvas);
    }
}
