use super::position;
use crate::{board, color};
use std::fmt;

pub trait Piece: fmt::Debug {
    fn position(&self) -> &position::Position;

    fn attacks(&self, board: &board::Board) -> Vec<position::Position>;

    fn color(&self) -> &color::Color;

    fn get_id(&self) -> u8;

    fn piece(&self) -> PieceEnum;

    fn moves(&self, board: &board::Board) -> Vec<position::Position>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PieceEnum {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
}
