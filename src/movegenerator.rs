

use ::Pos;
use types::Move;
use types::Pc;
use types::Square;
use types::Color::*;
use types::PieceType::*;



pub fn generate_moves(pos: &Pos) -> Vec<Move> {
    let mut moves = Vec::new();

    let wps = pos.board.pieces[pos.turn as usize + Pawn as usize];

    let wps_up = wps.up();

    println!("{}", wps_up);

    moves.push(Move {
        from: Square(12),
        to: Square(20),
        piece: Pc(White, Knight),
        capture: None,
        promotion: None
    });

    moves
}
