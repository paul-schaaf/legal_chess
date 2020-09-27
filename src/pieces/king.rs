use super::{piece, position};
use crate::{board, color};

#[derive(Debug)]
pub struct King {
    id: u8,
    color: color::Color,
    position: position::Position,
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
}

#[cfg(test)]
mod tests {
    use super::piece::Piece;
    use super::*;

    #[test]
    fn king_bottom_left() {
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
    fn king_middle() {
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
}
