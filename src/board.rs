use super::pieces::{bishop, king, knight, pawn, piece, position, queen, rook};
use crate::color;
use std::slice;

#[derive(Debug)]
pub struct Board {
    board: Vec<Vec<Option<Box<dyn piece::Piece>>>>,
}

impl Board {
    pub fn empty() -> Self {
        let mut empty_board: Vec<Vec<Option<Box<dyn piece::Piece>>>> = vec![];
        for _ in 0..8 {
            let mut empty_file: Vec<Option<Box<dyn piece::Piece>>> = vec![];
            for _ in 0..8 {
                empty_file.push(None);
            }
            empty_board.push(empty_file);
        }
        Self { board: empty_board }
    }

    pub fn initial() -> Self {
        let mut board: Vec<Vec<Option<Box<dyn piece::Piece>>>> = vec![];
        for _ in 0..8 {
            let mut empty_file: Vec<Option<Box<dyn piece::Piece>>> = vec![];
            for _ in 0..8 {
                empty_file.push(None);
            }
            board.push(empty_file);
        }

        let mut id_count = 0;

        for k in 0..2 {
            let (modifier, color) = if k == 0 {
                (0, color::Color::WHITE)
            } else {
                (5, color::Color::BLACK)
            };
            for i in 0..8 {
                board[i][1 + modifier] = Some(Box::new(pawn::Pawn {
                    id: id_count,
                    color,
                    position: position::Position(i as u8 + 1, 2 + modifier as u8),
                }));
                id_count += 1;
            }
        }

        for k in 0..2 {
            let (modifier, color) = if k == 0 {
                (0, color::Color::WHITE)
            } else {
                (7, color::Color::BLACK)
            };

            let rook_positions = [(1, 1), (8, 1)];

            for rook in &rook_positions {
                board[rook.0 - 1][rook.1 - 1 + modifier] = Some(Box::new(rook::Rook {
                    id: id_count,
                    color,
                    position: position::Position(rook.0 as u8, rook.1 as u8 + modifier as u8),
                }));
                id_count += 1;
            }

            let knight_positions = [(2, 1), (7, 1)];

            for knight in &knight_positions {
                board[knight.0 - 1][knight.1 - 1 + modifier] = Some(Box::new(knight::Knight {
                    id: id_count,
                    color,
                    position: position::Position(knight.0 as u8, knight.1 as u8 + modifier as u8),
                }));
                id_count += 1;
            }

            let bishop_positions = [(3, 1), (6, 1)];

            for bishop in &bishop_positions {
                board[bishop.0 - 1][bishop.1 - 1 + modifier] = Some(Box::new(bishop::Bishop {
                    id: id_count,
                    color,
                    position: position::Position(bishop.0 as u8, bishop.1 as u8 + modifier as u8),
                }));
                id_count += 1;
            }

            board[3][modifier] = Some(Box::new(queen::Queen {
                id: id_count,
                color,
                position: position::Position(4, 1 + modifier as u8),
            }));

            board[4][modifier] = Some(Box::new(king::King {
                id: id_count,
                color,
                position: position::Position(5, 1 + modifier as u8),
            }));
        }

        Self { board }
    }

    pub fn iter(&self) -> slice::Iter<Vec<Option<Box<dyn piece::Piece>>>> {
        self.board.iter()
    }

    pub fn get_square(&self, position: position::Position) -> &Option<Box<dyn piece::Piece>> {
        &self.board[position.0 as usize - 1][position.1 as usize - 1]
    }

    pub fn set_square(
        &mut self,
        square: Option<Box<dyn piece::Piece>>,
        position: position::Position,
    ) {
        self.board[position.0 as usize - 1][position.1 as usize - 1] = square;
    }

