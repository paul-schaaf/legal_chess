use super::position;
use crate::board;
use crate::pieces::piece;

#[derive(PartialEq)]
enum Axis {
    Horizontal,
    Vertical,
}

pub fn straight_sliding(piece: &dyn piece::Piece, board: &board::Board) -> Vec<position::Position> {
    let moves_and_bounds = [
        (Axis::Horizontal, 1, 8),
        (Axis::Horizontal, -1, 1),
        (Axis::Vertical, 1, 8),
        (Axis::Vertical, -1, 1),
    ];

    let mut moves = vec![];

    let mut move_piece = |file: u8, rank: u8| -> bool {
        let move_position = position::Position(file, rank);
        if let Some(p) = board.get_square(move_position) {
            if *p.color() != *piece.color() {
                moves.push(move_position);
            }
            return true;
        } else {
            moves.push(move_position);
        }
        return false;
    };

    for entry in &moves_and_bounds {
        let mut current_file = piece.position().0 as i8;
        let mut current_rank = piece.position().1 as i8;
        if entry.0 == Axis::Horizontal {
            while current_file != entry.2 {
                current_file += entry.1;
                if move_piece(current_file as u8, current_rank as u8) {
                    break;
                }
            }
        } else {
            while current_rank != entry.2 {
                current_rank += entry.1;
                if move_piece(current_file as u8, current_rank as u8) {
                    break;
                }
            }
        }
    }

    moves
}

pub fn diagonal_sliding(piece: &dyn piece::Piece, board: &board::Board) -> Vec<position::Position> {
    let moves_and_bounds = [
        ((1, 1), (8, 8)),
        ((1, -1), (8, 1)),
        ((-1, 1), (1, 8)),
        ((-1, -1), (1, 1)),
    ];

    let mut moves = vec![];

    for entry in &moves_and_bounds {
        let mut current_file = piece.position().0 as i8;
        let mut current_rank = piece.position().1 as i8;
        while current_file != (entry.1).0 && current_rank != (entry.1).1 {
            current_file += (entry.0).0;
            current_rank += (entry.0).1;
            let move_position = position::Position(current_file as u8, current_rank as u8);
            if let Some(p) = board.get_square(move_position) {
                if *p.color() != *piece.color() {
                    moves.push(move_position);
                }
                break;
            }
            moves.push(move_position);
        }
    }

    moves
}
