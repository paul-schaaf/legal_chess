use super::pieces::to_piece::ToPiece;
use super::pieces::{piece, position};
use super::{attack, board, chessmove, color};

pub struct Game {
    board: board::Board,
    en_passant: Option<chessmove::ChessMove>,
    side_to_move: color::Color,
    castling_rights_white: (bool, bool),
    castling_rights_black: (bool, bool),
    half_moves: u16,
    full_moves: u16,
    white_king: position::Position,
    black_king: position::Position,
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
            white_king: position::Position(5, 1),
            black_king: position::Position(5, 8),
        }
    }

    pub fn from_game_arr(game_arr: &[&str]) -> Self {
        let mut board = board::Board::empty();
        let mut file = 1;
        let mut rank = 1;
        let mut white_king = position::Position(0, 0);
        let mut black_king = position::Position(0, 0);

        for (i, string_piece) in game_arr.iter().enumerate().take(64) {
            let square = string_piece.to_piece(i as u8, position::Position(file, rank));
            if let Some(p) = &square {
                match (p.color(), p.piece()) {
                    (color::Color::WHITE, piece::PieceEnum::KING) => {
                        white_king = position::Position(file, rank)
                    }
                    (color::Color::BLACK, piece::PieceEnum::KING) => {
                        black_king = position::Position(file, rank)
                    }
                    (_, _) => (),
                }
            }
            board.set_square(square, position::Position(file, rank));
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
            white_king,
            black_king,
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

    pub fn current_king_position(&self) -> position::Position {
        match self.side_to_move() {
            color::Color::WHITE => self.white_king,
            color::Color::BLACK => self.black_king,
        }
    }

    pub fn legal_moves(&self) -> Vec<chessmove::ChessMove> {
        let other_side = match self.side_to_move() {
            color::Color::WHITE => color::Color::BLACK,
            color::Color::BLACK => color::Color::WHITE,
        };
        let attacked_board = attack::get_attacked_squares(self.board(), other_side);
        let king_position = self.current_king_position();
        let attacked_king_square =
            &attacked_board[king_position.0 as usize - 1][king_position.1 as usize - 1];
        match attacked_king_square {
            Some(v) => {
                let king = match self.board().get_square(king_position) {
                    Some(king) => king,
                    _ => panic!(),
                };
                if v.len() > 1 {
                    self.king_moves(king, &attacked_board)
                } else {
                    self.king_moves(king, &attacked_board)
                }
            }
            None => {
                let mut moves: Vec<chessmove::ChessMove> = vec![];
                for piece in self.board.pieces_of_color(*self.side_to_move()) {
                    moves.append(
                        &mut (piece
                            .moves(&self.board, king_position)
                            .iter()
                            .map(|pos| chessmove::ChessMove {
                                source_file: piece.position().0,
                                source_rank: piece.position().1,
                                target_file: pos.0,
                                target_rank: pos.1,
                            })
                            .collect::<Vec<_>>()),
                    );
                }
                moves
            }
        }
    }

    fn king_moves(
        &self,
        king: &Box<dyn piece::Piece>,
        attacked_board: &std::vec::Vec<
            std::vec::Vec<std::option::Option<std::vec::Vec<&std::boxed::Box<dyn piece::Piece>>>>,
        >,
    ) -> Vec<chessmove::ChessMove> {
        let moves = king.moves(self.board(), *king.position());
        moves
            .iter()
            .filter(|mv| attacked_board[mv.0 as usize - 1][mv.1 as usize - 1].is_none())
            .map(|mv| chessmove::ChessMove {
                source_file: king.position().0,
                source_rank: king.position().1,
                target_file: mv.0,
                target_rank: mv.1,
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::super::pieces::{bishop, king, knight, queen, rook};
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

    #[test]
    fn two_attackers_king_can_capture() {
        let mut game = Game::new();
        game.board = board::Board::empty();
        game.white_king = position::Position(1, 1);

        let white_king_pos = position::Position(1, 1);
        let white_king = king::King {
            id: 1,
            color: color::Color::WHITE,
            position: white_king_pos,
        };
        game.board
            .set_square(Some(Box::new(white_king)), white_king_pos);

        let black_rook_pos = position::Position(8, 1);
        let black_rook = rook::Rook {
            id: 2,
            color: color::Color::BLACK,
            position: black_rook_pos,
        };
        game.board
            .set_square(Some(Box::new(black_rook)), black_rook_pos);

        let black_queen_pos = position::Position(2, 2);
        let black_queen = queen::Queen {
            id: 3,
            color: color::Color::BLACK,
            position: black_queen_pos,
        };
        game.board
            .set_square(Some(Box::new(black_queen)), black_queen_pos);

        let moves = game.legal_moves();
        assert_eq!(1, moves.len());
        assert_eq!(
            chessmove::ChessMove {
                source_file: 1,
                source_rank: 1,
                target_file: 2,
                target_rank: 2
            },
            moves[0]
        );
    }

    #[test]
    fn two_attackers_king_cannot_capture() {
        let mut game = Game::new();
        game.board = board::Board::empty();
        game.white_king = position::Position(1, 1);

        let white_king_pos = position::Position(1, 1);
        let white_king = king::King {
            id: 1,
            color: color::Color::WHITE,
            position: white_king_pos,
        };
        game.board
            .set_square(Some(Box::new(white_king)), white_king_pos);

        let black_rook_pos = position::Position(8, 1);
        let black_rook = rook::Rook {
            id: 2,
            color: color::Color::BLACK,
            position: black_rook_pos,
        };
        game.board
            .set_square(Some(Box::new(black_rook)), black_rook_pos);

        let black_queen_pos = position::Position(2, 2);
        let black_queen = queen::Queen {
            id: 3,
            color: color::Color::BLACK,
            position: black_queen_pos,
        };
        game.board
            .set_square(Some(Box::new(black_queen)), black_queen_pos);

        let black_knight_pos = position::Position(4, 3);
        let black_knight = knight::Knight {
            id: 4,
            color: color::Color::BLACK,
            position: black_knight_pos,
        };
        game.board
            .set_square(Some(Box::new(black_knight)), black_knight_pos);

        let moves = game.legal_moves();
        assert_eq!(0, moves.len());
    }

    #[test]
    fn scholars_mate() {
        let mut game = Game::new();

        let white_queen_pos = position::Position(6, 7);
        let white_queen = queen::Queen {
            id: 100,
            color: color::Color::WHITE,
            position: white_queen_pos,
        };
        game.board
            .set_square(Some(Box::new(white_queen)), white_queen_pos);

        let white_bishop_pos = position::Position(3, 4);
        let white_bishop = bishop::Bishop {
            id: 100,
            color: color::Color::WHITE,
            position: white_bishop_pos,
        };
        game.board
            .set_square(Some(Box::new(white_bishop)), white_bishop_pos);

        game.side_to_move = color::Color::BLACK;

        let moves = game.legal_moves();
        assert_eq!(0, moves.len());
    }

    #[test]
    fn initial_game_setup_legal_moves() {
        let game = Game::new();

        let moves = game.legal_moves();
        assert_eq!(20, moves.len());
    }
}
