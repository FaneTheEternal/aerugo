#![allow(dead_code)]

use std::sync::mpsc::TryRecvError;

#[derive(PartialEq)]
pub enum GameState {
    None,
    Exit,
    NOP,
}

pub enum Settings {
    SetMaxFPS {
        fps: u8
    },
    ReSize {
        size: ScreenSize
    },
}

pub struct ScreenSize {
    ratio: Ratio,
}

pub trait RatioTrait {
    fn size(&self) -> (u32, u32);
}

pub enum Ratio {
    FiveToFour {
        ratio: FiveToFour
    },

}

pub enum FiveToFour {
    SXGA,
    QSXGA,
}

impl RatioTrait for FiveToFour {
    fn size(&self) -> (u32, u32) {
        match *self {
            FiveToFour::SXGA => (1280, 1024),
            FiveToFour::QSXGA => (2560, 2048),
        }
    }
}

pub enum FourToThree {
    QVGA,
    VGA,
    PAL,
    SVGA,
    XGA,
    R1152x864,
    R1280x960,
    SXGAplus,
    UGA,
    QXGA,
}

// impl RatioTrait for FourToThree {
//     fn size(&self) -> (u32, u32) {
//         match *self {
//             FourToThree::QVGA => (320, 240),
//             FourToThree::VGA => (640, 480),
//             FourToThree::PAL => (768, 576),
//         }
//     }
// }

pub struct FutureLoader<T: 'static + Send + Clone> {
    loaded: Option<T>,
    taker: std::sync::mpsc::Receiver<T>,
    killer: std::sync::mpsc::Sender<()>,
}

impl<T: Send + Clone> FutureLoader<T> {
    pub fn make(f: fn() -> T) -> FutureLoader<T>
    {
        let (sv, rv) = std::sync::mpsc::channel();
        let (sk, rk) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            sv.send(f()).unwrap();
            rk.recv().unwrap();
        });
        FutureLoader {
            loaded: None,
            taker: rv,
            killer: sk,
        }
    }

    pub fn loaded(&self) -> bool {
        self.loaded.is_some()
    }

    pub fn value(&self) -> T {
        self.loaded.clone().unwrap()
    }

    pub fn touch(&mut self) -> Option<T> {
        match self.taker.try_recv() {
            Ok(v) => {
                self.killer.send(()).unwrap();
                self.loaded.replace(v);
                self.loaded.clone()
            }
            Err(e) => {
                match e {
                    TryRecvError::Empty => { None }
                    TryRecvError::Disconnected => {
                        self.loaded.clone()
                    }
                }
            }
        }
    }
}
