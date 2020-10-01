use super::{piece, piece::Piece, position};
use crate::{board, color};

#[derive(Debug)]
pub struct King {
    pub id: u8,
    pub color: color::Color,
    pub position: position::Position,
}

impl piece::Piece for King {
    fn get_id(&self) -> u8 {
        self.id
    }

    fn color(&self) -> &color::Color {
        &self.color
    }

    fn position(&self) -> &position::Position {
        &self.position
    }

    fn piece(&self) -> piece::PieceEnum {
        piece::PieceEnum::KING
    }

    fn attacks(&self, _board: &board::Board) -> Vec<position::Position> {
        let position = self.position;
        let mut attacked_positions = vec![];

        if position.0 != 1 {
            attacked_positions.push(position::Position(position.0 - 1, position.1));
            if position.1 != 8 {
                attacked_positions.push(position::Position(position.0 - 1, position.1 + 1));
            }
            if position.1 != 1 {
                attacked_positions.push(position::Position(position.0 - 1, position.1 - 1));
            }
        }
        if position.0 != 8 {
            attacked_positions.push(position::Position(position.0 + 1, position.1));
            if position.1 != 8 {
                attacked_positions.push(position::Position(position.0 + 1, position.1 + 1));
            }
            if position.1 != 1 {
                attacked_positions.push(position::Position(position.0 + 1, position.1 - 1));
            }
        }

        if position.1 != 1 {
            attacked_positions.push(position::Position(position.0, position.1 - 1));
        }
        if position.1 != 8 {
            attacked_positions.push(position::Position(position.0, position.1 + 1));
        }

        attacked_positions
    }

    fn moves_ignoring_pins(&self, board: &board::Board) -> Vec<position::Position> {
        let position = self.position;
        let mut positions_to_move_to = vec![];

        if position.0 != 1 {
            self.move_if_empty_or_enemy(
                &mut positions_to_move_to,
                board,
                position::Position(position.0 - 1, position.1),
            );
            if position.1 != 8 {
                self.move_if_empty_or_enemy(
                    &mut positions_to_move_to,
                    board,
                    position::Position(position.0 - 1, position.1 + 1),
                );
            }
            if position.1 != 1 {
                self.move_if_empty_or_enemy(
                    &mut positions_to_move_to,
                    board,
                    position::Position(position.0 - 1, position.1 - 1),
                );
            }
        }

        if position.0 != 8 {
            self.move_if_empty_or_enemy(
                &mut positions_to_move_to,
                board,
                position::Position(position.0 + 1, position.1),
            );
            if position.1 != 8 {
                self.move_if_empty_or_enemy(
                    &mut positions_to_move_to,
                    board,
                    position::Position(position.0 + 1, position.1 + 1),
                );
            }
            if position.1 != 1 {
                self.move_if_empty_or_enemy(
                    &mut positions_to_move_to,
                    board,
                    position::Position(position.0 + 1, position.1 - 1),
                );
            }
        }

        if position.1 != 1 {
            self.move_if_empty_or_enemy(
                &mut positions_to_move_to,
                board,
                position::Position(position.0, position.1 - 1),
            );
        }

        if position.1 != 8 {
            self.move_if_empty_or_enemy(
                &mut positions_to_move_to,
                board,
                position::Position(position.0, position.1 + 1),
            );
        }

        positions_to_move_to
    }
}

impl King {
    fn move_if_empty_or_enemy(
        &self,
        positions: &mut Vec<position::Position>,
        board: &board::Board,
        position: position::Position,
    ) {
        match board.get_square(position) {
            None => positions.push(position),
            Some(p) => {
                if *p.color() != *self.color() {
                    positions.push(position);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::pawn;
    use super::piece::Piece;
    use super::*;

    #[test]
    fn king_bottom_left_attacks() {
        let king = King {
            id: 1,
            color: color::Color::WHITE,
            position: position::Position(1, 1),
        };
        let attacked_positions = king.attacks(&board::Board::empty());

        let expected = vec![
            position::Position(1, 2),
            position::Position(2, 1),
            position::Position(2, 2),
        ];

        assert_eq!(3, attacked_positions.len());
        for position in expected {
            assert!(attacked_positions.contains(&position));
        }
    }

    #[test]
    fn king_middle_attacks() {
        let king = King {
            id: 1,
            color: color::Color::WHITE,
            position: position::Position(5, 4),
        };
        let attacked_positions = king.attacks(&board::Board::empty());
        let expected = vec![
            position::Position(5, 5),
            position::Position(6, 5),
            position::Position(6, 4),
            position::Position(6, 3),
            position::Position(5, 3),
            position::Position(4, 3),
            position::Position(4, 4),
            position::Position(4, 5),
        ];

        assert_eq!(8, attacked_positions.len());
        for position in expected {
            assert!(attacked_positions.contains(&position));
        }
    }

    #[test]
    fn king_bottom_left_moves_black_pawn() {
        let king = King {
            id: 1,
            color: color::Color::WHITE,
            position: position::Position(1, 1),
        };
        let mut board = board::Board::empty();

        let black_pawn = pawn::Pawn {
            id: 2,
            color: color::Color::BLACK,
            position: position::Position(2, 2),
        };

        board.set_square(Some(Box::new(black_pawn)), position::Position(2, 2));

        let possible_moves = king.moves_ignoring_pins(&board);

        let expected = vec![
            position::Position(1, 2),
            position::Position(2, 1),
            position::Position(2, 2),
        ];

        assert_eq!(3, possible_moves.len());
        for position in expected {
            assert!(possible_moves.contains(&position));
        }
    }

    #[test]
    fn king_bottom_left_moves_white_pawn() {
        let king = King {
            id: 1,
            color: color::Color::WHITE,
            position: position::Position(1, 1),
        };
        let mut board = board::Board::empty();

        let white_pawn = pawn::Pawn {
            id: 2,
            color: color::Color::WHITE,
            position: position::Position(2, 2),
        };

        board.set_square(Some(Box::new(white_pawn)), position::Position(2, 2));

        let possible_moves = king.moves_ignoring_pins(&board);

        let expected = vec![position::Position(1, 2), position::Position(2, 1)];

        assert_eq!(2, possible_moves.len());
        for position in expected {
            assert!(possible_moves.contains(&position));
        }
    }
}
