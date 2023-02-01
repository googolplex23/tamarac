//castling move check code by crippa#9590

use cozy_chess::{Move, Board, Square, Piece, Color};
use std::io;
use vampirc_uci::parse;
use vampirc_uci::{UciMessage, MessageList, UciTimeControl, Serializable};

const VALUES: [[i32; 6]; 1] = [[100, 300, 300, 500, 900,10000]]; //king value must be greater than 9x queen + 2x rest of pieces
const POS_INFINITY: i32 = i32::MAX - 256;
const NEG_INFINITY: i32 = i32::MIN + 256;

fn list_moves(brd: &Board) -> Vec<Move> {
    let mut move_list = Vec::new();
	brd.generate_moves(|moves| {
        // Unpack dense move set into move list
        move_list.extend(moves);
        false
    });
    move_list
}

fn evaluate(brd: &Board) -> i32 {
	let mut total: i32 = 0;
	for clr in Color::ALL{
		for x in 0..5 {
			let mut pieces: i32 = brd.colored_pieces(clr, *Piece::ALL.get(x).unwrap()).len().try_into().unwrap();
            pieces = pieces * VALUES[0][x];
			if clr == Color::Black {
				pieces = 0 - pieces;
			}
			total = pieces + total;
		}
		if clr == Color::Black {
			total = 0 - total;
		}
        
	}
	if brd.side_to_move() == Color::Black {
		total = 0 - total;
	}
    //println!("{}", total);
	total
}

fn alphabeta(brd: &Board, depth: u32, mut alpha: i32, beta: i32) -> i32 {
	if depth == 0 {
		return evaluate(brd);
	}
	let move_list = list_moves(brd);
	let mut bestscore = NEG_INFINITY;
	for mov in move_list {
		let mut new_brd = brd.clone();
		new_brd.play(mov);
		let score = alphabeta(&new_brd, depth - 1, 0 - beta, 0 - alpha);
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

fn root_alphabeta(brd: &Board, depth: u32,) -> Move {
	let mut move_list = Vec::new();
	brd.generate_moves(|moves| {
        // Unpack dense move set into move list
        move_list.extend(moves);
        false
    });
	let mut bestscore = NEG_INFINITY;
	let mut bestmove = None;
	let mut alpha = NEG_INFINITY;
	for mov in move_list {
		let mut new_brd = brd.clone();
		new_brd.play(mov);
		let score = alphabeta(&new_brd, depth - 1, NEG_INFINITY, 0 - alpha);
		if score > bestscore {
			bestmove = Some(mov);
			bestscore = score;
			if score > alpha {
				alpha = score;
			}
		}
	}
	bestmove.unwrap()
}

fn search(brd: &Board) -> Move {
    root_alphabeta(brd, 7)
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
	assert_eq!(Piece::ALL.get(0).unwrap(), &Piece::Pawn);
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
                UciMessage::Go { time_control, search_control } => { //for some reason you need to put in a time control or it doesn't work. use go infinite for testing
                    let bestmove = search(&board).to_string();
                    println!("bestmove {bestmove}");
                },
                _ => (),
            }
        }
    }
}
