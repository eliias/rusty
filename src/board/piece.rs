use std::fmt::{Display, Formatter};
use std::pin::Pin;
use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Piece {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
    EMPTY,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::PAWN => write!(f, "p"),
            Piece::ROOK => write!(f, "r"),
            Piece::BISHOP => write!(f, "b"),
            Piece::KNIGHT => write!(f, "n"),
            Piece::QUEEN => write!(f, "q"),
            Piece::KING => write!(f, "k"),
            Piece::EMPTY => write!(f, "."),
        }
    }
}

impl FromStr for Piece {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "p" => Ok(Piece::PAWN),
            "r" => Ok(Piece::ROOK),
            "b" => Ok(Piece::BISHOP),
            "n" => Ok(Piece::KING),
            "q" => Ok(Piece::QUEEN),
            "k" => Ok(Piece::KING),
            "." => Ok(Piece::EMPTY),
            _ => Err(())
        }
    }
}
