use super::piece;
use super::position;
use crate::{board, color};

#[derive(Debug, PartialEq)]
pub struct Pawn {
    pub id: u8,
    pub position: position::Position,
    pub color: color::Color,
}

impl piece::Piece for Pawn {
    fn get_id(&self) -> u8 {
        return self.id;
    }

    fn attacks(&self, _board: &board::Board) -> Vec<position::Position> {
        let position = self.position();
        if *self.color() == color::Color::WHITE {
            if position.1 == 8 {
                panic!("White pawn cannot stand on rank 8");
            }

            if position.0 == 1 {
                return vec![position::Position(position.0 + 1, position.1 + 1)];
            } else if position.0 == 8 {
                return vec![position::Position(position.0 - 1, position.1 + 1)];
            } else {
                return vec![
                    position::Position(position.0 - 1, position.1 + 1),
                    position::Position(position.0 + 1, position.1 + 1),
                ];
            }
        } else {
            if position.1 == 1 {
                panic!("Black pawn cannot stand on rank 1");
            }

            if position.0 == 1 {
                return vec![position::Position(position.0 + 1, position.1 - 1)];
            } else if position.0 == 8 {
                return vec![position::Position(position.0 - 1, position.1 - 1)];
            } else {
                return vec![
                    position::Position(position.0 - 1, position.1 - 1),
                    position::Position(position.0 + 1, position.1 - 1),
                ];
            }
        }
    }

    fn position(&self) -> &position::Position {
        &(self.position)
    }

    fn color(&self) -> &color::Color {
        &(self.color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use piece::Piece;

    #[test]
    fn attrs() {
        let p = Pawn {
            id: 1,
            position: position::Position(2, 2),
            color: color::Color::WHITE,
        };
        assert_eq!(&position::Position(2, 2), p.position());
        assert_eq!(&color::Color::WHITE, p.color());
    }

    // comparing with assert_eq is not possible because it cares about order
    #[test]
    fn white_pawn_attacks_middle() {
        let expected = vec![position::Position(6, 3), position::Position(4, 3)];
        let p = Pawn {
            id: 1,
            position: position::Position(5, 2),
            color: color::Color::WHITE,
        };

        let actual = p.attacks(&board::Board::empty());

        for square in expected {
            assert!(actual.contains(&square));
        }
    }

    #[test]
    fn white_pawn_attacks_file_one() {
        let expected = vec![position::Position(2, 3)];
        let p = Pawn {
            id: 1,
            position: position::Position(1, 2),
            color: color::Color::WHITE,
        };
        assert_eq!(expected, p.attacks(&board::Board::empty()));
    }

    #[test]
    fn white_pawn_attacks_file_eight() {
        let expected = vec![position::Position(7, 3)];
        let p = Pawn {
            id: 1,
            position: position::Position(8, 2),
            color: color::Color::WHITE,
        };
        assert_eq!(expected, p.attacks(&board::Board::empty()));
    }

    // comparing with assert_eq is not possible because it cares about order
    #[test]
    fn black_pawn_attacks_middle() {
        let expected = vec![position::Position(6, 6), position::Position(4, 6)];
        let p = Pawn {
            id: 1,
            position: position::Position(5, 7),
            color: color::Color::BLACK,
        };

        let actual = p.attacks(&board::Board::empty());

        for square in expected {
            assert!(actual.contains(&square));
        }
    }

    #[test]
    fn black_pawn_attacks_file_one() {
        let expected = vec![position::Position(2, 6)];
        let p = Pawn {
            id: 1,
            position: position::Position(1, 7),
            color: color::Color::BLACK,
        };
        assert_eq!(expected, p.attacks(&board::Board::empty()));
    }

    #[test]
    fn black_pawn_attacks_file_eight() {
        let expected = vec![position::Position(7, 6)];
        let p = Pawn {
            id: 1,
            position: position::Position(8, 7),
            color: color::Color::BLACK,
        };
        assert_eq!(expected, p.attacks(&board::Board::empty()));
    }
}
