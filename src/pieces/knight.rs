use super::{piece,position};
use crate::color;

pub struct Knight {
    pub position: position::Position,
    pub color: color::Color,
}

impl piece::Piece for Knight {
    fn attacks(
        &self,
        board: &Vec<Vec<Option<Box<dyn piece::Piece>>>>,
    ) -> Vec<position::Position> {

        let positions: [[i8;2];8] = [[1,2],[1,-2],[-1,2],[-1,-2],[2,1],[2, -1],[-2,1],[-2,-1]];

        let mut attacks = Vec::new();

        for i in 0..8 {
            let file = positions[i][0]+ self.position.0 as i8;
            let rank =  positions[i][1] + self.position.1 as i8;

            if file >= 1 && file <= 8 && rank >= 1 && rank <= 8 {
                attacks.push(position::Position(file as u8, rank as u8))
            }
        }

        attacks

    }

    fn position(&self) -> &position::Position {
        &self.position
    }

    fn color(&self) -> &color::Color {
        &self.color
    }

}

impl Knight{
    fn is_valid_position(&self, pos: &position::Position) -> bool{
        pos.0 >= 1 && pos.0 <= 8 && pos.1 >= 1 && pos.1 <= 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use piece::Piece;

    #[test]
    fn attrs() {
        let k = Knight {
            position: position::Position(2, 2),
            color: color::Color::WHITE,
        };
        assert_eq!(&position::Position(2, 2), k.position());
        assert_eq!(&color::Color::WHITE, k.color());
    }
}