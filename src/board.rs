use std::fmt::{Display, Formatter};
use crate::board::generator::{Move, MoveGenerator};
use crate::board::perft::Perft;
use crate::board::piece::Piece;
use crate::board::piece::Piece::{BISHOP, KNIGHT, QUEEN, ROOK};
use crate::{Color, DEFAULT_COLORS, DEFAULT_PIECES};
use crate::board::fen::Fen;

mod tests;
pub(crate) mod color;
pub(crate) mod defaults;
pub(crate) mod fen;
pub(crate) mod generator;
pub(crate) mod perft;
pub(crate) mod piece;

#[derive(Copy, Clone)]
pub struct Board {
    pub(crate) pieces: [&'static Piece; 64],
    pub(crate) colors: [&'static Color; 64],
}

impl Default for Board {
    fn default() -> Self {
        Board {
            pieces: DEFAULT_PIECES,
            colors: DEFAULT_COLORS,
        }
    }
}

impl MoveGenerator for Board {
    fn in_bounds(&self, index: isize) -> bool {
        index > -1 && index < 64
    }

    fn is_empty_field(&self, index: isize) -> bool {
        self.pieces[index as usize] == &Piece::EMPTY
    }

    fn is_valid_field(&self, rank: isize, file: isize) -> bool {
        rank > -1 && rank < 8 && file > -1 && file < 8
    }

    fn can_take(&self, index: isize, current_color: &Color) -> bool {
        self.colors[index as usize] != current_color && self.pieces[index as usize] != &Piece::EMPTY
    }

    fn calc_index(&self, rank: usize, file: usize) -> usize {
        (63 - 7) - rank * 8 + file
    }

    fn calc_relative_index(&self, position: usize, plus_ranks: isize, plus_files: isize) -> isize {
        let (rank, file) = self.calc_rank_and_file(position);
        let target_rank = (rank as isize) + plus_ranks;
        let target_file = (file as isize) + plus_files;

        if !self.is_valid_field(target_rank, target_file) {
            return -1;
        }

        self.calc_index(target_rank as usize, target_file as usize) as isize
    }

    fn calc_rank_and_file(&self, position: usize) -> (usize, usize) {
        let rank = 7 - position / 8;
        let file = position % 8;

        (rank, file)
    }

    fn current_direction(&self, color: &Color) -> isize {
        match color {
            Color::WHITE => 1,
            Color::BLACK => -1,
            _ => panic!("invalid color provided")
        }
    }

    fn generate_pawn_moves(&self, position: usize, color: &Color) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();
        let direction = self.current_direction(color);
        let (rank, _) = self.calc_rank_and_file(position);

        // check if pawn can move one field forward
        let index = self.calc_relative_index(position, direction * 1, 0);
        if self.in_bounds(index) && self.is_empty_field(index) {
            // check if we need to convert pawn
            if (*color == Color::WHITE && rank == 7) || (*color == Color::BLACK && rank == 0) {
                for promote_to in [&ROOK, &BISHOP, &KNIGHT, &QUEEN] {
                    list.push(Move { from: position, to: index as usize, promote_to, ..Default::default() })
                }
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        // check if pawn can move two fields forward (given that current rank = 1
        // for white and rank = 6 for black) and next two fields are both empty
        if (*color == Color::WHITE && rank == 1) || (*color == Color::BLACK && rank == 6) {
            let index1 = self.calc_relative_index(position, direction * 1, 0);
            if self.in_bounds(index1) && self.is_empty_field(index1) {
                let index2 = self.calc_relative_index(position, direction * 2, 0);
                if self.in_bounds(index2) && self.is_empty_field(index2) {
                    list.push(Move { from: position, to: index2 as usize, ..Default::default() })
                }
            }
        }

        // check if pawn can take +1,+1, +1,-1
        let index = self.calc_relative_index(position, direction * 1, 1);
        if self.in_bounds(index) && self.can_take(index, color) {
            list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
        }

        let index = self.calc_relative_index(position, direction * 1, -1);
        if self.in_bounds(index) && self.can_take(index, color) {
            list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
        }

        // todo: check if pawn can move en passant

        // return all valid moves
        list
    }

    fn generate_rook_moves(&self, position: usize, color: &Color) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();

        // vertical movement
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 1, 0);
            if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
                if self.can_take(index, color) {
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    list.push(Move { from: position, to: index as usize, ..Default::default() });
                }
            } else {
                // stops as soon as an invalid field is hit or we can take
                break;
            }
        }

        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, -1, 0);
            if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
                if self.can_take(index, color) {
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    list.push(Move { from: position, to: index as usize, ..Default::default() });
                }
            } else {
                // stops as soon as an invalid field is hit or we can take
                break;
            }
        }

        // horizontal movement
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 0, 1);
            if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
                if self.can_take(index, color) {
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    list.push(Move { from: position, to: index as usize, ..Default::default() });
                }
            } else {
                // stops as soon as an invalid field is hit or we can take
                break;
            }
        }

        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 0, -1);
            if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
                if self.can_take(index, color) {
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    list.push(Move { from: position, to: index as usize, ..Default::default() });
                }
            } else {
                // stops as soon as an invalid field is hit or we can take
                break;
            }
        }

        list
    }

    fn generate_bishop_moves(&self, position: usize, color: &Color) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();

        // move top right
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 1, 1);
            if self.in_bounds(index) {
                if self.is_empty_field(index) {
                    // move until no more valid/empty fields left
                    list.push(Move { from: position, to: index as usize, ..Default::default() })
                } else if self.can_take(index, color) {
                    // create capture move and stop
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    // stop immediately if move is invalid
                    break;
                }
            }
        }

        // move bottom right
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, -1, 1);
            if self.in_bounds(index) {
                if self.is_empty_field(index) {
                    // move until no more valid/empty fields left
                    list.push(Move { from: position, to: index as usize, ..Default::default() })
                } else if self.can_take(index, color) {
                    // create capture move and stop
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    // stop immediately if move is invalid
                    break;
                }
            }
        }

        // move bottom left
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, -1, -1);
            if self.in_bounds(index) {
                if self.is_empty_field(index) {
                    // move until no more valid/empty fields left
                    list.push(Move { from: position, to: index as usize, ..Default::default() })
                } else if self.can_take(index, color) {
                    // create capture move and stop
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    // stop immediately if move is invalid
                    break;
                }
            }
        }

        // move top left
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 1, -1);
            if self.in_bounds(index) {
                if self.is_empty_field(index) {
                    // move until no more valid/empty fields left
                    list.push(Move { from: position, to: index as usize, ..Default::default() })
                } else if self.can_take(index, color) {
                    // create capture move and stop
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    // stop immediately if move is invalid
                    break;
                }
            }
        }

        list
    }

    fn generate_knight_moves(&self, position: usize, color: &Color) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();
        let direction = self.current_direction(color);

        // check if bishop can move +2-1
        let index = self.calc_relative_index(position, direction * 2, -1);
        if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
            if self.can_take(index, color) {
                list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        // check if bishop can move +1-2
        let index = self.calc_relative_index(position, direction * 1, -2);
        if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
            if self.can_take(index, color) {
                list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        // check if bishop can move +2+1
        let index = self.calc_relative_index(position, direction * 2, 1);
        if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
            if self.can_take(index, color) {
                list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        // check if bishop can move +1+2
        let index = self.calc_relative_index(position, direction * 1, 2);
        if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
            if self.can_take(index, color) {
                list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        // check if bishop can move -2-1
        let index = self.calc_relative_index(position, direction * -2, -1);
        if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
            if self.can_take(index, color) {
                list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        // check if bishop can move -1-2
        let index = self.calc_relative_index(position, direction * -1, -2);
        if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
            if self.can_take(index, color) {
                list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        // check if bishop can move -2+1
        let index = self.calc_relative_index(position, direction * -2, 1);
        if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
            if self.can_take(index, color) {
                list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        // check if bishop can move -1+2
        let index = self.calc_relative_index(position, direction * -1, 2);
        if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
            if self.can_take(index, color) {
                list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
            } else {
                list.push(Move { from: position, to: index as usize, ..Default::default() })
            }
        }

        list
    }

    fn generate_queen_moves(&self, position: usize, color: &Color) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();

        // vertical movement
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 1, 0);
            if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
                if self.can_take(index, color) {
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    list.push(Move { from: position, to: index as usize, ..Default::default() });
                }
            } else {
                // stops as soon as an invalid field is hit or we can take
                break;
            }
        }

        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, -1, 0);
            if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
                if self.can_take(index, color) {
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    list.push(Move { from: position, to: index as usize, ..Default::default() });
                }
            } else {
                // stops as soon as an invalid field is hit or we can take
                break;
            }
        }

        // horizontal movement
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 0, 1);
            if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
                if self.can_take(index, color) {
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    list.push(Move { from: position, to: index as usize, ..Default::default() });
                }
            } else {
                // stops as soon as an invalid field is hit or we can take
                break;
            }
        }

        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 0, -1);
            if self.in_bounds(index) && (self.is_empty_field(index) || self.can_take(index, color)) {
                if self.can_take(index, color) {
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    list.push(Move { from: position, to: index as usize, ..Default::default() });
                }
            } else {
                // stops as soon as an invalid field is hit or we can take
                break;
            }
        }

        // move top right
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 1, 1);
            if self.in_bounds(index) {
                if self.is_empty_field(index) {
                    // move until no more valid/empty fields left
                    list.push(Move { from: position, to: index as usize, ..Default::default() })
                } else if self.can_take(index, color) {
                    // create capture move and stop
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    // stop immediately if move is invalid
                    break;
                }
            }
        }

        // move bottom right
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, -1, 1);
            if self.in_bounds(index) {
                if self.is_empty_field(index) {
                    // move until no more valid/empty fields left
                    list.push(Move { from: position, to: index as usize, ..Default::default() })
                } else if self.can_take(index, color) {
                    // create capture move and stop
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    // stop immediately if move is invalid
                    break;
                }
            }
        }

        // move bottom left
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, -1, -1);
            if self.in_bounds(index) {
                if self.is_empty_field(index) {
                    // move until no more valid/empty fields left
                    list.push(Move { from: position, to: index as usize, ..Default::default() })
                } else if self.can_take(index, color) {
                    // create capture move and stop
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    // stop immediately if move is invalid
                    break;
                }
            }
        }

        // move top left
        let mut index = position as isize;
        while index > -1 {
            index = self.calc_relative_index(index as usize, 1, -1);
            if self.in_bounds(index) {
                if self.is_empty_field(index) {
                    // move until no more valid/empty fields left
                    list.push(Move { from: position, to: index as usize, ..Default::default() })
                } else if self.can_take(index, color) {
                    // create capture move and stop
                    list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() });
                    break;
                } else {
                    // stop immediately if move is invalid
                    break;
                }
            }
        }

        list
    }

    fn generate_king_moves(&self, position: usize, color: &Color) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();

        for rank in -1..=1 {
            for file in -1..=1 {
                let index = self.calc_relative_index(position, rank, file);
                if self.in_bounds(index) {
                    // todo: check if field is check

                    if self.can_take(index, color) {
                        list.push(Move { from: position, to: index as usize, capture: self.pieces[index as usize], ..Default::default() })
                    } else if self.is_empty_field(index) {
                        list.push(Move { from: position, to: index as usize, ..Default::default() })
                    }
                }
            }
        }

        list
    }

    fn generate(&self, to_move: &Color) -> Vec<Move> {
        let mut all_moves: Vec<Move> = Vec::new();

        for (position, piece) in self.pieces.iter().enumerate() {
            let color = self.colors[position];
            if color != to_move {
                continue;
            }

            // pawn moves
            let moves = match piece {
                Piece::PAWN => self.generate_pawn_moves(position, to_move),
                Piece::ROOK => self.generate_rook_moves(position, to_move),
                Piece::KNIGHT => self.generate_knight_moves(position, to_move),
                Piece::BISHOP => self.generate_bishop_moves(position, to_move),
                Piece::QUEEN => self.generate_queen_moves(position, to_move),
                // Piece::KING => self.generate_king_moves(position, to_move),
                Piece::EMPTY => Vec::new(),
                _ => Vec::new(),
            };

            // add to list
            all_moves.extend(moves)
        }

        return all_moves;
    }

    fn apply(&self, m: Move) -> Board {
        let mut b = self.clone();

        b.colors[m.from] = &Color::NONE;
        b.pieces[m.from] = &Piece::EMPTY;
        b.colors[m.to] = self.colors[m.from];
        b.pieces[m.to] = self.pieces[m.from];

        b
    }

    fn undo(&self, m: Move) -> Board {
        let mut b = self.clone();

        b.colors[m.from] = self.colors[m.to];
        b.pieces[m.from] = self.pieces[m.to];
        b.colors[m.to] = &Color::NONE;
        b.pieces[m.to] = &Piece::EMPTY;

        b
    }
}

