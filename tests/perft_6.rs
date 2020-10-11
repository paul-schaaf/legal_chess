extern crate legal_chess;

mod perft;

use legal_chess::game;
use std::collections::HashMap;

const GAME_ARR: [u8; 73] = [
    2, 0, 1, 0, 0, 11, 0, 12, 0, 1, 0, 0, 0, 0, 11, 0, 0, 1, 3, 4, 14, 13, 11, 0, 0, 0, 1, 0, 0,
    11, 0, 0, 0, 5, 0, 1, 11, 0, 15, 0, 2, 1, 3, 0, 0, 13, 11, 12, 6, 1, 0, 14, 4, 0, 11, 16, 0, 1,
    0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0,
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
