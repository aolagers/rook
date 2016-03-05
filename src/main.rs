#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
extern crate rand;

mod bitboard;
mod eval;
mod types;
mod movegenerator;

use bitboard::BitBoard;
use types::{Pc, Color, PieceType, Move, Square};
use types::Color::*;
use types::PieceType::*;

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

#[derive(Debug)]
pub struct BBoard {
    pieces: [BitBoard; 12],
    whites: BitBoard,
    blacks: BitBoard,
    occupied: BitBoard,
    free: BitBoard
}
impl BBoard {
    fn empty() -> Self {
        BBoard {
            pieces: [BitBoard(0); 12],
            whites: BitBoard(0),
            blacks: BitBoard(0),
            occupied: BitBoard(0),
            free: BitBoard(0xffff_ffff_ffff_ffff)
        }
    }

    fn clear(&mut self, sq: BitBoard) {
        debug_assert!(sq.count_bits() == 1);
        for bb in self.pieces.iter_mut() {
            bb.0 = bb.0 & !sq.0;
        }

        self.recalc();
    }

    fn set(&mut self, sq: BitBoard, p: Pc) {
        debug_assert!(sq.count_bits() == 1);
        // println!("sq {}", sq.0);
        // println!("sq {}", sq.to_str());

        self.clear(sq);
        let Pc(c, k) = p;
        let idx = c as usize + k as usize;
        self.pieces[idx] = self.pieces[idx] | sq;
        // println!("{:?}", self.pieces[p.color as usize + p.kind as usize]);
        self.recalc();
    }

    fn get(&self, sq: BitBoard) -> Option<Pc> {
        let mut found = None;
        for (idx, bb) in self.pieces.iter().enumerate() {
            if (*bb & sq).is_not_empty() {
                let color = if idx / 6 == 0 { White } else { Black };
                let kind = match idx % 6 {
                    0 => Pawn,
                    1 => Knight,
                    2 => Bishop,
                    3 => Rook,
                    4 => Queen,
                    5 => King,
                    _ => panic!("invalid piece type")
                };

                found = Some(Pc (color, kind));
            }
        }

        return found;
    }

    pub fn get_squares(&self, piece: Pc) -> Vec<BitBoard> {
        let mut locations = Vec::new();
        let Pc(c, t) = piece;
        let arr = self.pieces[c as usize + t as usize];

        for i in 0..64 {
            let sqbb = BitBoard(1 << i);
            if (arr & sqbb).is_not_empty() {
                locations.push(sqbb);
            }
        }
        locations
    }

    pub fn mine(&self, color: Color) -> BitBoard {
        match color {
            White => self.whites,
            Black => self.blacks
        }
    }

    pub fn theirs(&self, color: Color) -> BitBoard {
        match color {
            White => self.blacks,
            Black => self.whites
        }
    }

    fn recalc(&mut self) {
        self.whites = BitBoard(0);
        self.blacks = BitBoard(0);
        for i in 0..6  { self.whites = self.whites | self.pieces[i]; }
        for i in 6..12 { self.blacks = self.blacks | self.pieces[i]; }
        self.occupied = self.whites | self.blacks;
        self.free = !self.occupied;
    }
}

