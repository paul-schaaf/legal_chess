use crate::color;
use crate::pieces::{pawn, piece, position};
use std::collections;

fn get_attacked_squares(
    board: [[Box<dyn piece::Piece>; 8]; 8],
    color: color::Color,
) -> Vec<Vec<Option<AttackedSquare>>> {
    let mut map: collections::HashMap<&str, Vec<Box<dyn piece::Piece>>> =
        collections::HashMap::new();

    let p = pawn::Pawn {
        position: position::Position(2, 2),
        color: color::Color::WHITE,
    };

    match map.get_mut("01") {
        None => {
            map.insert("01", vec![Box::new(p)]);
        }
        Some(v) => v.push(Box::new(p)),
    }

    for file in &board {
        for piece in file {
            if *piece.color() == color {}
        }
    }

    vec![]
}

pub struct AttackedSquare {
    pub attackers: Vec<Box<dyn piece::Piece>>,
}
