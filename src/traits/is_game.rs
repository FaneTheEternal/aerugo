#![allow(dead_code)]
extern crate sdl2;

use sdl2::keyboard::Keycode;

use crate::control::user_actions::UserActions;
use crate::types::utility::GameState;
use self::sdl2::surface::Surface;
use std::path::Path;
use std::sync::Mutex;
use self::sdl2::render::WindowCanvas;
use self::sdl2::image::{LoadTexture};


pub trait IsGame {
    fn new() -> Self where Self: Sized;
    fn init(&mut self, canvas: WindowCanvas);
    fn borrow_canvas(&mut self) -> WindowCanvas;
    fn release(&mut self, canvas: WindowCanvas);
    fn get_title(self: &Self) -> String;
    fn tick(self: &mut Self, _actions: &UserActions) -> GameState;
}

pub struct Game {
    canvas: Mutex<Option<WindowCanvas>>,
}

pub struct SimpleGame<'a> {
    base: Game,

    wink: Surface<'a>,
    wink_path: &'a Path,
}

impl<'a> IsGame for SimpleGame<'a> {
    fn new() -> SimpleGame<'a> {
        let base = Game {
            canvas: Mutex::new(None),
        };
        let wink_path = Path::new("assets/wink.png");
        let simple_game = SimpleGame {
            base,
            wink: Surface::load_bmp(Path::new("bpm/wink_clean.bmp")).expect("Cant load wink file"),
            wink_path,
        };
        simple_game
    }

    fn init(&mut self, canvas: WindowCanvas) {
        self.base.canvas.get_mut().unwrap().replace(canvas);
    }

    fn borrow_canvas(&mut self) -> WindowCanvas {
        self.base.canvas.lock().unwrap().take().expect("Haven't canvas")
    }

    fn release(&mut self, canvas: WindowCanvas) {
        let mut _canvas = self.base.canvas.lock().unwrap();
        (*_canvas).replace(canvas);
        (*_canvas).as_mut().unwrap().present()
    }

    fn get_title(&self) -> String {
        "Simple Game".to_string()
    }

    fn tick(&mut self, _actions: &UserActions) -> GameState {
        let mut state = GameState::NOP;
        let mut canvas = self.borrow_canvas();
        for key in _actions.dump_keys() {
            match key {
                Keycode::Escape => state = GameState::Exit,
                _ => {}
            }
        }
        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(100, 100, 100, 255));
        canvas.clear();

        let _screen_size = canvas.output_size().unwrap();

        let texture_wink = texture_creator.load_texture(self.wink_path).unwrap();
        canvas.copy(&texture_wink, None, None).unwrap();

        self.release(canvas);
        state
    }
}
