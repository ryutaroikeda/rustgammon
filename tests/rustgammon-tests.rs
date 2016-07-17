extern crate rustgammon;
use rustgammon::rustgammon::*;

#[test]
fn test_is_all_home_true_for_empty() {
    let game: Backgammon = Default::default();
    assert!(game.is_all_home(Color::Red));
    assert!(game.is_all_home(Color::White));
}

#[test]
fn test_is_all_home_false_for_non_home_checker() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(0, 1);
    game.white_board.set(0, 1);
    assert!(!game.is_all_home(Color::Red));
    assert!(!game.is_all_home(Color::White));
}

#[test]
fn test_is_blocked_false_for_empty() {
    let game: Backgammon = Default::default();
    assert!(!game.is_blocked(Color::Red, 0));
}

#[test]
fn test_is_blocked_true_for_blocked() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 2);
    assert!(game.is_blocked(Color::White, game.get_opposite_pos(1)));
}

#[test]
fn test_can_do_submove_true_for_empty() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let submove = Submove { from: 1, die: 1 };
    assert!(game.can_do_submove(Color::Red, &submove));
}

#[test]
fn test_can_do_submove_false_for_not_moving_bar() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(0, 1);
    let submove = Submove { from: 1, die: 1 };
    assert!(!game.can_do_submove(Color::Red, &submove));
}

#[test]
fn test_can_do_submove_false_for_bearing_off_without_all_in_home() {
    let mut game: Backgammon = Default::default();
    let home_pos = BOARD_SIZE - 2;
    game.red_board.set(1, 1);
    game.red_board.set(home_pos, 1);
    let submove = Submove { from: home_pos, die: 1 };
    assert!(!game.can_do_submove(Color::Red, &submove));
}

#[test]
fn test_can_do_submove_false_for_blocked_pos() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let white_pos = game.get_opposite_pos(2);
    game.white_board.set(white_pos, 2);
    let submove = Submove { from: 1, die: 1 };
    assert!(!game.can_do_submove(Color::Red, &submove));
}

#[test]
fn test_can_do_submove_false_for_bearing_off_lower() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(BEARING_OFF_POS - 1, 1);
    game.red_board.set(BEARING_OFF_POS - 2, 1);
    let submove = Submove { from: BEARING_OFF_POS - 1, die: 2 };
    assert!(!game.can_do_submove(Color::Red, &submove));
}

#[test]
fn test_list_submoves_lists_empty_for_empty() {
    let game: Backgammon = Default::default();
    let die = 1;
    let submoves = game.list_submoves(Color::Red, die);
    assert_eq!(submoves.len(), 0);
}

#[test]
fn test_list_submoves_lists_one_submove() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let die = 1;
    let submoves = game.list_submoves(Color::Red, die);
    assert_eq!(submoves.len(), 1);
    assert_eq!(submoves[0].from, 1);
    assert_eq!(submoves[0].die, 1);
}

#[test]
fn test_do_submove_does_submove_and_hits_blot() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let white_pos = game.get_opposite_pos(2);
    game.white_board.set(white_pos, 1);
    let submove = Submove { from: 1, die: 1 };
    game.do_submove(Color::Red, &submove);
    assert_eq!(game.red_board.get(1), 0);
    assert_eq!(game.red_board.get(2), 1);
    assert_eq!(game.white_board.get(white_pos), 0);
    assert_eq!(game.white_board.get(0), 1);
}

#[test]
fn test_list_moves_with_ordered_dice_r_lists_move_for_two_dice() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let dice = vec!(1, 1);
    let moves = game.list_moves_with_ordered_dice_r(Color::Red, &dice);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].submoves.len(), 2);
    assert_eq!(moves[0].submoves[1].from, 1);
    assert_eq!(moves[0].submoves[1].die, 1);
    assert_eq!(moves[0].submoves[0].from, 2);
    assert_eq!(moves[0].submoves[0].die, 1);
}

#[test]
fn test_list_moves_with_ordered_dice_r_lists_empty_for_zero_dice() {
    let game: Backgammon = Default::default();
    let moves = game.list_moves_with_ordered_dice_r(Color::Red, &Vec::new());
    assert_eq!(moves.len(), 0);
}

#[test]
fn test_list_moves_with_ordered_dice_r_lists_empty_for_no_legal_move() {
    let game: Backgammon = Default::default();
    let dice = vec!(1);
    let moves = game.list_moves_with_ordered_dice_r(Color::Red, &dice);
    assert_eq!(moves.len(), 0);
}

