extern crate legal_chess;

mod perft;

use legal_chess::game;
use std::collections::HashMap;

#[rustfmt::skip]
const GAME_ARR: [u8; 73] = [
    2, 1, 0, 0, 0, 0, 11, 12,
    3, 1, 0, 0, 0, 0, 11, 13,
    4, 1, 0,4, 0, 11, 0, 14,
    5, 0, 0, 0, 0, 0, 1, 15,
    6, 3, 0, 0, 0, 0,14, 0,
    0, 13, 0, 0, 0, 0, 11, 16,
    0, 1, 0, 0, 0, 0, 11, 0,
    2,1, 0, 0, 0, 0, 11, 12,
    0, 0, 1, 1, 0, 0, 1, 8, 0,
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
        &mut vec![],
        &mut HashMap::new(),
    );

    assert_eq!(44, moves);
}

#[test]
fn perft_5_depth_2() {
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

    assert_eq!(1486, moves);
}

#[test]
fn perft_5_depth_3() {
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

    assert_eq!(62379, moves);
}

#[test]
fn perft_5_depth_4() {
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

    assert_eq!(2103487, moves);
}
