//castling move check code by crippa#9590

use cozy_chess::{Move, Board, Square, Piece, Color};
use std::io;
use std::sync::atomic::{AtomicU64, AtomicI32, Ordering};
use std::time::{Instant, Duration};
use vampirc_uci::parse;
use vampirc_uci::{UciMessage, MessageList, UciTimeControl};

const VALUES: ([i32; 6], i32, i32) = ([100, 300, 300, 500, 900,10000],50,40); //king value must be greater than 9x queen + 2x rest of pieces
const POS_INFINITY: i32 = 10240;
const NEG_INFINITY: i32 = -10240;

static NODE_CTR: AtomicU64 = AtomicU64::new(0);
static SCORE: AtomicI32 = AtomicI32::new(0);

/*
struct PVtable(Vec<Entry>);

struct Entry {
	hash = 
}
*/
fn list_moves(brd: &Board) -> Vec<Move> {
    let mut move_list = Vec::new();
	let mut capture_list = Vec::new();
	let enemy_pieces = brd.colors(!brd.side_to_move());
	brd.generate_moves(|moves| {
		let mut captures = moves.clone();
		let mut non_captures = moves.clone();
		
		captures.to &= enemy_pieces;
		non_captures.to &= !enemy_pieces;
        // Unpack dense move set into move list
        move_list.extend(non_captures);
		capture_list.extend(captures);
        false
    });
    capture_list.append(&mut move_list);
	capture_list
}

fn list_captures(brd: &Board) -> Vec<Move> {
    let enemy_pieces = brd.colors(!brd.side_to_move());
    let mut move_list = Vec::new();
    brd.generate_moves(|moves| {
        let mut captures = moves.clone();
        // Bitmask to efficiently get all captures set-wise.
        // Excluding en passant square for convenience.
        captures.to &= enemy_pieces;
        move_list.extend(captures);
        false
    });
    move_list
}


fn evaluate(brd: &Board) -> i32 {
	let mut all_total: i32 = 0;
	for clr in Color::ALL{
		let mut total: i32 = 0;
		for x in 0..5 {
			let mut pieces: i32 = brd.colored_pieces(clr, *Piece::ALL.get(x).unwrap()).len().try_into().unwrap();
            /*
            if x == 2 && pieces == 2 {
                total = total + VALUES.1
            }
            */
            pieces = pieces * VALUES.0[x];
			total = total + pieces;
		}
        //total = total + 10 * list_moves(brd).len() as i32;
        //10 * (list_moves(brd).len() as i32))
		if clr == Color::Black {
			total = 0 - total;
		}
		all_total = all_total + total
        
	}
    /*
    let mut move_count = 0;
    brd.generate_moves(|moves| {
        // Unpack dense move set into move list
        move_count += moves.len();
        false
    });
    all_total = all_total + 10 * move_count as i32;
    */
	if brd.side_to_move() == Color::Black {
		all_total = 0 - all_total;
	}
    //println!("{}", total);
	all_total
}

fn alphabeta(brd: &Board, depth: i32, maxdepth: i32, stoptime: Instant, mut alpha: i32, beta: i32) -> i32 {
    NODE_CTR.store(NODE_CTR.load(Ordering::Relaxed) + 1, Ordering::Relaxed); //increment node count
	if Instant::now().checked_duration_since(stoptime).is_some() {
		return 0
	}
	if depth == 0 {
		return quiesce(brd,stoptime,alpha,beta);
        //return evaluate(brd);
	}
	let move_list = list_moves(brd);
	if move_list.len() == 0 {
		if brd.checkers().len() == 0 {
			return 0 + (maxdepth-depth);
		} else {
			return NEG_INFINITY + (maxdepth-depth);
		}
	}
	let mut bestscore = NEG_INFINITY;
	for mov in move_list {
		let mut new_brd = brd.clone();
		new_brd.play(mov);
		let score = 0 - alphabeta(&new_brd, depth - 1, maxdepth, stoptime, 0 - beta, 0 - alpha);
		if score >= beta {
			return score;
		}
		if score > bestscore {
			bestscore = score;
			if score > alpha {
				alpha = score;
			}
		}
	}
	bestscore
}


fn quiesce(brd: &Board, stoptime: Instant, mut alpha: i32, beta: i32) -> i32 {
    NODE_CTR.store(NODE_CTR.load(Ordering::Relaxed) + 1, Ordering::Relaxed); //increment node count
	if Instant::now().checked_duration_since(stoptime).is_some() {
		return 0
	}
	let mut bestscore = evaluate(brd);
    if bestscore >= beta {
        return beta;
    }
	
	if bestscore < alpha - VALUES.0[4] {
		return alpha
	}
	
    if alpha < bestscore {
        alpha = bestscore;
    }
	let move_list = list_captures(brd);
	for mov in move_list {
		let mut new_brd = brd.clone();
		new_brd.play(mov);
		let score = 0 - quiesce(&new_brd, stoptime, 0 - beta, 0 - alpha);
		if score >= beta {
			return score;
		}
		if score > bestscore {
			bestscore = score;
			if score > alpha {
				alpha = score;
			}
		}
	}
	bestscore
}


