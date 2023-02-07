use cozy_chess::{Move, Board, Square, Piece, Color};
use std::io;
use std::sync::atomic::{Ordering};
use std::time::{Instant};
use std::collections::HashMap;

pub mod movegen;
pub mod evaluate;

use crate::NODE_CTR;
use evaluate::evaluate;

pub fn quiesce(brd: &Board, mut transtable: HashMap<u64,Move>, stoptime: Instant, mut alpha: i32, beta: i32) -> (i32,HashMap<u64,Move>) {
    NODE_CTR.store(NODE_CTR.load(Ordering::Relaxed) + 1, Ordering::Relaxed); //increment node count
	if Instant::now().checked_duration_since(stoptime).is_some() {
		return (0,transtable)
	}
	let mut bestscore = evaluate(brd);
    if bestscore >= beta {
        return (beta,transtable);
    }
	
	if bestscore < alpha - crate::VALUES.0[4] {
		return (alpha,transtable)
	}
	
    if alpha < bestscore {
        alpha = bestscore;
    }
	let mut move_list = movegen::list_captures(brd);
    match transtable.get(&brd.hash()){
        Some(hash) => {
            //move_list.retain(|&x| x != *hash);
            move_list.insert(0, *hash);
        },
        None => (),
    }
    let mut bestmove = None;
	for mov in move_list {
		let mut new_brd = brd.clone();
		new_brd.play(mov);
        let result = quiesce(&new_brd, transtable, stoptime, 0 - beta, 0 - alpha);
        transtable = result.1;
		let score = 0 - result.0;
		if score >= beta {
            transtable.insert(brd.hash(),mov);
			return (score,transtable);
		}
		if score > bestscore {
            bestmove = Some(mov);
			bestscore = score;
			if score > alpha {
				alpha = score;
			}
		}
	}
    if bestmove.is_some() {
        transtable.insert(brd.hash(),bestmove.unwrap());
    }
	(bestscore,transtable)
    
}