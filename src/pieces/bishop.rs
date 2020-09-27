use super::{piece, position};
use crate::{board, color};

#[derive(Debug)]
pub struct Bishop {
    pub id: u8,
    pub position: position::Position,
    pub color: color::Color,
}

impl piece::Piece for Bishop {
    fn get_id(&self) -> u8 {
        self.id
    }

    fn color(&self) -> &color::Color {
        &self.color
    }

    fn position(&self) -> &position::Position {
        &self.position
    }

    fn attacks(&self, board: &board::Board) -> Vec<position::Position> {
        let moves_and_bounds = [
            ((1, 1), (8, 8)),
            ((1, -1), (8, 1)),
            ((-1, 1), (1, 8)),
            ((-1, -1), (1, 1)),
        ];

        let mut attacked_positions = vec![];

        for entry in &moves_and_bounds {
            let mut current_file = self.position.0 as i8;
            let mut current_rank = self.position.1 as i8;
            while current_file != (entry.1).0 && current_rank != (entry.1).1 {
                current_file += (entry.0).0;
                current_rank += (entry.0).1;
                let attacked_position = position::Position(current_file as u8, current_rank as u8);
                attacked_positions.push(attacked_position);
                if let Some(_) = board.get_square(attacked_position) {
                    break;
                }
            }
        }

        attacked_positions
    }
}

#[cfg(test)]
mod tests {
    use super::super::{knight, pawn};
    use super::*;
    use std::iter;

    #[test]
    fn bishop_bottom_left_empty_board() {
        let mut empty_board = board::Board::empty();

        let position = position::Position(1, 1);

        let bishop = Bishop {
            id: 1,
            color: color::Color::WHITE,
            position,
        };

        empty_board.set_square(Some(Box::new(bishop)), position);

        let bishop = match empty_board.get_square(position) {
            None => panic!(),
            Some(b) => b,
        };

        let attacked_positions = bishop.attacks(&empty_board);

        let mut count = 1;
        let expected_attacked_positions = iter::from_fn(|| {
            count += 1;
            if count < 9 {
                Some(position::Position(count, count))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
        assert_eq!(expected_attacked_positions, attacked_positions);
    }

    #[test]
    fn bishop_bottom_left_middle_obstacle() {
        let mut empty_board = board::Board::empty();

        let bishop_position = position::Position(1, 1);

        let bishop = Bishop {
            id: 1,
            color: color::Color::WHITE,
            position: bishop_position,
        };

        let pawn_position = position::Position(5, 5);

        let pawn = pawn::Pawn {
            id: 1,
            color: color::Color::WHITE,
            position: pawn_position,
        };

        empty_board.set_square(Some(Box::new(bishop)), bishop_position);
        empty_board.set_square(Some(Box::new(pawn)), pawn_position);

        let bishop = match empty_board.get_square(bishop_position) {
            None => panic!(),
            Some(b) => b,
        };

        let attacked_positions = bishop.attacks(&empty_board);

        let mut count = 1;
        let expected_attacked_positions = iter::from_fn(|| {
            count += 1;
            if count < 6 {
                Some(position::Position(count, count))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
        assert_eq!(expected_attacked_positions, attacked_positions);
    }

    #[test]
    fn bishop_top_middle_with_obstacles() {
        let mut empty_board = board::Board::empty();

        let bishop_position = position::Position(4, 8);

        let bishop = Bishop {
            id: 1,
            color: color::Color::WHITE,
            position: bishop_position,
        };

        let knight_position = position::Position(2, 6);

        let knight = knight::Knight {
            id: 2,
            color: color::Color::WHITE,
            position: knight_position,
        };

        let pawn_position = position::Position(7, 5);

        let pawn = pawn::Pawn {
            id: 3,
            color: color::Color::BLACK,
            position: pawn_position,
        };

        empty_board.set_square(Some(Box::new(bishop)), bishop_position);
        empty_board.set_square(Some(Box::new(knight)), knight_position);
        empty_board.set_square(Some(Box::new(pawn)), pawn_position);

        let bishop = match empty_board.get_square(bishop_position) {
            None => panic!(),
            Some(b) => b,
        };

        let attacked_positions = bishop.attacks(&empty_board);
        let mut expected_attacked_positions = vec![];
        expected_attacked_positions.push(position::Position(5, 7));
        expected_attacked_positions.push(position::Position(6, 6));
        expected_attacked_positions.push(position::Position(7, 5));
        expected_attacked_positions.push(position::Position(3, 7));
        expected_attacked_positions.push(position::Position(2, 6));

        assert_eq!(expected_attacked_positions, attacked_positions);
    }
}