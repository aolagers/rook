

use board::Pos;
use types::Move;
use types::Pc;
use types::Color::*;
use types::PieceType::*;
use bitboard::BitBoard;

pub fn generate_moves(pos: &Pos) -> Vec<Move> {

    let mut moves = Vec::new();

    pawn_moves(pos, &mut moves);
    knight_moves(pos, &mut moves);
    bishop_moves(pos, &mut moves);
    rook_moves(pos, &mut moves);
    queen_moves(pos, &mut moves);
    king_moves(pos, &mut moves);

    moves
}

fn pawn_moves(pos: &Pos, moves: &mut Vec<Move>) {

    let pawn_positions = pos.board.get_squares(Pc(pos.turn, Pawn));

    for pp in pawn_positions {
        let (all_moves, all_attacks) = match pos.turn {
            White => (PAWN_MOVES_WHITE[pp], PAWN_ATTACKS_WHITE[pp]),
            Black => (PAWN_MOVES_BLACK[pp], PAWN_ATTACKS_BLACK[pp])
        };

        let possible_moves = all_moves & pos.board.free;
        let possible_attacks = all_attacks & pos.board.theirs(pos.turn);

        for m in possible_moves {
            moves.push(Move {
                from: pp,
                to: m,
                piece: Pc(pos.turn, Pawn),
                capture: None,
                promotion: None,
                castling: None
            });
        }

        for m in possible_attacks {
            moves.push(Move {
                from: pp,
                to: m,
                piece: Pc(pos.turn, Pawn),
                capture: pos.board.get(m),
                promotion: None,
                castling: None
            });
        }
    }
}

fn knight_moves(pos: &Pos, moves: &mut Vec<Move>) {
    let knight_positions = pos.board.get_squares(Pc(pos.turn, Knight));
    for kp in knight_positions {

        let all_moves = KNIGHT_MOVES[kp];
        let possible_mvs = all_moves & !pos.board.mine(pos.turn);
        for to in possible_mvs {
            moves.push(Move {
                from: kp,
                to: to,
                piece: Pc(pos.turn, Knight),
                capture: pos.board.get(to),
                promotion: None,
                castling: None
            });
        }
    }
}

fn king_moves(pos: &Pos, moves: &mut Vec<Move>) {
    let k_positions = pos.board.get_squares(Pc(pos.turn, King));

    for k in k_positions {
        let all_moves = KING_MOVES[k];
        let possible_mvs = all_moves & !pos.board.mine(pos.turn);

        for to in possible_mvs {
            moves.push(Move {
                from: k,
                to: to,
                piece: Pc(pos.turn, King),
                capture: pos.board.get(to),
                promotion: None,
                castling: None
            });
        }
    }
}
fn bishop_moves(pos: &Pos, moves: &mut Vec<Move>) {
    ray_moves(pos, Pc(pos.turn, Bishop), [BitBoard::nw, BitBoard::ne, BitBoard::sw, BitBoard::se], moves);
}
fn rook_moves(pos: &Pos, moves: &mut Vec<Move>) {
    ray_moves(pos, Pc(pos.turn, Rook), [BitBoard::up, BitBoard::down, BitBoard::left, BitBoard::right], moves);
}
fn queen_moves(pos: &Pos, moves: &mut Vec<Move>) {
    ray_moves(pos, Pc(pos.turn, Queen), [BitBoard::nw, BitBoard::ne, BitBoard::sw, BitBoard::se], moves);
    ray_moves(pos, Pc(pos.turn, Queen), [BitBoard::up, BitBoard::down, BitBoard::left, BitBoard::right], moves);
}

fn ray_moves(pos: &Pos, piece: Pc, direction_func: [fn(&BitBoard) -> BitBoard; 4], moves: &mut Vec<Move>) {
    let diag_pieces = pos.board.get_squares(piece);
    for dp in diag_pieces {
        for f in direction_func.iter() {
            let mut to = f(&dp);
            while to.is_not_empty() && (to & pos.board.mine(pos.turn)).is_empty() {
                if (to & pos.board.mine(pos.turn)).is_empty() {
                    let capt = pos.board.get(to);
                    moves.push(Move {
                        from: dp,
                        to: to,
                        piece: piece,
                        capture: capt,
                        promotion: None,
                        castling: None
                    });
                    if let Some(_) = capt {
                        break;
                    }
                }

                to = f(&to);
            }
        }
    }
}



lazy_static! {
    static ref KNIGHT_MOVES: [BitBoard; 64] = {
        let mut kmoves = [BitBoard(0); 64];
        for i in 0..64 {
            let p = BitBoard(1 << i);
            kmoves[i] =
                p.up().up().left()      |
                p.up().up().right()     |
                p.up().left().left()    |
                p.up().right().right()  |
                p.down().left().left()  |
                p.down().right().right()|
                p.down().down().left()  |
                p.down().down().right();
        }
        kmoves
    };

    static ref KING_MOVES: [BitBoard; 64] = {
        let mut kmoves = [BitBoard(0); 64];
        for i in 0..64 {
            let p = BitBoard(1 << i);
            kmoves[i] = p.up() | p.down() | p.left() | p.right() |
                        p.ne() | p.nw()   | p.se()   | p.sw();
        }
        kmoves
    };

    static ref PAWN_MOVES_WHITE: [BitBoard; 64] = {
        let mut moves = [BitBoard(0); 64];
        for i in 0..64 {
            let p = BitBoard(1 << i);
            moves[i] = p.up();
            if i > 7 && i < 16 { // row 2
                moves[i] = p.up() | p.up().up();
            }
        }
        moves
    };
    static ref PAWN_MOVES_BLACK: [BitBoard; 64] = {
        let mut moves = [BitBoard(0); 64];
        for i in 0..64 {
            let p = BitBoard(1 << i);
            moves[i] = p.down();
            if i > 47 && i < 56 { // row 7
                moves[i] = p.down() | p.down().down();
            }
        }
        moves
    };

    static ref PAWN_ATTACKS_WHITE: [BitBoard; 64] = {
        let mut moves = [BitBoard(0); 64];
        for i in 0..64 {
            let p = BitBoard(1 << i);
            moves[i] = p.ne() | p.nw();
        }
        moves
    };
    static ref PAWN_ATTACKS_BLACK: [BitBoard; 64] = {
        let mut moves = [BitBoard(0); 64];
        for i in 0..64 {
            let p = BitBoard(1 << i);
            moves[i] = p.se() | p.sw();
        }
        moves
    };
}

use std::ops::Index;

impl Index<BitBoard> for [BitBoard] {
    type Output = BitBoard;

    fn index(&self, ib: BitBoard) -> &BitBoard {
        &self[ib.largets_bit() - 1]
    }
}

//use test::Bencher;

//#[bench]
//fn bench_movegen() {
//}
