extern crate legal_chess;

use legal_chess::{
    chessmove, game,
    pieces::{piece, position},
};

#[derive(Debug, PartialEq)]
pub struct Counter(pub u128);

pub fn perft(
    game: &mut game::Game,
    depth: u8,
    ep_counter: &mut Counter,
    castle_counter: &mut Counter,
    capture_counter: &mut Counter,
    moves_made: &mut Vec<chessmove::ChessMove>,
) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = game.legal_moves();

    if depth == 1 {
        for mv in &moves {
            if game
                .board()
                .get_square(position::Position((mv.1).0, (mv.1).1))
                .is_some()
            {
                capture_counter.0 += 1;
            }

            let piece = match game
                .board()
                .get_square(position::Position((mv.0).0, (mv.0).1))
            {
                None => panic!(),
                Some(v) => v,
            };

            if piece.piece() == piece::PieceEnum::KING
                && ((mv.0).0 as i8 - (mv.1).0 as i8).abs() == 2
            {
                castle_counter.0 += 1;
                moves_made.push(mv.clone());
                println!("{:?}", moves_made);
                moves_made.pop();
            } else if piece.piece() == piece::PieceEnum::PAWN {
                if (mv.0).0 != (mv.1).0
                    && game
                        .board()
                        .get_square(position::Position((mv.1).0, (mv.1).1))
                        .is_none()
                {
                    ep_counter.0 += 1;
                }
            }
        }
        return moves.len();
    }

    let mut nodes = 0;

    for mv in moves {
        moves_made.push(mv.clone());

        game.make_move(mv, None);

        nodes += perft(
            game,
            depth - 1,
            ep_counter,
            castle_counter,
            capture_counter,
            moves_made,
        );

        game.undo_last_move();
        moves_made.pop();
    }

    nodes
}
