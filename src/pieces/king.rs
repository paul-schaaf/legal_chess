use super::{piece, piece::Piece, position};
use crate::{board, chessmove, color};

#[derive(Debug)]
pub struct King {
    pub color: color::Color,
    pub position: position::Position,
}

impl piece::Piece for King {
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
        piece::PieceEnum::KING
    }

    fn attacks(
        &self,
        _board: &board::Board,
        _enemy_king_pos: position::Position,
    ) -> Vec<position::Position> {
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

    fn moves_ignoring_pins(
        &self,
        board: &board::Board,
        _en_passant: &Option<position::Position>,
    ) -> Vec<chessmove::ChessMove> {
        let position = self.position;
        let mut chessmoves = vec![];

        if position.0 != 1 {
            self.move_if_empty_or_enemy(
                &mut chessmoves,
                board,
                position::Position(position.0 - 1, position.1),
            );
            if position.1 != 8 {
                self.move_if_empty_or_enemy(
                    &mut chessmoves,
                    board,
                    position::Position(position.0 - 1, position.1 + 1),
                );
            }
            if position.1 != 1 {
                self.move_if_empty_or_enemy(
                    &mut chessmoves,
                    board,
                    position::Position(position.0 - 1, position.1 - 1),
                );
            }
        }

        if position.0 != 8 {
            self.move_if_empty_or_enemy(
                &mut chessmoves,
                board,
                position::Position(position.0 + 1, position.1),
            );
            if position.1 != 8 {
                self.move_if_empty_or_enemy(
                    &mut chessmoves,
                    board,
                    position::Position(position.0 + 1, position.1 + 1),
                );
            }
            if position.1 != 1 {
                self.move_if_empty_or_enemy(
                    &mut chessmoves,
                    board,
                    position::Position(position.0 + 1, position.1 - 1),
                );
            }
        }

        if position.1 != 1 {
            self.move_if_empty_or_enemy(
                &mut chessmoves,
                board,
                position::Position(position.0, position.1 - 1),
            );
        }

        if position.1 != 8 {
            self.move_if_empty_or_enemy(
                &mut chessmoves,
                board,
                position::Position(position.0, position.1 + 1),
            );
        }

        chessmoves
    }
}

impl King {
    fn move_if_empty_or_enemy(
        &self,
        positions: &mut Vec<chessmove::ChessMove>,
        board: &board::Board,
        position: position::Position,
    ) {
        let chessmove = chessmove::ChessMove {
            from: (self.position().0, self.position().1),
            to: (position.0, position.1),
            promotion: None,
        };
        match board.get_square(position) {
            None => positions.push(chessmove),
            Some(p) => {
                if *p.color() != *self.color() {
                    positions.push(chessmove);
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
            color: color::Color::WHITE,
            position: position::Position(1, 1),
        };
        let attacked_positions = king.attacks(&board::Board::empty(), position::Position(0, 0));

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
            color: color::Color::WHITE,
            position: position::Position(5, 4),
        };
        let attacked_positions = king.attacks(&board::Board::empty(), position::Position(0, 0));
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
            color: color::Color::WHITE,
            position: position::Position(1, 1),
        };
        let mut board = board::Board::empty();

        let black_pawn = pawn::Pawn {
            color: color::Color::BLACK,
            position: position::Position(2, 2),
        };

        board.set_square(Some(Box::new(black_pawn)), position::Position(2, 2));

        let possible_moves = king.moves_ignoring_pins(&board, &None);

        let expected = vec![
            chessmove::ChessMove {
                from: (1, 1),
                to: (1, 2),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (1, 1),
                to: (2, 1),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (1, 1),
                to: (2, 2),
                promotion: None,
            },
        ];

        assert_eq!(3, possible_moves.len());
        for position in expected {
            assert!(possible_moves.contains(&position));
        }
    }

    #[test]
    fn king_bottom_left_moves_white_pawn() {
        let king = King {
            color: color::Color::WHITE,
            position: position::Position(1, 1),
        };
        let mut board = board::Board::empty();

        let white_pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: position::Position(2, 2),
        };

        board.set_square(Some(Box::new(white_pawn)), position::Position(2, 2));

        let possible_moves = king.moves_ignoring_pins(&board, &None);

        let expected = vec![
            chessmove::ChessMove {
                from: (1, 1),
                to: (1, 2),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (1, 1),
                to: (2, 1),
                promotion: None,
            },
        ];

        assert_eq!(2, possible_moves.len());
        for position in expected {
            assert!(possible_moves.contains(&position));
        }
    }
}
