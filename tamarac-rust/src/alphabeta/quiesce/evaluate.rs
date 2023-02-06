use cozy_chess::{Move, Board, Square, Piece, Color};
use crate::VALUES;

pub fn evaluate(brd: &Board) -> i32 {
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