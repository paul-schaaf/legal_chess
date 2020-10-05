use super::piece;
use super::position;
use crate::{board, chessmove, color};

#[derive(Debug, PartialEq)]
pub struct Pawn {
    pub position: position::Position,
    pub color: color::Color,
}

impl piece::Piece for Pawn {
    fn piece(&self) -> piece::PieceEnum {
        piece::PieceEnum::PAWN
    }

    fn mut_position(&mut self) -> &mut position::Position {
        &mut self.position
    }

    fn attacks(
        &self,
        _board: &board::Board,
        _enemy_king_pos: position::Position,
    ) -> Vec<position::Position> {
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

    fn moves_ignoring_pins(
        &self,
        board: &board::Board,
        en_passant: &Option<position::Position>,
    ) -> Vec<chessmove::ChessMove> {
        let position = self.position();
        let mut moves = vec![];

        let attacked_squares = self.attacks(board, position::Position(0, 0));
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
                if position.1 == 2
                    && board
                        .get_square(position::Position(position.0, 3))
                        .is_none()
                    && board
                        .get_square(position::Position(position.0, 4))
                        .is_none()
                {
                    moves.push(position::Position(position.0, 4));
                }

                if position.1 != 8
                    && board
                        .get_square(position::Position(position.0, position.1 + 1))
                        .is_none()
                {
                    moves.push(position::Position(position.0, position.1 + 1));
                }
            }
            color::Color::BLACK => {
                if position.1 == 7
                    && board
                        .get_square(position::Position(position.0, 6))
                        .is_none()
                    && board
                        .get_square(position::Position(position.0, 5))
                        .is_none()
                {
                    moves.push(position::Position(position.0, 5));
                }

                if position.1 != 1
                    && board
                        .get_square(position::Position(position.0, position.1 - 1))
                        .is_none()
                {
                    moves.push(position::Position(position.0, position.1 - 1));
                }
            }
        }

        if let Some(en_passant) = en_passant {
            match self.color() {
                color::Color::WHITE => {
                    if self.position.1 == 5
                        && en_passant.1 == 6
                        && ((en_passant.0 == self.position.0 - 1)
                            || (en_passant.0 == self.position.0 + 1))
                    {
                        moves.push(*en_passant);
                    }
                }
                color::Color::BLACK => {
                    if self.position.1 == 4
                        && en_passant.1 == 3
                        && ((en_passant.0 == self.position.0 - 1)
                            || (en_passant.0 == self.position.0 + 1))
                    {
                        moves.push(*en_passant);
                    }
                }
            }
        }
        let mut chessmoves = vec![];

        for mv in &moves {
            if mv.1 == 1 || mv.1 == 8 {
                for pc in piece::PROMOTION_PIECES.iter().copied() {
                    chessmoves.push(chessmove::ChessMove {
                        from: (self.position().0, self.position().1),
                        to: (mv.0, mv.1),
                        promotion: Some(pc),
                    });
                }
            } else {
                chessmoves.push(chessmove::ChessMove {
                    from: (self.position().0, self.position().1),
                    to: (mv.0, mv.1),
                    promotion: None,
                })
            }
        }

