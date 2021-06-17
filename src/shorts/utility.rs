#![allow(dead_code)]

pub use sdl2::rect::Rect;
pub use std::convert::TryInto;

#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new($x as i32, $y as i32, ($w as i32).try_into().unwrap(), ($h as i32).try_into().unwrap())
    };
    ($w:expr, $h:expr) => {
        Rect::new(0, 0, ($w as i32).try_into().unwrap(), ($h as i32).try_into().unwrap())
    }
}