impl Fen for Board {
    fn load_fen(fen: str) -> Board {
        todo!()
    }

    fn print_fen() -> str {
        todo!()
    }
}

impl Perft for Board {
    fn perft(&self, depth: usize) -> usize {
        self._perft(depth, self.clone(), &Color::WHITE)
    }

    fn _perft(&self, depth: usize, board: Board, color: &Color) -> usize {
        if depth < 1 {
            return 0;
        }

        let legal_moves = board.generate(color);
        let mut nodes = legal_moves.len();
        for m in legal_moves {
            let b = board.apply(m);
            // println!("{}", b);
            nodes += b._perft(depth - 1, b, if *color == Color::WHITE { &Color::BLACK } else { &Color::WHITE });
        }

        nodes
    }

    fn divide(&self, depth: usize) -> usize {
        let board = self.clone();
        let legal_moves = board.generate(&Color::WHITE);
        let mut total_nodes = legal_moves.len();
        for m in legal_moves {
            let b = board.apply(m);
            let nodes = b._perft(depth - 1, b, &Color::BLACK);
            println!("{}: {}", m, nodes);
            total_nodes += nodes;
        }
        total_nodes
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = "".to_string();

        for index in 0..64 {
            let color = self.colors[index];
            match color {
                Color::WHITE => str.push_str(self.pieces[index].to_string().to_uppercase().as_str()),
                Color::BLACK => str.push_str(self.pieces[index].to_string().as_str()),
                Color::NONE => str.push_str(self.pieces[index].to_string().as_str()),
            }

            if (index + 1) % 8 == 0 && index > 0 {
                str.push_str("\n".to_string().as_str());
            }
        }

        write!(f, "{}", str)
    }
}
