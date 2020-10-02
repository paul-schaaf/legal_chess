use crate::board;
use crate::color;
use crate::pieces::{piece, position};

pub fn get_attacked_squares(
    board: &board::Board,
    color: color::Color,
    enemy_king_pos: position::Position,
) -> Vec<Vec<Vec<&Box<dyn piece::Piece>>>> {
    let mut attacked_board: Vec<Vec<Vec<&Box<dyn piece::Piece>>>> = vec![];

    for i in 0..8 {
        attacked_board.push(vec![]);
        for _ in 0..8 {
            attacked_board[i].push(vec![]);
        }
    }

    board.iter().for_each(|file| {
        file.iter().for_each(|square| match square {
            None => (),
            Some(piece) => {
                if *piece.color() == color {
                    let attacked_positions = piece.attacks(board, enemy_king_pos);
                    attacked_positions.iter().for_each(|position| {
                        let attackers =
                            &mut attacked_board[position.0 as usize - 1][position.1 as usize - 1];
                        attackers.push(piece);
                    })
                }
            }
        })
    });

    attacked_board
}

#[cfg(test)]
mod tests {
    use super::super::pieces::{pawn, piece, position};
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

        let attacked_board =
            get_attacked_squares(&empty_board, color::Color::WHITE, position::Position(0, 0));
        assert_eq!(8, attacked_board.len());
        assert!(attacked_board.iter().all(|file| file.len() == 8));

        assert_eq!(1, attacked_board[1][2].len());

        let actual = attacked_board[1][2][0];

        assert_eq!(pawn_id, actual.get_id());
    }

    #[test]
    fn initial_board_attacks_white() {
        initial_board_attacks(color::Color::WHITE);
    }

    #[test]
    fn initial_board_attacks_black() {
        initial_board_attacks(color::Color::BLACK);
    }

    fn initial_board_attacks(color: color::Color) {
        let board = board::Board::initial();
        let attacked_squares = get_attacked_squares(&board, color, position::Position(0, 0));

        for i in 0..8 {
            assert_eq!(0, attacked_squares[i][3].len());
            assert_eq!(0, attacked_squares[i][4].len());
        }

        let modifier = if color == color::Color::WHITE { 0 } else { 3 };

        assert_attacked_by(
            &attacked_squares[0][2 + modifier],
            vec![(piece::PieceEnum::PAWN, 1), (piece::PieceEnum::KNIGHT, 1)],
        );
        assert_attacked_by(
            &attacked_squares[1][2 + modifier],
            vec![(piece::PieceEnum::PAWN, 2)],
        );
        assert_attacked_by(
            &attacked_squares[2][2 + modifier],
            vec![(piece::PieceEnum::PAWN, 2), (piece::PieceEnum::KNIGHT, 1)],
        );
        assert_attacked_by(
            &attacked_squares[3][2 + modifier],
            vec![(piece::PieceEnum::PAWN, 2)],
        );
        assert_attacked_by(
            &attacked_squares[4][2 + modifier],
            vec![(piece::PieceEnum::PAWN, 2)],
        );
        assert_attacked_by(
            &attacked_squares[5][2 + modifier],
            vec![(piece::PieceEnum::PAWN, 2), (piece::PieceEnum::KNIGHT, 1)],
        );
        assert_attacked_by(
            &attacked_squares[6][2 + modifier],
            vec![(piece::PieceEnum::PAWN, 2)],
        );
        assert_attacked_by(
            &attacked_squares[7][2 + modifier],
            vec![(piece::PieceEnum::PAWN, 1), (piece::PieceEnum::KNIGHT, 1)],
        );

        let modifier = if color == color::Color::WHITE { 0 } else { 5 };

        assert_attacked_by(
            &attacked_squares[0][1 + modifier],
            vec![(piece::PieceEnum::ROOK, 1)],
        );
        assert_attacked_by(
            &attacked_squares[1][1 + modifier],
            vec![(piece::PieceEnum::BISHOP, 1)],
        );
        assert_attacked_by(
            &attacked_squares[2][1 + modifier],
            vec![(piece::PieceEnum::QUEEN, 1)],
        );
        assert_attacked_by(
            &attacked_squares[3][1 + modifier],
            vec![
                (piece::PieceEnum::QUEEN, 1),
                (piece::PieceEnum::BISHOP, 1),
                (piece::PieceEnum::KING, 1),
                (piece::PieceEnum::KNIGHT, 1),
            ],
        );
        assert_attacked_by(
            &attacked_squares[4][1 + modifier],
            vec![
                (piece::PieceEnum::QUEEN, 1),
                (piece::PieceEnum::BISHOP, 1),
                (piece::PieceEnum::KING, 1),
                (piece::PieceEnum::KNIGHT, 1),
            ],
        );
        assert_attacked_by(
            &attacked_squares[5][1 + modifier],
            vec![(piece::PieceEnum::KING, 1)],
        );
        assert_attacked_by(
            &attacked_squares[6][1 + modifier],
            vec![(piece::PieceEnum::BISHOP, 1)],
        );
        assert_attacked_by(
            &attacked_squares[7][1 + modifier],
            vec![(piece::PieceEnum::ROOK, 1)],
        );

        let modifier = if color == color::Color::WHITE { 0 } else { 7 };

        assert_attacked_by(&attacked_squares[0][0 + modifier], vec![]);
        assert_attacked_by(
            &attacked_squares[1][0 + modifier],
            vec![(piece::PieceEnum::ROOK, 1)],
        );
        assert_attacked_by(
            &attacked_squares[2][0 + modifier],
            vec![(piece::PieceEnum::QUEEN, 1)],
        );
        assert_attacked_by(
            &attacked_squares[3][0 + modifier],
            vec![(piece::PieceEnum::KING, 1)],
        );
        assert_attacked_by(
            &attacked_squares[4][0 + modifier],
            vec![(piece::PieceEnum::QUEEN, 1)],
        );
        assert_attacked_by(
            &attacked_squares[5][0 + modifier],
            vec![(piece::PieceEnum::KING, 1)],
        );
        assert_attacked_by(
            &attacked_squares[6][0 + modifier],
            vec![(piece::PieceEnum::ROOK, 1)],
        );
        assert_attacked_by(&attacked_squares[7][0 + modifier], vec![]);
    }

    fn assert_attacked_by(
        actual_attackers: &Vec<&std::boxed::Box<dyn piece::Piece>>,
        attackers: Vec<(piece::PieceEnum, u8)>,
    ) {
        let expected_sum_of_attackers = attackers.iter().fold(0, |acc, attacker| acc + attacker.1);
        assert_eq!(expected_sum_of_attackers as usize, actual_attackers.len());

        for attacker in attackers {
            let attacking_pieces = actual_attackers
                .iter()
                .filter(|v| v.piece() == attacker.0)
                .collect::<Vec<_>>();
            assert_eq!(attacker.1 as usize, attacking_pieces.len());
            for piece in attacking_pieces {
                assert_eq!(attacker.0, piece.piece());
            }
        }
    }
}
