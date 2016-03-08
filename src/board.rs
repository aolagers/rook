use std::fmt;
use bitboard::BitBoard;
use types::{Pc, Color, Move, Castling};
use types::Color::*;
use types::PieceType::*;
use movegenerator;
use eval;

#[derive(Debug)]
pub struct Board {
    /// Array of bitboards, one for each piece. 2 colors * 6 pieces = 12 bitboards
    pub pieces: [BitBoard; 12],

    /// White pieces
    pub whites: BitBoard,

    /// Black pieces
    pub blacks: BitBoard,

    /// All pieces on the board
    pub occupied: BitBoard
}

impl Board {
    // Empty board
    fn empty() -> Self {
        Board {
            pieces: [BitBoard::empty(); 12],
            whites: BitBoard::empty(),
            blacks: BitBoard::empty(),
            occupied: BitBoard::empty(),
        }
    }

    fn clear(&mut self, sq: BitBoard) {
        debug_assert!(sq.count_bits() == 1);

        for bb in self.pieces.iter_mut() {
            *bb = *bb & !sq;
        }
        self.recalc();
    }

    /// Returns a new board with the same position
    pub fn duplicate(&self) -> Board {
        Board {
            pieces: self.pieces.clone(),
            whites: self.whites,
            blacks: self.blacks,
            occupied: self.occupied
        }
    }

    /// Put a piece on a square
    fn set(&mut self, sq: BitBoard, p: Pc) {
        debug_assert!(sq.count_bits() == 1);

        self.clear(sq);
        let Pc(c, k) = p;
        let idx = c as usize + k as usize;
        self.pieces[idx] = self.pieces[idx] | sq;
        self.recalc();
    }

    /// Get the piece on a given square
    pub fn get(&self, sq: BitBoard) -> Option<Pc> {
        if (sq.count_bits() != 1) {
            println!("{}", sq);
        }
        debug_assert!(sq.count_bits() == 1);

        if (sq & self.occupied).is_empty() { return None; }

        if (sq & self.pieces[0]).is_not_empty()  { return Some(Pc(White, Pawn)); }
        if (sq & self.pieces[6]).is_not_empty()  { return Some(Pc(Black, Pawn)); }

        if (sq & self.pieces[1]).is_not_empty()  { return Some(Pc(White, Knight)); }
        if (sq & self.pieces[7]).is_not_empty()  { return Some(Pc(Black, Knight)); }

        if (sq & self.pieces[2]).is_not_empty()  { return Some(Pc(White, Bishop)); }
        if (sq & self.pieces[8]).is_not_empty()  { return Some(Pc(Black, Bishop)); }

        if (sq & self.pieces[3]).is_not_empty()  { return Some(Pc(White, Rook)); }
        if (sq & self.pieces[9]).is_not_empty()  { return Some(Pc(Black, Rook)); }

        if (sq & self.pieces[5]).is_not_empty()  { return Some(Pc(White, King)); }
        if (sq & self.pieces[11]).is_not_empty() { return Some(Pc(Black, King)); }

        if (sq & self.pieces[4]).is_not_empty()  { return Some(Pc(White, Queen)); }
        if (sq & self.pieces[10]).is_not_empty() { return Some(Pc(Black, Queen)); }

        panic!("invalid board!");
    }

    /// Get a BitBoard the positions of all pieces of this type
    pub fn get_squares(&self, piece: Pc) -> BitBoard {
        let Pc(c, t) = piece;
        self.pieces[c as usize + t as usize]
    }

    /// Get a bitboard of all pieces by the given player
    pub fn mine(&self, color: Color) -> BitBoard {
        match color {
            White => self.whites,
            Black => self.blacks
        }
    }

    /// Get a bitboard of all the enemy pieces of the given player
    pub fn theirs(&self, color: Color) -> BitBoard {
        match color {
            White => self.blacks,
            Black => self.whites
        }
    }

    fn recalc(&mut self) {
        self.whites = BitBoard::empty();
        self.blacks = BitBoard::empty();
        for i in 0..6  { self.whites = self.whites | self.pieces[i]; }
        for i in 6..12 { self.blacks = self.blacks | self.pieces[i]; }
        self.occupied = self.whites | self.blacks;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n    ");
        for i in 0..8 {
            write!(f, "{} ", (7-i)+1);
            for j in 0..8 {
                let sq = BitBoard::from_square(((7-i)*8 + j));
                match self.get(sq) {
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
    pub board: Board,
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
            board: Board::empty(),
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
                let sq = BitBoard::from_square(r*8 + c);
                let p = self.board.get(sq);
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
            let sq = if col < 8 { BitBoard::from_square(idx) } else { BitBoard::empty() };
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
        // self.history.push(mv);

        //if let Pc(_, Pawn) = mv.piece { self.halfmoves = 0; }
        //if let Some(_) = mv.capture { self.halfmoves = 0; }
    }

    pub fn unmake_move(&mut self, mv: Move) {
        // self.history.pop();
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
