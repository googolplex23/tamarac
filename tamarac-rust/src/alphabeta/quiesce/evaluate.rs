use cozy_chess::{Move, Board, Square, Piece, Color};
use crate::VALUES;

fn amt_moves(brd: &Board) -> i32 {
    let mut total_moves = 0;
    brd.generate_moves(|moves| {
        // Unpack dense move set into move list
        total_moves += moves.len();
        false
    });
    return total_moves.try_into().unwrap();
}

pub fn evaluate(brd: &Board) -> i32 {
	let mut all_total: i32 = 0;
	for clr in Color::ALL{
		let mut total: i32 = 0;
		for x in 0..5 {
			let mut pieces: i32 = brd.colored_pieces(clr, *Piece::ALL.get(x).unwrap()).len().try_into().unwrap();
            match x {
                0 => {
                    if pieces == 0 {
                        total = total - VALUES.3;
                    }
                },
                2 => {
                    if pieces == 2 {
                        total = total + VALUES.1;
                    }
                },
                _ => (),
            }
            pieces = pieces * VALUES.0[x];
			total = total + pieces;
		}
        
        //tot5 * (list_moves(brd).len() as i32));
		if clr == Color::Black {
			total = 0 - total;
		}
		all_total = all_total + total
        
	}
    //all_total =  all_total + 5 * amt_moves(brd) as i32;
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