use super::{bishop, king, knight, pawn, piece, position, queen, relative_position, rook};
use crate::{board, chessmove, color};
use std::fmt;

pub trait Piece: fmt::Debug {
    fn position(&self) -> &position::Position;

    fn mut_position(&mut self) -> &mut position::Position;

    fn set_position(&mut self, position: &position::Position) {
        self.mut_position().0 = position.0;
        self.mut_position().1 = position.1;
    }

    fn attacks(
        &self,
        board: &board::Board,
        enemy_king_pos: position::Position,
    ) -> Vec<position::Position>;

    fn color(&self) -> &color::Color;

    fn piece(&self) -> PieceEnum;

    fn moves_ignoring_pins(
        &self,
        board: &board::Board,
        en_passant: &Option<position::Position>,
    ) -> Vec<chessmove::ChessMove>;

    fn moves(
        &self,
        board: &board::Board,
        king: position::Position,
        en_passant: &Option<position::Position>,
    ) -> Vec<chessmove::ChessMove> {
        let mut moves_ignoring_pins = self.moves_ignoring_pins(board, en_passant);
        if self.piece() == piece::PieceEnum::PAWN {
            if let Some(ep) = en_passant {
                if moves_ignoring_pins
                    .iter()
                    .any(|mv| position::Position((mv.to).0, (mv.to).1) == *ep)
                {
                    if king.1 == self.position().1 {
                        let modifier = if king.0 > self.position().0 { -1 } else { 1 };
                        let mut pos = position::Position((king.0 as i8) as u8, king.1);
                        loop {
                            pos = position::Position((pos.0 as i8 + modifier) as u8, pos.1);
                            if pos.0 == 0 || pos.0 == 9 {
                                break;
                            }
                            if !(pos == *self.position()
                                || pos == position::Position(ep.0, self.position().1))
                            {
                                if let Some(p) = board.get_square(pos) {
                                    if p.color() != self.color() {
                                        if [PieceEnum::QUEEN, PieceEnum::ROOK].contains(&p.piece())
                                        {
                                            moves_ignoring_pins = moves_ignoring_pins
                                                .into_iter()
                                                .filter(|mv| (mv.to).0 != ep.0)
                                                .collect::<Vec<_>>();
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
                }
            }
        }

        match self.valid_moves_during_pin(board, king) {
            None => moves_ignoring_pins,
            Some(valid_moves_during_pin) => moves_ignoring_pins
                .into_iter()
                .filter(|mv| {
                    let pos = position::Position((mv.to).0, (mv.to).1);
                    valid_moves_during_pin.contains(&pos)
                })
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

        let (mover, direction) =
            match relative_position::get_line_to_other_piece(self.position(), &king) {
                None => return None,
                Some(v) => v,
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
                    }
                    match (piece.piece(), &direction) {
                        (PieceEnum::QUEEN, _)
                        | (PieceEnum::ROOK, relative_position::Direction::STRAIGHT(..))
                        | (PieceEnum::BISHOP, relative_position::Direction::DIAGONAL(..)) => {
                            moves.push(new_position);
                            break;
                        }
                        (_, _) => return None,
                    }
                }
            }
        }

        Some(moves)
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

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum PromotionPiece {
    Rook,
    Knight,
    Bishop,
    Queen,
}

pub static PROMOTION_PIECES: [PromotionPiece; 4] = [
    PromotionPiece::Rook,
    PromotionPiece::Knight,
    PromotionPiece::Bishop,
    PromotionPiece::Queen,
];

pub fn promotion_piece_to_piece(
    promotion_piece: PromotionPiece,
    color: color::Color,
    position: position::Position,
) -> Box<dyn Piece> {
    match promotion_piece {
        PromotionPiece::Rook => Box::new(rook::Rook { color, position }),
        PromotionPiece::Knight => Box::new(knight::Knight { color, position }),
        PromotionPiece::Bishop => Box::new(bishop::Bishop { color, position }),
        PromotionPiece::Queen => Box::new(queen::Queen { color, position }),
    }
}

pub fn type_to_piece(
    piece_type: PieceEnum,
    color: color::Color,
    position: position::Position,
) -> Box<dyn Piece> {
    match piece_type {
        PieceEnum::PAWN => Box::new(pawn::Pawn { color, position }),
        PieceEnum::ROOK => Box::new(rook::Rook { color, position }),
        PieceEnum::KNIGHT => Box::new(knight::Knight { color, position }),
        PieceEnum::QUEEN => Box::new(queen::Queen { color, position }),
        PieceEnum::KING => Box::new(king::King { color, position }),
        PieceEnum::BISHOP => Box::new(bishop::Bishop { color, position }),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{bishop, king, knight, pawn, queen};
    use super::*;

    #[test]
    fn pinned_knight() {
        let knight = knight::Knight {
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            position: position::Position(4, 1),
            color: color::Color::WHITE,
        };

        let black_queen = queen::Queen {
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
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            position: position::Position(4, 1),
            color: color::Color::WHITE,
        };

        let black_knight = knight::Knight {
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
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
            position: position::Position(4, 1),
            color: color::Color::WHITE,
        };

        let black_queen = queen::Queen {
            position: position::Position(4, 8),
            color: color::Color::BLACK,
        };

        let black_pawn = pawn::Pawn {
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
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king_pos = position::Position(5, 1);
        let king = king::King {
            position: king_pos,
            color: color::Color::WHITE,
        };

        let black_queen = queen::Queen {
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
            position: knight_pos,
            color: color::Color::WHITE,
        };

        let king_pos = position::Position(1, 1);
        let king = king::King {
            position: king_pos,
            color: color::Color::WHITE,
        };

        let black_bishop = bishop::Bishop {
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
            position: knight_pos,
            color: color::Color::WHITE,
        };

        let king_pos = position::Position(1, 1);
        let king = king::King {
            position: king_pos,
            color: color::Color::WHITE,
        };

        let black_bishop = bishop::Bishop {
            position: position::Position(6, 6),
            color: color::Color::BLACK,
        };

        let black_knight_pos = position::Position(5, 5);
        let black_knight = knight::Knight {
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
            position: position::Position(4, 4),
            color: color::Color::WHITE,
        };

        let king = king::King {
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
