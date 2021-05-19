#![allow(dead_code)]

pub enum GameState {
    None,
    Exit,
    NOP,
    SetSettings {
        settings: Settings
    },
}

pub enum Settings {
    SetMaxFPS {
        fps: u8
    },
    ReSize {
        size: ScreenSize
    }
}

pub struct ScreenSize {
    ratio: Ratio
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