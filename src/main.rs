//#![allow(dead_code)]
//#![allow(unused_variables)]
#![allow(unused_must_use)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate ansi_term;
//extern crate test;
extern crate time;


use ansi_term::Style;

mod bitboard;
mod eval;
mod types;
mod movegenerator;
mod board;

use std::io;

use bitboard::BitBoard;
use types::Move;
use types::Color::*;

use board::Pos;

//use rand::Rng;


#[test]
fn perft1() {
    let mut game1 = Pos::start();
    assert_eq!(game1.perft(0), 1);
    assert_eq!(game1.perft(1), 20);
    assert_eq!(game1.perft(2), 400);
    assert_eq!(game1.perft(3), 8902);
    assert_eq!(game1.perft(4), 197281);
}

#[test]
fn perft2() {
    let mut game2 = Pos::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    assert_eq!(game2.perft(1), 14);
    assert_eq!(game2.perft(2), 191);
    assert_eq!(game2.perft(3), 2812);
    assert_eq!(game2.perft(4), 43238);
}

use time::PreciseTime;

fn main() {
    //let mut game = Pos::start();
    let mut game = Pos::from_fen("8/pppppppp/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1");
    //let mut game = Pos::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");

    let yel = ansi_term::Colour::Yellow;
    let bold = yel.bold();
    println!("{}", bold.paint(format!("{}", game)));

    while true {
        if (game.turn == Black) || true {
            println!("thinking...");

            let start = PreciseTime::now();

            let (_, nodes, best_move) = game.negamax_start(5);
            let end = PreciseTime::now();
            let dur = start.to(end);
            println!("{} nodes in {:.2} s {:.2} knps", nodes, dur.num_milliseconds() as f64 / 1000.0, nodes as f64 / dur.num_milliseconds() as f64);
            match best_move {
                Some(mv) => {
                    game.make_move(mv);
                    println!("{} eval: {}", game, eval::evaluate(&game));
                },
                None => {
                    println!("No moves for {:?}", game.turn);
                    break;
                }
            }
        } else {
            //let m = move_input(&mut game);
            let m = Move::from_input(&game);
            println!("{}", m);
            game.make_move(m);
            println!("{}", game);
        }
    }
}
