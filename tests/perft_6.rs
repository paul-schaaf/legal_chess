extern crate legal_chess;

mod perft;

use legal_chess::game;
use std::collections::HashMap;

const GAME_ARR: [&str; 73] = [
    "R", "-", "P", "-", "-", "p", "-", "r", "-", "P", "-", "-", "-", "-", "p", "-", "-", "P", "N",
    "B", "b", "n", "p", "-", "-", "-", "P", "-", "-", "p", "-", "-", "-", "Q", "-", "P", "p", "-",
    "q", "-", "R", "P", "N", "-", "-", "n", "p", "r", "K", "P", "-", "b", "B", "-", "p", "k", "-",
    "P", "-", "-", "-", "-", "p", "-", "-", "-", "0", "0", "0", "0", "0", "10", "w",
];

#[test]
fn perft_6_depth_1() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        1,
        &mut perft::Counter(0),
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );

    assert_eq!(46, moves);
}

#[test]
fn perft_6_depth_2() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        2,
        &mut perft::Counter(0),
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );

    assert_eq!(2079, moves);
}

#[test]
fn perft_6_depth_3() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        3,
        &mut perft::Counter(0),
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );

    assert_eq!(89890, moves);
}

#[test]
fn perft_6_depth_4() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        4,
        &mut perft::Counter(0),
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );

    assert_eq!(3894594, moves);
}
