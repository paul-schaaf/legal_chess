use crate::color;
use crate::pieces::{piece};

fn get_attacked_squares(
    board: Vec<Vec<Option<Box<dyn piece::Piece>>>>,
    color: color::Color,
) -> Vec<Vec<Option<Vec<Box<dyn piece::Piece>>>>> {
    if board.len() != 8 || board.iter().any(|x| x.len() != 8) { 
        panic!("Invalid board dimensions");
    }
    

    vec![]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Invalid board dimensions")]
    fn invalid_file_size() {
        get_attacked_squares(vec!(), color::Color::BLACK);
    }

    #[test]
    #[should_panic(expected = "Invalid board dimensions")]
    fn invalid_rank_size() {
        get_attacked_squares(vec!(vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!()), color::Color::BLACK);
    }
}
