extern crate legal_chess;

use legal_chess::{
    game,
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
) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = game.legal_moves();

    if depth == 1 {
        for mv in &moves {
            if game
                .board()
                .get_square(position::Position((mv.to).0, (mv.to).1))
                .is_some()
            {
                capture_counter.0 += 1;
            }

            let piece = match game
                .board()
                .get_square(position::Position((mv.from).0, (mv.from).1))
            {
                None => panic!(),
                Some(v) => v,
            };

            if piece.piece() == piece::PieceEnum::KING
                && ((mv.from).0 as i8 - (mv.to).0 as i8).abs() == 2
            {
                castle_counter.0 += 1;
            } else if piece.piece() == piece::PieceEnum::PAWN {
                if (mv.from).0 != (mv.to).0
                    && game
                        .board()
                        .get_square(position::Position((mv.to).0, (mv.to).1))
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
        game.make_move(mv);

        nodes += perft(game, depth - 1, ep_counter, castle_counter, capture_counter);

        game.undo_last_move();
    }

    nodes
}
