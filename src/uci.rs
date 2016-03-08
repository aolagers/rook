#![allow(unused_must_use)]

extern crate rook;

use rook::pos::Pos;
use rook::types::{Move};

use std::fs::File;
use std::io;
use std::io::prelude::*;

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
            let args: Vec<&str> = line.split(" ").collect();

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
                    let mv = Move::from_str(&game, mv).unwrap();
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
                        let mv = Move::from_str(&game, m).unwrap();
                        game.make_move(mv);
                    }
                }
            }
            else if line.starts_with("go ") {
                let mut depth = 4;
                if args.len() > 1 && args[1] == "depth" {

                    log.write_all("got depth cmd".as_bytes());
                    match args[2].parse::<usize>() {
                        Ok(d) => {depth = d;},
                        _ => {}
                    }
                }

                let (_, _, best_move) = game.negamax_start(depth);
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
