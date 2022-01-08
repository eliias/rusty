use crate::Board;

pub trait Fen {
    fn from_fen(fen: &str) -> Board;
    fn to_fen(&self) -> String;
}
