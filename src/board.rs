use super::pieces::{piece, position};
use std::slice;

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
