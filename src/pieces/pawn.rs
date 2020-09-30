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

    fn piece(&self) -> piece::PieceEnum {
        piece::PieceEnum::PAWN
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

    fn moves(&self, board: &board::Board) -> Vec<position::Position> {
        let position = self.position();
        let mut moves = vec![];

        let attacked_squares = self.attacks(board);
        let mut attack_moves = attacked_squares
            .into_iter()
            .filter(|pos| match board.get_square(*pos) {
                None => false,
                Some(piece) => *piece.color() != *self.color(),
            })
            .collect::<Vec<_>>();
        moves.append(&mut attack_moves);

        match self.color() {
            color::Color::WHITE => {
                if position.1 == 2 {
                    if board
                        .get_square(position::Position(position.0, 3))
                        .is_none()
                        && board
                            .get_square(position::Position(position.0, 4))
                            .is_none()
                    {
                        moves.push(position::Position(position.0, 4));
                    }
                }
                if position.1 != 8 {
                    if board
                        .get_square(position::Position(position.0, position.1 + 1))
                        .is_none()
                    {
                        moves.push(position::Position(position.0, position.1 + 1));
                    }
                }
            }
            color::Color::BLACK => {
                if position.1 == 7 {
                    if board
                        .get_square(position::Position(position.0, 6))
                        .is_none()
                        && board
                            .get_square(position::Position(position.0, 5))
                            .is_none()
                    {
                        moves.push(position::Position(position.0, 5));
                    }
                }
                if position.1 != 1 {
                    if board
                        .get_square(position::Position(position.0, position.1 - 1))
                        .is_none()
                    {
                        moves.push(position::Position(position.0, position.1 - 1));
                    }
                }
            }
        }
        moves
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
    use super::super::pawn;
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

    #[test]
    fn white_pawn_moves() {
        let mut board = board::Board::initial();

        let black_pawn_pos = position::Position(5, 3);
        let black_pawn = pawn::Pawn {
            id: 1,
            color: color::Color::BLACK,
            position: black_pawn_pos,
        };

        board.set_square(Some(Box::new(black_pawn)), black_pawn_pos);

        let white_pawn = pawn::Pawn {
            id: 2,
            color: color::Color::WHITE,
            position: position::Position(4, 2),
        };

        let expected_moves = vec![
            position::Position(4, 3),
            position::Position(4, 4),
            position::Position(5, 3),
        ];
        let moves = white_pawn.moves(&board);
        assert_eq!(3, moves.len());
        for mv in expected_moves {
            assert!(moves.contains(&mv));
        }
    }

    #[test]
    fn white_pawn_blocked() {
        let mut board = board::Board::initial();
        let white_pawn = pawn::Pawn {
            id: 1,
            color: color::Color::WHITE,
            position: position::Position(4, 2),
        };
        let white_pawn_2_pos = position::Position(4, 3);
        let white_pawn_2 = pawn::Pawn {
            id: 2,
            color: color::Color::WHITE,
            position: white_pawn_2_pos,
        };
        board.set_square(Some(Box::new(white_pawn_2)), white_pawn_2_pos);

        let moves = white_pawn.moves(&board);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn white_pawn_blocked_2() {
        let mut board = board::Board::initial();
        let white_pawn = pawn::Pawn {
            id: 1,
            color: color::Color::WHITE,
            position: position::Position(4, 2),
        };
        let white_pawn_2_pos = position::Position(4, 4);
        let white_pawn_2 = pawn::Pawn {
            id: 2,
            color: color::Color::WHITE,
            position: white_pawn_2_pos,
        };
        board.set_square(Some(Box::new(white_pawn_2)), white_pawn_2_pos);

        let moves = white_pawn.moves(&board);
        assert_eq!(1, moves.len());
        assert_eq!(position::Position(4, 3), moves[0]);
    }

    #[test]
    fn black_pawn_moves() {
        let mut board = board::Board::initial();

        let white_pawn_pos = position::Position(6, 6);
        let white_pawn = pawn::Pawn {
            id: 1,
            color: color::Color::WHITE,
            position: white_pawn_pos,
        };

        board.set_square(Some(Box::new(white_pawn)), white_pawn_pos);

        let black_pawn = pawn::Pawn {
            id: 2,
            color: color::Color::BLACK,
            position: position::Position(5, 7),
        };

        let expected_moves = vec![
            position::Position(5, 6),
            position::Position(5, 5),
            position::Position(6, 6),
        ];
        let moves = black_pawn.moves(&board);
        assert_eq!(3, moves.len());
        for mv in expected_moves {
            assert!(moves.contains(&mv));
        }
    }

    #[test]
    fn black_pawn_blocked() {
        let mut board = board::Board::initial();
        let black_pawn = pawn::Pawn {
            id: 1,
            color: color::Color::BLACK,
            position: position::Position(5, 7),
        };
        let black_pawn_2_pos = position::Position(5, 6);
        let black_pawn_2 = pawn::Pawn {
            id: 2,
            color: color::Color::BLACK,
            position: black_pawn_2_pos,
        };
        board.set_square(Some(Box::new(black_pawn_2)), black_pawn_2_pos);

        let moves = black_pawn.moves(&board);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn black_pawn_blocked_2() {
        let mut board = board::Board::initial();
        let black_pawn = pawn::Pawn {
            id: 1,
            color: color::Color::BLACK,
            position: position::Position(5, 7),
        };
        let black_pawn_2_pos = position::Position(5, 5);
        let black_pawn_2 = pawn::Pawn {
            id: 2,
            color: color::Color::BLACK,
            position: black_pawn_2_pos,
        };
        board.set_square(Some(Box::new(black_pawn_2)), black_pawn_2_pos);

        let moves = black_pawn.moves(&board);
        assert_eq!(1, moves.len());
        assert_eq!(position::Position(5, 6), moves[0]);
    }
}
