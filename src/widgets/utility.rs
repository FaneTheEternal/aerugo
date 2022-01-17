use crate::widgets::base::{Widget};

pub fn fmt_vec(children: &Vec<Widget>) -> String {
    let mut s = String::new();
    children.iter()
        .for_each(|e| { s.push_str(e.str().as_str()) });
    s
}
