use std::fmt;
use std::fmt::Formatter;

// REFACTOR

pub enum Colors {
    None,
    White,
    Black,
    RoyalBlue,
    Lightgray,
    Transparent,
    Custom(&'static str),
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Colors::None => write!(f, "none"),
            Colors::White => write!(f, "white"),
            Colors::Black => write!(f, "black"),
            Colors::RoyalBlue => write!(f, "royalBlue"),
            Colors::Lightgray => write!(f, "lightgray"),
            Colors::Lightgray => write!(f, "transparent"),
            Colors::Custom(s) => write!(f, "{}", s),
            _ => write!(f, "Color not found")
        }
    }
}