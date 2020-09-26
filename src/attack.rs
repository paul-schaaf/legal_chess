use crate::board;
use crate::color;
use crate::pieces::piece;

pub fn get_attacked_squares(
    board: &board::Board,
    color: color::Color,
) -> Vec<Vec<Option<Vec<&Box<dyn piece::Piece>>>>> {
    let mut attacked_board: Vec<Vec<Option<Vec<&Box<dyn piece::Piece>>>>> = vec![];

    for i in 0..8 {
        attacked_board.push(vec![]);
        for _ in 0..8 {
            attacked_board[i].push(None);
        }
    }

    board.iter().for_each(|file| {
        file.iter().for_each(|square| match square {
            None => (),
            Some(piece) => {
                let attacked_positions = piece.attacks(board);
                attacked_positions.iter().for_each(|position| {
                    let square = attacked_board[position.0 as usize][position.1 as usize].take();
                    let new_square = match square {
                        None => Some(vec![piece]),
                        Some(mut v) => {
                            v.push(piece);
                            Some(v)
                        }
                    };
                    attacked_board[position.0 as usize - 1][position.1 as usize - 1] = new_square;
                })
            }
        })
    });

    attacked_board
}

#[cfg(test)]
mod tests {
    use super::super::pieces::{pawn, position};
    use super::*;

    #[test]
    fn correct_attacked_board() {
        let mut empty_board = board::Board::empty();

        let position = position::Position(1, 2);

        let pawn_id = 1;
        let pawn = pawn::Pawn {
            id: pawn_id,
            position,
            color: color::Color::WHITE,
        };

        empty_board.set_square(Some(Box::new(pawn)), position);

        let attacked_board = get_attacked_squares(&empty_board, color::Color::BLACK);
        assert_eq!(8, attacked_board.len());
        assert!(attacked_board.iter().all(|file| file.len() == 8));

        let actual = match &attacked_board[1][2] {
            None => panic!(),
            Some(v) => {
                assert_eq!(1, v.len());
                v[0]
            }
        };

        assert_eq!(pawn_id, actual.get_id());
    }
}
