//castling move check code by crippa#9590

use cozy_chess::{Move, Board, Square, Piece, Color};
use std::io;
use std::sync::atomic::{AtomicU64, AtomicI32, Ordering};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use vampirc_uci::parse;
use vampirc_uci::{UciMessage, MessageList, UciTimeControl};

mod alphabeta;

const VALUES: ([i32; 6], i32, i32) = ([100, 300, 300, 500, 900,10000],50,40); //king value must be greater than 9x queen + 2x rest of pieces
const POS_INFINITY: i32 = 10240;
const NEG_INFINITY: i32 = -10240;

static NODE_CTR: AtomicU64 = AtomicU64::new(0);
//static SCORE: AtomicI32 = AtomicI32::new(0);

/*
struct PVtable(Vec<Entry>);

struct Entry {
	hash = 
}
*/






fn search(brd: &Board, time: &Duration) -> Move {
    NODE_CTR.store(0, Ordering::Relaxed);
    let start = Instant::now();
	let mut depth = 1;
	let mut mov = None;
	let mut best_moves = Vec::new();
    let mut transtable = HashMap::new();
    let mut score_list = Vec::new();
	while start.elapsed() < *time {
        let result = alphabeta::root_alphabeta(brd, transtable, start + *time, depth);
		transtable = result.1;
        mov = Some(result.0);
		best_moves.push(mov);
		depth = depth + 1;
        score_list.push(result.2);
	}
	best_moves.pop();
	mov = best_moves.pop().unwrap();
    score_list.pop();
    let mut score = score_list.pop().unwrap();
	depth = depth - 1;
    let end = start.elapsed().as_millis() as u64;
    let nodes = NODE_CTR.load(Ordering::Relaxed);
    let nps = nodes*1000/end;
	//let mut score = SCORE.load(Ordering::Relaxed);
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