//fn BNS

fn root_alphabeta(brd: &Board, stoptime: Instant, depth: i32) -> Move {
    NODE_CTR.store(NODE_CTR.load(Ordering::Relaxed) + 1, Ordering::Relaxed); //increment node count
	let move_list = list_moves(brd);
	let mut bestscore = NEG_INFINITY;
	let mut bestmove = None;
	let mut alpha = NEG_INFINITY;
	for mov in move_list {
		let mut new_brd = brd.clone();
		new_brd.play(mov);
		let score = 0 - alphabeta(&new_brd, depth - 1, depth, stoptime, NEG_INFINITY, 0 - alpha);
		if score > bestscore {
			bestmove = Some(mov);
			bestscore = score;
			//println!("info string score {score} bestscore {bestscore}");
			if score > alpha {
				alpha = score;
			}
		}
	}
    SCORE.store(bestscore, Ordering::Relaxed);
	bestmove.unwrap()
}

fn search(brd: &Board, time: &Duration) -> Move {
    NODE_CTR.store(0, Ordering::Relaxed);
    let start = Instant::now();
	let mut depth = 1;
	let mut mov = None;
	let mut best_moves = Vec::new();
	while start.elapsed() < *time {
		mov = Some(root_alphabeta(brd, start + *time, depth));
		best_moves.push(mov);
		depth = depth + 1;
	}
	best_moves.pop();
	mov = best_moves.pop().unwrap();
	depth = depth - 1;
    let end = start.elapsed().as_millis() as u64;
    let nodes = NODE_CTR.load(Ordering::Relaxed);
    let nps = nodes*1000/end;
	let mut score = SCORE.load(Ordering::Relaxed);
	if -10000 < score && score < 10000 {
		println!("info depth {depth} nodes {nodes} nps {nps} time {end} score cp {score}");
	} else if score >= 10000 {
		score = ((POS_INFINITY-score)+1)/2;
	    println!("info depth {depth} nodes {nodes} nps {nps} time {end} score mate {score}");
	} else {
		score = (POS_INFINITY+score)/2;
		println!("info depth {depth} nodes {nodes} nps {nps} time {end} score mate {score}");
	}
    mov.unwrap()
    
}

fn check_castling_move(board: &Board, mut mv: Move) -> Move {
    if board.piece_on(mv.from) == Some(Piece::King) {
        mv.to = match (mv.from, mv.to) {
            (Square::E1, Square::G1) => Square::H1,
            (Square::E8, Square::G8) => Square::H8,
            (Square::E1, Square::C1) => Square::A1,
            (Square::E8, Square::C8) => Square::A8,
            _ => mv.to,
        };
    }
    mv
}

fn main() {
	//assert_eq!(0, evaluate(&Board::default()));
    let mut board = Board::default();
    loop {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        let messages: MessageList = parse(cmd.as_str());
        for m in messages {
            match m {
                UciMessage::Uci => {
                    println!("{}", UciMessage::Id{name: Some(String::from("tamarac")),author: None,});
                    println!("{}", UciMessage::Id{name: None,author: Some(String::from("Leo Fisher")),});
                    println!("{}", UciMessage::UciOk{});
                },
                UciMessage::IsReady => println!("{}", UciMessage::ReadyOk{}),
                UciMessage::Position { startpos, fen, moves } => {
                    if startpos {
                        board = Board::default();
                    } else {
                        board = fen.unwrap().as_str().parse().unwrap();
                    }
                    for mov in moves {
                        let mut mv = mov.to_string().parse().unwrap();
                        mv = check_castling_move(&board, mv);
                        board.play(mv);
                    }
                },
                UciMessage::Quit => panic!(),
                UciMessage::Go { time_control, search_control: _ } => { //for some reason you need to put in a time control or it doesn't work. use go infinite for testing
					let mut time = Duration::ZERO;	
					match time_control {
						Some(UciTimeControl::MoveTime(movetime)) => {
							time = movetime.to_std().unwrap();
						},
						Some(UciTimeControl::TimeLeft{white_time, black_time, white_increment: _, black_increment: _, moves_to_go: _}) => {
							if white_time.is_some() && black_time.is_some() {
								let mut remaining_ms = None;
								if board.side_to_move() == Color::Black {
									remaining_ms = Some(black_time.unwrap().to_std().unwrap());
								} else {
									remaining_ms = Some(white_time.unwrap().to_std().unwrap());
								}
								//time = Duration::from_millis(std::cmp::min(5000, remaining_ms.unwrap().as_millis() as i32 / VALUES.2).try_into().unwrap());
								time = Duration::from_millis((remaining_ms.unwrap().as_millis() as i32 / VALUES.2).try_into().unwrap());
							}
						},
						_ => time = Duration::new(5,0), 
					}
                    let bestmove = search(&board,&time).to_string();
                    println!("bestmove {bestmove}");
                },
                _ => (),
            }
        }
    }
}
