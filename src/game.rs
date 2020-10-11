use super::pieces::{piece, position, relative_position};
use super::{attack, attack::AttackedBoard, board, chessmove, color};

pub struct PreviousGameState {
    en_passant: Option<position::Position>,
    side_to_move: color::Color,
    castling_rights_white: (bool, bool),
    castling_rights_black: (bool, bool),
    half_moves: u16,
    full_moves: u16,
    u8_board: [u8; 64],
}

pub struct Game {
    board: board::Board,
    en_passant: Option<position::Position>,
    side_to_move: color::Color,
    castling_rights_white: (bool, bool),
    castling_rights_black: (bool, bool),
    half_moves: u16,
    full_moves: u16,
    white_king: position::Position,
    black_king: position::Position,
    previous_game_states: Vec<PreviousGameState>,
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
            previous_game_states: vec![],
        }
    }

    pub fn undo_last_move(&mut self) {
        if self.previous_game_states.is_empty() {
            panic!("No more previous game states");
        }

        let previous_game_state = match self.previous_game_states.pop() {
            None => panic!("No more previous game states"),
            Some(v) => v,
        };

        self.castling_rights_black = previous_game_state.castling_rights_black;
        self.castling_rights_white = previous_game_state.castling_rights_white;
        self.en_passant = previous_game_state.en_passant;
        self.full_moves = previous_game_state.full_moves;
        self.half_moves = previous_game_state.half_moves;
        self.side_to_move = previous_game_state.side_to_move;

        let (board, white_king, black_king) =
            board::Board::from_u8_board(&previous_game_state.u8_board);
        self.board = board;
        self.white_king = white_king;
        self.black_king = black_king;
    }

    pub fn make_move(&mut self, mv: chessmove::ChessMove) {
        if !self.legal_moves().contains(&mv) {
            panic!("Not a legal move");
        }

        self.add_previous_game_state();

        let piece = self
            .board
            .take_piece(position::Position((mv.from).0, (mv.from).1));

        if piece.piece() == piece::PieceEnum::PAWN {
            match ((mv.from).1, (mv.to).1) {
                (2, 4) => self.en_passant = Some(position::Position((mv.to).0, 3)),
                (7, 5) => self.en_passant = Some(position::Position((mv.to).0, 6)),
                (_, _) => {
                    if let Some(ep) = self.en_passant {
                        if ep == position::Position((mv.to).0, (mv.to).1) {
                            match self.side_to_move {
                                color::Color::WHITE => {
                                    self.remove_piece(position::Position((mv.to).0, 5))
                                }
                                color::Color::BLACK => {
                                    self.remove_piece(position::Position((mv.to).0, 4))
                                }
                            };
                        }
                    }
                    self.en_passant = None
                }
            }
        } else {
            self.en_passant = None;
        }

        if piece.piece() == piece::PieceEnum::KING {
            match self.side_to_move {
                color::Color::WHITE => {
                    self.white_king = position::Position((mv.to).0, (mv.to).1);
                    self.castling_rights_white = (false, false);
                }
                color::Color::BLACK => {
                    self.black_king = position::Position((mv.to).0, (mv.to).1);
                    self.castling_rights_black = (false, false);
                }
            }

            let castle_kingside = (mv.from).0 + 2 == (mv.to).0;
            let castle_queenside = (mv.from).0 as i8 - 2 == (mv.to).0 as i8;

            if castle_kingside {
                self.move_piece(
                    position::Position((mv.from).0 + 3, (mv.from).1),
                    position::Position((mv.from).0 + 1, (mv.to).1),
                );
            } else if castle_queenside {
                self.move_piece(
                    position::Position((mv.from).0 - 4, (mv.from).1),
                    position::Position((mv.from).0 - 1, (mv.to).1),
                );
            }
        }

        match ((mv.from).0, (mv.from).1) {
            (1, 8) => self.castling_rights_black = (self.castling_rights_black.0, false),
            (8, 8) => self.castling_rights_black = (false, self.castling_rights_black.1),
            (1, 1) => self.castling_rights_white = (self.castling_rights_white.0, false),
            (8, 1) => self.castling_rights_white = (false, self.castling_rights_white.1),
            (_, _) => (),
        }

        match ((mv.to).0, (mv.to).1) {
            (1, 8) => self.castling_rights_black = (self.castling_rights_black.0, false),
            (8, 8) => self.castling_rights_black = (false, self.castling_rights_black.1),
            (1, 1) => self.castling_rights_white = (self.castling_rights_white.0, false),
            (8, 1) => self.castling_rights_white = (false, self.castling_rights_white.1),
            (_, _) => (),
        }

        let position = position::Position((mv.to).0, (mv.to).1);

        let mut piece = match mv.promotion {
            None => piece,
            Some(promotion_piece) => {
                piece::promotion_piece_to_piece(promotion_piece, self.side_to_move, position)
            }
        };
        piece.set_position(&position);
        self.board.set_square(Some(piece), position);

        match self.side_to_move {
            color::Color::BLACK => {
                self.full_moves += 1;
                self.side_to_move = color::Color::WHITE;
            }
            color::Color::WHITE => {
                self.side_to_move = color::Color::BLACK;
            }
        }
    }

    fn add_previous_game_state(&mut self) {
        self.previous_game_states.push(PreviousGameState {
            u8_board: self.board.to_u8_board(),
            castling_rights_black: self.castling_rights_black,
            castling_rights_white: self.castling_rights_white,
            en_passant: self.en_passant,
            full_moves: self.full_moves,
            half_moves: self.half_moves,
            side_to_move: self.side_to_move,
        });
    }

    fn move_piece(&mut self, from: position::Position, to: position::Position) {
        let mut pc = self.board.take_piece(from);
        let position = to;
        pc.set_position(&position);
        self.board.set_square(Some(pc), position);
    }

    fn remove_piece(&mut self, sqr: position::Position) {
        self.board.take_piece(sqr);
    }

    pub fn from_game_arr(game_arr: &[u8]) -> Self {
        let mut board_slice: [u8; 64] = [0; 64];
        board_slice.copy_from_slice(&game_arr[0..64]);

        let (board, white_king, black_king) = board::Board::from_u8_board(&board_slice);

        let en_passant: Option<position::Position> = if game_arr[64] == 0 {
            None
        } else {
            Some(position::Position(game_arr[64], game_arr[65]))
        };
        let castling_rights_white = if game_arr[66] == 1 && game_arr[67] == 1 {
            (true, true)
        } else if game_arr[66] == 1 {
            (true, false)
        } else if game_arr[67] == 1 {
            (false, true)
        } else {
            (false, false)
        };

        let castling_rights_black = if game_arr[68] == 1 && game_arr[69] == 1 {
            (true, true)
        } else if game_arr[68] == 1 {
            (true, false)
        } else if game_arr[69] == 1 {
            (false, true)
        } else {
            (false, false)
        };

        let half_moves = game_arr[70];
        let full_moves = game_arr[71];

        let side_to_move = if game_arr[72] == 0 {
            color::Color::WHITE
        } else {
            color::Color::BLACK
        };

        Self {
            board,
            en_passant,
            castling_rights_white,
            castling_rights_black,
            side_to_move,
            half_moves: half_moves as u16,
            full_moves: full_moves as u16,
            white_king,
            black_king,
            previous_game_states: vec![],
        }
    }

    pub fn to_game_arr(&self) -> [u8; 73] {
        let mut game_arr = [0; 73];

        game_arr[..64].copy_from_slice(&self.board.to_u8_board());

        match self.en_passant {
            None => (),
            Some(ep) => {
                game_arr[64] = ep.0;
                game_arr[65] = ep.1;
            }
        };

        game_arr[66] = self.castling_rights_white.0 as u8;
        game_arr[67] = self.castling_rights_white.1 as u8;
        game_arr[68] = self.castling_rights_black.0 as u8;
        game_arr[69] = self.castling_rights_black.1 as u8;

        game_arr[70] = self.half_moves as u8;
        game_arr[71] = self.full_moves as u8;

        game_arr[72] = match self.side_to_move {
            color::Color::WHITE => 0,
            color::Color::BLACK => 1,
        };

        game_arr
    }

    pub fn board(&self) -> &board::Board {
        &self.board
    }

    pub fn en_passant(&self) -> &Option<position::Position> {
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
        let attacked_board =
            attack::get_attacked_squares(self.board(), other_side, self.current_king_position());
        let king_position = self.current_king_position();
        let king_square_attackers =
            &attacked_board[king_position.0 as usize - 1][king_position.1 as usize - 1];

        let king = match self.board().get_square(king_position) {
            Some(king) => king,
            _ => panic!("No King found at king position"),
        };

        if king_square_attackers.len() > 1 {
            self.king_moves(king, &attacked_board)
        } else {
            let mut moves: Vec<chessmove::ChessMove> = vec![];
            for piece in self.board.pieces_of_color_except_king(*self.side_to_move()) {
                moves.append(&mut (piece.moves(&self.board, king_position, &self.en_passant)));
            }

            if king_square_attackers.is_empty() {
                moves.append(&mut self.king_moves(king, &attacked_board));
                moves
            } else {
                let attacker = king_square_attackers[0];
                if attacker.piece() == piece::PieceEnum::KNIGHT {
                    moves = moves
                        .into_iter()
                        .filter(|mv| {
                            (mv.to).0 == attacker.position().0 && (mv.to).1 == attacker.position().1
                        })
                        .collect::<Vec<_>>();
                    moves.append(&mut self.king_moves(king, &attacked_board));
                    moves
                } else if attacker.piece() == piece::PieceEnum::PAWN {
                    moves = moves
                        .into_iter()
                        .filter(|mv| {
                            if let Some(en_passant) = self.en_passant {
                                match self
                                    .board()
                                    .get_square(position::Position((mv.from).0, (mv.from).1))
                                {
                                    None => panic!(),
                                    Some(piece) => {
                                        if piece.piece() == piece::PieceEnum::PAWN {
                                            ((mv.to).0 == en_passant.0 && (mv.to).1 == en_passant.1)
                                                || ((mv.to).0 == attacker.position().0
                                                    && (mv.to).1 == attacker.position().1)
                                        } else {
                                            (mv.to).0 == attacker.position().0
                                                && (mv.to).1 == attacker.position().1
                                        }
                                    }
                                }
                            } else {
                                (mv.to).0 == attacker.position().0
                                    && (mv.to).1 == attacker.position().1
                            }
                        })
                        .collect::<Vec<_>>();
                    moves.append(&mut self.king_moves(king, &attacked_board));
                    moves
                } else {
                    let (mover, _) = match relative_position::get_line_to_other_piece(
                        king.position(),
                        attacker.position(),
                    ) {
                        None => panic!(),
                        Some(v) => v,
                    };
                    let mut allowed_positions = vec![];

                    let mut new_file = king.position().0 as i8;
                    let mut new_rank = king.position().1 as i8;

                    loop {
                        new_file += mover.0;
                        new_rank += mover.1;

                        let new_position = position::Position(new_file as u8, new_rank as u8);

                        allowed_positions.push(new_position);
                        if self.board.get_square(new_position).is_some() {
                            break;
                        }
                    }

                    moves = moves
                        .into_iter()
                        .filter(|mv| {
                            allowed_positions.contains(&position::Position((mv.to).0, (mv.to).1))
                        })
                        .collect::<Vec<_>>();

                    moves.append(&mut self.king_moves(king, &attacked_board));
                    moves
                }
            }
        }
    }

    fn king_moves(
        &self,
        king: &Box<dyn piece::Piece>,
        attacked_board: &AttackedBoard,
    ) -> Vec<chessmove::ChessMove> {
        if king.piece() != piece::PieceEnum::KING {
            panic!("Given piece is not a king");
        }

        let moves = king.moves(self.board(), *king.position(), &None);

        let mut moves = moves
            .into_iter()
            .filter(|mv| square_safe(&position::Position((mv.to).0, (mv.to).1), attacked_board))
            .collect::<Vec<_>>();

        if square_under_attack(king.position(), &attacked_board) {
            return moves;
        }

        let castling_rights = match self.side_to_move {
            color::Color::BLACK => self.castling_rights_black,
            color::Color::WHITE => self.castling_rights_white,
        };

        if castling_rights.0 && self.can_castle_kingside(king, attacked_board) {
            moves.push(chessmove::ChessMove {
                from: (king.position().0, king.position().1),
                to: (king.position().0 + 2, king.position().1),
                promotion: None,
            });
        }

        if castling_rights.1 && self.can_castle_queenside(king, attacked_board) {
            moves.push(chessmove::ChessMove {
                from: (king.position().0, king.position().1),
                to: (king.position().0 - 2, king.position().1),
                promotion: None,
            });
        }

        moves
    }

    fn is_empty_square(&self, position: &position::Position) -> bool {
        self.board.get_square(*position).is_none()
    }

    fn can_castle_kingside(
        &self,
        king: &Box<dyn piece::Piece>,
        attacked_board: &AttackedBoard,
    ) -> bool {
        let one_right_of_king = position::Position(king.position().0 + 1, king.position().1);
        let two_right_of_king = position::Position(king.position().0 + 2, king.position().1);

        self.is_empty_square(&one_right_of_king)
            && self.is_empty_square(&two_right_of_king)
            && square_safe(&one_right_of_king, attacked_board)
            && square_safe(&two_right_of_king, attacked_board)
    }

    fn can_castle_queenside(
        &self,
        king: &Box<dyn piece::Piece>,
        attacked_board: &AttackedBoard,
    ) -> bool {
        let one_left_of_king = position::Position(king.position().0 - 1, king.position().1);
        let two_left_of_king = position::Position(king.position().0 - 2, king.position().1);
        let three_left_king = position::Position(king.position().0 - 3, king.position().1);

        self.is_empty_square(&one_left_of_king)
            && self.is_empty_square(&two_left_of_king)
            && self.is_empty_square(&three_left_king)
            && square_safe(&one_left_of_king, attacked_board)
            && square_safe(&two_left_of_king, attacked_board)
    }
}

