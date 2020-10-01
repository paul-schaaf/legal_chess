use super::position;
use crate::board;

pub fn straight_attacks(
    position: position::Position,
    board: &board::Board,
    enemy_king_pos: position::Position,
) -> Vec<position::Position> {
    let moves_and_bounds = [((1, 0), 8), ((-1, 0), 1), ((0, 1), 8), ((0, -1), 1)];

    let mut attacked_positions = vec![];

    for entry in &moves_and_bounds {
        let mut current_file = position.0 as i8;
        let mut current_rank = position.1 as i8;
        if (entry.0).1 == 0 {
            while current_file != (entry.1) {
                current_file += (entry.0).0;
                let attacked_position = position::Position(current_file as u8, current_rank as u8);
                attacked_positions.push(attacked_position);
                if board.get_square(attacked_position).is_some()
                    && attacked_position != enemy_king_pos
                {
                    break;
                }
            }
        } else {
            while current_rank != (entry.1) {
                current_rank += (entry.0).1;
                let attacked_position = position::Position(current_file as u8, current_rank as u8);
                attacked_positions.push(attacked_position);
                if board.get_square(attacked_position).is_some()
                    && attacked_position != enemy_king_pos
                {
                    break;
                }
            }
        }
    }

    attacked_positions
}

pub fn diagonal_attacks(
    position: position::Position,
    board: &board::Board,
    enemy_king_pos: position::Position,
) -> Vec<position::Position> {
    let moves_and_bounds = [
        ((1, 1), (8, 8)),
        ((1, -1), (8, 1)),
        ((-1, 1), (1, 8)),
        ((-1, -1), (1, 1)),
    ];

    let mut attacked_positions = vec![];

    for entry in &moves_and_bounds {
        let mut current_file = position.0 as i8;
        let mut current_rank = position.1 as i8;
        while current_file != (entry.1).0 && current_rank != (entry.1).1 {
            current_file += (entry.0).0;
            current_rank += (entry.0).1;
            let attacked_position = position::Position(current_file as u8, current_rank as u8);
            attacked_positions.push(attacked_position);
            if board.get_square(attacked_position).is_some() && attacked_position != enemy_king_pos
            {
                break;
            }
        }
    }

    attacked_positions
}
