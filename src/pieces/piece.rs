use super::{piece, position};
use crate::color;
use std::fmt;

pub trait Piece: fmt::Debug {
    fn position(&self) -> &position::Position;

    fn attacks(&self, board: &Vec<Vec<Option<Box<dyn piece::Piece>>>>) -> Vec<position::Position>;

    fn color(&self) -> &color::Color;

    fn get_id(&self) -> u8;
}
