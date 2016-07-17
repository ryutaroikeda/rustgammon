extern crate rustgammon;

use rustgammon::rustgammon::*;
use rustgammon::commandlineplayer::*;

#[test]
fn test_parse_move_bear_off_first_and_move_other() {
    let mut game: Backgammon = Default::default();
    let player = CommandLinePlayer { color: Color::White };
    game.white_board.set(24, 1);
    game.white_board.set(21, 1);
    let roll = (2, 3);
    let command = "24/off 21/24";
    let mov = player.parse_command(&game, roll, command).unwrap();
    assert_eq!(mov.submoves.len(), 2);
    assert_eq!(mov.submoves[1].from, 24);
    assert_eq!(mov.submoves[1].die, 2);
    assert_eq!(mov.submoves[0].from, 21);
    assert_eq!(mov.submoves[0].die, 3);
}

#[test]
fn test_parse_move_bear_off_second_and_move_other() {
    let mut game: Backgammon = Default::default();
    let player = CommandLinePlayer { color: Color::White };
    game.white_board.set(24, 1);
    game.white_board.set(21, 1);
    let roll = (2, 3);
    let command = "21/24 24/off";
    let mov = player.parse_command(&game, roll, command).unwrap();
    assert_eq!(mov.submoves.len(), 2);
    assert_eq!(mov.submoves[1].from, 21);
    assert_eq!(mov.submoves[1].die, 3);
    assert_eq!(mov.submoves[0].from, 24);
    assert_eq!(mov.submoves[0].die, 2);
}

#[test]
fn test_parse_move_bear_off_two() {
    let mut game: Backgammon = Default::default();
    let player = CommandLinePlayer { color: Color::White };
    game.white_board.set(24, 1);
    game.white_board.set(23, 1);
    let roll = (2, 3);
    let command = "22/off 24/off";
    let mov = player.parse_command(&game, roll, command).unwrap();
    assert_eq!(mov.submoves.len(), 2);
    assert_eq!(mov.submoves[1].from, 22);
    assert_eq!(mov.submoves[1].die, 3);
    assert_eq!(mov.submoves[0].from, 24);
    assert_eq!(mov.submoves[0].die, 2);
}

#[test]
fn test_parse_move_bear_off_four() {
    let mut game: Backgammon = Default::default();
    let player = CommandLinePlayer { color: Color::White };
    game.white_board.set(24, 1);
    game.white_board.set(23, 3);
    let roll = (2, 2);
    let command = "23/off 23/off 23/off 24/off";
    let mov = player.parse_command(&game, roll, command).unwrap();
    assert_eq!(mov.submoves.len(), 4);
    assert_eq!(mov.submoves[3].from, 23);
    assert_eq!(mov.submoves[3].die, 2);
    assert_eq!(mov.submoves[2].from, 23);
    assert_eq!(mov.submoves[2].die, 2);
    assert_eq!(mov.submoves[1].from, 23);
    assert_eq!(mov.submoves[1].die, 2);
    assert_eq!(mov.submoves[0].from, 24);
    assert_eq!(mov.submoves[0].die, 2);
}

#[test]
fn test_bear_off_one_checker() {
    let mut game: Backgammon = Default::default();
    let player = CommandLinePlayer { color: Color::Red };
    game.red_board.set(24, 1);
    let roll = (4, 5);
    let command = "1/off";
    let mov = player.parse_command(&game, roll, command).unwrap();
    assert_eq!(mov.submoves.len(), 1);
    assert_eq!(mov.submoves[0].from, 24);
    assert_eq!(mov.submoves[0].die, 5);
}

