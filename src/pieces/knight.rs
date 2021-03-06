use super::{piece, position};
use crate::{board, chessmove, color};

#[derive(Debug)]
pub struct Knight {
    pub position: position::Position,
    pub color: color::Color,
}

impl piece::Piece for Knight {
    fn moves_ignoring_pins(
        &self,
        board: &board::Board,
        _en_passant: &Option<position::Position>,
        _king_pos: position::Position,
    ) -> Vec<chessmove::ChessMove> {
        let positions: [[i8; 2]; 8] = [
            [1, 2],
            [1, -2],
            [-1, 2],
            [-1, -2],
            [2, 1],
            [2, -1],
            [-2, 1],
            [-2, -1],
        ];

        let mut attacks = Vec::new();

        for position in &positions {
            let file = position[0] + self.position.0 as i8;
            let rank = position[1] + self.position.1 as i8;
            let move_position = position::Position(file as u8, rank as u8);
            let mv = chessmove::ChessMove {
                from: (self.position().0, self.position().1),
                to: (move_position.0, move_position.1),
                promotion: None,
            };
            if file >= 1 && file <= 8 && rank >= 1 && rank <= 8 {
                match board.get_square(position::Position(file as u8, rank as u8)) {
                    Some(piece) => {
                        if *piece.color() != self.color {
                            attacks.push(mv)
                        }
                    }
                    None => attacks.push(mv),
                }
            }
        }

        attacks
    }

    fn piece(&self) -> piece::PieceEnum {
        piece::PieceEnum::KNIGHT
    }

    fn attacks(
        &self,
        _board: &board::Board,
        _enemy_king_pos: position::Position,
    ) -> Vec<position::Position> {
        let positions: [[i8; 2]; 8] = [
            [1, 2],
            [1, -2],
            [-1, 2],
            [-1, -2],
            [2, 1],
            [2, -1],
            [-2, 1],
            [-2, -1],
        ];

        let mut attacks = Vec::new();

        for position in &positions {
            let file = position[0] + self.position.0 as i8;
            let rank = position[1] + self.position.1 as i8;

            if file >= 1 && file <= 8 && rank >= 1 && rank <= 8 {
                attacks.push(position::Position(file as u8, rank as u8))
            }
        }

        attacks
    }

    fn position(&self) -> &position::Position {
        &self.position
    }

    fn mut_position(&mut self) -> &mut position::Position {
        &mut self.position
    }

    fn color(&self) -> &color::Color {
        &self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use piece::Piece;

    #[test]
    fn attrs() {
        let k = Knight {
            position: position::Position(2, 2),
            color: color::Color::WHITE,
        };
        assert_eq!(&position::Position(2, 2), k.position());
        assert_eq!(&color::Color::WHITE, k.color());
    }

    #[test]
    fn all_positions() {
        let k = Knight {
            position: position::Position(6, 6),
            color: color::Color::WHITE,
        };

        let expected = vec![
            position::Position(5, 4),
            position::Position(4, 5),
            position::Position(5, 8),
            position::Position(8, 5),
            position::Position(4, 7),
            position::Position(7, 4),
            position::Position(8, 7),
            position::Position(7, 8),
        ];
        let actual = k.attacks(&board::Board::empty(), position::Position(0, 0));

        for square in expected {
            assert!(actual.contains(&square));
        }
    }

    #[test]
    fn in_file_g() {
        let k = Knight {
            position: position::Position(7, 6),
            color: color::Color::WHITE,
        };

        let expected = vec![
            position::Position(6, 4),
            position::Position(5, 5),
            position::Position(6, 8),
            position::Position(5, 7),
            position::Position(8, 4),
            position::Position(8, 8),
        ];
        let actual = k.attacks(&board::Board::empty(), position::Position(0, 0));

        for square in expected {
            assert!(actual.contains(&square));
        }
    }

    #[test]
    fn in_file_h() {
        let k = Knight {
            position: position::Position(8, 6),
            color: color::Color::WHITE,
        };

        let expected = vec![
            position::Position(7, 4),
            position::Position(6, 5),
            position::Position(7, 8),
            position::Position(6, 7),
        ];
        let actual = k.attacks(&board::Board::empty(), position::Position(0, 0));

        for square in expected {
            assert!(actual.contains(&square));
        }
    }
}