fn square_safe(
    position: &position::Position,
    attacked_board: &[std::vec::Vec<std::vec::Vec<&std::boxed::Box<dyn piece::Piece>>>],
) -> bool {
    attacked_board[position.0 as usize - 1][position.1 as usize - 1].is_empty()
}

fn square_under_attack(
    position: &position::Position,
    attacked_board: &[std::vec::Vec<std::vec::Vec<&std::boxed::Box<dyn piece::Piece>>>],
) -> bool {
    !square_safe(position, attacked_board)
}

#[cfg(test)]
mod tests {
    use super::super::pieces::{bishop, king, knight, pawn, queen, rook};
    use super::*;

    const INITIAL_GAME_ARR: [u8; 73] = [
        2, 1, 0, 0, 0, 0, 11, 12, 3, 1, 0, 0, 0, 0, 11, 13, 4, 1, 0, 0, 0, 0, 11, 14, 5, 1, 0, 0,
        0, 0, 11, 15, 6, 1, 0, 0, 0, 0, 11, 16, 4, 1, 0, 0, 0, 0, 11, 14, 3, 1, 0, 0, 0, 0, 11, 13,
        2, 1, 0, 0, 0, 0, 11, 12, 0, 0, 1, 1, 1, 1, 0, 1, 0,
    ];

