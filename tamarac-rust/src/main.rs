//castling move check code by crippa#9590

use cozy_chess::{Move, Board, Square, Piece};
use std::io;
use vampirc_uci::parse;
use vampirc_uci::{UciMessage, MessageList, UciTimeControl, Serializable};

fn search(brd: &Board) -> Move {
    let mut move_list = Vec::new();
    brd.generate_moves(|moves| {
        // Unpack dense move set into move list
        move_list.extend(moves);
        false
    });
    *move_list.get(0).unwrap()
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
