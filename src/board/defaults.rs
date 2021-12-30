use crate::board::piece::Piece;
use crate::board::piece::Piece::{BISHOP, EMPTY, KING, KNIGHT, PAWN, QUEEN, ROOK};
use crate::Color;
use crate::Color::{BLACK, NONE, WHITE};

//#[derive(Copy, Clone)]
pub const DEFAULT_PIECES: [&Piece; 64] = [
    &ROOK,  &KNIGHT, &BISHOP, &QUEEN, &KING,  &BISHOP, &KNIGHT, &ROOK,
    &PAWN,  &PAWN,   &PAWN,   &PAWN,  &PAWN,  &PAWN,   &PAWN,   &PAWN,
    &EMPTY, &EMPTY,  &EMPTY,  &EMPTY, &EMPTY, &EMPTY,  &EMPTY,  &EMPTY,
    &EMPTY, &EMPTY,  &EMPTY,  &EMPTY, &EMPTY, &EMPTY,  &EMPTY,  &EMPTY,
    &EMPTY, &EMPTY,  &EMPTY,  &EMPTY, &EMPTY, &EMPTY,  &EMPTY,  &EMPTY,
    &EMPTY, &EMPTY,  &EMPTY,  &EMPTY, &EMPTY, &EMPTY,  &EMPTY,  &EMPTY,
    &PAWN,  &PAWN,   &PAWN,   &PAWN,  &PAWN,  &PAWN,   &PAWN,   &PAWN,
    &ROOK,  &KNIGHT, &BISHOP, &QUEEN, &KING,  &BISHOP, &KNIGHT, &ROOK,
];

//#[derive(Copy, Clone)]
pub const DEFAULT_COLORS: [&Color; 64] = [
    &BLACK, &BLACK, &BLACK, &BLACK, &BLACK, &BLACK, &BLACK, &BLACK,
    &BLACK, &BLACK, &BLACK, &BLACK, &BLACK, &BLACK, &BLACK, &BLACK,
    &NONE, &NONE, &NONE, &NONE, &NONE, &NONE, &NONE, &NONE,
    &NONE, &NONE, &NONE, &NONE, &NONE, &NONE, &NONE, &NONE,
    &NONE, &NONE, &NONE, &NONE, &NONE, &NONE, &NONE, &NONE,
    &NONE, &NONE, &NONE, &NONE, &NONE, &NONE, &NONE, &NONE,
    &WHITE, &WHITE, &WHITE, &WHITE, &WHITE, &WHITE, &WHITE, &WHITE,
    &WHITE, &WHITE, &WHITE, &WHITE, &WHITE, &WHITE, &WHITE, &WHITE,
];
