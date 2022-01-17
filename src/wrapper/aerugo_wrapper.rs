#![allow(dead_code)]

use std::rc::Rc;

use sdl2::Sdl;
use sdl2::image::InitFlag;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::event::Event;

use crate::control::user_actions::UserActions;
use super::main_wrapper::{SVGPreload, SVGPreloadSerialized, load_svg};
use crate::widgets::prelude::*;
use crate::widgets::aerugo::AERUGO;

use crate::shorts::utility::*;
use crate::rect;
use std::time::{Duration, Instant};
use std::cell::RefCell;
use crate::types::utility::FutureLoader;
use sdl2::keyboard::Keycode;

pub struct Aerugo {
    /// User interactions
    interactions: UserActions,
    /// Media API
    sdl: Sdl,
    /// Manager of resources
    core: Rc<RefCell<AerugoCore>>,
}

impl Aerugo {
    pub fn new() -> Aerugo {
        let interactions = UserActions::new();
        let sdl = sdl2::init().unwrap();
        Aerugo {
            interactions,
            sdl,
            core: Rc::new(RefCell::new(AerugoCore::new())),
        }
    }

    pub fn run(self) -> Result<(), String> {
        let mut engine = self;
        let video_subsystem = engine.sdl.video().unwrap();
        let window = video_subsystem.window("Aerugo", 800, 600)
            .position_centered()
            // .fullscreen_desktop()
            .build().unwrap();
        let mut canvas = window.into_canvas()
            // .accelerated()
            .build().unwrap();

        // allow alpha
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let mut event_pump = engine.sdl.event_pump().unwrap();

        const CAPTURE_FPS: bool = false;
        const COUNTING_FPS: bool = true;
        const FPS_BATCH: u64 = 1000;

        let mut start_time = Instant::now();
        let mut fps_counter = 0;

        let creator = canvas.texture_creator();
        let (w, h) = canvas.output_size()?;
        let mut context = BuildContext::ini(
            Rc::from(creator),
            rect!(w, h),
            SVGPreload::default(),  // awhile empty
            engine.core.clone(),
        );
        let mut interface = SFClosureWidget::new(Box::new(AERUGO));

        'game: loop {
            if COUNTING_FPS { fps_counter += 1 }
            {
                let mut core = engine.core.borrow_mut();
                core.tick();

                // try load resources
                if core.svg.is_some() & context.svgs.is_empty() {
                    context.svgs = Rc::from(core.svg().unwrap().clone())
                }
            }

            engine.interactions.tick();

            for e in event_pump.poll_iter() {
                match e {
                    Event::Quit { .. } => {
                        break 'game;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Backquote), .. } => { break 'game; }
                    Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                        engine.interactions.push_key(keycode)
                    }
                    Event::KeyUp { keycode: Some(keycode), repeat: false, .. } => {
                        engine.interactions.release_key(keycode)
                    }
                    Event::MouseMotion { x, y, .. } => {
                        engine.interactions.mouse.r#move(x, y)
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        engine.interactions.mouse.click(mouse_btn)
                    }
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        engine.interactions.mouse.release(mouse_btn)
                    }
                    _ => {}
                }
            }

            context.update(engine.interactions.clone());
            interface.update(context.clone());

            canvas.clear();
            interface.render(&mut canvas);
            canvas.present();

            interface.touch();


            if COUNTING_FPS & (FPS_BATCH == fps_counter) {
                let fps = (fps_counter as f32) / start_time.elapsed().as_secs_f32();
                println!("FPS: {}", fps as u32);
                start_time = Instant::now();
                fps_counter = 0;
            }

            std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 1024));
        }

        Ok(())
    }
}

struct SVGLoader {
    loader: FutureLoader<SVGPreloadSerialized>,
    loaded: SVGPreload,
}

impl SVGLoader {
    pub fn make() -> SVGLoader {
        fn f() -> SVGPreloadSerialized {
            let svg = load_svg();
            let serialized: SVGPreloadSerialized = svg.iter().map(|(key, tree)| {
                let str_tree = tree.to_string(&usvg::XmlOptions::default());
                (key.clone(), str_tree)
            }).collect();
            serialized
        }
        let loader = FutureLoader::make(f);
        SVGLoader { loader, loaded: Default::default() }
    }

    pub fn touch(&mut self) -> Option<SVGPreload> {
        if self.loaded.is_empty() {
            match self.loader.touch() {
                None => { return None; }
                Some(v) => {
                    self.loaded = v.iter().map(|(key, tree)| {
                        (
                            key.clone(),
                            usvg::Tree::from_str(tree, &usvg::Options::default()).unwrap()
                        )
                    }).collect();
                }
            }
        }
        Some(self.loaded.clone())
    }
}


pub struct AerugoCore {
    svg: Option<SVGPreload>,
    svg_loader: SVGLoader,
    await_loader: Option<FutureLoader<()>>,
    await_loader_passed: bool,
    pub image_context: sdl2::image::Sdl2ImageContext,
    pub ttf_context: Rc<Sdl2TtfContext>,
}

impl AerugoCore {
    pub fn new() -> AerugoCore {
        // ini all flags
        let image_flags = InitFlag::JPG | InitFlag::PNG | InitFlag::TIF | InitFlag::WEBP;
        let image_context = sdl2::image::init(image_flags).unwrap();
        let ttf_context = Rc::from(sdl2::ttf::init().unwrap());
        AerugoCore {
            svg: None,
            svg_loader: SVGLoader::make(),
            await_loader: None,
            await_loader_passed: false,
            image_context,
            ttf_context,
        }
    }

    pub fn tick(&mut self) {
        // check SVG
        let mut start_await = false;
        if self.svg.is_none() {
            self.svg = self.svg_loader.touch();
            if self.svg.is_some() { start_await = true; }
        }

        if start_await {
            self.await_loader.replace(FutureLoader::make(|| {
                std::thread::sleep(Duration::from_secs(0));
            }));
        }

        if self.await_loader.is_some() & !self.await_loader_passed {
            match self.await_loader.as_mut().unwrap().touch() {
                None => {}
                Some(_) => { self.await_loader_passed = true; }
            }
        }
    }

    pub fn svg(&self) -> Option<&SVGPreload> {
        self.svg.as_ref()
    }

    pub fn resources_loaded(&self) -> bool {
        self.svg.is_some() & self.await_loader_passed
    }
}
