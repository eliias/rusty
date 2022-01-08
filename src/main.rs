use crate::board::Board;
use crate::board::color::Color;
use crate::board::defaults::{DEFAULT_COLORS, DEFAULT_PIECES};
use crate::board::fen::Fen;
use crate::board::perft::Perft;

mod board;
mod cli;

fn main() {
    cli::input();

    // let board = Board { ..Default::default() };
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1");
    // println!("{}", board);
    let depth = 2;
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
