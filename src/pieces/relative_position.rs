use super::position;

#[derive(PartialEq, Debug)]
pub enum RelativePosition {
    Up,
    UpRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    UpLeft,
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    STRAIGHT(RelativePosition),
    DIAGONAL(RelativePosition),
}

pub fn get_line_to_other_piece(
    self_position: &position::Position,
    other_position: position::Position,
) -> Option<((i8, i8), Direction)> {
    if self_position.0 == other_position.0 {
        if self_position.1 > other_position.1 {
            Some(((0, -1), Direction::STRAIGHT(RelativePosition::Bottom)))
        } else {
            Some(((0, 1), Direction::STRAIGHT(RelativePosition::Up)))
        }
    } else if self_position.1 == other_position.1 {
        if self_position.0 > other_position.0 {
            Some(((-1, 0), Direction::STRAIGHT(RelativePosition::Left)))
        } else {
            Some(((1, 0), Direction::STRAIGHT(RelativePosition::Right)))
        }
    } else {
        let diff_file = self_position.0 as i8 - other_position.0 as i8;
        let diff_rank = self_position.1 as i8 - other_position.1 as i8;
        if diff_file.abs() == diff_rank.abs() {
            if self_position.0 < other_position.0 {
                if self_position.1 < other_position.1 {
                    Some(((1, 1), Direction::DIAGONAL(RelativePosition::UpRight)))
                } else {
                    Some(((1, -1), Direction::DIAGONAL(RelativePosition::BottomRight)))
                }
            } else if self_position.1 < other_position.1 {
                Some(((-1, 1), Direction::DIAGONAL(RelativePosition::UpLeft)))
            } else {
                Some(((-1, -1), Direction::DIAGONAL(RelativePosition::BottomLeft)))
            }
        } else {
            None
        }
    }
}
