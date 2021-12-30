use crate::Board;

pub trait Fen {
    fn load_fen(fen: str) -> Board;
    fn print_fen() -> str;
}
