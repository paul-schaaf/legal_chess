extern crate legal_chess;

mod perft;

use legal_chess::game;

const GAME_ARR: [&str; 73] = [
    "R", "P", "-", "-", "-", "b", "p", "r", "-", "P", "-", "p", "-", "n", "-", "-", "-", "P", "N",
    "-", "-", "-", "p", "-", "-", "B", "-", "-", "P", "-", "p", "-", "K", "B", "-", "P", "N", "p",
    "q", "k", "-", "P", "Q", "-", "-", "n", "p", "-", "-", "P", "-", "-", "-", "p", "b", "-", "R",
    "P", "p", "-", "-", "-", "-", "r", "-", "-", "1", "1", "1", "1", "0", "1", "w",
];

#[test]
fn depth_1() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    assert_eq!(48, perft::perft(&mut game, 1));
}

#[test]
fn depth_2() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    assert_eq!(2039, perft::perft(&mut game, 2));
}

/* #[test]
fn depth_3() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    assert_eq!(97862, perft::perft(&mut game, 3));
} */