impl fmt::Display for BBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n    ");
        for i in 0..8 {
            write!(f, "{} ", (7-i)+1);
            for j in 0..8 {
                match self.get(BitBoard(1 << ((7-i)*8 + j))) {
                    None => write!(f, ". "),
                    Some(p) => p.fmt(f)
                };
            }
            write!(f, "\n    ");
        }
        write!(f, "  ");
        for i in 0..8 { write!(f, "{} ", (i + 'A' as u8) as char); }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Pos {
    pub board: BBoard,
    pub turn: Color,
    pub history: Vec<Move>,
    pub moves: usize,
    pub halfmoves: usize
}
impl Pos {
    fn empty() -> Self {
        Pos {
            turn: White,
            history: Vec::new(),
            board: BBoard::empty(),
            moves: 0,
            halfmoves: 0
        }
    }
    fn start() -> Self {
        Pos::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    fn test() -> Self {
        //Pos::from_fen("1n4n1/pppppppp/8/8/8/8/PPPPPPPP/1N4N1 w KQkq - 0 1")
        Pos::from_fen("8/bbb2bbb/8/8/8/8/BBBB4/8 w KQkq - 0 1")
    }


    fn from_fen(s: &str) -> Self {
        let mut pos = Pos::empty();
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
            let sq = if col < 8 { BitBoard(1 << idx) } else {BitBoard(0)};
            col += 1;
            match c {
                'P' => { pos.board.set(sq, Pc(White, Pawn)); },
                'R' => { pos.board.set(sq, Pc(White, Rook)); },
                'N' => { pos.board.set(sq, Pc(White, Knight)); },
                'B' => { pos.board.set(sq, Pc(White, Bishop)); },
                'Q' => { pos.board.set(sq, Pc(White, Queen)); },
                'K' => { pos.board.set(sq, Pc(White, King)); },
                'p' => { pos.board.set(sq, Pc(Black, Pawn)); },
                'r' => { pos.board.set(sq, Pc(Black, Rook)); },
                'n' => { pos.board.set(sq, Pc(Black, Knight)); },
                'b' => { pos.board.set(sq, Pc(Black, Bishop)); },
                'q' => { pos.board.set(sq, Pc(Black, Queen)); },
                'k' => { pos.board.set(sq, Pc(Black, King)); },
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

    fn make_move(&mut self, mv: Move) {
        debug_assert!(mv.from.count_bits() == 1);
        debug_assert!(mv.to.count_bits() == 1);

        if self.turn == Black { self.moves += 1; }
        self.halfmoves += 1;
        self.turn = self.turn.other();

        self.board.clear(mv.from);
        self.board.set(mv.to, mv.piece);
        self.history.push(mv);
    }

    fn unmake_move(&mut self, mv: Move) {
        self.history.pop();
        self.turn = self.turn.other();
        if self.turn == Black { self.moves -= 1; }
        self.halfmoves -= 1;

        self.board.clear(mv.to);
        if mv.capture != None {
            self.board.set(mv.to, mv.capture.unwrap());
        }
        self.board.set(mv.from, mv.piece);
    }
}
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos {{\n");
        write!(f, "  turn: {:?}\n", self.turn);
        write!(f, "  history:");

        for (i, m) in self.history.iter().enumerate() {
            if i % 2 == 0 {
                write!(f, "\n    {}: {}", 1+(i/2), m);
            } else {
                write!(f, " {}", m);
            }
        }

        write!(f, "\n  board: {}", self.board);
        write!(f, "\n}}");
        Ok(())
    }
}
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


    fn negamax(&self, depth: usize) -> (i64, Position) {
        if depth == 0 {
            if self.turn == White {
                return (self.score, self.clone());
            } else {
                return (-self.score, self.clone());
            }
        }

        let mut best_score = -1000000;
        let mut best_pos = self.clone();
        let mvs = self.generate_moves();
        if mvs.len() == 0 {
            let new = self.clone();
            if -new.score > best_score {
                best_score = -new.score;
                best_pos = new;
            }
        }
        for m in mvs {
            let new = self.mv(m);
            let (score, pos) = new.negamax(depth - 1);

            if -score > best_score{
                best_score = -score;
                best_pos = pos;
            }
        }
        (best_score, best_pos)
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

fn main() {
    let mut game = Pos::start();
    //let mut game = Pos::from_fen("8/bbbbbbbb/8/8/8/8/BBBBBBBB/8 w KQkq - 0 1");

    for _ in 0..500 {
        println!("{}", game);
        let score =  eval::evaluate(&game) as f64 / 100.0;
        println!("{}", score);

        let moves = movegenerator::generate_moves(&game);
        //for m in moves.iter() {            println!("{}", m);        }
        if moves.len() == 0 {
            println!("no more moves for {:?}!", game.turn);
            break;
        } else {
            //println!("{}", moves.len());
        }

        let rnd_move = rand::random::<usize>();

        game.make_move(moves[rnd_move % moves.len()]);

        std::thread::sleep_ms(100);
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
