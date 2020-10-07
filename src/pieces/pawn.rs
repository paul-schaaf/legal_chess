use super::position;
use super::{piece, piece::Piece, piece::PieceEnum};
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
        let modifier = match *self.color() {
            color::Color::WHITE => 1,
            color::Color::BLACK => -1,
        };

        if position.0 == 1 {
            return vec![position::Position(
                position.0 + 1,
                (position.1 as i8 + modifier) as u8,
            )];
        } else if position.0 == 8 {
            return vec![position::Position(
                position.0 - 1,
                (position.1 as i8 + modifier) as u8,
            )];
        } else {
            return vec![
                position::Position(position.0 - 1, (position.1 as i8 + modifier) as u8),
                position::Position(position.0 + 1, (position.1 as i8 + modifier) as u8),
            ];
        }
    }

    fn moves_ignoring_pins(
        &self,
        board: &board::Board,
        en_passant: &Option<position::Position>,
        king_pos: position::Position,
    ) -> Vec<chessmove::ChessMove> {
        let mut moves = vec![];
        self.add_attack_moves(board, &mut moves);
        self.add_forward_moves(board, &mut moves);
        self.add_en_passant_moves(&mut moves, en_passant, king_pos, board);
        self.transform_mvs_to_chessmoves(&moves)
    }

    fn position(&self) -> &position::Position {
        &(self.position)
    }

    fn color(&self) -> &color::Color {
        &(self.color)
    }
}

