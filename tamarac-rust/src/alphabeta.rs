use cozy_chess::{Move, Board, Square, Piece, Color};
use std::io;
use std::sync::atomic::{AtomicU64, AtomicI32, Ordering};
use std::time::{Instant, Duration};
use std::collections::HashMap;

mod quiesce;
//mod movegen;

use quiesce::movegen::list_moves;

pub fn alphabeta(brd: &Board, mut transtable: HashMap<u64,Move>, depth: i32, maxdepth: i32, stoptime: Instant, mut alpha: i32, beta: i32) -> (i32,HashMap<u64,Move>) {
    crate::NODE_CTR.store(crate::NODE_CTR.load(Ordering::Relaxed) + 1, Ordering::Relaxed); //increment node count
	if Instant::now().checked_duration_since(stoptime).is_some() {
		return (0,transtable)
	}
	if depth == 0 {
		return quiesce::quiesce(brd,transtable,stoptime,alpha,beta);
        //return evaluate(brd);
	}
	let mut move_list = list_moves(brd);
	if move_list.len() == 0 {
		if brd.checkers().len() == 0 {
			return (0 + (maxdepth-depth),transtable);
		} else {
			let matescore = crate::NEG_INFINITY + (maxdepth-depth);
			//println!("info string depth {depth} maxdepth {maxdepth} matescore {matescore}");
			return (crate::NEG_INFINITY + (maxdepth-depth),transtable);
		}
	}
    match transtable.get(&brd.hash()){
        Some(hash) => {
            //move_list.retain(|&x| x != *hash);
            move_list.insert(0, *hash);
        },
        None => (),
    }
    
	let mut bestscore = crate::NEG_INFINITY;
    let mut bestmove = None;
	for mov in move_list {
		let mut new_brd = brd.clone();
		new_brd.play(mov);
		let result = alphabeta(&new_brd, transtable, depth - 1, maxdepth, stoptime, 0-beta , 0 - alpha);
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

pub fn root_alphabeta(brd: &Board, mut transtable: HashMap<u64,Move>, stoptime: Instant, depth: i32) -> (Move,HashMap<u64,Move>,i32) {
    crate::NODE_CTR.store(crate::NODE_CTR.load(Ordering::Relaxed) + 1, Ordering::Relaxed); //increment node count
	let move_list = list_moves(brd);
	let mut bestscore = crate::NEG_INFINITY;
	let mut bestmove = None;
	let mut alpha = crate::NEG_INFINITY;
	for mov in move_list {
		let mut new_brd = brd.clone();
		new_brd.play(mov);
        let result = alphabeta(&new_brd, transtable, depth - 1, depth, stoptime, crate::NEG_INFINITY, 0 - alpha);
        transtable = result.1;
		let score = 0 - result.0;
		if score > bestscore {
			bestmove = Some(mov);
			bestscore = score;
			//println!("info string score {score} bestscore {bestscore}");
			if score > alpha {
				alpha = score;
			}
		}
	}
    //crate::SCORE.store(bestscore, Ordering::Relaxed);
	//println!("{}", bestscore);
	(bestmove.unwrap(),transtable,bestscore)
}