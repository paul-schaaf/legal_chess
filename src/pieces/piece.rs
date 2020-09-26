use super::{piece, position};
use crate::color;

pub trait Piece<'a> {
    fn position(&self) -> &position::Position;

    fn attacks(
        &self,
        board: &'a Vec<Vec<Option<Box<dyn piece::Piece>>>>,
    ) -> Vec<position::Position>;

    fn color(&self) -> &color::Color;
}
