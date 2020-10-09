extern crate legal_chess;

mod perft;

use legal_chess::game;
use std::collections::HashMap;

#[rustfmt::skip]
const GAME_ARR: [u8; 73] = [
    2, 1, 15, 4, 13, 0, 1, 12,
    0, 11, 0, 4, 1, 14, 11, 0,
    0, 0, 0,1, 0, 0, 11, 0,
    5, 1, 0, 0, 0, 0, 11, 0,
    0, 0, 0, 1, 0, 0,0, 16,
    2, 0, 3, 0, 0, 13, 11, 0,
    6, 1, 0, 0, 0, 14, 11, 0,
    0,1, 0, 0, 0, 3, 11, 12,
    0, 0, 0, 0, 1, 1, 0, 1, 30,
];

#[test]
fn perft_4_depth_1() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    assert_eq!(true, game.castling_rights_black().0);
    assert_eq!(true, game.castling_rights_black().1);
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

    assert_eq!(6, moves);
}

#[test]
fn perft_4_depth_2() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    assert_eq!(true, game.castling_rights_black().0);
    assert_eq!(true, game.castling_rights_black().1);
    assert_eq!(false, game.castling_rights_white().0);
    assert_eq!(false, game.castling_rights_white().1);
    assert_eq!(None, *game.en_passant());

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

    assert_eq!(87, capture_counter.0);
    assert_eq!(6, castle_counter.0);
    assert_eq!(264, moves);
}

#[test]
fn perft_4_depth_3() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    assert_eq!(true, game.castling_rights_black().0);
    assert_eq!(true, game.castling_rights_black().1);
    assert_eq!(false, game.castling_rights_white().0);
    assert_eq!(false, game.castling_rights_white().1);
    assert_eq!(None, *game.en_passant());

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

    assert_eq!(4, ep_counter.0);
    assert_eq!(1021 - 4, capture_counter.0);
    assert_eq!(0, castle_counter.0);
    assert_eq!(9467, moves);
}

#[test]
fn perft_4_depth_4() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);
    assert_eq!(true, game.castling_rights_black().0);
    assert_eq!(true, game.castling_rights_black().1);
    assert_eq!(false, game.castling_rights_white().0);
    assert_eq!(false, game.castling_rights_white().1);
    assert_eq!(None, *game.en_passant());

    let mut capture_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);
    let mut ep_counter = perft::Counter(0);

    let mut moves_by_origin = HashMap::new();

    let moves = perft::perft(
        &mut game,
        4,
        &mut ep_counter,
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut moves_by_origin,
    );

    assert_eq!(0, ep_counter.0);
    assert_eq!(131393, capture_counter.0);
    assert_eq!(7795, castle_counter.0);
    assert_eq!(422333, moves);
}
