use std::fmt;

use self::PieceType::*;
use self::Color::*;
use bitboard::BitBoard;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum Color {
    White = 0,
    Black = 6
}
impl Color {
    pub fn other(self) -> Color {
        match self {
            White => Black,
            Black => White
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub struct Pc (pub Color, pub PieceType);

impl fmt::Display for Pc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            Pc(Black, Pawn) => '♙',
            Pc(Black, Bishop) => '♗',
            Pc(Black, Knight) => '♘',
            Pc(Black, Rook) => '♖',
            Pc(Black, Queen) => '♕',
            Pc(Black, King) => '♔',
            Pc(White, Pawn) => '♟',
            Pc(White, Bishop) => '♝',
            Pc(White, Knight) => '♞',
            Pc(White, Rook) => '♜',
            Pc(White, Queen) => '♛',
            Pc(White, King)=> '♚'
        };

        write!(f, "{} ", c)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Square(pub usize);
impl Square {
    pub fn to_str(&self) -> String {
        let r = self.0 / 8;
        let c = self.0 % 8;
        debug_assert!(r <= 7);
        debug_assert!(c <= 7);

        let mut s = String::new();
        s.push(('a' as u8 + c as u8) as char);
        s.push(('1' as u8 + r as u8) as char);
        s
    }
    pub fn from_str(s: &str) -> Square {
        let mut it = s.chars();
        let cc = it.next().unwrap();
        let rc = it.next().unwrap();
        let c = cc as u8 - 'a' as u8;
        let r = rc as u8 - '1' as u8;
        debug_assert!(r <= 7);
        debug_assert!(c <= 7);
        Square((r*8 + c) as usize)
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

use board::Pos;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub from: BitBoard,
    pub to: BitBoard,
    pub piece: Pc,
    pub capture: Option<Pc>,
    pub promotion: Option<Pc>
}
impl Move {
    pub fn from_str(pos: &Pos, s: &str) -> Move {
        let mut chars = s.chars();
        let fr_str: String = s.chars().take(2).collect();
        let to_str: String = s.chars().skip(2).take(2).collect();

        Move {
            from: BitBoard::from_str(&fr_str),
            to: BitBoard::from_str(&to_str),
            piece: pos.board.get(BitBoard::from_str(&fr_str)).unwrap(),
            capture: pos.board.get(BitBoard::from_str(&to_str)),
            promotion: None
        }
    }

    pub fn from_input(pos: &Pos) -> Move {
        use std::io::{self, Read};
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

    pub fn to_str(&self) -> String {
        format!("{}{}", self.from.to_str(), self.to.to_str())
    }

}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{} → {}", self.piece, self.from.to_str(), self.to.to_str());
        if let Some(capt) = self.capture { write!(f, " x{}", capt);  }
        if let Some(promotion) = self.promotion { write!(f, "={}", promotion); }

        Ok(())
    }
}
