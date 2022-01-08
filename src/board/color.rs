use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Color {
    WHITE,
    BLACK,
    NONE,
}

pub fn inverse_color(color: Color) -> Color {
    if color == Color::WHITE {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::WHITE => write!(f, "w"),
            Color::BLACK => write!(f, "b"),
            Color::NONE => write!(f, ""),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "w" => Ok(Color::WHITE),
            "b" => Ok(Color::BLACK),
            "." => Ok(Color::NONE),
            _ => Err(())
        }
    }
}
