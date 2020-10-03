use super::{piece, position, sliding_attacks, sliding_moves};
use crate::{board, color};

#[derive(Debug)]
pub struct Rook {
    pub color: color::Color,
    pub position: position::Position,
}

impl piece::Piece for Rook {
    fn color(&self) -> &color::Color {
        &self.color
    }

    fn position(&self) -> &position::Position {
        &self.position
    }

    fn mut_position(&mut self) -> &mut position::Position {
        &mut self.position
    }

    fn piece(&self) -> piece::PieceEnum {
        piece::PieceEnum::ROOK
    }

    fn attacks(
        &self,
        board: &board::Board,
        enemy_king_pos: position::Position,
    ) -> Vec<position::Position> {
        sliding_attacks::straight_attacks(self.position, board, enemy_king_pos)
    }

    fn moves_ignoring_pins(
        &self,
        board: &board::Board,
        _en_passant: &Option<position::Position>,
    ) -> Vec<position::Position> {
        sliding_moves::straight_sliding(self, board)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{bishop, knight, pawn};
    use super::*;
    use std::iter;

    #[test]
    fn rook_bottom_left_empty_board() {
        let mut empty_board = board::Board::empty();

        let position = position::Position(1, 1);

        let rook = Rook {
            color: color::Color::WHITE,
            position,
        };

        empty_board.set_square(Some(Box::new(rook)), position);

        let rook = match empty_board.get_square(position) {
            None => panic!(),
            Some(b) => b,
        };

        let attacked_positions = rook.attacks(&empty_board, position::Position(0, 0));

        let mut count_file = 1;
        let expected_attacked_positions = iter::from_fn(|| {
            count_file += 1;
            if count_file < 9 {
                Some(position::Position(count_file, rook.position().1))
            } else {
                None
            }
        });
        let mut count_rank = 1;
        let expected_attacked_positions = expected_attacked_positions.chain(iter::from_fn(|| {
            count_rank += 1;
            if count_rank < 9 {
                Some(position::Position(rook.position().0, count_rank))
            } else {
                None
            }
        }));

        assert_eq!(
            expected_attacked_positions.collect::<Vec<_>>(),
            attacked_positions
        );
    }

    #[test]
    fn rook_middle_with_obstacles() {
        let mut empty_board = board::Board::empty();

        let position = position::Position(5, 4);
        let rook = Rook {
            color: color::Color::WHITE,
            position,
        };
        empty_board.set_square(Some(Box::new(rook)), position);

        let pawn_position = position::Position(5, 5);
        let pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: pawn_position,
        };
        empty_board.set_square(Some(Box::new(pawn)), pawn_position);

        let rook_position = position::Position(8, 4);
        let other_rook = Rook {
            color: color::Color::BLACK,
            position: rook_position,
        };
        empty_board.set_square(Some(Box::new(other_rook)), rook_position);

        let bishop_position = position::Position(5, 2);
        let bishop = bishop::Bishop {
            color: color::Color::BLACK,
            position: bishop_position,
        };
        empty_board.set_square(Some(Box::new(bishop)), bishop_position);

        let knight_position = position::Position(3, 4);
        let knight = knight::Knight {
            color: color::Color::WHITE,
            position: knight_position,
        };
        empty_board.set_square(Some(Box::new(knight)), knight_position);

        let rook = match empty_board.get_square(position) {
            None => panic!(),
            Some(b) => b,
        };

        let attacked_positions = rook.attacks(&empty_board, position::Position(0, 0));
        let mut expected_attacked_positions = vec![];
        // horizontal right
        expected_attacked_positions.push(position::Position(6, 4));
        expected_attacked_positions.push(position::Position(7, 4));
        expected_attacked_positions.push(position::Position(8, 4));
        // horizontal left
        expected_attacked_positions.push(position::Position(4, 4));
        expected_attacked_positions.push(position::Position(3, 4));
        // vertical top
        expected_attacked_positions.push(position::Position(5, 5));
        // vertical bottom
        expected_attacked_positions.push(position::Position(5, 3));
        expected_attacked_positions.push(position::Position(5, 2));

        assert_eq!(expected_attacked_positions, attacked_positions);
    }

    // TODO: tests for rook moves
}
