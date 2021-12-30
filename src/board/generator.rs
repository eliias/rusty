use std::fmt::{Display, Formatter};
use crate::board::piece::Piece;
use crate::{Board, Color};

#[derive(Clone, Copy)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub promote_to: &'static Piece,
    pub capture: &'static Piece,
}

impl Default for Move {
    fn default() -> Self {
        Move {
            from: 0,
            to: 0,
            promote_to: &Piece::EMPTY,
            capture: &Piece::EMPTY
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let from_rank = 7 - self.from / 8;
        let from_file = self.from % 8;
        let to_rank = 7 - self.to / 8;
        let to_file = self.to % 8;

        let files = "abcdefgh";
        let human_from_file = files.chars().nth(from_file).unwrap();
        let human_to_file = files.chars().nth(to_file).unwrap();

        write!(f, "{}{}{}{}", human_from_file, from_rank + 1, human_to_file, to_rank + 1)
    }
}

pub trait MoveGenerator {
    fn in_bounds(&self, index: isize) -> bool;
    fn is_empty_field(&self, index: isize) -> bool;
    fn is_valid_field(&self, rank: isize, file: isize) -> bool;
    fn can_take(&self, index: isize, current_color: &Color) -> bool;

    fn calc_index(&self, rank: usize, file: usize) -> usize;
    fn calc_relative_index(&self, index: usize, plus_ranks: isize, plus_files: isize) -> isize;
    fn calc_rank_and_file(&self, index: usize) -> (usize, usize);

    fn current_direction(&self, color: &Color) -> isize;

    fn generate_pawn_moves(&self, position: usize, color: &Color) -> Vec<Move>;
    fn generate_rook_moves(&self, position: usize, color: &Color) -> Vec<Move>;
    fn generate_bishop_moves(&self, position: usize, color: &Color) -> Vec<Move>;
    fn generate_knight_moves(&self, position: usize, color: &Color) -> Vec<Move>;
    fn generate_queen_moves(&self, position: usize, color: &Color) -> Vec<Move>;
    fn generate_king_moves(&self, position: usize, color: &Color) -> Vec<Move>;

    fn generate(&self, next: &Color) -> Vec<Move>;

    fn apply(&self, m: Move) -> Board;
    fn undo(&self, m: Move) -> Board;
}
