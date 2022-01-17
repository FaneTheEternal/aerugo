#![allow(dead_code)]

pub use sdl2::rect::Rect;
pub use std::convert::TryInto;
pub use std::collections::HashSet;

#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new($x as i32, $y as i32, ($w as i32).try_into().unwrap(), ($h as i32).try_into().unwrap())
    };
    ($w:expr, $h:expr) => {
        Rect::new(0, 0, ($w as i32).try_into().unwrap(), ($h as i32).try_into().unwrap())
    }
}

/// https://riptutorial.com/rust/example/4149/create-a-hashset-macro
///
/// # Example
///
/// ```
/// let my_set = set![1, 2, 3, 4];
/// ```
#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

/// Make SVG type
/// # Example
///
/// ```
/// let svg: SVG = svg!("regular", "arrow-alt-circle-left.svg")
/// ```
#[macro_export]
macro_rules! svg {
    ($t:expr, $n:expr) => {
        ($t.into(), $n.into())
    }
}

/// Make `String` from any
///
/// # Example
///
/// ```
/// let s = py_str!("some_text");
/// ```
#[macro_export]
macro_rules! str {
    ($f:expr) => {
        $f.to_string()
    }
}
