#![allow(dead_code)]

use std::fmt;
use std::io::{self, Read};

use self::PieceType::*;
use self::Color::*;
use bitboard::BitBoard;
use pos::Pos;


/// Player color
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum Color {
    White = 0,
    Black = 6,
}

impl Color {

    /// Returns the opposite color
    pub fn other(self) -> Color {
        match self {
            White => Black,
            Black => White,
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
    King = 5,
}

/// Chess piece. Consists of color and piece type.
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub struct Pc(pub Color, pub PieceType);
impl Pc {
    pub fn as_index(&self) -> usize {
        self.0 as usize + self.1 as usize
    }
}

impl fmt::Display for Pc {
    /// Piece is printed as a [unicode chess
    /// symbol](https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode).
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
            Pc(White, King) => '♚',
        };

        write!(f, "{} ", c)
    }
}

/// Chess move
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    /// Origin square
    pub from: BitBoard,
    /// Target square
    pub to: BitBoard,
    /// Piece
    pub piece: Pc,
    /// If this is a capturing move, contains the captured piece.
    pub capture: Option<Pc>,
    /// If this is a promoting move, contains the type of the new piece.
    pub promotion: Option<Pc>,
    pub castling: Option<Castling>,
}

impl Move {
    /// Parse a move from string.
    /// For example, 'e2e4' for a simple pawn move or 'e7e8q' for promotion to queen
    pub fn from_str(pos: &Pos, s: &str) -> Option<Move> {
        let fr_str: String = s.chars().take(2).collect();
        let to_str: String = s.chars().skip(2).take(2).collect();
        let pr_in = s.chars().skip(4).next();

        let fr = match BitBoard::from_str(&fr_str) {
            None => { return None; },
            Some(sq) => sq
        };

        let to = match BitBoard::from_str(&to_str) {
            None => { return None; },
            Some(sq) => sq
        };

        let pc = match pos.board.get(fr) {
            None => { return None; },
            Some(p) => p
        };

        let Pc(color, _) = pc;

        let pr_type = match pr_in {
            None => None,
            Some(prs) => {
                let pt = match prs {
                    'q' => Queen,
                    'r' => Rook,
                    'k' => Knight,
                    'b' => Bishop,
                    _ => panic!("invalid promotion type")
                };
                Some(Pc(color, pt))
            }
        };

        // let mut castling = None;
        // if let Pc(_, King) = pc {
        //     if let Some(cst) = Castling::from_square(to) {
        //         castling = Some(cst);
        //     }
        // }

        Some(Move {
            from: fr,
            to: to,
            piece: pc,
            capture: pos.board.get(to),
            promotion: pr_type,
            castling: None,
        })
    }

    pub fn from_input(pos: &Pos) -> Move {
        loop {
            println!("> ");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let mv = Move::from_str(pos, input.trim());
            match mv {
                Some(m) => { return m; },
                None => {
                    println!("Invalid move: '{}'", input.trim());
                }
            }
        }
    }

    pub fn to_str(&self) -> String {
        let promo_str = match self.promotion {
            None => "",
            Some(Pc(_, p)) => {
                match p {
                    Queen => "q",
                    Rook => "r",
                    Knight => "n",
                    Bishop => "b",
                    _ => panic!("invalid promotion")
                }
            }
        };
        format!("{}{}{}", self.from.to_str(), self.to.to_str(), promo_str)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // if let Some(castling) = self.castling {
            // if castling.intersects(WHITE_KINGSIDE | BLACK_KINGSIDE) {
            //     write!(f, "O-O");
            // }
            // if castling.intersects(WHITE_QUEENSIDE | BLACK_QUEENSIDE) {
            //     write!(f, "O-O-O");
            // }
        {
            write!(f,
                   "{}{} → {}",
                   self.piece,
                   self.from.to_str(),
                   self.to.to_str());
            if let Some(capt) = self.capture {
                write!(f, " x{}", capt);
            } else {
                write!(f, "    ");
            }
            if let Some(promotion) = self.promotion {
                write!(f, "={}", promotion);
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Castling {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside
}
impl Castling {
    pub fn from_str(s: &str) -> u8 {
        let mut rights = 0;
        for c in s.chars() {
            match c {
                'K' => rights = rights | Castling::WhiteKingside as u8,
                'Q' => rights = rights | Castling::WhiteQueenside as u8,
                'k' => rights = rights | Castling::BlackKingside as u8,
                'q' => rights = rights | Castling::BlackQueenside as u8,
                _ => {}
            }
        }
        rights
    }

    fn empty_pattern(self) -> BitBoard {
        match self {
            Castling::WhiteKingside  => BitBoard::new(0x0000_0000_0000_0060),
            Castling::WhiteQueenside => BitBoard::new(0x0000_0000_0000_000e),
            Castling::BlackKingside  => BitBoard::new(0x6000_0000_0000_0000),
            Castling::BlackQueenside => BitBoard::new(0x0e00_0000_0000_0000),
        }
    }

    pub fn get_rook_move(self) -> (BitBoard, BitBoard) {
        (BitBoard::empty(), BitBoard::empty())
        // match self {
        //     WHITE_KINGSIDE => (BitBoard(1 << 7), BitBoard(1 << 5)),
        //     WHITE_QUEENSIDE => (BitBoard(1 << 0), BitBoard(1 << 3)),
        //     BLACK_KINGSIDE => (BitBoard(1 << 63), BitBoard(1 << 61)),
        //     BLACK_QUEENSIDE => (BitBoard(1 << 56), BitBoard(1 << 59)),
        // }
    }
}
