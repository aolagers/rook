use std::fmt;

use bitboard::BitBoard;
use types::{Pc, Color};
use types::Color::*;
use types::PieceType::*;

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
    /// Empty board
    pub fn empty() -> Self {
        Board {
            pieces: [BitBoard::empty(); 12],
            whites: BitBoard::empty(),
            blacks: BitBoard::empty(),
            occupied: BitBoard::empty(),
        }
    }

    /// Clear one square
    pub fn clear(&mut self, sq: BitBoard) {
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
    pub fn set(&mut self, sq: BitBoard, p: Pc) {
        debug_assert!(sq.count_bits() == 1);

        self.clear(sq);
        let Pc(c, k) = p;
        let idx = c as usize + k as usize;
        self.pieces[idx] = self.pieces[idx] | sq;
        self.recalc();
    }

    /// Get the piece on a given square
    pub fn get(&self, sq: BitBoard) -> Option<Pc> {
        if sq.count_bits() != 1 {
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
