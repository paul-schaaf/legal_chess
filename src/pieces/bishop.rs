use super::{piece, position, sliding_attacks, sliding_moves};
use crate::{board, chessmove, color};

#[derive(Debug)]
pub struct Bishop {
    pub position: position::Position,
    pub color: color::Color,
}

impl piece::Piece for Bishop {
    fn color(&self) -> &color::Color {
        &self.color
    }

    fn position(&self) -> &position::Position {
        &self.position
    }

    fn mut_position(&mut self) -> &mut position::Position {
        &mut self.position
    }

    fn attacks(
        &self,
        board: &board::Board,
        enemy_king_pos: position::Position,
    ) -> Vec<position::Position> {
        sliding_attacks::diagonal_attacks(self.position, board, enemy_king_pos)
    }

    fn piece(&self) -> piece::PieceEnum {
        piece::PieceEnum::BISHOP
    }

    fn moves_ignoring_pins(
        &self,
        board: &board::Board,
        _en_passant: &Option<position::Position>,
    ) -> Vec<chessmove::ChessMove> {
        sliding_moves::diagonal_sliding(self, board)
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
            color: color::Color::WHITE,
            position,
        };

        empty_board.set_square(Some(Box::new(bishop)), position);

        let bishop = match empty_board.get_square(position) {
            None => panic!(),
            Some(b) => b,
        };

        let attacked_positions = bishop.attacks(&empty_board, position::Position(0, 0));

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
    fn bishop_bottom_left_middle_obstacle_attacks() {
        let mut empty_board = board::Board::empty();

        let bishop_position = position::Position(1, 1);

        let bishop = Bishop {
            color: color::Color::WHITE,
            position: bishop_position,
        };

        let pawn_position = position::Position(5, 5);

        let pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: pawn_position,
        };

        empty_board.set_square(Some(Box::new(bishop)), bishop_position);
        empty_board.set_square(Some(Box::new(pawn)), pawn_position);

        let bishop = match empty_board.get_square(bishop_position) {
            None => panic!(),
            Some(b) => b,
        };

        let attacked_positions = bishop.attacks(&empty_board, position::Position(0, 0));

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
    fn bishop_top_middle_with_obstacles_attacks() {
        let mut empty_board = board::Board::empty();

        let bishop_position = position::Position(4, 8);

        let bishop = Bishop {
            color: color::Color::WHITE,
            position: bishop_position,
        };

        let knight_position = position::Position(2, 6);

        let knight = knight::Knight {
            color: color::Color::WHITE,
            position: knight_position,
        };

        let pawn_position = position::Position(7, 5);

        let pawn = pawn::Pawn {
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

        let attacked_positions = bishop.attacks(&empty_board, position::Position(0, 0));
        let mut expected_attacked_positions = vec![];
        expected_attacked_positions.push(position::Position(5, 7));
        expected_attacked_positions.push(position::Position(6, 6));
        expected_attacked_positions.push(position::Position(7, 5));
        expected_attacked_positions.push(position::Position(3, 7));
        expected_attacked_positions.push(position::Position(2, 6));

        assert_eq!(expected_attacked_positions, attacked_positions);
    }

    #[test]
    fn bishop_bottom_left_moves() {
        let mut empty_board = board::Board::empty();

        let position = position::Position(1, 1);

        let bishop = Bishop {
            color: color::Color::WHITE,
            position,
        };

        empty_board.set_square(Some(Box::new(bishop)), position);

        let bishop = match empty_board.get_square(position) {
            None => panic!(),
            Some(b) => b,
        };

        let actual_moves = bishop.moves_ignoring_pins(&empty_board, &None);

        let mut count = 1;
        let expected_moves = iter::from_fn(|| {
            count += 1;
            if count < 9 {
                Some(chessmove::ChessMove {
                    from: (bishop.position().0, bishop.position().1),
                    to: (count, count),
                    promotion: None,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

        assert_eq!(expected_moves, actual_moves);
    }

    #[test]
    fn bishop_middle_moves() {
        // TODO
    }

    #[test]
    fn bishop_middle_obstacle_moves() {
        let (board, white_king, _) = board::Board::from_string_board(&[
            "R", "P", "-", "-", "-", "b", "p", "r", "-", "P", "-", "p", "-", "n", "-", "-", "-",
            "P", "N", "-", "-", "-", "p", "-", "-", "B", "-", "-", "P", "-", "p", "-", "K", "B",
            "-", "P", "N", "p", "q", "k", "-", "P", "Q", "-", "-", "n", "p", "-", "-", "P", "-",
            "-", "-", "p", "b", "-", "R", "P", "p", "-", "-", "-", "-", "r",
        ]);

        let bishop = match board.get_square(position::Position(1, 6)) {
            None => panic!(),
            Some(b) => b,
        };

        let moves = bishop.moves(&board, white_king, &None);

        assert_eq!(6, moves.len());
    }
}
