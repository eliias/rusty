use crate::board::Board;
use crate::board::color::Color;
use crate::board::defaults::{DEFAULT_COLORS, DEFAULT_PIECES};
use crate::board::perft::Perft;

mod board;

fn main() {
    let board = Board { ..Default::default() };
    let depth = 3;
    let perft = board.divide(depth);
    // let perft = board.perft(depth);
    println!("perft for depth: {} => {}", depth, perft);

    // let moves = board.generate(&Color::WHITE);
    // println!("number of moves: {}", moves.len());
    // for m in moves {
    //     board.apply(m);
    //     println!("Move: {} -> {}", m.from, m.to);
    //     println!("{}", board.to_string());
    //     board.undo(m);
    // }
}
