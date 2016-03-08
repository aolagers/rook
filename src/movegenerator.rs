

use pos::Pos;
use types::Move;
use types::Pc;
use types::Color::*;
use types::PieceType::*;
use bitboard::BitBoard;

pub fn generate_legal_moves(pos: &Pos) -> Vec<Move> {
    let mut p2 = pos.duplicate();
    let mut legalmoves = Vec::new();

    let allmoves = generate_moves(pos);

    for m in allmoves {
        p2.make_move(m);
        let atk = generate_attack_map(&p2);
        if (p2.board.pieces[pos.turn as usize + King as usize] & atk).is_not_empty() {
            // CHECK
        } else {
            legalmoves.push(m.clone());
        }

        p2.unmake_move(m);
    }

    legalmoves
}

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

pub fn generate_attack_map(pos: &Pos) -> BitBoard {
    let mut moves = Vec::new();
    let mut atk = pawn_moves(pos, &mut moves);
    atk = atk | knight_moves(pos, &mut moves);
    atk = atk | bishop_moves(pos, &mut moves);
    atk = atk | rook_moves(pos, &mut moves);
    atk = atk | queen_moves(pos, &mut moves);
    atk = atk | king_moves(pos, &mut moves);

    atk
}

fn pawn_moves(pos: &Pos, moves: &mut Vec<Move>) -> BitBoard {
    let mut threatens = BitBoard::empty();

    let pawn_positions = pos.board.get_squares(Pc(pos.turn, Pawn));
    for pp in pawn_positions {
        let (all_moves, all_attacks) = match pos.turn {
            White =>
                (PAWN_MOVES_WHITE[pp] &
                 !(BitBoard::new(0x0000_0000_00ff_0000) & !pp & pos.board.occupied).up(),
                 PAWN_ATTACKS_WHITE[pp]),
            Black =>
                (PAWN_MOVES_BLACK[pp] &
                 !(BitBoard::new(0x0000_ff00_0000_0000) & !pp & pos.board.occupied).down(),
                 PAWN_ATTACKS_BLACK[pp])
        };

        let possible_moves = all_moves & !pos.board.occupied;
        let possible_attacks = all_attacks & pos.board.theirs(pos.turn);

        for m in possible_moves | possible_attacks {
            let prom = if (m & BitBoard::new(0xff00_0000_0000_00ff)).is_not_empty() {
                Some(Pc(pos.turn, Queen))
            } else {
                None
            };
            moves.push(Move {
                from: pp,
                to: m,
                piece: Pc(pos.turn, Pawn),
                capture: pos.board.get(m),
                promotion: prom,
                castling: None,
            });
        }

        threatens = threatens | (all_attacks & !pos.board.mine(pos.turn));
    }

    threatens
}

fn knight_moves(pos: &Pos, moves: &mut Vec<Move>) -> BitBoard {
    let mut threatens = BitBoard::empty();
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
                castling: None,
            });
        }

        threatens = threatens | possible_mvs;
    }
    threatens
}

fn king_moves(pos: &Pos, moves: &mut Vec<Move>) -> BitBoard {
    let mut threatens = BitBoard::empty();
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
                castling: None,
            });
        }
        threatens = threatens | possible_mvs;

        // castling
        //
        // if pos.turn == White {
        //     let ks =
        //         pos.castling_rights.contains(WHITE_KINGSIDE) &&
        //         (pos.board.occupied & castling_pattern(WHITE_KINGSIDE)).is_empty();
        //     if ks {
        //         let to = k.right().right();
        //         moves.push(Move {
        //             from: k,
        //             to: to,
        //             piece: Pc(pos.turn, King),
        //             capture: pos.board.get(to),
        //             promotion: None,
        //             castling: Some(WHITE_KINGSIDE),
        //         });
        //     }
        // }
    }

    threatens
}

fn bishop_moves(pos: &Pos, moves: &mut Vec<Move>) -> BitBoard {
    ray_moves(pos,
              Pc(pos.turn, Bishop),
              [BitBoard::nw, BitBoard::ne, BitBoard::sw, BitBoard::se],
              moves)
}

fn rook_moves(pos: &Pos, moves: &mut Vec<Move>) -> BitBoard {
    ray_moves(pos,
              Pc(pos.turn, Rook),
              [BitBoard::up, BitBoard::down, BitBoard::left, BitBoard::right],
              moves)
}

fn queen_moves(pos: &Pos, moves: &mut Vec<Move>) -> BitBoard {
    ray_moves(pos,
        Pc(pos.turn, Queen),
        [BitBoard::nw, BitBoard::ne,   BitBoard::sw,   BitBoard::se],    moves)
    | ray_moves(pos,
        Pc(pos.turn, Queen),
        [BitBoard::up, BitBoard::down, BitBoard::left, BitBoard::right], moves)
}

fn ray_moves(pos: &Pos,
             piece: Pc,
             direction_func: [fn(&BitBoard) -> BitBoard; 4],
             moves: &mut Vec<Move>)
             -> BitBoard {
    let mut threatens = BitBoard::empty();
    let diag_pieces = pos.board.get_squares(piece);
    for dp in diag_pieces {
        for f in direction_func.iter() {
            let mut to = f(&dp);
            while to.is_not_empty() && (to & pos.board.mine(pos.turn)).is_empty() {
                threatens = threatens | to;
                if (to & pos.board.mine(pos.turn)).is_empty() {
                    let capt = pos.board.get(to);
                    moves.push(Move {
                        from: dp,
                        to: to,
                        piece: piece,
                        capture: capt,
                        promotion: None,
                        castling: None,
                    });
                    if let Some(_) = capt {
                        break;
                    }
                }

                to = f(&to);
            }
        }
    }

    threatens
}

lazy_static! {
    static ref KNIGHT_MOVES: [BitBoard; 64] = {
        let mut kmoves = [BitBoard::empty(); 64];
        for i in 0..64 {
            let p = BitBoard::from_square(i);
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
        let mut kmoves = [BitBoard::empty(); 64];
        for i in 0..64 {
            let p = BitBoard::from_square(i);
            kmoves[i] = p.up() | p.down() | p.left() | p.right() |
                        p.ne() | p.nw()   | p.se()   | p.sw();
        }
        kmoves
    };

    static ref PAWN_MOVES_WHITE: [BitBoard; 64] = {
        let mut moves = [BitBoard::empty(); 64];
        for i in 0..64 {
            let p = BitBoard::from_square(i);
            moves[i] = p.up();
            if i > 7 && i < 16 { // row 2
                moves[i] = p.up() | p.up().up();
            }
        }
        moves
    };
    static ref PAWN_MOVES_BLACK: [BitBoard; 64] = {
        let mut moves = [BitBoard::empty(); 64];
        for i in 0..64 {
            let p = BitBoard::from_square(i);
            moves[i] = p.down();
            if i > 47 && i < 56 { // row 7
                moves[i] = p.down() | p.down().down();
            }
        }
        moves
    };

    static ref PAWN_ATTACKS_WHITE: [BitBoard; 64] = {
        let mut moves = [BitBoard::empty(); 64];
        for i in 0..64 {
            let p = BitBoard::from_square(i);
            moves[i] = p.ne() | p.nw();
        }
        moves
    };
    static ref PAWN_ATTACKS_BLACK: [BitBoard; 64] = {
        let mut moves = [BitBoard::empty(); 64];
        for i in 0..64 {
            let p = BitBoard::from_square(i);
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

// use test::Bencher;

// #[bench]
// fn bench_movegen() {
// }
