use super::{piece, position};
use crate::{board, color};
use std::fmt;

pub trait Piece: fmt::Debug {
    fn position(&self) -> &position::Position;

    fn attacks(&self, board: &board::Board) -> Vec<position::Position>;

    fn color(&self) -> &color::Color;

    fn get_id(&self) -> u8;
}
