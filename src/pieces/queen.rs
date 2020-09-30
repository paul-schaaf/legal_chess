use super::{piece, position, sliding_attacks, sliding_moves};
use crate::{board, color};

#[derive(Debug)]
pub struct Queen {
    pub id: u8,
    pub color: color::Color,
    pub position: position::Position,
}

impl piece::Piece for Queen {
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
        piece::PieceEnum::QUEEN
    }

    fn moves(&self, board: &board::Board) -> Vec<position::Position> {
        let mut straight_moves = sliding_attacks::straight_attacks(self.position, board);
        let mut diagonal_moves = sliding_attacks::diagonal_attacks(self.position, board);
        straight_moves.append(&mut diagonal_moves);
        straight_moves
    }

    fn attacks(&self, board: &board::Board) -> Vec<position::Position> {
        let mut straight_attacks = sliding_moves::straight_sliding(self, board);
        let mut diagonal_attacks = sliding_moves::diagonal_sliding(self, board);
        straight_attacks.append(&mut diagonal_attacks);
        straight_attacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn queen_bottom_left() {
        let mut empty_board = board::Board::empty();

        let position = position::Position(1, 1);

        let queen = Queen {
            id: 1,
            color: color::Color::WHITE,
            position,
        };

        empty_board.set_square(Some(Box::new(queen)), position);

        let queen = match empty_board.get_square(position) {
            None => panic!(),
            Some(b) => b,
        };

        let attacked_positions = queen.attacks(&empty_board);

        let mut count_file = 1;
        let expected_attacked_positions = iter::from_fn(|| {
            count_file += 1;
            if count_file < 9 {
                Some(position::Position(count_file, queen.position().1))
            } else {
                None
            }
        });
        let mut count_rank = 1;
        let expected_attacked_positions = expected_attacked_positions.chain(iter::from_fn(|| {
            count_rank += 1;
            if count_rank < 9 {
                Some(position::Position(queen.position().0, count_rank))
            } else {
                None
            }
        }));

        let mut count_diagonal = 1;
        let expected_attacked_positions = expected_attacked_positions.chain(iter::from_fn(|| {
            count_diagonal += 1;
            if count_diagonal < 9 {
                Some(position::Position(count_diagonal, count_diagonal))
            } else {
                None
            }
        }));

        assert_eq!(
            expected_attacked_positions.collect::<Vec<_>>(),
            attacked_positions
        );
    }

    // TODO: queen moves tests
}
