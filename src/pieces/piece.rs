use super::position;
use crate::{board, color};
use std::fmt;

enum RelativeKingPosition {
    Up,
    UpRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    UpLeft,
}

#[derive(PartialEq, Debug)]
enum Direction {
    STRAIGHT,
    DIAGONAL,
}

pub trait Piece: fmt::Debug {
    fn position(&self) -> &position::Position;

    fn attacks(
        &self,
        board: &board::Board,
        enemy_king_pos: position::Position,
    ) -> Vec<position::Position>;

    fn color(&self) -> &color::Color;

    fn get_id(&self) -> u8;

    fn piece(&self) -> PieceEnum;

    fn moves_ignoring_pins(
        &self,
        board: &board::Board,
        en_passant: &Option<position::Position>,
    ) -> Vec<position::Position>;

    fn moves(
        &self,
        board: &board::Board,
        king: position::Position,
        en_passant: &Option<position::Position>,
    ) -> Vec<position::Position> {
        let moves_ignoring_pins = self.moves_ignoring_pins(board, en_passant);
        match self.valid_moves_during_pin(board, king) {
            None => moves_ignoring_pins,
            Some(valid_moves_during_pin) => moves_ignoring_pins
                .into_iter()
                .filter(|mv| valid_moves_during_pin.contains(mv))
                .collect::<Vec<_>>(),
        }
    }

