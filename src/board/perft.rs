use crate::{Board, Color};

pub trait Perft {
    fn perft(&self, depth: usize) -> usize;
    fn _perft(&self, depth: usize, board: Board, color: &Color) -> usize;

    fn divide(&self, max_depth: usize) -> usize;
}
