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
    /// If this a castling, contain the type
    pub castling: Option<CastlingMove>,
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

        let mut castling = None;
        if let Pc(_, King) = pc {
             if let Some(cst) = CastlingMove::from_squares(fr | to) {
                castling = Some(cst);
            }
        }

        Some(Move {
            from: fr,
            to: to,
            piece: pc,
            capture: pos.board.get(to),
            promotion: pr_type,
            castling: castling,
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
pub enum CastlingMove {
    WhiteKingside   = 0b0001,
    WhiteQueenside  = 0b0010,
    BlackKingside   = 0b0100,
    BlackQueenside  = 0b1000
}

use self::CastlingMove::*;

impl CastlingMove {

    pub fn empty_pattern(&self) -> BitBoard {
        match *self {
            WhiteKingside  => BitBoard::new(0x0000_0000_0000_0060),
            WhiteQueenside => BitBoard::new(0x0000_0000_0000_000e),
            BlackKingside  => BitBoard::new(0x6000_0000_0000_0000),
            BlackQueenside => BitBoard::new(0x0e00_0000_0000_0000),
        }
    }

    pub fn get_rook_move(&self) -> (BitBoard, BitBoard) {
        match *self {
            WhiteKingside  =>
                (BitBoard::from_square(7), BitBoard::from_square(5)),
            WhiteQueenside  =>
                (BitBoard::from_square(0), BitBoard::from_square(3)),
            BlackKingside  =>
                (BitBoard::from_square(63), BitBoard::from_square(61)),
            BlackQueenside  =>
                (BitBoard::from_square(56), BitBoard::from_square(59)),
        }
    }

    pub fn from_squares(sq: BitBoard) -> Option<CastlingMove> {
        if sq == BitBoard::from_square(4) | BitBoard::from_square(6) {
            return Some(WhiteKingside);
        } else if sq == BitBoard::from_square(4) | BitBoard::from_square(2) {
            return Some(WhiteQueenside);
        } else if sq == BitBoard::from_square(60) | BitBoard::from_square(62) {
            return Some(BlackQueenside);
        } else if sq == BitBoard::from_square(60) | BitBoard::from_square(58) {
            return Some(BlackQueenside);
        } else {
            return None;
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            WhiteKingside  => 'K',
            WhiteQueenside => 'Q',
            BlackKingside  => 'k',
            BlackQueenside => 'q',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
             'K' => WhiteKingside,
             'Q' => WhiteQueenside,
             'k' => BlackKingside,
             'q' => BlackQueenside,
             _   => panic!("invalid casling right!")
        }
    }

    pub fn str_to_flags(s: &str) -> u8 {
        let mut flg = 0;
        for c in s.chars() {
            if c == '-' { return 0; }

            flg |= CastlingMove::from_char(c) as u8;
        }
        flg
    }
    
    pub fn flags_to_str(f: u8) -> String {
        if f == 0 { return "-".to_string(); }
        let mut s = String::new();
        for v in [WhiteKingside, WhiteQueenside, BlackKingside, BlackQueenside].iter() {
            if (*v as u8 & f) != 0 {
                s.push(v.to_char());
            }
        }
        s
    }
}
