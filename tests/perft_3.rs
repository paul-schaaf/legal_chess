extern crate legal_chess;

mod perft;

use legal_chess::game;
use std::collections::HashMap;

#[rustfmt::skip]
const GAME_ARR: [&str; 73] = [
    "-", "-", "-", "-", "K", "-", "-", "-",
    "-", "-", "-", "R", "P", "-", "-", "-",
    "-", "-", "-","-", "-", "-", "p", "-",
    "-", "-", "-", "-", "-", "p", "-", "-",

    "-", "P", "-", "-", "-", "-","-", "-",
    "-", "-", "-", "p", "-", "-", "-", "-",
    "-", "P", "-", "-", "-", "-", "-", "-",
    "-","-", "-", "k", "r", "-", "-", "-",
    "-", "-", "0", "0", "0", "0", "0", "1", "w",
];

#[test]
fn perft_3_depth_1() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    assert_eq!(false, game.castling_rights_black().0);
    assert_eq!(false, game.castling_rights_black().1);
    assert_eq!(false, game.castling_rights_white().0);
    assert_eq!(false, game.castling_rights_white().1);
    assert_eq!(None, *game.en_passant());

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

    assert_eq!(1, capture_counter.0);
    assert_eq!(14, moves);
}

#[test]
fn perft_3_depth_2() {
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

    assert_eq!(14, capture_counter.0);
    assert_eq!(191, moves);
}

#[test]
fn perft_3_depth_3() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);
    let mut ep_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        3,
        &mut ep_counter,
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );

    assert_eq!(2, ep_counter.0);
    assert_eq!(209 - 2, capture_counter.0);
    assert_eq!(2812, moves);
}

#[test]
fn perft_3_depth_4() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);
    let mut ep_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        4,
        &mut ep_counter,
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );

    assert_eq!(123, ep_counter.0);
    assert_eq!(3348 - 123, capture_counter.0);
    assert_eq!(43238, moves);
}

#[test]
fn perft_3_depth_5() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);
    let mut ep_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        5,
        &mut ep_counter,
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );

    assert_eq!(1165, ep_counter.0);
    assert_eq!(52051 - 1165, capture_counter.0);
    assert_eq!(674624, moves);
}
