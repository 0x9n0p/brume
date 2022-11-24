use std::fmt;
use std::fmt::Formatter;

// REFACTOR

pub enum Colors {
    White,
    Black,
    RoyalBlue,
    Custom(&'static str),
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Colors::White => write!(f, "white"),
            Colors::Black => write!(f, "black"),
            Colors::RoyalBlue => write!(f, "royalBlue"),
            Colors::Custom(s) => write!(f, "{}", s),
            _ => write!(f, "Color not found")
        }
    }
}