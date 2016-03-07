use std::fmt;
use std::io::{self, Read};

use self::PieceType::*;
use self::Color::*;
use bitboard::BitBoard;
use board::Pos;


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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub from: BitBoard,
    pub to: BitBoard,
    pub piece: Pc,
    pub capture: Option<Pc>,
    pub promotion: Option<Pc>,
    pub castling: Option<Castling>
}
impl Move {
    pub fn from_str(pos: &Pos, s: &str) -> Move {
        let fr_str: String = s.chars().take(2).collect();
        let to_str: String = s.chars().skip(2).take(2).collect();

        Move {
            from: BitBoard::from_str(&fr_str),
            to: BitBoard::from_str(&to_str),
            piece: pos.board.get(BitBoard::from_str(&fr_str)).unwrap(),
            capture: pos.board.get(BitBoard::from_str(&to_str)),
            promotion: None,
            castling: None
        }
    }

    pub fn from_input(pos: &Pos) -> Move {
        let mut input = String::new();

        print!("> ");
        io::stdin().read_line(&mut input);

        let mut sp = input.trim().split(" ");
        let fr = BitBoard::from_str(sp.next().unwrap());
        let to = BitBoard::from_str(sp.next().unwrap());

        Move {
            from: fr,
            to: to,
            piece: pos.board.get(fr).unwrap(),
            capture: pos.board.get(to),
            promotion: None,
            castling: None
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}{}", self.from.to_str(), self.to.to_str())
    }

}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(castling) = self.castling {
            if castling.intersects(WHITE_KINGSIDE  | BLACK_KINGSIDE) {
                write!(f, "O-O");
            }
            if castling.intersects(WHITE_QUEENSIDE | BLACK_QUEENSIDE) {
                write!(f, "O-O-O");
            }
        } else {
            write!(f, "{}{} → {}", self.piece, self.from.to_str(), self.to.to_str());
            if let Some(capt) = self.capture { write!(f, " x{}", capt);  } else {
                write!(f, "    ");
            }
            if let Some(promotion) = self.promotion { write!(f, "={}", promotion); }
        }

        Ok(())
    }
}

bitflags! {
    pub flags Castling: usize {
        const WHITE_KINGSIDE  = 0b0001,
        const WHITE_QUEENSIDE = 0b0010,
        const BLACK_KINGSIDE  = 0b0100,
        const BLACK_QUEENSIDE = 0b1000
    }
}

impl Castling {
    pub fn from_str(s: &str) -> Self {
        let mut rights = Castling::empty();
        for c in s.chars() {
            match c {
                'K' => rights = rights | WHITE_KINGSIDE,
                'Q' => rights = rights | WHITE_QUEENSIDE,
                'k' => rights = rights | BLACK_KINGSIDE,
                'q' => rights = rights | BLACK_QUEENSIDE,
                _ => {}
            }
        }
        rights
    }
}

impl fmt::Display for Castling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.contains(WHITE_KINGSIDE)  { write!(f, "K"); }
        if self.contains(WHITE_QUEENSIDE) { write!(f, "Q"); }
        if self.contains(BLACK_KINGSIDE)  { write!(f, "k"); }
        if self.contains(BLACK_QUEENSIDE) { write!(f, "q"); }
        Ok(())
    }
}
