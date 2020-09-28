use super::pieces::position;
use super::pieces::to_piece::ToPiece;
use super::{board, chessmove, color};

pub struct Game {
    board: board::Board,
    en_passant: Option<chessmove::ChessMove>,
    side_to_move: color::Color,
    castling_rights_white: (bool, bool),
    castling_rights_black: (bool, bool),
    half_moves: u16,
    full_moves: u16,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: board::Board::initial(),
            en_passant: None,
            side_to_move: color::Color::WHITE,
            castling_rights_white: (true, true),
            castling_rights_black: (true, true),
            half_moves: 0,
            full_moves: 1,
        }
    }

    pub fn from_game_arr(game_arr: &[&str]) -> Self {
        let mut board = board::Board::empty();
        let mut file = 1;
        let mut rank = 1;
        for i in 0..64 {
            board.set_square(
                game_arr[i].to_piece(i as u8, position::Position(file, rank)),
                position::Position(file, rank),
            );
            if rank == 8 {
                rank = 1;
                file += 1;
            } else {
                rank += 1;
            }
        }
        let en_passant: Option<chessmove::ChessMove> = if game_arr[64] == "-" {
            None
        } else {
            Some(chessmove::ChessMove {
                source_file: game_arr[64].as_bytes()[0],
                source_rank: game_arr[65].as_bytes()[0],
                target_file: game_arr[66].as_bytes()[0],
                target_rank: game_arr[67].as_bytes()[0],
            })
        };
        let castling_rights_white = if game_arr[68] == "1" && game_arr[69] == "1" {
            (true, true)
        } else if game_arr[68] == "1" {
            (true, false)
        } else if game_arr[69] == "1" {
            (false, true)
        } else {
            (false, false)
        };

        let castling_rights_black = if game_arr[70] == "1" && game_arr[71] == "1" {
            (true, true)
        } else if game_arr[70] == "1" {
            (true, false)
        } else if game_arr[71] == "1" {
            (false, true)
        } else {
            (false, false)
        };

        let half_moves = match game_arr[72].parse() {
            Ok(v) => v,
            _ => panic!(),
        };

        let full_moves = match game_arr[73].parse() {
            Ok(v) => v,
            _ => panic!(),
        };

        Self {
            board,
            en_passant,
            castling_rights_white,
            castling_rights_black,
            side_to_move: if game_arr[74] == "w" {
                color::Color::WHITE
            } else {
                color::Color::BLACK
            },
            half_moves,
            full_moves,
        }
    }

    pub fn board(&self) -> &board::Board {
        &self.board
    }

    pub fn en_passant(&self) -> &Option<chessmove::ChessMove> {
        &self.en_passant
    }

    pub fn side_to_move(&self) -> &color::Color {
        &self.side_to_move
    }

    pub fn castling_rights_white(&self) -> (bool, bool) {
        self.castling_rights_white
    }

    pub fn castling_rights_black(&self) -> (bool, bool) {
        self.castling_rights_black
    }

    pub fn half_moves(&self) -> u16 {
        self.half_moves
    }

    pub fn full_moves(&self) -> u16 {
        self.full_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_GAME_ARR: [&str; 75] = [
        "R", "P", "-", "-", "-", "-", "p", "r", "N", "P", "-", "-", "-", "-", "p", "n", "B", "P",
        "-", "-", "-", "-", "p", "b", "Q", "P", "-", "-", "-", "-", "p", "q", "K", "P", "-", "-",
        "-", "-", "p", "k", "B", "P", "-", "-", "-", "-", "p", "b", "N", "P", "-", "-", "-", "-",
        "p", "n", "R", "P", "-", "-", "-", "-", "p", "r", "-", "-", "-", "-", "1", "1", "1", "1",
        "0", "1", "w",
    ];

    #[test]
    fn from_game_arr_initial_board() {
        let actual_game = Game::from_game_arr(&INITIAL_GAME_ARR);
        let actual_board = actual_game.board();
        let actual_board = actual_board.to_string_board();

        let expected_board = board::Board::initial();
        let expected_board = expected_board.to_string_board();

        assert_eq!(expected_board, actual_board);
    }

    #[test]
    fn from_game_arr_initial_en_passant() {
        let actual_game = Game::from_game_arr(&INITIAL_GAME_ARR);

        assert_eq!(None, *actual_game.en_passant());
    }

    #[test]
    fn from_game_arr_initial_side_to_move() {
        let actual_game = Game::from_game_arr(&INITIAL_GAME_ARR);

        assert_eq!(color::Color::WHITE, *actual_game.side_to_move());
    }

    #[test]
    fn from_game_arr_initial_castling_rights() {
        let actual_game = Game::from_game_arr(&INITIAL_GAME_ARR);

        assert_eq!((true, true), actual_game.castling_rights_white());
        assert_eq!((true, true), actual_game.castling_rights_black());
    }

    #[test]
    fn from_game_arr_initial_half_moves() {
        let actual_game = Game::from_game_arr(&INITIAL_GAME_ARR);

        assert_eq!(0, actual_game.half_moves());
    }

    #[test]
    fn from_game_arr_initial_full_moves() {
        let actual_game = Game::from_game_arr(&INITIAL_GAME_ARR);

        assert_eq!(1, actual_game.full_moves());
    }
}
