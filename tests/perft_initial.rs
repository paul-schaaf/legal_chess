extern crate legal_chess;

mod perft;

use legal_chess::game;

#[test]
fn depth_2() {
    let mut game = game::Game::new();

    assert_eq!(400, perft::perft(&mut game, 2));
}

#[test]
fn depth_3() {
    let mut game = game::Game::new();

    assert_eq!(8902, perft::perft(&mut game, 3));
}

#[test]
fn depth_4() {
    let mut game = game::Game::new();

    assert_eq!(197281, perft::perft(&mut game, 4));
}

#[test]
fn depth_5() {
    let mut game = game::Game::new();

    assert_eq!(4865609, perft::perft(&mut game, 5));
}
