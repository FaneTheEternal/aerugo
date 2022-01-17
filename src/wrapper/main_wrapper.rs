use sdl2::{VideoSubsystem};
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::render::BlendMode;

use crate::control::user_actions::UserActions;
use crate::shorts::is_game::{IsGame, SimpleGame};
use crate::types::utility::GameState;

use std::time::{Instant, Duration};
use std::{thread, fs};
use std::path::Path;
use std::collections::HashMap;
use usvg::Tree;
use std::borrow::BorrowMut;


#[allow(dead_code)]
pub struct MainWrapper {
    user_actions: UserActions,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,

    pps: u8,
    // Polls per second
    pps_ns: u64,    // nanoseconds between Polls
}

type PpsOpt = Option<u8>;

#[allow(dead_code)]
impl MainWrapper {
    pub fn new(_pps: PpsOpt) -> MainWrapper {
        let pps = match _pps {
            Some(n) => n,
            _ => 60u8,
        };
        MainWrapper::build(pps)
    }

    fn build(pps: u8) -> MainWrapper {
        let user_actions = UserActions::new();
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let pps_ns = 1_000_000_000u64 / pps as u64;

        MainWrapper {
            user_actions,
            sdl_context,
            video_subsystem,
            pps,
            pps_ns,
        }
    }

    pub fn run(&mut self) {
        let mut game = SimpleGame::new();
        let title = game.get_title();
        let window = self.video_subsystem.window(&title, 800, 600)
            .position_centered()
            // .fullscreen_desktop()
            .allow_highdpi()
            .build()
            .unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let mut builder = window.into_canvas();
        // builder = builder.software();
        builder = builder.accelerated();
        builder = builder.target_texture();
        let mut canvas = builder.build().expect("Cant create canvas");
        canvas.set_blend_mode(BlendMode::Blend);

        let svg = load_svg();
        game.pre_load(svg);

        game.init(canvas);

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        const COUNTING_FPS: bool = true;
        const FPS_BATCH: u64 = 1000;

        let mut start_time = Instant::now();
        let mut fps_counter = 0;

        'main_loop: loop {
            if COUNTING_FPS && fps_counter > FPS_BATCH {
                start_time = Instant::now();
                fps_counter = 0;
            }
            self.user_actions.tick();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'main_loop;
                    }
                    Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => self.user_actions.push_key(keycode),
                    Event::KeyUp { keycode: Some(keycode), repeat: false, .. } => self.user_actions.release_key(keycode),
                    Event::MouseMotion { x, y, .. } => self.user_actions.mouse.r#move(x, y),
                    Event::MouseButtonDown { mouse_btn, .. } => self.user_actions.mouse.click(mouse_btn),
                    Event::MouseButtonUp { mouse_btn, .. } => self.user_actions.mouse.release(mouse_btn),
                    _ => {}
                }
            }
            match game.tick_actions(&self.user_actions.clone()) {
                GameState::NOP => {}
                GameState::Exit => break 'main_loop,
                _ => println!("Mute unexpected game state!"),
            }
            if COUNTING_FPS && fps_counter == FPS_BATCH {
                let duration_time = start_time.elapsed();
                let secs = duration_time.as_secs();
                println!("{} FPS", FPS_BATCH / secs);
            }
            if COUNTING_FPS { fps_counter += 1; }

            thread::sleep(Duration::from_nanos(self.pps_ns));
        }
    }
}

pub type SVGPreload = HashMap<(String, String), Tree>;
pub type SVGPreloadSerialized = HashMap<(String, String), String>;

pub fn load_svg() -> SVGPreload {
    let mut svgs = HashMap::new();

    fn load_type(svgs: &mut SVGPreload, r#type: String) {
        let svg_root: &Path = Path::new("./svgs/");

        let mut opt = usvg::Options::default();
        opt.fontdb.load_system_fonts();

        for entry in fs::read_dir(svg_root.join(r#type)).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let clazz: String = path.parent().unwrap()
                .file_name().unwrap()
                .to_str().unwrap().to_string();
            let name = path.file_name().unwrap()
                .to_str().unwrap().to_string();
            let svg = (clazz, name);

            let svg_data = std::fs::read(&path).unwrap();
            let rtree = usvg::Tree::from_data(&svg_data, &opt).unwrap();

            svgs.insert(svg, rtree);
        }
    }

    // regular
    load_type(svgs.borrow_mut(), String::from("regular"));
    // solid
    load_type(svgs.borrow_mut(), String::from("solid"));

    svgs
}