impl Pawn {
    fn transform_mvs_to_chessmoves(
        &self,
        moves: &Vec<position::Position>,
    ) -> Vec<chessmove::ChessMove> {
        let mut chessmoves = vec![];

        for mv in moves {
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

    fn add_en_passant_moves(
        &self,
        moves: &mut Vec<position::Position>,
        en_passant: &Option<position::Position>,
        king_pos: position::Position,
        board: &board::Board,
    ) {
        if let Some(en_passant) = en_passant {
            let (ep_capture_rank, ep_capturer_rank) = match self.color() {
                color::Color::WHITE => (6, 5),
                color::Color::BLACK => (3, 4),
            };
            if en_passant.1 == ep_capture_rank
                && self.position.1 == ep_capturer_rank
                && ((en_passant.0 == self.position.0 - 1) || (en_passant.0 == self.position.0 + 1))
            {
                if !self.cannot_en_passant_due_to_discovered_check(king_pos, *en_passant, &board) {
                    moves.push(*en_passant);
                }
            }
        }
    }

    fn add_attack_moves(&self, board: &board::Board, moves: &mut Vec<position::Position>) {
        moves.append(
            &mut self
                .attacks(board, position::Position(0, 0))
                .into_iter()
                .filter(|pos| match board.get_square(*pos) {
                    None => false,
                    Some(piece) => *piece.color() != *self.color(),
                })
                .collect::<Vec<_>>(),
        );
    }

    fn add_forward_moves(&self, board: &board::Board, moves: &mut Vec<position::Position>) {
        let (initial, initial_one, initial_two, last, moving) = match self.color() {
            color::Color::WHITE => (2, 3, 4, 8, 1),
            color::Color::BLACK => (7, 6, 5, 1, -1),
        };

        if self.position().1 == initial
            && board.is_empty(position::Position(self.position().0, initial_one))
            && board.is_empty(position::Position(self.position().0, initial_two))
        {
            moves.push(position::Position(self.position().0, initial_two));
        }

        if self.position().1 != last
            && board.is_empty(position::Position(
                self.position().0,
                (self.position().1 as i8 + moving) as u8,
            ))
        {
            moves.push(position::Position(
                self.position().0,
                (self.position().1 as i8 + moving) as u8,
            ));
        }
    }

    fn cannot_en_passant_due_to_discovered_check(
        &self,
        king_pos: position::Position,
        ep: position::Position,
        board: &board::Board,
    ) -> bool {
        if king_pos.1 == self.position().1 {
            let modifier = if king_pos.0 > self.position().0 {
                -1
            } else {
                1
            };
            let mut pos = position::Position((king_pos.0 as i8) as u8, king_pos.1);
            loop {
                pos = position::Position((pos.0 as i8 + modifier) as u8, pos.1);
                if pos.0 == 0 || pos.0 == 9 {
                    break;
                }
                if !(pos == *self.position() || pos == position::Position(ep.0, self.position().1))
                {
                    if let Some(p) = board.get_square(pos) {
                        if p.color() != self.color() {
                            if [PieceEnum::QUEEN, PieceEnum::ROOK].contains(&p.piece()) {
                                return true;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::super::{bishop, king, pawn, piece::Piece, queen, rook};
    use super::*;

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
        let moves = white_pawn.moves_ignoring_pins(&board, &None, position::Position(0, 0));
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

        let moves = white_pawn.moves_ignoring_pins(&board, &None, position::Position(0, 0));
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

        let moves = white_pawn.moves_ignoring_pins(&board, &None, position::Position(0, 0));
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
        let moves = black_pawn.moves_ignoring_pins(&board, &None, position::Position(0, 0));
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

        let moves = black_pawn.moves_ignoring_pins(&board, &None, position::Position(0, 0));
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

        let moves = black_pawn.moves_ignoring_pins(&board, &None, position::Position(0, 0));
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

        let moves = black_pawn.moves_ignoring_pins(
            &board,
            &Some(position::Position(5, 3)),
            position::Position(0, 0),
        );
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

    // 8/8/8/K1Pp3q/8/8/8/8 w - - 0 1 invalid en passant because of discovered check
    #[test]
    fn invalid_en_passant_because_of_discovered_check_white() {
        let mut board = board::Board::empty();

        let pawn = pawn::Pawn {
            position: position::Position(3, 5),
            color: color::Color::WHITE,
        };

        let king = king::King {
            position: position::Position(1, 5),
            color: color::Color::WHITE,
        };

        let queen = queen::Queen {
            position: position::Position(8, 5),
            color: color::Color::BLACK,
        };

        let black_pawn = pawn::Pawn {
            position: position::Position(4, 5),
            color: color::Color::BLACK,
        };

        board.set_square(Some(Box::new(pawn)), position::Position(3, 5));
        board.set_square(Some(Box::new(king)), position::Position(1, 5));
        board.set_square(Some(Box::new(queen)), position::Position(8, 5));
        board.set_square(Some(Box::new(black_pawn)), position::Position(4, 5));

        if let Some(pawn) = board.get_square(position::Position(3, 5)) {
            assert_eq!(
                1,
                pawn.moves(
                    &board,
                    position::Position(1, 5),
                    &Some(position::Position(4, 6))
                )
                .len()
            );
        } else {
            panic!();
        }
    }

    // 8/8/8/8/Q2Pp2k/8/8/8 w - - 0 1 invalid en passant because of discovered check
    #[test]
    fn invalid_en_passant_because_of_discovered_check_black() {
        let mut board = board::Board::empty();

        let pawn = pawn::Pawn {
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            position: position::Position(8, 4),
            color: color::Color::BLACK,
        };

        let queen = queen::Queen {
            position: position::Position(1, 4),
            color: color::Color::WHITE,
        };

        let black_pawn = pawn::Pawn {
            position: position::Position(5, 4),
            color: color::Color::BLACK,
        };

        board.set_square(Some(Box::new(pawn)), position::Position(4, 4));
        board.set_square(Some(Box::new(king)), position::Position(8, 4));
        board.set_square(Some(Box::new(queen)), position::Position(1, 4));
        board.set_square(Some(Box::new(black_pawn)), position::Position(5, 4));

        if let Some(pawn) = board.get_square(position::Position(5, 4)) {
            assert_eq!(
                1,
                pawn.moves(
                    &board,
                    position::Position(8, 4),
                    &Some(position::Position(4, 3))
                )
                .len()
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn invalid_en_passant_because_of_discovered_check_black_blocked_queenside() {
        let mut board = board::Board::empty();

        let pawn = pawn::Pawn {
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            position: position::Position(8, 4),
            color: color::Color::BLACK,
        };

        let queen = queen::Queen {
            position: position::Position(1, 4),
            color: color::Color::WHITE,
        };

        let black_pawn = pawn::Pawn {
            position: position::Position(5, 4),
            color: color::Color::BLACK,
        };

        let black_rook = rook::Rook {
            position: position::Position(3, 4),
            color: color::Color::BLACK,
        };

        board.set_square(Some(Box::new(pawn)), position::Position(4, 4));
        board.set_square(Some(Box::new(king)), position::Position(8, 4));
        board.set_square(Some(Box::new(queen)), position::Position(1, 4));
        board.set_square(Some(Box::new(black_pawn)), position::Position(5, 4));
        board.set_square(Some(Box::new(black_rook)), position::Position(3, 4));

        if let Some(pawn) = board.get_square(position::Position(5, 4)) {
            assert_eq!(
                2,
                pawn.moves(
                    &board,
                    position::Position(8, 4),
                    &Some(position::Position(4, 3))
                )
                .len()
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn invalid_en_passant_because_of_discovered_check_black_blocked_kingside() {
        let mut board = board::Board::empty();

        let pawn = pawn::Pawn {
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            position: position::Position(8, 4),
            color: color::Color::BLACK,
        };

        let queen = queen::Queen {
            position: position::Position(1, 4),
            color: color::Color::WHITE,
        };

        let black_pawn = pawn::Pawn {
            position: position::Position(5, 4),
            color: color::Color::BLACK,
        };

        let white_bishop = bishop::Bishop {
            position: position::Position(6, 4),
            color: color::Color::WHITE,
        };

        board.set_square(Some(Box::new(pawn)), position::Position(4, 4));
        board.set_square(Some(Box::new(king)), position::Position(8, 4));
        board.set_square(Some(Box::new(queen)), position::Position(1, 4));
        board.set_square(Some(Box::new(black_pawn)), position::Position(5, 4));
        board.set_square(Some(Box::new(white_bishop)), position::Position(6, 4));

        if let Some(pawn) = board.get_square(position::Position(5, 4)) {
            assert_eq!(
                2,
                pawn.moves(
                    &board,
                    position::Position(8, 4),
                    &Some(position::Position(4, 3))
                )
                .len()
            );
        } else {
            panic!();
        }
    }
}
