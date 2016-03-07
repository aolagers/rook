#![allow(unused_must_use)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

mod bitboard;
mod eval;
mod types;
mod movegenerator;
mod board;


use std::io::prelude::*;
use std::io;
use std::fs::File;

use board::Pos;
use types::Move;

fn main() {
    let mut game = Pos::start();
    let mut log = File::create("log.txt").unwrap();

    loop {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();

            log.write_all(line.as_bytes());
            log.write_all("\n".as_bytes());

            let mut response = String::new();

            if line == "uci" {
                response.push_str("id name rook\n");
                response.push_str("id author Alex\n");
                response.push_str("uciok");
            }
            else if line.starts_with("isready") {
                response.push_str("readyok");
            }
            else if line.starts_with("ucinewgame") {
                game = Pos::empty();
            }
            else if line.starts_with("position fen") {
                println!("parsing fen");
                let fen_v = line.split(" ").skip(2).take(6).collect::<Vec<&str>>();
                let fen_str = fen_v.join(" ");
                println!("fen '{}'", fen_str);

                game = Pos::from_fen(&fen_str);
                println!("{}", game);
                for mv in line.split(" ").skip(9) {
                    let mv = Move::from_str(&game, mv);

                    game.make_move(mv);
                }
            }
            else if line.starts_with("position startpos") {
                game = Pos::start();
                let mut split = line.split(" ");
                let _ = split.next();
                let _ = split.next();
                if Some("moves") == split.next() {
                    for m in split {
                        let mv = Move::from_str(&game, m);
                        game.make_move(mv);
                    }
                }
            }
            else if line.starts_with("go") {
                println!("info depth 3");

                let (_, best_move) = game.negamax_start(3);
                let res = format!("bestmove {}", best_move.unwrap().to_str());
                response.push_str(&res);
            }

            if response != "" {
                println!("{}", response);
                log.write_all("> ".as_bytes());
                log.write_all(response.as_bytes());
                log.write_all("\n".as_bytes());

            }
        }
    }
}
