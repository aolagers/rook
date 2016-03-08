
use std::fmt;
use bitboard::BitBoard;
use types::{Pc, Color, Move, Castling};
use types::Color::*;
use types::PieceType::*;
use movegenerator;
use eval;

#[derive(Debug)]
pub struct BBoard {
    pub pieces: [BitBoard; 12],
    pub whites: BitBoard,
    pub blacks: BitBoard,
    pub occupied: BitBoard
}

impl BBoard {
    fn empty() -> Self {
        BBoard {
            pieces: [BitBoard(0); 12],
            whites: BitBoard(0),
            blacks: BitBoard(0),
            occupied: BitBoard(0)
        }
    }

    fn clear(&mut self, sq: BitBoard) {
        debug_assert!(sq.count_bits() == 1);

        for bb in self.pieces.iter_mut() {
            bb.0 = bb.0 & !sq.0;
        }
        self.recalc();
    }

    pub fn duplicate(&self) -> BBoard {
        BBoard {
            pieces: self.pieces.clone(),
            whites: self.whites,
            blacks: self.blacks,
            occupied: self.occupied
        }
    }

    fn set(&mut self, sq: BitBoard, p: Pc) {
        debug_assert!(sq.count_bits() == 1);

        self.clear(sq);
        let Pc(c, k) = p;
        let idx = c as usize + k as usize;
        self.pieces[idx] = self.pieces[idx] | sq;
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

    pub fn get_squares(&self, piece: Pc) -> BitBoard {
        let Pc(c, t) = piece;
        self.pieces[c as usize + t as usize]
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
    pub castling_rights: Option<Castling>,
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
            halfmoves: 0,
            castling_rights: None
        }
    }

    pub fn start() -> Self {
        Pos::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn duplicate(&self) -> Pos {
        let n = self;
        Pos {
            turn: n.turn,
            history: n.history.clone(),
            board: n.board.duplicate(),
            moves: n.moves,
            halfmoves: n.halfmoves,
            castling_rights: n.castling_rights
        }
    }

    pub fn to_fen(&self) -> String {
        let s = String::new();
        for r in (0..8).rev() {
            for c in 0..8 {
                let p = self.board.get(BitBoard(1 << (r*8 + c)));
            }
        }
        s
    }

    pub fn from_fen(s: &str) -> Self {
        let mut pos = Pos::empty();
        let mut parts = s.split(" ");
        let board = parts.next().unwrap();
        let turn = parts.next().unwrap();
        let castling = parts.next().unwrap();
        let _passant = parts.next().unwrap();
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
        pos.castling_rights = None;
        pos.halfmoves = halfmoves.parse::<usize>().unwrap();
        pos.moves = moves.parse::<usize>().unwrap();
        pos
    }

    pub fn make_move(&mut self, mv: Move) {
        debug_assert!(mv.from.count_bits() == 1);
        debug_assert!(mv.to.count_bits() == 1);

        if self.turn == Black { self.moves += 1; }
        self.halfmoves += 1;
        self.turn = self.turn.other();

        self.board.clear(mv.from);
        match mv.promotion {
            None =>    { self.board.set(mv.to, mv.piece); },
            Some(p) => { self.board.set(mv.to, p); }
        }
        match mv.castling {
            None => {},
            Some(cst) => {
                let (fr, to) = cst.get_rook_move();
                let p = self.board.get(fr).unwrap();
                self.board.set(to, p);
                self.board.clear(fr);
            }
        }
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
        let moves = movegenerator::generate_legal_moves(self);
        for m in moves {
            self.make_move(m);
            nodes += self.perft(depth - 1);
            self.unmake_move(m);
        }
        return nodes;
    }

    pub fn negamax_start(&self, depth: usize) -> (i64, usize, Option<Move>) {
        let mut pos = self.duplicate();
        let mut best_score = i64::min_value();
        let mut best_move = None;
        let mut nodes = 0;

        let lmoves = movegenerator::generate_legal_moves(self);

        for m in lmoves {
            pos.make_move(m);
            let (score, n) = pos.negamax_iter(depth - 1);
            nodes += n;
            if -score > best_score {
                best_score = -score;
                best_move = Some(m);
            }
            pos.unmake_move(m);
        }
        /*
        if best_move == None {
            println!("no legal moves found. all moves:");
            let moves = movegenerator::generate_moves(self);
            for m in moves {
                println!("{}", m);
            }

        }
        */
        (best_score, nodes, best_move)
    }

    fn negamax_iter(&mut self, depth: usize) -> (i64, usize) {
        if depth == 0 {
            let score = eval::evaluate(self);
            if self.turn == White {
                return (score, 1);
            } else {
                return (-score, 1);
            }
        }
        let mut nodes = 0;
        let mut best_score = i64::min_value() + 1;
        let moves = movegenerator::generate_moves(self);
        for m in moves {
            self.make_move(m);
            let (score, n) = self.negamax_iter(depth - 1);
            nodes += n;
            if -score > best_score {
                best_score = -score;
            }
            self.unmake_move(m);
        }

        (best_score, nodes)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "Pos {{\n");
        //write!(f, "  turn: {:?}\n", self.turn);
        //write!(f, "  cast: {}\n", self.castling_rights);

        //write!(f, "  history:");

        for (i, m) in self.history.iter().enumerate() {
            if i % 2 == 0 {
                write!(f, "\n    {:2}: {}", 1+(i/2), m);
            } else {
                write!(f, " {}", m);
            }
        }

        write!(f, "\n\n");
        write!(f, "       {:?}    {:?}", self.turn, self.castling_rights);

        write!(f, "{}\n", self.board);
        //write!(f, "\n}}");
        Ok(())
    }
}