    pub fn to_string_board(&self) -> [[&str; 8]; 8] {
        let mut string_board = [["-"; 8]; 8];
        for (file_index, file) in string_board.iter_mut().enumerate() {
            for (rank_index, rank) in file.iter_mut().enumerate().take(8) {
                match &self.board[file_index][rank_index] {
                    None => *rank = "-",
                    Some(piece) => match (*piece.color(), piece.piece()) {
                        (color::Color::WHITE, piece::PieceEnum::PAWN) => *rank = "P",
                        (color::Color::BLACK, piece::PieceEnum::PAWN) => *rank = "p",
                        (color::Color::WHITE, piece::PieceEnum::ROOK) => *rank = "R",
                        (color::Color::BLACK, piece::PieceEnum::ROOK) => *rank = "r",
                        (color::Color::WHITE, piece::PieceEnum::KNIGHT) => *rank = "N",
                        (color::Color::BLACK, piece::PieceEnum::KNIGHT) => *rank = "n",
                        (color::Color::WHITE, piece::PieceEnum::BISHOP) => *rank = "B",
                        (color::Color::BLACK, piece::PieceEnum::BISHOP) => *rank = "b",
                        (color::Color::WHITE, piece::PieceEnum::QUEEN) => *rank = "Q",
                        (color::Color::BLACK, piece::PieceEnum::QUEEN) => *rank = "q",
                        (color::Color::WHITE, piece::PieceEnum::KING) => *rank = "K",
                        (color::Color::BLACK, piece::PieceEnum::KING) => *rank = "k",
                    },
                }
            }
        }
        string_board
    }

    pub fn pieces_of_color(&self, color: color::Color) -> Vec<&Box<dyn piece::Piece>> {
        let mut pieces = vec![];
        for file in self.board.iter() {
            for square in file.iter() {
                match square {
                    Some(piece) => {
                        if *piece.color() == color {
                            pieces.push(piece);
                        }
                    }
                    None => (),
                }
            }
        }
        pieces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial() {
        let initial_board = Board::initial();
        for i in 0..8 {
            for j in 2..6 {
                match *initial_board.get_square(position::Position(i + 1, j + 1)) {
                    Some(_) => panic!("Should've been: None"),
                    _ => (),
                }
            }
        }

        for k in 0..2 {
            let (modifier, color) = if k == 0 {
                (0, color::Color::WHITE)
            } else {
                (5, color::Color::BLACK)
            };
            for i in 0..8 {
                match initial_board.get_square(position::Position(i + 1, 2 + modifier)) {
                    Some(piece) => {
                        assert_eq!(piece::PieceEnum::PAWN, piece.piece());
                        assert_eq!(color, *piece.color());
                    }
                    _ => panic!("Should've been a pawn"),
                }
            }
        }

        for k in 0..2 {
            let (modifier, color) = if k == 0 {
                (0, color::Color::WHITE)
            } else {
                (7, color::Color::BLACK)
            };

            let rook_positions = [(1, 1), (8, 1)];

            for rook in &rook_positions {
                match initial_board.get_square(position::Position(rook.0, rook.1 + modifier)) {
                    Some(piece) => {
                        assert_eq!(piece::PieceEnum::ROOK, piece.piece());
                        assert_eq!(color, *piece.color());
                    }
                    _ => panic!("Should've been a rook"),
                }
            }

            let knight_positions = [(2, 1), (7, 1)];

            for knight in &knight_positions {
                match initial_board.get_square(position::Position(knight.0, knight.1 + modifier)) {
                    Some(piece) => {
                        assert_eq!(piece::PieceEnum::KNIGHT, piece.piece());
                        assert_eq!(color, *piece.color());
                    }
                    _ => panic!("Should've been a knight"),
                }
            }

            let bishop_positions = [(3, 1), (6, 1)];

            for bishop in &bishop_positions {
                match initial_board.get_square(position::Position(bishop.0, bishop.1 + modifier)) {
                    Some(piece) => {
                        assert_eq!(piece::PieceEnum::BISHOP, piece.piece());
                        assert_eq!(color, *piece.color());
                    }
                    _ => panic!("Should've been a bishop"),
                }
            }

            match initial_board.get_square(position::Position(4, 1 + modifier)) {
                Some(piece) => {
                    assert_eq!(piece::PieceEnum::QUEEN, piece.piece());
                    assert_eq!(color, *piece.color());
                }
                _ => panic!("Should've been a queen"),
            }

            match initial_board.get_square(position::Position(5, 1 + modifier)) {
                Some(piece) => {
                    assert_eq!(piece::PieceEnum::KING, piece.piece());
                    assert_eq!(color, *piece.color());
                }
                _ => panic!("Should've been a king"),
            }
        }
    }
}
