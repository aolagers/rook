// #![allow(dead_code)]
// #![allow(unused_variables)]
#![allow(unused_must_use)]

//! Global comments for rook chess engine core

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate ansi_term;
// extern crate test;
extern crate time;

pub mod types;
pub mod board;
pub mod bitboard;
pub mod eval;
pub mod movegenerator;

use board::Pos;

#[test]
fn perft1() {
    let mut game1 = Pos::start();
    assert_eq!(game1.perft(0), 1);
    assert_eq!(game1.perft(1), 20);
    assert_eq!(game1.perft(2), 400);
    assert_eq!(game1.perft(3), 8902);
    assert_eq!(game1.perft(4), 197281);
    //assert_eq!(game1.perft(5), 4865609);

}

#[test]
fn perft2() {
    let mut game2 = Pos::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    assert_eq!(game2.perft(1), 14);
    assert_eq!(game2.perft(2), 191);
    assert_eq!(game2.perft(3), 2812);
    assert_eq!(game2.perft(4), 43238);
}

#[test]
fn perft3() {
    let mut game2 = Pos::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");

    println!("{}", game2);
    assert_eq!(game2.perft(1), 48); // castling missing
    assert_eq!(game2.perft(2), 2039);
    assert_eq!(game2.perft(3), 97862);
    assert_eq!(game2.perft(4), 4085603);
    assert_eq!(game2.perft(5), 193690690);

}

#[test]
fn dont_move_into_check() {
    let mut game = Pos::from_fen("8/8/8/8/8/ppp5/2p5/K7 w KQkq - 0 1");
    let (_, nodes, best_move) = game.negamax_start(4);
    assert_eq!(best_move, None);
}

#[test]
fn dont_move_making_discovered_check() {
    let mut game = Pos::from_fen("P7/P7/P7/P7/P7/P7/P6r/KP5r w KQkq - 0 1");
    println!("{}", game);
    let (_, nodes, best_move) = game.negamax_start(4);
    assert_eq!(best_move, None);
}


#[test]
fn is_checkmate() {
    let game = Pos::from_fen("3r2k1/ppp2ppr/8/8/8/P4n1P/2P3q1/4K3 w KQkq - 0 1");
    let (_, nodes, best_move) = game.negamax_start(4);
    assert_eq!(best_move, None);
}

#[test]
fn pawn_double_start() {
    let game = Pos::from_fen("8/p7/8/8/8/7p/7P/8 w KQkq - 0 1");
    let (_, nodes, best_move) = game.negamax_start(4);
    assert_eq!(best_move, None);
}