        chessmoves
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
            position: position::Position(5, 2),
            color: color::Color::WHITE,
        };

        let actual = p.attacks(&board::Board::empty(), position::Position(0, 0));

        for square in expected {
            assert!(actual.contains(&square));
        }
    }

    #[test]
    fn white_pawn_attacks_file_one() {
        let expected = vec![position::Position(2, 3)];
        let p = Pawn {
            position: position::Position(1, 2),
            color: color::Color::WHITE,
        };
        assert_eq!(
            expected,
            p.attacks(&board::Board::empty(), position::Position(0, 0))
        );
    }

    #[test]
    fn white_pawn_attacks_file_eight() {
        let expected = vec![position::Position(7, 3)];
        let p = Pawn {
            position: position::Position(8, 2),
            color: color::Color::WHITE,
        };
        assert_eq!(
            expected,
            p.attacks(&board::Board::empty(), position::Position(0, 0))
        );
    }

    // comparing with assert_eq is not possible because it cares about order
    #[test]
    fn black_pawn_attacks_middle() {
        let expected = vec![position::Position(6, 6), position::Position(4, 6)];
        let p = Pawn {
            position: position::Position(5, 7),
            color: color::Color::BLACK,
        };

        let actual = p.attacks(&board::Board::empty(), position::Position(0, 0));

        for square in expected {
            assert!(actual.contains(&square));
        }
    }

    #[test]
    fn black_pawn_attacks_file_one() {
        let expected = vec![position::Position(2, 6)];
        let p = Pawn {
            position: position::Position(1, 7),
            color: color::Color::BLACK,
        };
        assert_eq!(
            expected,
            p.attacks(&board::Board::empty(), position::Position(0, 0))
        );
    }

    #[test]
    fn black_pawn_attacks_file_eight() {
        let expected = vec![position::Position(7, 6)];
        let p = Pawn {
            position: position::Position(8, 7),
            color: color::Color::BLACK,
        };
        assert_eq!(
            expected,
            p.attacks(&board::Board::empty(), position::Position(0, 0))
        );
    }

    #[test]
    fn white_pawn_moves() {
        let mut board = board::Board::initial();

        let black_pawn_pos = position::Position(5, 3);
        let black_pawn = pawn::Pawn {
            color: color::Color::BLACK,
            position: black_pawn_pos,
        };

        board.set_square(Some(Box::new(black_pawn)), black_pawn_pos);

        let white_pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: position::Position(4, 2),
        };

        let expected_moves = vec![
            chessmove::ChessMove {
                from: (white_pawn.position().0, white_pawn.position().1),
                to: (4, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (white_pawn.position().0, white_pawn.position().1),
                to: (4, 4),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (white_pawn.position().0, white_pawn.position().1),
                to: (5, 3),
                promotion: None,
            },
        ];
        let moves = white_pawn.moves_ignoring_pins(&board, &None);
        assert_eq!(3, moves.len());
        for mv in expected_moves {
            assert!(moves.contains(&mv));
        }
    }

    #[test]
    fn white_pawn_blocked() {
        let mut board = board::Board::initial();
        let white_pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: position::Position(4, 2),
        };
        let white_pawn_2_pos = position::Position(4, 3);
        let white_pawn_2 = pawn::Pawn {
            color: color::Color::WHITE,
            position: white_pawn_2_pos,
        };
        board.set_square(Some(Box::new(white_pawn_2)), white_pawn_2_pos);

        let moves = white_pawn.moves_ignoring_pins(&board, &None);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn white_pawn_blocked_2() {
        let mut board = board::Board::initial();
        let white_pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: position::Position(4, 2),
        };
        let white_pawn_2_pos = position::Position(4, 4);
        let white_pawn_2 = pawn::Pawn {
            color: color::Color::WHITE,
            position: white_pawn_2_pos,
        };
        board.set_square(Some(Box::new(white_pawn_2)), white_pawn_2_pos);

        let moves = white_pawn.moves_ignoring_pins(&board, &None);
        assert_eq!(1, moves.len());
        assert_eq!(
            chessmove::ChessMove {
                from: (white_pawn.position().0, white_pawn.position().1),
                to: (4, 3),
                promotion: None,
            },
            moves[0]
        );
    }

    #[test]
    fn black_pawn_moves() {
        let mut board = board::Board::initial();

        let white_pawn_pos = position::Position(6, 6);
        let white_pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: white_pawn_pos,
        };

        board.set_square(Some(Box::new(white_pawn)), white_pawn_pos);

        let black_pawn = pawn::Pawn {
            color: color::Color::BLACK,
            position: position::Position(5, 7),
        };

        let expected_moves = vec![
            chessmove::ChessMove {
                from: (black_pawn.position().0, black_pawn.position().1),
                to: (5, 6),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (black_pawn.position().0, black_pawn.position().1),
                to: (5, 5),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (black_pawn.position().0, black_pawn.position().1),
                to: (6, 6),
                promotion: None,
            },
        ];
        let moves = black_pawn.moves_ignoring_pins(&board, &None);
        assert_eq!(3, moves.len());
        for mv in expected_moves {
            assert!(moves.contains(&mv));
        }
    }

    #[test]
    fn black_pawn_blocked() {
        let mut board = board::Board::initial();
        let black_pawn = pawn::Pawn {
            color: color::Color::BLACK,
            position: position::Position(5, 7),
        };
        let black_pawn_2_pos = position::Position(5, 6);
        let black_pawn_2 = pawn::Pawn {
            color: color::Color::BLACK,
            position: black_pawn_2_pos,
        };
        board.set_square(Some(Box::new(black_pawn_2)), black_pawn_2_pos);

        let moves = black_pawn.moves_ignoring_pins(&board, &None);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn black_pawn_blocked_2() {
        let mut board = board::Board::initial();
        let black_pawn = pawn::Pawn {
            color: color::Color::BLACK,
            position: position::Position(5, 7),
        };
        let black_pawn_2_pos = position::Position(5, 5);
        let black_pawn_2 = pawn::Pawn {
            color: color::Color::BLACK,
            position: black_pawn_2_pos,
        };
        board.set_square(Some(Box::new(black_pawn_2)), black_pawn_2_pos);

        let moves = black_pawn.moves_ignoring_pins(&board, &None);
        assert_eq!(1, moves.len());
        assert_eq!(
            chessmove::ChessMove {
                from: (black_pawn.position().0, black_pawn.position().1),
                to: (5, 6),
                promotion: None,
            },
            moves[0]
        );
    }

    #[test]
    fn en_passant() {
        let mut board = board::Board::empty();
        let black_pawn = pawn::Pawn {
            color: color::Color::BLACK,
            position: position::Position(4, 4),
        };
        let white_pawn_pos = position::Position(5, 4);
        let white_pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: white_pawn_pos,
        };
        board.set_square(Some(Box::new(white_pawn)), white_pawn_pos);

        let moves = black_pawn.moves_ignoring_pins(&board, &Some(position::Position(5, 3)));
        let expected = vec![
            chessmove::ChessMove {
                from: (black_pawn.position().0, black_pawn.position().1),
                to: (4, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (black_pawn.position().0, black_pawn.position().1),
                to: (5, 3),
                promotion: None,
            },
        ];
        assert_eq!(expected.len(), moves.len());
        for mv in expected {
            assert!(moves.contains(&mv));
        }
    }
}
