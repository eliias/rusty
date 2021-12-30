#[cfg(test)]
use crate::{Board, DEFAULT_COLORS, DEFAULT_PIECES};
use crate::board::generator::MoveGenerator;

#[test]
fn test_calc_row_and_rank_at_0() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };
    let result = board.calc_row_and_rank(0);
    assert_eq!(result, (7, 0));
}

#[test]
fn test_calc_row_and_rank_at_7() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };
    let result = board.calc_row_and_rank(7);
    assert_eq!(result, (7, 7));
}

#[test]
fn test_calc_row_and_rank_at_8() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };
    let result = board.calc_row_and_rank(8);
    assert_eq!(result, (6, 0));
}

#[test]
fn test_calc_row_and_rank_at_9() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };
    let result = board.calc_row_and_rank(9);
    assert_eq!(result, (6, 1));
}

#[test]
fn test_calc_row_and_rank_at_14() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };
    let result = board.calc_row_and_rank(14);
    assert_eq!(result, (6, 6));
}

#[test]
fn test_calc_row_and_rank_at_15() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };
    let result = board.calc_row_and_rank(15);
    assert_eq!(result, (6, 7));
}

#[test]
fn test_calc_row_and_rank_at_56() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };
    let result = board.calc_row_and_rank(56);
    assert_eq!(result, (0, 0));
}

#[test]
fn test_calc_row_and_rank_at_63() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };
    let result = board.calc_row_and_rank(63);
    assert_eq!(result, (0, 7));
}

#[test]
fn test_calc_index_at_0_0() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };

    let result = board.calc_index(0, 0);
    assert_eq!(result, 56);
}

#[test]
fn test_calc_index_at_0_7() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };

    let result = board.calc_index(0, 7);
    assert_eq!(result, 63);
}

#[test]
fn test_calc_index_at_1_0() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };

    let result = board.calc_index(1, 0);
    assert_eq!(result, 48);
}

#[test]
fn test_calc_index_at_7_7() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };

    let result = board.calc_index(7, 7);
    assert_eq!(result, 7);
}


#[test]
fn test_calc_index_at_7_0() {
    let board = Board {
        pieces: DEFAULT_PIECES,
        colors: DEFAULT_COLORS,
    };

    let result = board.calc_index(7, 0);
    assert_eq!(result, 0);
}
