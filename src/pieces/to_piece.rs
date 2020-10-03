use super::{bishop, king, knight, pawn, piece, position, queen, rook};
use crate::color;

pub trait ToPiece {
    fn to_piece(&self, position: position::Position) -> Option<Box<dyn piece::Piece>>;
}

impl ToPiece for &str {
    fn to_piece(&self, position: position::Position) -> Option<Box<dyn piece::Piece>> {
        match *self {
            "-" => None,
            "p" => Some(Box::new(pawn::Pawn {
                color: color::Color::BLACK,
                position,
            })),
            "P" => Some(Box::new(pawn::Pawn {
                color: color::Color::WHITE,
                position,
            })),
            "r" => Some(Box::new(rook::Rook {
                color: color::Color::BLACK,
                position,
            })),
            "R" => Some(Box::new(rook::Rook {
                color: color::Color::WHITE,
                position,
            })),
            "n" => Some(Box::new(knight::Knight {
                color: color::Color::BLACK,
                position,
            })),
            "N" => Some(Box::new(knight::Knight {
                color: color::Color::WHITE,
                position,
            })),
            "b" => Some(Box::new(bishop::Bishop {
                color: color::Color::BLACK,
                position,
            })),
            "B" => Some(Box::new(bishop::Bishop {
                color: color::Color::WHITE,
                position,
            })),
            "q" => Some(Box::new(queen::Queen {
                color: color::Color::BLACK,
                position,
            })),
            "Q" => Some(Box::new(queen::Queen {
                color: color::Color::WHITE,
                position,
            })),
            "k" => Some(Box::new(king::King {
                color: color::Color::BLACK,
                position,
            })),
            "K" => Some(Box::new(king::King {
                color: color::Color::WHITE,
                position,
            })),
            _ => panic!("Invalid piece string"),
        }
    }
}
