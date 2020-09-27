use super::pieces::{king, pawn, piece, position, queen, rook};
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
                board[knight.0 - 1][knight.1 - 1 + modifier] = Some(Box::new(rook::Rook {
                    id: id_count,
                    color,
                    position: position::Position(knight.0 as u8, knight.1 as u8 + modifier as u8),
                }));
                id_count += 1;
            }

            let bishop_positions = [(3, 1), (6, 1)];

            for bishop in &bishop_positions {
                board[bishop.0 - 1][bishop.1 - 1 + modifier] = Some(Box::new(rook::Rook {
                    id: id_count,
                    color,
                    position: position::Position(bishop.0 as u8, bishop.1 as u8 + modifier as u8),
                }));
                id_count += 1;
            }

            board[3][0 + modifier] = Some(Box::new(queen::Queen {
                id: id_count,
                color,
                position: position::Position(4, 1 + modifier as u8),
            }));

            board[4][0 + modifier] = Some(Box::new(king::King {
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
            let (modifier, color) = if k == 0 {(0, color::Color::WHITE)} else {(5, color::Color::BLACK)};
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

    }
}
