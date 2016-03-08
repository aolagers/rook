use types::PieceType::*;
use types::Pc;
use board::Pos;
use bitboard::BitBoard;

use types::Color::*;

const PAWN: [i64; 64] = [
    00,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    05,  5, 10, 25, 25, 10,  5,  5,
    00,  0,  0, 20, 20,  0,  0,  0,
    05, -5,-10,  0,  0,-10, -5,  5,
    05, 10, 10,-20,-20, 10, 10,  5,
    00,  0,  0,  0,  0,  0,  0,  0,
];

const KNIGHT: [i64; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const BISHOP: [i64; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];
const ROOK: [i64; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];
const QUEEN: [i64; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const KING: [i64; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];

fn bonus(piece: Pc, i: usize) -> i64 {
    let Pc(c, k) = piece;

    let lookup = match k {
        Pawn => PAWN,
        Knight => KNIGHT,
        Bishop => BISHOP,
        Rook => ROOK,
        Queen => QUEEN,
        King => KING
    };

    match c {
        White => lookup[(7-(i/8))*8 + i % 8],
        Black => -lookup[i]
    }
}

const BASE_VALUES: [i64; 6] = [
    100,
    310,
    320,
    500,
    900,
    1_000_000
];

fn base_value(p: Pc) -> i64 {
    match p {
        Pc(White, t) => BASE_VALUES[t as usize],
        Pc(Black, t) => -BASE_VALUES[t as usize]
    }
}

pub fn evaluate(pos: &Pos) -> i64 {
    let mut score = 0;
    //let mut sq = 1 << 63;
    for i in 0..64 {
        match pos.board.get(BitBoard::from_square(i)) {
            Some(p) => {
                score = score + base_value(p) + bonus(p, i);
            }
            None => {}
        }
    }
/*
    for (i, p) in self.board.board.iter().enumerate() {
        if let P(c, t) = *p {
            let mut val = eval::base_value(t);
            val +=  eval::bonus(i, Pc { color: c, kind: t });
            if c == Black {
                val = -val;
            }
            score += val;
        }
    }
    */

    score
}
