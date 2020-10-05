extern crate legal_chess;

mod perft;

use legal_chess::game;

#[rustfmt::skip]
const GAME_ARR: [&str; 73] = [
    "R", "P", "-", "-", "-", "-", "p", "r",
    "N", "P", "-", "-", "-", "-", "p", "n",
    "B", "P", "-","B", "-", "p", "-", "b", 
    "Q", "-", "-", "-", "-", "-", "P", "q",
    "K", "N", "-", "-", "-", "-","b", "-",
    "-", "n", "-", "-", "-", "-", "p", "k",
    "-", "P", "-", "-", "-", "-", "p", "-",
    "R","P", "-", "-", "-", "-", "p", "r",
    "-", "-", "1", "1", "0", "0", "1", "8", "w",
];

#[test]
fn perft_5_depth_1() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    assert_eq!(false, game.castling_rights_black().0);
    assert_eq!(false, game.castling_rights_black().1);
    assert_eq!(true, game.castling_rights_white().0);
    assert_eq!(true, game.castling_rights_white().1);
    assert_eq!(None, *game.en_passant());

    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        1,
        &mut perft::Counter(0),
        &mut castle_counter,
        &mut capture_counter,
    );

    assert_eq!(44, moves);
}
