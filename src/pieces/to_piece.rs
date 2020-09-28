use super::{bishop, king, knight, pawn, piece, position, queen, rook};
use crate::color;

pub trait ToPiece {
    fn to_piece(&self, id: u8, position: position::Position) -> Option<Box<dyn piece::Piece>>;
}

impl ToPiece for &str {
    fn to_piece(&self, id: u8, position: position::Position) -> Option<Box<dyn piece::Piece>> {
        match self {
            &"-" => None,
            &"p" => Some(Box::new(pawn::Pawn {
                id,
                color: color::Color::BLACK,
                position,
            })),
            &"P" => Some(Box::new(pawn::Pawn {
                id,
                color: color::Color::WHITE,
                position,
            })),
            &"r" => Some(Box::new(rook::Rook {
                id,
                color: color::Color::BLACK,
                position,
            })),
            &"R" => Some(Box::new(rook::Rook {
                id,
                color: color::Color::WHITE,
                position,
            })),
            &"n" => Some(Box::new(knight::Knight {
                id,
                color: color::Color::BLACK,
                position,
            })),
            &"N" => Some(Box::new(knight::Knight {
                id,
                color: color::Color::WHITE,
                position,
            })),
            &"b" => Some(Box::new(bishop::Bishop {
                id,
                color: color::Color::BLACK,
                position,
            })),
            &"B" => Some(Box::new(bishop::Bishop {
                id,
                color: color::Color::WHITE,
                position,
            })),
            &"q" => Some(Box::new(queen::Queen {
                id,
                color: color::Color::BLACK,
                position,
            })),
            &"Q" => Some(Box::new(queen::Queen {
                id,
                color: color::Color::WHITE,
                position,
            })),
            &"k" => Some(Box::new(king::King {
                id,
                color: color::Color::BLACK,
                position,
            })),
            &"K" => Some(Box::new(king::King {
                id,
                color: color::Color::WHITE,
                position,
            })),
            _ => panic!("Invalid piece string"),
        }
    }
}
