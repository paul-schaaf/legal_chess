use super::{piece, position};
use crate::color;

pub trait Piece {
    fn position(&self) -> &position::Position;

    fn attacks(&self, board: &Vec<Vec<Option<Box<dyn piece::Piece>>>>) -> Vec<position::Position>;

    fn color(&self) -> &color::Color;
}
