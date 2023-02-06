use cozy_chess::{Move, Board, Square, Piece, Color};

pub fn list_moves(brd: &Board) -> Vec<Move> {
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

pub fn list_captures(brd: &Board) -> Vec<Move> {
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