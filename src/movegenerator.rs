

use ::Pos;
use types::Move;
use types::Pc;
use types::Square;
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
    let mut pawn_moves = Vec::new();
    let mut pawn_captures = Vec::new();

    for pp in pawn_positions {
        match pos.turn {
            White => {
                let m = pp.up();
                if (m & pos.board.free).is_not_empty() {
                    pawn_moves.push((pp, m));
                }
                if pp.0 & 0x0000_0000_0000_ff00 != 0 {
                    let dm = pp.up().up();
                    if ((pp.up() | pp.up().up()) & pos.board.occupied).is_empty() {
                        pawn_moves.push((pp, dm));
                    }
                }

                let atk1 = pp.ne();
                let atk2 = pp.nw();

                if (atk1 & pos.board.blacks).is_not_empty() {pawn_captures.push((pp, atk1));}
                if (atk2 & pos.board.blacks).is_not_empty() {pawn_captures.push((pp, atk2));}
            },
            Black => {
                let m = pp.down();
                if (m & pos.board.free).is_not_empty() {
                    pawn_moves.push((pp, m));
                }
                if pp.0 & 0x00ff_0000_0000_0000 != 0 {
                    let dm = pp.down().down();
                    if ((pp.down() | pp.down().down()) & pos.board.occupied).is_empty() {
                        pawn_moves.push((pp, dm));
                    }
                }

                let atk1 = pp.se();
                let atk2 = pp.sw();

                if (atk1 & pos.board.blacks).is_not_empty() {pawn_captures.push((pp, atk1));}
                if (atk2 & pos.board.blacks).is_not_empty() {pawn_captures.push((pp, atk2));}
            }
        }
    }

    for pm in pawn_moves {
        moves.push(Move {
            from: pm.0,
            to: pm.1,
            piece: Pc(pos.turn, Pawn),
            capture: None,
            promotion: None
        });
    }
    for pc in pawn_captures {
        moves.push(Move {
            from: pc.0,
            to: pc.1,
            piece: Pc(pos.turn, Pawn),
            capture: pos.board.get(pc.1),
            promotion: None
        });
    }
}

fn knight_moves(pos: &Pos, moves: &mut Vec<Move>) {
    let knight_positions = pos.board.get_squares(Pc(pos.turn, Knight));

    for kp in knight_positions {
        let kmoves = [
            kp.up().up().left(),
            kp.up().up().right(),
            kp.up().left().left(),
            kp.up().right().right(),
            kp.down().left().left(),
            kp.down().right().right(),
            kp.down().down().left(),
            kp.down().down().right()
        ];
        for to in kmoves.iter() {
            if to.is_not_empty() && (*to & pos.board.mine(pos.turn)).is_empty() {
                moves.push(Move {
                    from: kp,
                    to: *to,
                    piece: Pc(pos.turn, Knight),
                    capture: pos.board.get(*to),
                    promotion: None
                });
            }
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

fn ray_moves(pos: &Pos, piece: Pc, direction: [fn(&BitBoard) -> BitBoard; 4], moves: &mut Vec<Move>) {
    let diag_pieces = pos.board.get_squares(piece);
    for dp in diag_pieces {
        for f in direction.iter() {
            let mut to = f(&dp);
            while to.is_not_empty() && (to & pos.board.mine(pos.turn)).is_empty() {
                if (to & pos.board.mine(pos.turn)).is_empty() {
                    moves.push(Move {
                        from: dp,
                        to: to,
                        piece: piece,
                        capture: pos.board.get(to),
                        promotion: None
                    });
                }

                to = f(&to);
            }
        }
    }
}

fn king_moves(pos: &Pos, moves: &mut Vec<Move>) {
    let k_positions = pos.board.get_squares(Pc(pos.turn, King));

    for k in k_positions {
        let kmoves = [
            k.up(), k.ne(), k.right(), k.se(), k.down(), k.sw(), k.left(), k.nw()
        ];
        for to in kmoves.iter() {
            if to.is_not_empty() && (*to & pos.board.mine(pos.turn)).is_empty() {
                moves.push(Move {
                    from: k,
                    to: *to,
                    piece: Pc(pos.turn, King),
                    capture: pos.board.get(*to),
                    promotion: None
                });
            }
        }
    }
}
