#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

#[macro_use]
extern crate bitflags;
extern crate rand;


mod bitboard;
mod eval;
mod types;
mod movegenerator;
mod board;

use bitboard::BitBoard;
use types::{Pc, Color, PieceType, Move, Square};
use types::Color::*;
use types::PieceType::*;

use board::Pos;

//use Piece::*;

use rand::Rng;
use std::fmt;

/*
#[derive(Eq, PartialEq, Copy, Clone)]
enum Piece {
    P(Color, PieceType),
    Empty
}
impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            Empty => '.',
            P(Black, t) => match t {
                Pawn => '♙',
                Bishop => '♗',
                Knight => '♘',
                Rook => '♖',
                Queen => '♕',
                King => '♔'
            },
            P(White, t) => match t {
                Pawn => '♟',
                Bishop => '♝',
                Knight => '♞',
                Rook => '♜',
                Queen => '♛',
                King => '♚'
            }
        };

        write!(f, "{} ", c)
    }
}


#[derive(Copy, Clone)]
struct Move {
    piece: Piece,
    from: Square,
    to: Square
}
impl Move {
    fn new(piece: Piece, from: Square, to: Square) -> Move {
        Move {
            piece: piece,
            from: from,
            to: to
        }
    }
}
impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?} → {:?}", self.piece, self.from, self.to)
    }
}
*/

/*
struct Board {
    board: [Piece; 64]
}
impl Clone for Board {
    fn clone(&self) -> Board {
        Board { board: self.board }
    }
}
impl Board {
    fn new() -> Board {
        Board {
            board: [Piece::Empty; 64]
        }
    }
    fn set(&mut self, pos: usize, piece: Piece) {
        self.board[pos] = piece;
    }
}
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        write!(f, "Board {{\n    ");
        for i in 0..8 {
            write!(f, "{} ", (7-i)+1);
            for j in 0..8 {
                self.board[(7-i)*8 + j].fmt(f);
                // write!(f, "{}");
            }
            write!(f, "\n    ");
        }
        write!(f, "  ");
        for i in 0..8 {
            write!(f, "{} ", (i + 'A' as u8) as char);

        }
        s.push_str("\n}");
        write!(f, "{}", s)
    }
}
*/
/*
#[derive(Clone, Debug)]
struct Position {
    turn: Color,
    score: i64,
    halfmoves: usize,
    moves: usize,
    history: Vec<Move>,
    board: Board,

}
impl Position {
    fn empty() -> Position {
        Position {
            turn: White,
            board: Board::new(),
            score: 0,
            history: Vec::new(),
            halfmoves: 0,
            moves: 0

        }
    }
    fn start() -> Position {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    fn test() -> Position {
        Position::from_fen("1n4n1/pppppppp/8/8/8/8/PPPPPPPP/1N4N1 w KQkq - 0 1")
    }

    fn from_fen(s: &str) -> Position {
        let mut pos = Position::empty();
        let mut parts = s.split(" ");
        let board = parts.next().unwrap();
        let turn = parts.next().unwrap();
        let castling = parts.next().unwrap();
        let passant = parts.next().unwrap();
        let halfmoves = parts.next().unwrap();
        let moves = parts.next().unwrap();

        let mut row = 7;
        let mut col = 0;
        for c in board.chars() {
            let idx = row * 8 + col;
            col += 1;
            match c {
                'p' => { pos.board.set(idx, P(Black, Pawn)); },
                'P' => { pos.board.set(idx, P(White, Pawn)); },
                'r' => { pos.board.set(idx, P(Black, Rook)); },
                'R' => { pos.board.set(idx, P(White, Rook)); },
                'n' => { pos.board.set(idx, P(Black, Knight)); },
                'N' => { pos.board.set(idx, P(White, Knight)); },
                'b' => { pos.board.set(idx, P(Black, Bishop)); },
                'B' => { pos.board.set(idx, P(White, Bishop)); },
                'q' => { pos.board.set(idx, P(Black, Queen)); },
                'Q' => { pos.board.set(idx, P(White, Queen)); },
                'k' => { pos.board.set(idx, P(Black, King)); },
                'K' => { pos.board.set(idx, P(White, King)); },
                '/' => { row -= 1; col = 0; },
                n @ '0' ... '8' => {
                    let nm = n as u8 - '0' as u8;
                    col += (nm - 1) as usize;
                },
                _ => { col -= 1; }
            }
        }

        pos.turn = match turn {
            "w" => White,
            "b" => Black,
            _ => panic!("invalid fen string")
        };

        pos.halfmoves = halfmoves.parse::<usize>().unwrap();
        pos.moves = halfmoves.parse::<usize>().unwrap();
        pos
    }

    fn mv(&self, mv: Move) -> Position {
        let mut n = self.clone();
        if n.turn == Black { n.turn = White; }
        else { n.turn = Black; }
        n.board.board[mv.to.0] = self.board.board[mv.from.0];
        n.board.board[mv.from.0] = Empty;

        n.score = n.evaluate();
        n.history.push(mv);
        n.halfmoves += 1;
        n
    }

    fn evaluate(&self) -> i64 {
        let mut score = 0;
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
        score
    }




    fn search(&self, depth: usize) -> Vec<Position> {
        let mut positions = Vec::new();
        if depth == 0 {
            positions.push(self.clone());
            return positions;
        }

        let mvs = self.generate_moves();
        for m in mvs {
            let newpos = self.mv(m);
            let mut lower = newpos.search(depth - 1);
            positions.append(&mut lower);
        }
        return positions;
    }

    fn generate_moves(&self) -> Vec<Move> {
        Vec::new()
    }
}
*/

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
use std::io::{self, Read};

