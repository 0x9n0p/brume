use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum Font {
    SansSerif,
    Custom(&'static str),
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Font::SansSerif => write!(f, "sans-serif"),
            Font::Custom(s) => write!(f, "{}%", s),
            _ => write!(f, "Size not found")
        }
    }
}