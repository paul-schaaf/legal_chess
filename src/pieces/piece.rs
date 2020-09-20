use super::position;
use crate::color;

pub trait Piece {
    fn position(&self) -> &position::Position;

    fn attacks(&self, board: [[&str; 8]; 8]) -> Vec<position::Position>;

    fn color(&self) -> &color::Color;
}
