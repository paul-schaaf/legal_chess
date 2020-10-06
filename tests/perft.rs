extern crate legal_chess;

use legal_chess::{
    chessmove, game,
    pieces::{piece, position},
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Counter(pub u128);

pub fn perft(
    game: &mut game::Game,
    depth: u8,
    ep_counter: &mut Counter,
    castle_counter: &mut Counter,
    capture_counter: &mut Counter,
    move_stack: &mut Vec<chessmove::ChessMove>,
    moves_by_origin: &mut HashMap<
        (chessmove::ChessMove, chessmove::ChessMove),
        Vec<Vec<chessmove::ChessMove>>,
    >,
) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = game.legal_moves();

    if depth == 1 {
        for mv in &moves {
            let mut move_sequence = vec![];

            for past_move in move_stack.iter() {
                move_sequence.push(past_move.clone());
            }

            move_sequence.push(mv.clone());

            let first_move = move_sequence[0];
            let second_move = if move_sequence.len() > 1 {
                move_sequence[1]
            } else {
                move_sequence[0]
            };

            match moves_by_origin.get_mut(&(first_move, second_move)) {
                None => moves_by_origin.insert((first_move, second_move), vec![move_sequence]),
                Some(v) => {
                    v.push(move_sequence);
                    None
                }
            };

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
        move_stack.push(mv);
        game.make_move(mv);

        nodes += perft(
            game,
            depth - 1,
            ep_counter,
            castle_counter,
            capture_counter,
            move_stack,
            moves_by_origin,
        );

        move_stack.pop();
        game.undo_last_move();
    }

    nodes
}
