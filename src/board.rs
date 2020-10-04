use super::pieces::to_piece::ToPiece;
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

        for k in 0..2 {
            let (modifier, color) = if k == 0 {
                (0, color::Color::WHITE)
            } else {
                (5, color::Color::BLACK)
            };
            for i in 0..8 {
                board[i][1 + modifier] = Some(Box::new(pawn::Pawn {
                    color,
                    position: position::Position(i as u8 + 1, 2 + modifier as u8),
                }));
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
                    color,
                    position: position::Position(rook.0 as u8, rook.1 as u8 + modifier as u8),
                }));
            }

            let knight_positions = [(2, 1), (7, 1)];

            for knight in &knight_positions {
                board[knight.0 - 1][knight.1 - 1 + modifier] = Some(Box::new(knight::Knight {
                    color,
                    position: position::Position(knight.0 as u8, knight.1 as u8 + modifier as u8),
                }));
            }

            let bishop_positions = [(3, 1), (6, 1)];

            for bishop in &bishop_positions {
                board[bishop.0 - 1][bishop.1 - 1 + modifier] = Some(Box::new(bishop::Bishop {
                    color,
                    position: position::Position(bishop.0 as u8, bishop.1 as u8 + modifier as u8),
                }));
            }

            board[3][modifier] = Some(Box::new(queen::Queen {
                color,
                position: position::Position(4, 1 + modifier as u8),
            }));

            board[4][modifier] = Some(Box::new(king::King {
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

    pub fn take_piece(&mut self, position: position::Position) -> Box<dyn piece::Piece> {
        let piece = self.board[position.0 as usize - 1][position.1 as usize - 1].take();

        match piece {
            None => panic!("No piece at position: {:?}", position),
            Some(piece) => piece,
        }
    }

    pub fn set_square(
        &mut self,
        square: Option<Box<dyn piece::Piece>>,
        position: position::Position,
    ) {
        self.board[position.0 as usize - 1][position.1 as usize - 1] = square;
    }

    pub fn to_string_board<'a, 'b>(&'a self) -> [&'b str; 64] {
        let mut string_board = ["-"; 64];

        let mut index = 0;

        for file in &self.board {
            for square in file {
                match square {
                    None => string_board[index] = "-",
                    Some(piece) => match (*piece.color(), piece.piece()) {
                        (color::Color::WHITE, piece::PieceEnum::PAWN) => string_board[index] = "P",
                        (color::Color::BLACK, piece::PieceEnum::PAWN) => string_board[index] = "p",
                        (color::Color::WHITE, piece::PieceEnum::ROOK) => string_board[index] = "R",
                        (color::Color::BLACK, piece::PieceEnum::ROOK) => string_board[index] = "r",
                        (color::Color::WHITE, piece::PieceEnum::KNIGHT) => {
                            string_board[index] = "N"
                        }
                        (color::Color::BLACK, piece::PieceEnum::KNIGHT) => {
                            string_board[index] = "n"
                        }
                        (color::Color::WHITE, piece::PieceEnum::BISHOP) => {
                            string_board[index] = "B"
                        }
                        (color::Color::BLACK, piece::PieceEnum::BISHOP) => {
                            string_board[index] = "b"
                        }
                        (color::Color::WHITE, piece::PieceEnum::QUEEN) => string_board[index] = "Q",
                        (color::Color::BLACK, piece::PieceEnum::QUEEN) => string_board[index] = "q",
                        (color::Color::WHITE, piece::PieceEnum::KING) => string_board[index] = "K",
                        (color::Color::BLACK, piece::PieceEnum::KING) => string_board[index] = "k",
                    },
                };
                index += 1;
            }
        }
        string_board
    }

    pub fn from_string_board(
        board_arr: &[&str; 64],
    ) -> (Self, position::Position, position::Position) {
        let mut board = Self::empty();
        let mut file = 1;
        let mut rank = 1;
        let mut white_king = position::Position(0, 0);
        let mut black_king = position::Position(0, 0);

        for string_piece in board_arr.iter().take(64) {
            let square = string_piece.to_piece(position::Position(file, rank));
            if let Some(p) = &square {
                match (p.color(), p.piece()) {
                    (color::Color::WHITE, piece::PieceEnum::KING) => {
                        white_king = position::Position(file, rank)
                    }
                    (color::Color::BLACK, piece::PieceEnum::KING) => {
                        black_king = position::Position(file, rank)
                    }
                    (_, _) => (),
                }
            }
            board.set_square(square, position::Position(file, rank));
            if rank == 8 {
                rank = 1;
                file += 1;
            } else {
                rank += 1;
            }
        }

        (board, white_king, black_king)
    }

    pub fn pieces_of_color_except_king(&self, color: color::Color) -> Vec<&Box<dyn piece::Piece>> {
        let mut pieces = vec![];
        for file in self.board.iter() {
            for square in file.iter() {
                if let Some(piece) = square {
                    if *piece.color() == color && piece.piece() != piece::PieceEnum::KING {
                        pieces.push(piece);
                    }
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
                if let Some(_) = initial_board.get_square(position::Position(i + 1, j + 1)) {
                    panic!("Should've been: None");
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
