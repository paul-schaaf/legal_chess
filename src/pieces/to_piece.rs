use super::{bishop, king, knight, pawn, piece, position, queen, rook};
use crate::color;

pub trait ToPiece {
    fn to_piece(&self, position: position::Position) -> Option<Box<dyn piece::Piece>>;
}

impl ToPiece for u8 {
    fn to_piece(&self, position: position::Position) -> Option<Box<dyn piece::Piece>> {
        match *self {
            0 => None,
            11 => Some(Box::new(pawn::Pawn {
                color: color::Color::BLACK,
                position,
            })),
            1 => Some(Box::new(pawn::Pawn {
                color: color::Color::WHITE,
                position,
            })),
            12 => Some(Box::new(rook::Rook {
                color: color::Color::BLACK,
                position,
            })),
            2 => Some(Box::new(rook::Rook {
                color: color::Color::WHITE,
                position,
            })),
            13 => Some(Box::new(knight::Knight {
                color: color::Color::BLACK,
                position,
            })),
            3 => Some(Box::new(knight::Knight {
                color: color::Color::WHITE,
                position,
            })),
            14 => Some(Box::new(bishop::Bishop {
                color: color::Color::BLACK,
                position,
            })),
            4 => Some(Box::new(bishop::Bishop {
                color: color::Color::WHITE,
                position,
            })),
            15 => Some(Box::new(queen::Queen {
                color: color::Color::BLACK,
                position,
            })),
            5 => Some(Box::new(queen::Queen {
                color: color::Color::WHITE,
                position,
            })),
            16 => Some(Box::new(king::King {
                color: color::Color::BLACK,
                position,
            })),
            6 => Some(Box::new(king::King {
                color: color::Color::WHITE,
                position,
            })),
            _ => panic!("Invalid piece number"),
        }
    }
}
