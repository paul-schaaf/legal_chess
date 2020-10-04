extern crate legal_chess;

mod perft;

use legal_chess::game;

#[test]
fn depth_1() {
    let mut game = game::Game::new();

    assert_eq!(
        20,
        perft::perft(
            &mut game,
            1,
            &mut perft::Counter(0),
            &mut perft::Counter(0),
            &mut perft::Counter(0),
        )
    );
}

#[test]
fn depth_2() {
    let mut game = game::Game::new();

    assert_eq!(
        400,
        perft::perft(
            &mut game,
            2,
            &mut perft::Counter(0),
            &mut perft::Counter(0),
            &mut perft::Counter(0),
        )
    );
}

#[test]
fn depth_3() {
    let mut game = game::Game::new();

    let mut capture_counter = perft::Counter(0);
    let moves = perft::perft(
        &mut game,
        3,
        &mut perft::Counter(0),
        &mut perft::Counter(0),
        &mut capture_counter,
    );

    assert_eq!(34, capture_counter.0);

    assert_eq!(8902, moves);
}

#[test]
fn depth_4() {
    let mut game = game::Game::new();

    let mut capture_counter = perft::Counter(0);
    let moves = perft::perft(
        &mut game,
        4,
        &mut perft::Counter(0),
        &mut perft::Counter(0),
        &mut capture_counter,
    );

    assert_eq!(1576, capture_counter.0);
    assert_eq!(197281, moves);
}

#[test]
fn depth_5() {
    let mut game = game::Game::new();

    let mut capture_counter = perft::Counter(0);
    let mut ep_counter = perft::Counter(0);
    let mut castle_counter = perft::Counter(0);

    let moves = perft::perft(
        &mut game,
        5,
        &mut ep_counter,
        &mut castle_counter,
        &mut capture_counter,
    );

    assert_eq!(258, ep_counter.0);
    assert_eq!(82719 - 258, capture_counter.0);
    assert_eq!(128013, castle_counter.0);
    assert_eq!(4865609, moves);
}