    #[test]
    fn from_game_arr_initial_board() {
        let actual_game = Game::from_game_arr(&INITIAL_GAME_ARR);
        let actual_board = actual_game.board();
        let actual_board = actual_board.to_u8_board();

        let expected_board = board::Board::initial();
        let expected_board = expected_board.to_u8_board();

        for i in 0..64 {
            assert_eq!(expected_board[i], actual_board[i]);
        }
    }

    #[test]
    fn to_game_arr_initial_board() {
        let game = Game::new();
        let game_arr = game.to_game_arr();
        for i in 0..73 {
            assert_eq!(INITIAL_GAME_ARR[i], game_arr[i]);
        }
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
            color: color::Color::WHITE,
            position: white_king_pos,
        };
        game.board
            .set_square(Some(Box::new(white_king)), white_king_pos);

        let black_rook_pos = position::Position(8, 1);
        let black_rook = rook::Rook {
            color: color::Color::BLACK,
            position: black_rook_pos,
        };
        game.board
            .set_square(Some(Box::new(black_rook)), black_rook_pos);

        let black_queen_pos = position::Position(2, 2);
        let black_queen = queen::Queen {
            color: color::Color::BLACK,
            position: black_queen_pos,
        };
        game.board
            .set_square(Some(Box::new(black_queen)), black_queen_pos);

