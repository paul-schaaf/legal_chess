extern crate legal_chess;

use legal_chess::game;

pub fn perft(game: &mut game::Game, depth: u8) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = game.legal_moves();

    if depth == 1 {
        return moves.len();
    }

    let mut nodes = 0;

    for mv in moves {
        game.make_move(mv, None);

        nodes += perft(game, depth - 1);

        game.undo_last_move();
    }

    nodes
}
