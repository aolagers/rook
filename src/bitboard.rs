#![allow(unused_must_use)]
use std::ops::{BitOr, BitAnd, BitXor, Not};
use std::fmt;

const COL_A: u64 = 0x0101_0101_0101_0101;
const COL_H: u64 = 0x8080_8080_8080_8080;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn empty() -> Self {
        BitBoard(0)
    }
    pub fn from_square(sq: usize) -> Self {
        debug_assert!(sq < 64);
        BitBoard(1 << sq)
    }
    pub fn new(n: u64) -> Self {
        BitBoard(n)
    }
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    pub fn is_not_empty(&self) -> bool {
        self.0 != 0
    }
    pub fn up(&self) -> Self {
        BitBoard(self.0 << 8)
    }
    pub fn down(&self) -> Self {
        BitBoard(self.0 >> 8)
    }
    pub fn left(&self) -> Self {
        BitBoard((self.0 & !COL_A) >> 1)
    }
    pub fn right(&self) -> Self {
        BitBoard((self.0 & !COL_H) << 1)
    }
    pub fn nw(&self) -> Self {
        BitBoard((self.0 & !COL_A) << 7)
    }
    pub fn ne(&self) -> Self {
        BitBoard((self.0 & !COL_H) << 9)
    }
    pub fn sw(&self) -> Self {
        BitBoard((self.0 & !COL_A) >> 9)
    }
    pub fn se(&self) -> Self {
        BitBoard((self.0 & !COL_H) >> 7)
    }
    pub fn largets_bit(&self) -> usize {
        let mut cnt = 0;
        let mut div = self.0;
        while div != 0 {
            div >>= 1;
            cnt += 1;
        }
        cnt
    }
    pub fn count_bits(&self) -> usize {
        let mut cnt = 0;
        let mut rest = self.0;
        while rest != 0 {
            cnt += 1;
            rest = rest & rest - 1;
        }
        // for i in 0 .. 64 {
        // if self.0 & (1 << i) != 0 {
        // cnt += 1;
        // }
        // }

        cnt
    }

    pub fn to_str(&self) -> String {
        debug_assert!(self.count_bits() == 1);
        let lb = self.largets_bit() - 1;
        let r = lb / 8;
        let c = lb % 8;
        debug_assert!(r <= 7);
        debug_assert!(c <= 7);

        let mut s = String::new();
        s.push(('a' as u8 + c as u8) as char);
        s.push(('1' as u8 + r as u8) as char);
        s
    }

    pub fn from_str(s: &str) -> Option<BitBoard> {
        if s.len() != 2 { return None; }
        let mut it = s.trim().chars();
        let cc = it.next().unwrap();
        let rc = it.next().unwrap();
        let c = cc as u8 - 'a' as u8;
        let r = rc as u8 - '1' as u8;
        if r <= 7 && c <= 7 {
            Some(BitBoard(1 << (r * 8 + c)))
        } else {
            None
        }
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: BitBoard) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}

// impl BitOrAssign for BitBoard {
//     fn bitor_assign(&mut self, rhs: BitBoard) {
//         self.0 |= rhs.0;
//     }
// }

impl BitXor for BitBoard {
    type Output = BitBoard;
    fn bitxor(self, rhs: BitBoard) -> Self {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: BitBoard) -> Self {
        BitBoard(self.0 & rhs.0)
    }
}

impl Not for BitBoard {
    type Output = BitBoard;
    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                let idx = (7 - i) * 8 + j;
                if self.0 & (1 << idx) > 0 {
                    write!(f, "X ");
                } else {
                    write!(f, ". ");
                }
            }
            write!(f, "\n");
        }
        write!(f, "")
    }
}

impl IntoIterator for BitBoard {
    type Item = BitBoard;
    type IntoIter = BitBoardIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        BitBoardIntoIterator { bits_left: self.0 }
    }
}

pub struct BitBoardIntoIterator {
    bits_left: u64,
}

impl Iterator for BitBoardIntoIterator {
    type Item = BitBoard;
    fn next(&mut self) -> Option<BitBoard> {
        if self.bits_left == 0 { return None; }
        let lsb = self.bits_left.trailing_zeros();
        let sw = 63 - lsb;
        let r = BitBoard((self.bits_left << sw) >> sw);
        self.bits_left = self.bits_left & self.bits_left - 1;
        Some(r)

        // let mut b = self.bits_left;
        // if b == 0 {
        //     return None;
        // }
        // b |= b >> 1;
        // b |= b >> 2;
        // b |= b >> 4;
        // b |= b >> 8;
        // b |= b >> 16;
        // b |= b >> 32;
        // let lowerbits = b >> 1;
        // self.bits_left = self.bits_left & lowerbits;
        // return Some(BitBoard(lowerbits + 1));
    }
}

#[test]
fn bit_ops() {
    let a = BitBoard(1);
    let b = BitBoard(2);
    let c = BitBoard(3);

    assert_eq!(a | b, BitBoard(3));
    assert_eq!(a | c, BitBoard(3));
    assert_eq!(!c, BitBoard(!3));
    assert_eq!(a & b, BitBoard::empty());
    assert_eq!(b & c, BitBoard(2));
    assert_eq!(a ^ b, BitBoard(3));
    assert_eq!(b ^ c, BitBoard(1));
}

#[test]
fn bit_moves() {
    let a1 = BitBoard(1);
    let a8 = BitBoard(1 << 56);
    let f4 = BitBoard(1 << 3 * 8 + 4);

    assert_eq!(a1.left(), BitBoard(0));
    assert_eq!(a8.up(), BitBoard(0));
    assert_eq!(a1.up().up().up().up(), a8.down().down().down());
    assert_eq!(a1.left(), BitBoard::empty());
    assert_eq!(f4.ne().nw().sw().se(), f4);
    assert_eq!(f4.up().down().left().right(), f4);
    assert_eq!(BitBoard(0xffff_ffff_ffff_ffff)
                   .left().left().left().left().left().left().left()
                   .up().up().up().up().up().up().up(),
               a8);
}
