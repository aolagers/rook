
use rand;

use types::Color::*;

use bitboard::BitBoard;
use pos::Pos;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

// static HASH: Rc<RefCell<HashMap<u64, u64>>> = Rc::new(RefCell::new(HashMap::new()));


pub fn piece_hash(pc: usize, sq: BitBoard) -> u64 {
    debug_assert!(sq.count_bits() == 1);
    HASH_INIT[pc][sq.largets_bit() - 1]
}

pub fn inc() {
    let mut val = HASH.lock().unwrap();
    val.insert(1, 666);
}

pub fn full_hash(pos: &Pos) -> u64 {
    let mut res = 0;

    for (idx, bbs) in pos.board.pieces.iter().enumerate() {
        for bb in *bbs {
            res ^= piece_hash(idx, bb);
        }
    }

    let r = match pos.turn {
        White => res,
        Black => !res
    };

    let mut hm = HASH.lock().unwrap();
    if hm.contains_key(&r) {
        //println!("HASH HIT!!! {} {:?}", r, *hm);
        //panic!("saf");
        //println!("{:?}", *hm);
    } else {
        hm.insert(r, 1);
    }

    r
}

lazy_static! {
    static ref HASH: Mutex<HashMap<u64, u64>> = {
        Mutex::new(HashMap::new())
    };

    static ref HASH_INIT: [[u64; 64]; 12] = {

        let mut hashes = [[0; 64]; 12];

        for pn in 0..12 {
            for sq in 0..64 {
                hashes[pn][sq] = rand::random();
            }
        }

        hashes
    };
}

#[test]
fn full_hashing() {
    let p1 =        Pos::from_fen("3r2k1/ppp2ppr/8/8/8/P4n1P/2P3q1/4K3 w KQkq - 0 1");
    let p2 =        Pos::from_fen("3r2k1/ppp2ppr/8/8/8/P4n1P/2P3q1/4K3 w KQkq - 0 1");
    let p_turn =    Pos::from_fen("3r2k1/ppp2ppr/8/8/8/P4n1P/2P3q1/4K3 b KQkq - 0 1");
    let p_move =    Pos::from_fen("3r2k1/ppp2ppr/8/8/8/p4n1P/2P3q1/4K3 w KQkq - 0 1");
    let p_cast =    Pos::from_fen("3r2k1/ppp2ppr/8/8/8/P4n1P/2P3q1/4K3 w Kkq - 0 1");

    let p1h = full_hash(&p1);

    assert!(p1h == full_hash(&p2));
    assert!(p1h != full_hash(&p_turn));
    assert!(p1h != full_hash(&p_move));
    assert!(p1h != full_hash(&p_cast));
}