        let moves = game.legal_moves();
        assert_eq!(1, moves.len());
        assert_eq!(
            chessmove::ChessMove {
                from: (1, 1),
                to: (2, 2),
                promotion: None,
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
            color: color::Color::WHITE,
            position: white_king_pos,
        };
        game.board
            .set_square(Some(Box::new(white_king)), white_king_pos);

        let black_rook_pos = position::Position(8, 1);
        let black_rook = rook::Rook {
            color: color::Color::BLACK,
            position: black_rook_pos,
        };
        game.board
            .set_square(Some(Box::new(black_rook)), black_rook_pos);

        let black_queen_pos = position::Position(2, 2);
        let black_queen = queen::Queen {
            color: color::Color::BLACK,
            position: black_queen_pos,
        };
        game.board
            .set_square(Some(Box::new(black_queen)), black_queen_pos);

        let black_knight_pos = position::Position(4, 3);
        let black_knight = knight::Knight {
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
            color: color::Color::WHITE,
            position: white_queen_pos,
        };
        game.board
            .set_square(Some(Box::new(white_queen)), white_queen_pos);

        let white_bishop_pos = position::Position(3, 4);
        let white_bishop = bishop::Bishop {
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

    #[test]
    fn king_attacked_by_slider_cannot_move_back() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        set_piece(
            &mut game.board,
            Box::new(queen::Queen {
                color: color::Color::BLACK,
                position: position::Position(1, 8),
            }),
        );

        let white_king_pos = position::Position(1, 2);

        set_piece(
            &mut game.board,
            Box::new(king::King {
                color: color::Color::WHITE,
                position: white_king_pos,
            }),
        );
        game.white_king = position::Position(1, 2);

        let moves = game.legal_moves();
        assert_eq!(3, moves.len());
        assert!(!moves.contains(&chessmove::ChessMove {
            from: (1, 2),
            to: (1, 1),
            promotion: None
        }));
    }

    #[test]
    fn double_en_passant() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        set_piece(
            &mut game.board,
            Box::new(pawn::Pawn {
                color: color::Color::WHITE,
                position: position::Position(4, 4),
            }),
        );
        set_piece(
            &mut game.board,
            Box::new(pawn::Pawn {
                color: color::Color::BLACK,
                position: position::Position(3, 4),
            }),
        );
        set_piece(
            &mut game.board,
            Box::new(pawn::Pawn {
                color: color::Color::BLACK,
                position: position::Position(5, 4),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(king::King {
                color: color::Color::BLACK,
                position: position::Position(8, 8),
            }),
        );

        game.en_passant = Some(position::Position(4, 3));
        game.black_king = position::Position(8, 8);
        game.side_to_move = color::Color::BLACK;
        game.castling_rights_black = (false, false);

        let actual_legal_moves = game.legal_moves();
        let expected_legal_moves = vec![
            chessmove::ChessMove {
                from: (3, 4),
                to: (3, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (3, 4),
                to: (4, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 4),
                to: (4, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 4),
                to: (5, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (8, 8),
                to: (7, 8),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (8, 8),
                to: (7, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (8, 8),
                to: (8, 7),
                promotion: None,
            },
        ];

        assert_eq!(expected_legal_moves.len(), actual_legal_moves.len());
        for mv in &expected_legal_moves {
            assert!(actual_legal_moves.contains(mv));
        }
    }

    #[test]
    fn king_attacked_by_horse_in_initial_pos() {
        let mut game = Game::new();

        let black_knight_pos = position::Position(4, 3);
        let black_knight = knight::Knight {
            color: color::Color::BLACK,
            position: black_knight_pos,
        };

        game.board
            .set_square(Some(Box::new(black_knight)), black_knight_pos);

        let actual_legal_moves = game.legal_moves();
        let expected_legal_moves = vec![
            chessmove::ChessMove {
                from: (3, 2),
                to: (4, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 2),
                to: (4, 3),
                promotion: None,
            },
        ];
        assert_eq!(expected_legal_moves.len(), actual_legal_moves.len());
        for mv in &actual_legal_moves {
            assert!(expected_legal_moves.contains(mv));
        }
    }

    #[test]
    fn check_that_can_be_removed_by_en_passant() {
        let mut game = Game::new();

        game.en_passant = Some(position::Position(5, 6));

        game.board = board::Board::empty();

        let white_king_pos = position::Position(6, 4);
        let white_king = king::King {
            color: color::Color::WHITE,
            position: white_king_pos,
        };
        game.white_king = white_king_pos;
        game.board
            .set_square(Some(Box::new(white_king)), white_king_pos);

        let white_pawn_pos = position::Position(4, 5);
        let white_pawn = pawn::Pawn {
            color: color::Color::WHITE,
            position: white_pawn_pos,
        };
        game.board
            .set_square(Some(Box::new(white_pawn)), white_pawn_pos);

        let black_pawn_pos = position::Position(5, 5);
        let black_pawn = pawn::Pawn {
            color: color::Color::BLACK,
            position: black_pawn_pos,
        };
        game.board
            .set_square(Some(Box::new(black_pawn)), black_pawn_pos);

        let actual_legal_moves = game.legal_moves();
        let expected_legal_moves = vec![
            chessmove::ChessMove {
                from: (6, 4),
                to: (6, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (6, 4),
                to: (5, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (6, 4),
                to: (5, 4),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (6, 4),
                to: (5, 5),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (6, 4),
                to: (6, 5),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (6, 4),
                to: (7, 5),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (6, 4),
                to: (7, 4),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (6, 4),
                to: (7, 3),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (4, 5),
                to: (5, 6),
                promotion: None,
            },
        ];
        assert_eq!(expected_legal_moves.len(), actual_legal_moves.len());
        for mv in &expected_legal_moves {
            assert!(actual_legal_moves.contains(mv));
        }
    }

    #[test]
    fn scholars_mate_with_pawn() {
        let mut game = Game::new();

        set_piece(
            &mut game.board,
            Box::new(pawn::Pawn {
                color: color::Color::WHITE,
                position: position::Position(6, 7),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(bishop::Bishop {
                color: color::Color::WHITE,
                position: position::Position(3, 4),
            }),
        );

        game.side_to_move = color::Color::BLACK;

        let moves = game.legal_moves();
        assert_eq!(0, moves.len());
    }

    #[test]
    fn scholars_mate_black() {
        let mut game = Game::new();

        set_piece(
            &mut game.board,
            Box::new(queen::Queen {
                color: color::Color::BLACK,
                position: position::Position(6, 2),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(bishop::Bishop {
                color: color::Color::BLACK,
                position: position::Position(3, 5),
            }),
        );

        game.side_to_move = color::Color::WHITE;

        let moves = game.legal_moves();
        assert_eq!(0, moves.len());
    }

    #[test]
    fn knight_corner_mate() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        let black_king_pos = position::Position(8, 8);

        set_piece(
            &mut game.board,
            Box::new(king::King {
                color: color::Color::BLACK,
                position: black_king_pos,
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(pawn::Pawn {
                color: color::Color::BLACK,
                position: position::Position(8, 7),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(knight::Knight {
                color: color::Color::WHITE,
                position: position::Position(6, 7),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::WHITE,
                position: position::Position(7, 2),
            }),
        );

        game.black_king = black_king_pos;
        game.side_to_move = color::Color::BLACK;

        let actual_legal_moves = game.legal_moves();
        assert_eq!(0, actual_legal_moves.len());
    }

    #[test]
    fn black_cannot_castle_because_it_is_in_check() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(1, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(8, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(king::King {
                color: color::Color::BLACK,
                position: position::Position(5, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(bishop::Bishop {
                color: color::Color::WHITE,
                position: position::Position(2, 5),
            }),
        );

        game.side_to_move = color::Color::BLACK;

        let actual_legal_moves = game.legal_moves();
        let expected_legal_moves = vec![
            chessmove::ChessMove {
                from: (5, 8),
                to: (4, 8),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (5, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (6, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (6, 8),
                promotion: None,
            },
        ];
        assert_eq!(expected_legal_moves.len(), actual_legal_moves.len());

        for mv in &expected_legal_moves {
            assert!(actual_legal_moves.contains(mv));
        }
    }

    #[test]
    fn black_can_castle() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(1, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(8, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(king::King {
                color: color::Color::BLACK,
                position: position::Position(5, 8),
            }),
        );

        game.side_to_move = color::Color::BLACK;

        let actual_legal_moves = game.legal_moves();

        let mut expected_legal_moves = vec![
            chessmove::ChessMove {
                from: (5, 8),
                to: (4, 8),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (5, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (6, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (6, 8),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (4, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (3, 8),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (7, 8),
                promotion: None,
            },
        ];
        for i in 1..8 {
            expected_legal_moves.push(chessmove::ChessMove {
                from: (1, 8),
                to: (1, i),
                promotion: None,
            });
            expected_legal_moves.push(chessmove::ChessMove {
                from: (8, 8),
                to: (8, i),
                promotion: None,
            });
        }
        for i in 2..5 {
            expected_legal_moves.push(chessmove::ChessMove {
                from: (1, 8),
                to: (i, 8),
                promotion: None,
            });
        }
        for i in 6..8 {
            expected_legal_moves.push(chessmove::ChessMove {
                from: (8, 8),
                to: (i, 8),
                promotion: None,
            });
        }
        assert_eq!(expected_legal_moves.len(), actual_legal_moves.len());

        for mv in &expected_legal_moves {
            assert!(actual_legal_moves.contains(mv));
        }
    }

    #[test]
    fn black_can_castle_king_side() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(1, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(8, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(king::King {
                color: color::Color::BLACK,
                position: position::Position(5, 8),
            }),
        );

        game.side_to_move = color::Color::BLACK;
        game.castling_rights_black = (true, false);

        let actual_legal_moves = game.legal_moves();

        let mut expected_legal_moves = vec![
            chessmove::ChessMove {
                from: (5, 8),
                to: (4, 8),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (5, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (6, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (6, 8),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (4, 7),
                promotion: None,
            },
            chessmove::ChessMove {
                from: (5, 8),
                to: (7, 8),
                promotion: None,
            },
        ];
        for i in 1..8 {
            expected_legal_moves.push(chessmove::ChessMove {
                from: (1, 8),
                to: (1, i),
                promotion: None,
            });
            expected_legal_moves.push(chessmove::ChessMove {
                from: (8, 8),
                to: (8, i),
                promotion: None,
            });
        }
        for i in 2..5 {
            expected_legal_moves.push(chessmove::ChessMove {
                from: (1, 8),
                to: (i, 8),
                promotion: None,
            });
        }
        for i in 6..8 {
            expected_legal_moves.push(chessmove::ChessMove {
                from: (8, 8),
                to: (i, 8),
                promotion: None,
            });
        }
        assert_eq!(expected_legal_moves.len(), actual_legal_moves.len());

        for mv in &expected_legal_moves {
            assert!(actual_legal_moves.contains(mv));
        }
    }

    #[test]
    fn white_castling_rights_get_removed() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        set_piece(
            &mut game.board,
            Box::new(king::King {
                position: position::Position(5, 1),
                color: color::Color::WHITE,
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                position: position::Position(8, 1),
                color: color::Color::WHITE,
            }),
        );

        assert_eq!((true, true), game.castling_rights_white);

        game.make_move(chessmove::ChessMove {
            from: (5, 1),
            to: (5, 2),
            promotion: None,
        });

        assert_eq!(position::Position(5, 2), game.white_king);
        assert_eq!((false, false), game.castling_rights_white);
        assert_eq!(color::Color::BLACK, game.side_to_move);
    }

    #[test]
    fn en_passant_is_registered() {
        let mut game = Game::new();

        game.make_move(chessmove::ChessMove {
            from: (5, 2),
            to: (5, 4),
            promotion: None,
        });

        match game.en_passant {
            None => panic!(),
            Some(ep) => assert_eq!(position::Position(5, 3), ep),
        }

        assert!(game.board.get_square(position::Position(5, 2)).is_none());
        assert!(game.board.get_square(position::Position(5, 4)).is_some());
    }

    #[test]
    fn assert_make_move_moves_piece() {
        let mut game = Game::new();

        game.make_move(chessmove::ChessMove {
            from: (5, 2),
            to: (5, 4),
            promotion: None,
        });

        assert!(game.board.get_square(position::Position(5, 2)).is_none());
        assert!(game.board.get_square(position::Position(5, 4)).is_some());

        match game.board.get_square(position::Position(5, 4)) {
            None => panic!(),
            Some(p) => assert_eq!(position::Position(5, 4), *p.position()),
        }
    }

    #[test]
    fn undo_last_move() {
        let mut game = Game::new();

        game.make_move(chessmove::ChessMove {
            from: (5, 2),
            to: (5, 4),
            promotion: None,
        });

        assert!(game.board.get_square(position::Position(5, 2)).is_none());
        assert!(game.board.get_square(position::Position(5, 4)).is_some());
        assert_eq!(color::Color::BLACK, game.side_to_move);
        match game.en_passant {
            None => panic!(),
            Some(ep) => assert_eq!(position::Position(5, 3), ep),
        }

        game.undo_last_move();

        assert!(game.board.get_square(position::Position(5, 2)).is_some());
        assert!(game.board.get_square(position::Position(5, 4)).is_none());
        assert_eq!(color::Color::WHITE, game.side_to_move);
        if let Some(_) = game.en_passant() {
            panic!();
        }
    }

    #[test]
    fn castling_kingside_leads_to_pieces_having_moved() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(1, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(8, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(king::King {
                color: color::Color::BLACK,
                position: position::Position(5, 8),
            }),
        );

        game.side_to_move = color::Color::BLACK;
        game.castling_rights_black = (true, false);

        game.make_move(chessmove::ChessMove {
            from: (5, 8),
            to: (7, 8),
            promotion: None,
        });

        assert!(game.board.get_square(position::Position(7, 8)).is_some());
        assert!(game.board.get_square(position::Position(6, 8)).is_some());
    }

    #[test]
    fn castling_queenside_leads_to_pieces_having_moved() {
        let mut game = Game::new();
        game.board = board::Board::empty();

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(1, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(rook::Rook {
                color: color::Color::BLACK,
                position: position::Position(8, 8),
            }),
        );

        set_piece(
            &mut game.board,
            Box::new(king::King {
                color: color::Color::BLACK,
                position: position::Position(5, 8),
            }),
        );

        game.side_to_move = color::Color::BLACK;
        game.castling_rights_black = (true, true);

        game.make_move(chessmove::ChessMove {
            from: (5, 8),
            to: (3, 8),
            promotion: None,
        });

        assert!(game.board.get_square(position::Position(3, 8)).is_some());
        assert!(game.board.get_square(position::Position(4, 8)).is_some());
        assert!(game.board.get_square(position::Position(1, 8)).is_none());
        assert!(game.board.get_square(position::Position(5, 8)).is_none());
    }

    #[test]
    fn cannot_castle_through_check() {
        let game = Game::from_game_arr(&[
            2, 1, 0, 0, 0, 14, 11, 12, 0, 1, 0, 11, 0, 13, 0, 0, 0, 1, 3, 0, 0, 0, 11, 0, 0, 4, 0,
            0, 1, 0, 11, 0, 6, 4, 0, 1, 3, 11, 15, 16, 0, 1, 5, 0, 0, 13, 11, 0, 0, 11, 0, 0, 0,
            11, 14, 0, 2, 1, 0, 0, 0, 0, 0, 12, 0, 0, 1, 1, 1, 1, 0, 1, 0,
        ]);

        let moves = game.legal_moves();

        assert!(!moves.contains(&chessmove::ChessMove {
            from: (5, 1),
            to: (7, 1),
            promotion: None,
        },));
        assert!(moves.contains(&chessmove::ChessMove {
            from: (5, 1),
            to: (3, 1),
            promotion: None,
        },));
    }

    #[test]
    fn cannot_castle_through_check_2() {
        let game = Game::from_game_arr(&[
            2, 1, 0, 0, 0, 0, 11, 12, 0, 1, 0, 11, 0, 13, 0, 0, 0, 1, 3, 0, 0, 0, 11, 0, 0, 4, 0,
            0, 1, 0, 11, 0, 6, 14, 0, 1, 3, 11, 15, 16, 0, 1, 5, 0, 0, 13, 11, 0, 0, 11, 0, 0, 0,
            11, 14, 0, 2, 1, 0, 0, 0, 0, 0, 12, 0, 0, 1, 1, 1, 1, 0, 1, 0,
        ]);

        let moves = game.legal_moves();

        assert!(!moves.contains(&chessmove::ChessMove {
            from: (5, 1),
            to: (3, 1),
            promotion: None,
        },));
    }

    #[test]
    fn en_passant_anti_check_and_pawn_capture_anti_check() {
        #[rustfmt::skip]
        let game = Game::from_game_arr(&[
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 16, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 11, 11, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            6, 0, 0, 0, 0, 0, 0, 0,
            4, 3, 0, 0, 0, 0, 0, 1, 14,
        ]);

        match game.en_passant {
            None => panic!("there should be en passant"),
            Some(ep) => assert_eq!(position::Position(4, 3), ep),
        }

        let moves = game.legal_moves();

        assert!(moves.contains(&chessmove::ChessMove {
            from: (5, 4),
            to: (4, 3),
            promotion: None
        }));

        assert!(moves.contains(&chessmove::ChessMove {
            from: (5, 5),
            to: (4, 4),
            promotion: None
        }));
    }

    fn set_piece(board: &mut board::Board, piece: Box<dyn piece::Piece>) {
        let position = *piece.position();
        board.set_square(Some(piece), position);
    }
}
