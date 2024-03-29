#![allow(unused_must_use)]

extern crate rook;
use rook::eval;
use rook::movegenerator;
use rook::pos::Pos;
use rook::types::Move;
use rook::types::Color::*;

extern crate time;
use time::Instant;

use std::io;
use std::io::Write;
use std::process;

fn move_from_input(pos: &Pos) -> Move {
    loop {
        print!("{:?}> ", pos.turn);
        std::io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        if input.trim() == "q" || input.trim() == "quit" {
            println!("\nbye");
            process::exit(0);
        }
        let mv = Move::from_str(pos, input.trim());
        match mv {
            Some(m) => { return m; },
            None => {
                println!("Invalid move: '{}'. Type 'q' to quit.", input.trim());
            }
        }
    }
}

fn main() {
    // let yel = ansi_term::Colour::Red;
    // let bold = yel.bold();
    let mut game = Pos::start();
    // let mut game = Pos::from_fen("8/8/8/8/8/8/8/RR4rr w KQkq - 0 1");
    // let mut game = Pos::from_fen("8/8/1PP3k1/8/8/5pp1/1K6/8 w - - 0 1");
    // let mut game = Pos::from_fen("8/8/8/8/8/ppp5/2p5/K7 w KQkq - 0 1");
    // let mut game = Pos::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    // let mut game = Pos::from_fen("P7/P7/P7/P7/P7/P7/P6r/KP5r w KQkq - 0 1");
    // let mut game = Pos::from_fen("3r2k1/ppp2ppr/8/8/8/P4n1P/2P3q1/4K3 w KQkq - 0 1");
    // let mut game = Pos::from_fen("8/p7/8/8/8/8/7P/8 w KQkq - 0 1");

    // for m in movegenerator::legal_moves(&game) { println!("{}", m); }

    let human = false;
    let min_think = 0;

    let mut totaltime = 0;
    let mut totalnodes = 0;

    //println!("{:?}", hash::HASH);

    loop {
        println!("{}     eval: {}\n", game, eval::evaluate(&game));
        //println!("{}", hash::full_hash(&game));

        if (game.turn == Black) || !human {
            println!("\nthinking... ");

            let depth = match game.turn {
                Black => 3,
                White => 3
            };

            let start = Instant::now();
            let (_, nodes, best_move) = game.negamax_start(depth);
            let end = Instant::now();
            let dur = end - start;
            totaltime += dur.whole_milliseconds();
            totalnodes += nodes;
            let tl = min_think as i32 - dur.whole_milliseconds() as i32;
            if tl > 0 { std::thread::sleep(std::time::Duration::from_millis(tl as u64)); }

            println!("{:7} nodes in {:2.2} s {:3.2} knps",
                     nodes,
                     dur.whole_milliseconds() as f64 / 1000.0,
                     nodes as f64 / dur.whole_milliseconds() as f64);
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
            let legals = movegenerator::legal_moves(&game);

            // for l in legals.iter() { println!("{}", l); }

            let mut mv = None;
            let mut ok = false;

            while !ok {
                let m = move_from_input(&game);
                for lm in legals.iter() {
                    if *lm == m {
                        ok = true;
                        mv = Some(m);
                    }
                }
                if !ok {println!("Move not legal");}
            }

            game.make_move(mv.unwrap());
        }
    }
}