// This can be benchmarked.
/*
fn test_list_moves_with_ordered_dice_r_lists_double_ones() {
    let mut game: Backgammon = Default::default();
    // This should generate a lot of moves.
    game.red_board.board = [
        0,  1, 1, 1, 1, 1, 1,   1, 0, 1, 0, 1, 0,
            1, 0, 1, 0, 1, 0,   1, 0, 1, 0, 1, 0,   0,
    ];
    let dice = vec!(1, 1, 1, 1);
    let moves = game.list_moves_with_ordered_dice_r(Color::Red, &dice);
    println!("found {} moves", moves.len());
}
*/

#[test]
fn test_list_moves_lists_move_for_double_ones() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let dice_roll = (1, 1);
    let moves = game.list_moves(Color::Red, dice_roll);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].submoves.len(), 4);
}

#[test]
fn test_list_moves_lists_moves_for_both_dice() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let dice_roll = (1, 2);
    let moves = game.list_moves(Color::Red, dice_roll);
    assert_eq!(moves.len(), 2);
}

#[test]
fn test_list_moves_lists_higher_move() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let white_pos_1 = game.get_opposite_pos(4);
    let white_pos_2 = game.get_opposite_pos(5);
    game.white_board.set(white_pos_1, 2);
    game.white_board.set(white_pos_2, 2);
    let dice_roll = (1, 2);
    let moves = game.list_moves(Color::Red, dice_roll);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].submoves.len(), 1);
    assert_eq!(moves[0].submoves[0].die, 2);
}

#[test]
fn test_list_moves_lists_lower_move() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let white_pos_1 = game.get_opposite_pos(3);
    let white_pos_2 = game.get_opposite_pos(4);
    game.white_board.set(white_pos_1, 2);
    game.white_board.set(white_pos_2, 2);
    let dice_roll = (1, 2);
    let moves = game.list_moves(Color::Red, dice_roll);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].submoves.len(), 1);
    assert_eq!(moves[0].submoves[0].die, 1);
}

#[test]
fn test_equality_of_moves() {
    let first_submove  = Submove { from: 1, die: 1 };
    let second_submove = Submove { from: 2, die: 1 };
    let first_move     = Move { submoves: vec!(second_submove, first_submove) };
    let second_move    = Move { submoves: vec!(second_submove, first_submove) };
    assert!(first_move == second_move);
}

#[test]
fn test_can_do_move_true_for_legal_move() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let dice_roll = (1, 2);
    let first_submove = Submove { from: 1, die: 1 };
    let second_submove = Submove { from: 2, die: 2 };
    let legal_move = Move { submoves: vec!(second_submove, first_submove) };
    assert!(game.can_do_move(Color::Red, dice_roll, &legal_move));
}

#[test]
fn test_can_do_move_false_for_illegal_move() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let dice_roll = (1, 1);
    let first_submove = Submove { from: 1, die: 1 };
    let second_submove = Submove { from: 2, die: 1 };
    // This is illegal because it leaves two more dice unused.
    let illegal_move = Move { submoves: vec!(second_submove, first_submove) };
    assert!(!game.can_do_move(Color::Red, dice_roll, &illegal_move));
}

#[test]
fn test_do_move_does_move() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(1, 1);
    let first_submove = Submove { from: 1, die: 1 };
    let second_submove = Submove { from: 2, die: 2 };
    let legal_move = Move { submoves: vec!(second_submove, first_submove) };
    game.do_move(Color::Red, &legal_move);
    assert_eq!(game.get_board(Color::Red, 1), 0);
    assert_eq!(game.get_board(Color::Red, 4), 1);
}

#[test]
fn test_list_moves_with_one_checker_bearing_off() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(24, 1);
    let roll = (1, 2);
    let moves = game.list_moves(Color::Red, roll);
    // We must use 2 to bear off the last checker because it is the higher die.
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].submoves.len(), 1);
    assert_eq!(moves[0].submoves[0].from, 24);
    assert_eq!(moves[0].submoves[0].die, 2);
}

#[test]
fn test_list_moves_with_three_checkers_bearing_off() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(24, 2);
    game.red_board.set(23, 1);
    let roll = (2, 2);
    let moves = game.list_moves(Color::Red, roll);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].submoves.len(), 3);
}

#[test]
fn test_list_moves_with_one_checker_bearing_off_while_two_opposites_in_bar() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(24, 1);
    let white_bar_pos = game.get_opposite_pos(25);
    game.white_board.set(white_bar_pos, 2);
    let roll = (2, 3);
    let moves = game.list_moves(Color::Red, roll);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].submoves.len(), 1);
}

#[test]
fn test_do_submove_does_not_hit_blot_when_bearing_off() {
    let mut game: Backgammon = Default::default();
    game.red_board.set(24, 1);
    game.white_board.set(0, 1);
    let submove = Submove { from: 24, die: 1 };
    game.do_submove(Color::Red, &submove);
    assert_eq!(game.get_board(Color::White, 0), 1);
}

