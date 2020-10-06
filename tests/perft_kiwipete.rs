extern crate legal_chess;

mod perft;

use legal_chess::game;

use std::collections::HashMap;

const GAME_ARR: [&str; 73] = [
    "R", "P", "-", "-", "-", "b", "p", "r", "-", "P", "-", "p", "-", "n", "-", "-", "-", "P", "N",
    "-", "-", "-", "p", "-", "-", "B", "-", "-", "P", "-", "p", "-", "K", "B", "-", "P", "N", "p",
    "q", "k", "-", "P", "Q", "-", "-", "n", "p", "-", "-", "P", "-", "-", "-", "p", "b", "-", "R",
    "P", "p", "-", "-", "-", "-", "r", "-", "-", "1", "1", "1", "1", "0", "1", "w",
];

#[test]
fn perft_kiwipete_depth_1() {
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

    assert_eq!(2, castle_counter.0);
    assert_eq!(8, capture_counter.0);
    assert_eq!(48, moves);
}

#[test]
fn perft_kiwipete_depth_2() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    let mut castle_counter = perft::Counter(0);
    let mut capture_counter = perft::Counter(0);
    let mut ep_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        2,
        &mut ep_counter,
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );
    assert_eq!(1, ep_counter.0);
    assert_eq!(351 - 1, capture_counter.0);
    assert_eq!(2039, moves);
    assert_eq!(91, castle_counter.0);
}

#[test]
fn perft_kiwipete_depth_3() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    let mut castle_counter = perft::Counter(0);
    let mut ep_counter = perft::Counter(0);
    let mut capture_counter = perft::Counter(0);

    let amount_moves = perft::perft(
        &mut game,
        3,
        &mut ep_counter,
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut HashMap::new(),
    );
    assert_eq!(45, ep_counter.0);
    assert_eq!(17102 - 45, capture_counter.0);
    assert_eq!(3162, castle_counter.0, "Wrong castle number");
    assert_eq!(97862, amount_moves);
}

#[test]
fn perft_kiwipete_depth_4() {
    let mut game = game::Game::from_game_arr(&GAME_ARR);

    let mut castle_counter = perft::Counter(0);
    let mut ep_counter = perft::Counter(0);
    let mut capture_counter = perft::Counter(0);

    let mut moves_by_origin = HashMap::new();

    let amount_moves = perft::perft(
        &mut game,
        4,
        &mut ep_counter,
        &mut castle_counter,
        &mut capture_counter,
        &mut vec![],
        &mut moves_by_origin,
    );
    assert_eq!(1929, ep_counter.0);
    assert_eq!(128013, castle_counter.0);
    assert_eq!(757163 - 1929, capture_counter.0);
    assert_eq!(4085603, amount_moves);
}
