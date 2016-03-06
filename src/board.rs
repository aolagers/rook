
use std::fmt;
use bitboard::BitBoard;
use types::{Pc, Color, PieceType, Move, Square};
use types::Color::*;
use types::PieceType::*;
use movegenerator;
use eval;


#[derive(Debug)]
pub struct BBoard {
    pub pieces: [BitBoard; 12],
    pub whites: BitBoard,
    pub blacks: BitBoard,
    pub occupied: BitBoard,
    pub free: BitBoard
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

    fn duplicate(&self) -> BBoard {
        BBoard {
            pieces: self.pieces.clone(),
            whites: self.whites,
            blacks: self.blacks,
            occupied: self.occupied,
            free: self.free
        }
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

    pub fn get(&self, sq: BitBoard) -> Option<Pc> {
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
    pub fn empty() -> Self {
        Pos {
            turn: White,
            history: Vec::new(),
            board: BBoard::empty(),
            moves: 0,
            halfmoves: 0
        }
    }

    pub fn start() -> Self {
        Pos::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    fn test() -> Self {
        //Pos::from_fen("1n4n1/pppppppp/8/8/8/8/PPPPPPPP/1N4N1 w KQkq - 0 1")
        Pos::from_fen("8/bbb2bbb/8/8/8/8/BBBB4/8 w KQkq - 0 1")
    }

    fn duplicate(&self) -> Pos {
        let n = self;
        Pos {
            turn: n.turn,
            history: n.history.clone(),
            board: n.board.duplicate(),
            moves: n.moves,
            halfmoves: n.halfmoves
        }
    }

    pub fn from_fen(s: &str) -> Self {
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

    pub fn make_move(&mut self, mv: Move) {
        debug_assert!(mv.from.count_bits() == 1);
        debug_assert!(mv.to.count_bits() == 1);

        if self.turn == Black { self.moves += 1; }
        self.halfmoves += 1;
        self.turn = self.turn.other();

        self.board.clear(mv.from);
        self.board.set(mv.to, mv.piece);
        self.history.push(mv);

        //if let Pc(_, Pawn) = mv.piece { self.halfmoves = 0; }
        //if let Some(_) = mv.capture { self.halfmoves = 0; }
    }

    pub fn unmake_move(&mut self, mv: Move) {
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

    pub fn perft(&mut self, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }
        let mut nodes = 0;
        let moves = movegenerator::generate_moves(self);
        for m in moves {
            self.make_move(m);
            nodes += self.perft(depth - 1);
            println!("{}", self);
            self.unmake_move(m);
        }
        return nodes;
    }

    pub fn negamax_start(&self, depth: usize) -> (i64, Option<Move>) {
        let mut pos = self.duplicate();
        let mut best_score = i64::min_value();
        let mut best_move = None;

        let moves = movegenerator::generate_moves(self);
        for m in moves {
            pos.make_move(m);
            let score = pos.negamax_iter(depth - 1);
            if -score > best_score {
                best_score = -score;
                best_move = Some(m);
            }
            pos.unmake_move(m);
        }

        (best_score, best_move)
    }

    fn negamax_iter(&mut self, depth: usize) -> i64 {
        if depth == 0 {
            let score = eval::evaluate(self);
            if self.turn == White {
                return score;
            } else {
                return -score;
            }
        }

        let mut best_score = i64::min_value();
        let moves = movegenerator::generate_moves(self);
        for m in moves {
            self.make_move(m);
            let score = -self.negamax_iter(depth - 1);
            if score > best_score {
                best_score = score;
            }
            self.unmake_move(m);
        }

        best_score
    }

    /*
    fn negamax_iter(&self, depth: usize) -> i64 {
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
    */
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
