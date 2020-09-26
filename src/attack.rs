use crate::color;
use crate::pieces::piece;

pub fn get_attacked_squares(
    board: &Vec<Vec<Option<Box<dyn piece::Piece>>>>,
    color: color::Color,
) -> Vec<Vec<Option<Vec<&Box<dyn piece::Piece>>>>> {
    if board.len() != 8 || board.iter().any(|x| x.len() != 8) {
        panic!("Invalid board dimensions");
    }

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
                println!("Found a piece!");
                let attacked_positions = piece.attacks(board);
                attacked_positions.iter().for_each(|position| {
                    println!("Found an attacked position");
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
    #[should_panic(expected = "Invalid board dimensions")]
    fn invalid_input_board_file_size() {
        get_attacked_squares(&vec![], color::Color::BLACK);
    }

    #[test]
    #[should_panic(expected = "Invalid board dimensions")]
    fn invalid_input_board_rank_size() {
        get_attacked_squares(
            &vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            color::Color::BLACK,
        );
    }

    #[test]
    fn valid_attacked_board_size() {
        let mut empty_board: Vec<Vec<Option<Box<dyn piece::Piece>>>> = vec![];
        for _ in 0..8 {
            let mut empty_file: Vec<Option<Box<dyn piece::Piece>>> = vec![];
            for _ in 0..8 {
                empty_file.push(None);
            }
            empty_board.push(empty_file);
        }

        let attacked_board = get_attacked_squares(&empty_board, color::Color::BLACK);
        assert_eq!(8, attacked_board.len());
        assert!(attacked_board.iter().all(|file| file.len() == 8));
    }

    #[test]
    fn correct_attacked_board() {
        let mut empty_board: Vec<Vec<Option<Box<dyn piece::Piece>>>> = vec![];
        for _ in 0..8 {
            let mut empty_file: Vec<Option<Box<dyn piece::Piece>>> = vec![];
            for _ in 0..8 {
                empty_file.push(None);
            }
            empty_board.push(empty_file);
        }

        let pawn_id = 1;
        let pawn = pawn::Pawn {
            id: pawn_id,
            position: position::Position(1, 2),
            color: color::Color::WHITE,
        };

        empty_board[0][1] = Some(Box::new(pawn));

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
