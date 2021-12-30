use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq)]
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