    fn valid_moves_during_pin(
        &self,
        board: &board::Board,
        king: position::Position,
    ) -> Option<Vec<position::Position>> {
        if *self.position() == king {
            return None;
        }

        let (mover, direction) = match get_relative_king_position(self.position(), king) {
            None => return None,
            Some(RelativeKingPosition::Bottom) => ((0, -1), Direction::STRAIGHT),
            Some(RelativeKingPosition::Up) => ((0, 1), Direction::STRAIGHT),
            Some(RelativeKingPosition::Left) => ((-1, 0), Direction::STRAIGHT),
            Some(RelativeKingPosition::Right) => ((1, 0), Direction::STRAIGHT),
            Some(RelativeKingPosition::UpRight) => ((1, 1), Direction::DIAGONAL),
            Some(RelativeKingPosition::BottomRight) => ((1, -1), Direction::DIAGONAL),
            Some(RelativeKingPosition::BottomLeft) => ((-1, -1), Direction::DIAGONAL),
            Some(RelativeKingPosition::UpLeft) => ((-1, 1), Direction::DIAGONAL),
        };

        let mut moves = vec![];

        let mut new_file = self.position().0 as i8;
        let mut new_rank = self.position().1 as i8;

        loop {
            new_file += mover.0;
            new_rank += mover.1;

            let new_position = position::Position(new_file as u8, new_rank as u8);

            match board.get_square(new_position) {
                None => moves.push(new_position),
                Some(piece) => {
                    if piece.piece() != PieceEnum::KING {
                        return None;
                    } else if *piece.color() == *self.color() {
                        break;
                    } else {
                        return None;
                    }
                }
            }
        }

        let mut new_file = self.position().0 as i8;
        let mut new_rank = self.position().1 as i8;

        loop {
            new_file += -mover.0;
            new_rank += -mover.1;

            if new_file < 1 || new_file > 8 || new_rank < 1 || new_rank > 8 {
                return None;
            }

            let new_position = position::Position(new_file as u8, new_rank as u8);

            match board.get_square(new_position) {
                None => moves.push(new_position),
                Some(piece) => {
                    if *piece.color() == *self.color() {
                        return None;
                    } else if piece.piece() == PieceEnum::QUEEN
                        || (piece.piece() == PieceEnum::ROOK && direction == Direction::STRAIGHT)
                        || (piece.piece() == PieceEnum::BISHOP && direction == Direction::DIAGONAL)
                    {
                        moves.push(new_position);
                        break;
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(moves)
    }
}

fn get_relative_king_position(
    self_position: &position::Position,
    king: position::Position,
) -> Option<RelativeKingPosition> {
    if self_position.0 == king.0 {
        if self_position.1 > king.1 {
            Some(RelativeKingPosition::Bottom)
        } else {
            Some(RelativeKingPosition::Up)
        }
    } else if self_position.1 == king.1 {
        if self_position.0 > king.0 {
            Some(RelativeKingPosition::Left)
        } else {
            Some(RelativeKingPosition::Right)
        }
    } else {
        let diff_file = self_position.0 as i8 - king.0 as i8;
        let diff_rank = self_position.1 as i8 - king.1 as i8;
        if diff_file.abs() == diff_rank.abs() {
            if self_position.0 < king.0 {
                if self_position.1 < king.1 {
                    Some(RelativeKingPosition::UpRight)
                } else {
                    Some(RelativeKingPosition::BottomRight)
                }
            } else if self_position.1 < king.1 {
                Some(RelativeKingPosition::UpLeft)
            } else {
                Some(RelativeKingPosition::BottomLeft)
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PieceEnum {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
}

#[cfg(test)]
mod tests {
    use super::super::{bishop, king, knight, pawn, queen};
    use super::*;

    #[test]
    fn pinned_knight() {
        let knight = knight::Knight {
            id: 1,
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            id: 2,
            position: position::Position(4, 1),
            color: color::Color::WHITE,
        };

        let black_queen = queen::Queen {
            id: 3,
            position: position::Position(4, 8),
            color: color::Color::BLACK,
        };

        let mut board = board::Board::empty();

        board.set_square(Some(Box::new(king)), position::Position(4, 1));
        board.set_square(Some(Box::new(knight)), position::Position(4, 4));
        board.set_square(Some(Box::new(black_queen)), position::Position(4, 8));

        if let Some(knight) = board.get_square(position::Position(4, 4)) {
            if let Some(actual) = knight.valid_moves_during_pin(&board, position::Position(4, 1)) {
                let expected = vec![
                    position::Position(4, 2),
                    position::Position(4, 3),
                    position::Position(4, 5),
                    position::Position(4, 6),
                    position::Position(4, 7),
                    position::Position(4, 8),
                ];

                assert_eq!(expected.len(), actual.len());

                for square in expected {
                    assert!(actual.contains(&square));
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn not_pinned_knight_1() {
        let knight = knight::Knight {
            id: 1,
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            id: 2,
            position: position::Position(4, 1),
            color: color::Color::WHITE,
        };

        let black_knight = knight::Knight {
            id: 3,
            position: position::Position(4, 8),
            color: color::Color::BLACK,
        };

        let mut board = board::Board::empty();

        board.set_square(Some(Box::new(king)), position::Position(4, 1));
        board.set_square(Some(Box::new(knight)), position::Position(4, 4));
        board.set_square(Some(Box::new(black_knight)), position::Position(4, 8));

        if let Some(knight) = board.get_square(position::Position(4, 4)) {
            assert!(knight
                .valid_moves_during_pin(&board, position::Position(4, 1))
                .is_none());
        } else {
            panic!();
        }
    }

    #[test]
    fn not_pinned_bishop_1() {
        let knight = knight::Knight {
            id: 1,
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            id: 2,
            position: position::Position(4, 1),
            color: color::Color::WHITE,
        };

        let black_queen = queen::Queen {
            id: 3,
            position: position::Position(4, 8),
            color: color::Color::BLACK,
        };

        let black_pawn = pawn::Pawn {
            id: 4,
            position: position::Position(4, 7),
            color: color::Color::BLACK,
        };

        let mut board = board::Board::empty();

        board.set_square(Some(Box::new(king)), position::Position(4, 1));
        board.set_square(Some(Box::new(knight)), position::Position(4, 4));
        board.set_square(Some(Box::new(black_queen)), position::Position(4, 8));
        board.set_square(Some(Box::new(black_pawn)), position::Position(4, 7));

        if let Some(knight) = board.get_square(position::Position(4, 4)) {
            assert!(knight
                .valid_moves_during_pin(&board, position::Position(4, 1))
                .is_none());
        } else {
            panic!();
        }
    }

    #[test]
    fn not_pinned_knight_2() {
        let knight = knight::Knight {
            id: 1,
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king_pos = position::Position(5, 1);
        let king = king::King {
            id: 2,
            position: king_pos,
            color: color::Color::WHITE,
        };

        let black_queen = queen::Queen {
            id: 3,
            position: position::Position(4, 8),
            color: color::Color::BLACK,
        };

        let mut board = board::Board::empty();

        board.set_square(Some(Box::new(king)), king_pos);
        board.set_square(Some(Box::new(knight)), position::Position(4, 4));
        board.set_square(Some(Box::new(black_queen)), position::Position(4, 8));

        if let Some(knight) = board.get_square(position::Position(4, 4)) {
            assert!(knight.valid_moves_during_pin(&board, king_pos).is_none());
        } else {
            panic!();
        }
    }

    #[test]
    fn knight_pinned_by_bishop() {
        let knight_pos = position::Position(4, 4);
        let knight = knight::Knight {
            id: 1,
            position: knight_pos,
            color: color::Color::WHITE,
        };

        let king_pos = position::Position(1, 1);
        let king = king::King {
            id: 2,
            position: king_pos,
            color: color::Color::WHITE,
        };

        let black_bishop = bishop::Bishop {
            id: 3,
            position: position::Position(6, 6),
            color: color::Color::BLACK,
        };

        let mut board = board::Board::empty();

        board.set_square(Some(Box::new(king)), king_pos);
        board.set_square(Some(Box::new(knight)), knight_pos);
        board.set_square(Some(Box::new(black_bishop)), position::Position(6, 6));

        if let Some(knight) = board.get_square(knight_pos) {
            if let Some(actual) = knight.valid_moves_during_pin(&board, king_pos) {
                let expected = vec![
                    position::Position(2, 2),
                    position::Position(3, 3),
                    position::Position(5, 5),
                    position::Position(6, 6),
                ];
                assert_eq!(expected.len(), actual.len());

                for square in expected {
                    assert!(actual.contains(&square));
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn knight_not_pinned_by_bishop() {
        let knight_pos = position::Position(4, 4);
        let knight = knight::Knight {
            id: 1,
            position: knight_pos,
            color: color::Color::WHITE,
        };

        let king_pos = position::Position(1, 1);
        let king = king::King {
            id: 2,
            position: king_pos,
            color: color::Color::WHITE,
        };

        let black_bishop = bishop::Bishop {
            id: 3,
            position: position::Position(6, 6),
            color: color::Color::BLACK,
        };

        let black_knight_pos = position::Position(5, 5);
        let black_knight = knight::Knight {
            id: 4,
            position: black_knight_pos,
            color: color::Color::BLACK,
        };

        let mut board = board::Board::empty();

        board.set_square(Some(Box::new(king)), king_pos);
        board.set_square(Some(Box::new(knight)), knight_pos);
        board.set_square(Some(Box::new(black_bishop)), position::Position(6, 6));
        board.set_square(Some(Box::new(black_knight)), black_knight_pos);

        if let Some(knight) = board.get_square(knight_pos) {
            assert!(knight.valid_moves_during_pin(&board, king_pos).is_none());
        } else {
            panic!();
        }
    }

    #[test]
    fn not_pinned_knight_no_attacker() {
        let knight = knight::Knight {
            id: 1,
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            id: 2,
            position: position::Position(4, 1),
            color: color::Color::WHITE,
        };

        let mut board = board::Board::empty();

        board.set_square(Some(Box::new(king)), position::Position(4, 1));
        board.set_square(Some(Box::new(knight)), position::Position(4, 4));

        if let Some(knight) = board.get_square(position::Position(4, 4)) {
            assert!(knight
                .valid_moves_during_pin(&board, position::Position(4, 1))
                .is_none());
        } else {
            panic!();
        }
    }

    // TODO: incorporate new moves function into these tests / write new ones
}
