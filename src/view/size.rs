use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Edge {
    pub top: Size,
    pub right: Size,
    pub left: Size,
    pub bottom: Size,
}

impl Edge {
    pub fn new() -> Edge {
        Edge {
            top: Size::Pixel(0.0),
            right: Size::Pixel(0.0),
            left: Size::Pixel(0.0),
            bottom: Size::Pixel(0.0),
        }
    }
}

#[derive(Clone)]
pub enum Size {
    MatchParent,
    WrapContent,
    Pixel(f32),
    Percent(i32),
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Size::MatchParent => write!(f, "100%"),
            Size::WrapContent => write!(f, "fit-content"),
            Size::Pixel(p) => write!(f, "{}px", p),
            Size::Percent(p) => write!(f, "{}%", p),
            _ => write!(f, "Size not found")
        }
    }
}