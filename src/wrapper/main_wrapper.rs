use std::time::{Duration};

use sdl2::{VideoSubsystem};
use sdl2::Sdl;
use sdl2::event::Event;

use crate::control::user_actions::UserActions;
use crate::traits::is_game::{IsGame, SimpleGame};
use crate::types::utility::GameState;
use sdl2::image::InitFlag;


#[allow(dead_code)]
pub struct MainWrapper {
    user_actions: UserActions,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,

    pps: u8,
    // Polls per second
    pps_ns: u32,    // nanoseconds between Polls
}

type PpsOpt = Option<u8>;

#[allow(dead_code)]
impl MainWrapper {
    pub fn new(_pps: PpsOpt) -> MainWrapper {
        let pps = match _pps {
            Some(n) => n,
            _ => 144u8,
        };
        MainWrapper::build(pps)
    }

    fn build(pps: u8) -> MainWrapper {
        let user_actions = UserActions::new();
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let pps_ns = 1_000_000_000u32 / pps as u32;

        MainWrapper {
            user_actions,
            sdl_context,
            video_subsystem,
            pps,
            pps_ns,
        }
    }

    pub fn run(&mut self) {
        // let (tx, rx) = mpsc::channel();

        let mut game = SimpleGame::new();
        let title = game.get_title();
        let window = self.video_subsystem.window(&title, 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let builder = window.into_canvas();
        let canvas = builder.build().expect("Cant create canvas");
        game.init(canvas);

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'main_loop;
                    }
                    Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => self.user_actions.push_key(keycode),
                    Event::KeyUp { keycode: Some(keycode), repeat: false, .. } => self.user_actions.release_key(keycode),
                    Event::MouseMotion { x, y, .. } => self.user_actions.move_mouse(x, y),
                    Event::MouseButtonDown { mouse_btn, .. } => self.user_actions.push_mouse(mouse_btn),
                    Event::MouseButtonUp { mouse_btn, .. } => self.user_actions.release_mouse(mouse_btn),
                    _ => {}
                }
            }
            let state = game.tick(&self.user_actions);
            match state {
                GameState::NOP => {}
                GameState::Exit => break 'main_loop,
                _ => println!("Mute unexpected game state!"),
            }
            ::std::thread::sleep(Duration::new(0, self.pps_ns));
        }
    }
}