fn move_input(pos: &Pos) -> Move {
    let mut input = String::new();

    print!("> ");
    io::stdin().read_line(&mut input);

    let mut sp = input.trim().split(" ");
    let fr = BitBoard::from_str(sp.next().unwrap());
    let to = BitBoard::from_str(sp.next().unwrap());
    //  println!(" got '{}' '{}' '{}'", fr, to, input);
    Move {
        from: fr,
        to: to,
        piece: pos.board.get(fr).unwrap(),
        capture: pos.board.get(to),
        promotion: None
    }
}

fn main() {
    let mut game = Pos::start();
    //let mut game = Pos::from_fen("8/bbbbbbbb/8/8/8/8/BBBBBBBB/8 w KQkq - 0 1");
//    let mut game2 = Pos::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");

//    println!("{}", game2.perft(1));

    println!("{}", game);

    loop {
        //let score =  eval::evaluate(&game) as f64 / 100.0;
        //if game.halfmoves == 100 {
        //    println!("draw!");
        //    break;
        //}
        if (game.turn == Black) || true {
            println!("thinking...");
            let (sc, best_move) = game.negamax_start(4);
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
            let m = move_input(&mut game);
            println!("{}", m);
            game.make_move(m);
            println!("{}", game);
        }

//        let moves = movegenerator::generate_moves(&game);
        //for m in moves.iter() {            println!("{}", m);        }
//        if moves.len() == 0 {
//            println!("no more moves for {:?}!", game.turn);
//            break;
//        } else {
            //println!("{}", moves.len());
//        }

        //let rnd_move = rand::random::<usize>();

        //game.make_move(moves[rnd_move % moves.len()]);

    }

    /*
    let mv2 = Move {
        from: Square::from_str("d7"),
        to: Square::from_str("d5"),
        piece: game.board.get(Square::from_str("d7")).unwrap(),
        capture: None,
        promotion: None
    };

    game.make_move(mv1);
    game.make_move(mv2);
    let mv3 = Move {
        from: Square::from_str("e4"),
        to: Square::from_str("d5"),
        piece: game.board.get(Square::from_str("e4")).unwrap(),
        capture: game.board.get(Square::from_str("d5")),
        promotion: None
    };
    println!("{}", mv3);
    game.make_move(mv3);

    println!("{}", game);

    game.unmake_move(mv3);
    println!("{}", game);
    */

/*
    while false {
        let (sc, pos) = game.negamax(4);

        if pos.history.len() == game.halfmoves {
            println!("{:?} has no moves left!", game.turn);
            break;
        }
        game = game.mv(pos.history[game.halfmoves]);
        println!("{:#?}", game);
        // std::thread::sleep_ms((game.halfmoves as u32)/2 * 80);
    }
*/
/*
    let mut brd = BBoard::empty();
    brd.set(Square(12), Pc(Black, Bishop));
    brd.set(Square(36), Pc(White, Queen));
    brd.set(Square(34), Pc(Black, Knight));
    brd.set(Square(33), Pc(White, Pawn));
    let p = brd.get(Square(12));

    println!("{}", brd);
    match p {
        Some(p) => println!("{:?}", p),
        None => {}
    };
    */
}
