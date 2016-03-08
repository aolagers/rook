// #![allow(dead_code)]
// #![allow(unused_variables)]
#![allow(unused_must_use)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

extern crate rand;
extern crate ansi_term;
extern crate time;

use time::PreciseTime;

// use ansi_term::Style;

mod bitboard;
mod eval;
mod types;
mod movegenerator;
mod board;
mod hash;
mod pos;

// use std::io;

// use bitboard::BitBoard;
use types::Move;
use types::Color::*;

use pos::Pos;

fn main() {
    // let yel = ansi_term::Colour::Red;
    // let bold = yel.bold();
    let mut game = Pos::start();
    // let mut game = Pos::from_fen("k7/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    // let mut game = Pos::from_fen("8/8/1PP3k1/8/8/5pp1/1K6/8 w - - 0 1");
    // let mut game = Pos::from_fen("8/8/8/8/8/ppp5/2p5/K7 w KQkq - 0 1");
    // let mut game = Pos::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    // let mut game = Pos::from_fen("P7/P7/P7/P7/P7/P7/P6r/KP5r w KQkq - 0 1");
    // let mut game = Pos::from_fen("3r2k1/ppp2ppr/8/8/8/P4n1P/2P3q1/4K3 w KQkq - 0 1");
    // let mut game = Pos::from_fen("8/p7/8/8/8/8/7P/8 w KQkq - 0 1");

    // for m in movegenerator::generate_legal_moves(&game) { println!("{}", m); }

    let human = false;
    let min_think = 0;

    let mut totaltime = 0;
    let mut totalnodes = 0;

    //println!("{:?}", hash::HASH);

    loop {
        println!("{}     eval: {}", game, eval::evaluate(&game));
        //println!("{}", hash::full_hash(&game));

        if (game.turn == Black) || !human {
            println!("\nthinking... ");

            let depth = match game.turn {
                Black => 4,
                White => 2,
            };

            let start = PreciseTime::now();
            let (_, nodes, best_move) = game.negamax_start(depth);
            let end = PreciseTime::now();
            let dur = start.to(end);
            totaltime += dur.num_milliseconds();
            totalnodes += nodes;
            let tl = min_think as i32 - dur.num_milliseconds() as i32;
            if tl > 0 { std::thread::sleep(std::time::Duration::from_millis(tl as u64)); }

            println!("{:7} nodes in {:2.2} s {:3.2} knps",
                     nodes,
                     dur.num_milliseconds() as f64 / 1000.0,
                     nodes as f64 / dur.num_milliseconds() as f64);
            println!("{:6.0} knodes in {:2.2} s {:3.2} knps",
                  totalnodes as f64 / 1000.0,
                  totaltime as f64 / 1000.0,
                  totalnodes as f64 / totaltime as f64);

            match best_move {
                Some(mv) => {
                    game.make_move(mv);
                }
                None => {
                    println!("No moves for {:?}", game.turn);
                    break;
                }
            }

        } else {
            let legals = movegenerator::generate_legal_moves(&game);

            // for l in legals.iter() { println!("{}", l); }

            let mut mv = None;
            let mut ok = false;

            while !ok {
                let m = Move::from_input(&game);
                for lm in legals.iter() {
                    if *lm == m {
                        ok = true;
                        mv = Some(m);
                    }
                }
            }

            game.make_move(mv.unwrap());
        }
    }
}
