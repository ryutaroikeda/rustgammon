/*
 * Rustgammon
 *
 * Backgammon implementation in Rust.
 * 
 * @note Ignore stakes for now.
 *
 * @author ryutaroikeda94@gmail.com
 *
 */

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate rand;

pub const BOARD_SIZE: usize = 26;

pub type Position = usize;

pub type Checker = i8;

pub type InternalBoard = [Checker; BOARD_SIZE];

#[derive(Default)]
pub struct Board {
    board: InternalBoard,
}

#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Black,
}

#[derive(Default)]
pub struct Backgammon {
    red_board: Board,
    black_board: Board,
}

pub type Die = usize;
pub type DiceRoll = (Die, Die);

pub struct Submove {
    from: Position,
    to: Position,
}

pub struct Move {
    submoves: Vec<Submove>,
}

impl Board {

    fn get(&self, pos: Position) -> Checker {
        return self.board[pos];
    }

    fn set(&mut self, pos: Position, checkers: Checker) {
        self.board[pos] = checkers;
    }
}

const INITIAL_BOARD: InternalBoard = [
    0,  2, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 5,
        0, 0, 0, 0, 3, 0,   5, 0, 0, 0, 0, 0,   0
];

impl Color {
    fn opposite(self) -> Color {
        return match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}

impl Backgammon {
    fn init(&mut self) {
        self.red_board = Default::default();
        self.red_board.board = INITIAL_BOARD;
        self.black_board = Default::default();
        self.black_board.board = INITIAL_BOARD;
    }

    fn get_board(&self, color: Color) -> &Board {
        return match color {
            Color::Red => &self.red_board,
            Color::Black => &self.black_board,
        }
    }

    fn get_opposite_pos(&self, pos: Position) -> Position {
        return BOARD_SIZE - 1 - pos;
    }

    fn is_blocked(&self, color: Color, pos: Position) -> bool {
        let board = self.get_board(color.opposite());
        // Reverse the position to look at the opponent's board from your point of view.
        let opposite_pos = self.get_opposite_pos(pos);
        if 1 < board.get(opposite_pos) {
            return true;
        }
        return false;
    }

    fn is_all_home(&self, color: Color) -> bool {
        let board = self.get_board(color);
        let end_of_outer_board = 19;
        for pos in 0..end_of_outer_board {
            if 0 < board.get(pos) {
                return false;
            }
        }
        return true;
    }

    fn can_do_submove(&self, color: Color, submove: &Submove) -> bool {
        let board = self.get_board(color);
        // Make sure there is a checker to move.
        if 0 == board.get(submove.from) {
            return false;
        }
        // If there are checkers in the bar, they must be moved first.
        let bar_position = 0;
        if 0 < board.get(bar_position) && submove.from != bar_position {
            return false;
        }
        let bear_off_position = BOARD_SIZE - 1;
        // If we're bearing off a checker, make sure all checkers are on the home board.
        // @warning This only works because it is impossible to bear off a checker outside the home
        // board in one submove.
        if submove.to == bear_off_position && !self.is_all_home(color) {
            return false;
        }
        // Make sure the destination isn't blocked.
        if self.is_blocked(color, submove.to) {
            return false;
        }
        return true;
    }

    // List the submoves for a die.
    fn list_submoves(&self, color: Color, die: Die) -> Vec<Submove> {
        let mut submoves: Vec<Submove> = Vec::new();
        for pos in 0..BOARD_SIZE {
            let submove = Submove { from: pos, to: pos + die };
            if self.can_do_submove(color, &submove) {
                submoves.push(submove);
            }
        }
        return submoves;
    }

    // List all legal moves.
    // @fixme What do we do about duplicate moves?
    fn list_moves(&self, color: Color, roll: DiceRoll) -> Vec<Move> {
        // There are three cases:
        // 1. first move with first die,
        // 2. first move with second die,
        // 3. double.
        /*
        let mut dies = Vec::new();
        dies.push(roll.0);
        dies.push(roll.1);
        let is_double = roll.0 == roll.1;
        if is_double {
            dies.push(roll.0);
            dies.push(roll.1);
        }
        */
    }
}
/*
fn roll_dice() -> [i8; 2] {
    let a = rand::thread_rng().gen_range(1, 7);
    let b = rand::thread_rng().gen_range(1, 7);
    return [a, b];
}
*/
fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("rustgammon - Backgammon implementation in Rust");

    let mut game: Backgammon = Default::default();
    game.init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_all_home_true_for_empty() {
        let game: Backgammon = Default::default();
        assert!(game.is_all_home(Color::Red));
        assert!(game.is_all_home(Color::Black));
    }

    #[test]
    fn test_is_all_home_false_for_non_home_checker() {
        let mut game: Backgammon = Default::default();
        game.red_board.set(0, 1);
        game.black_board.set(0, 1);
        assert!(!game.is_all_home(Color::Red));
        assert!(!game.is_all_home(Color::Black));
    }

    #[test]
    fn test_is_blocked_false_for_empty() {
        let game: Backgammon = Default::default();
        assert!(!game.is_blocked(Color::Red, 0));
    }

    #[test]
    fn test_is_blocked_true_for_blocked() {
        let mut game: Backgammon = Default::default();
        game.red_board.set(0, 2);
        assert!(game.is_blocked(Color::Black, game.get_opposite_pos(0)));
    }

    #[test]
    fn test_can_do_submove_true_for_empty() {
        let mut game: Backgammon = Default::default();
        game.red_board.set(1, 1);
        let submove = Submove { from: 1, to: 1 };
        assert!(game.can_do_submove(Color::Red, &submove));
    }

    #[test]
    fn test_can_do_submove_false_for_not_moving_bar() {
        let mut game: Backgammon = Default::default();
        game.red_board.set(0, 1);
        let submove = Submove { from: 1, to: 1 };
        assert!(!game.can_do_submove(Color::Red, &submove));
    }

    #[test]
    fn test_can_do_submove_false_for_bearing_off_without_all_in_home() {
        let mut game: Backgammon = Default::default();
        let home_pos = BOARD_SIZE - 2;
        let bearing_off_pos = BOARD_SIZE - 1;
        game.red_board.set(1, 1);
        game.red_board.set(home_pos, 1);
        let submove = Submove { from: home_pos, to: bearing_off_pos };
        assert!(!game.can_do_submove(Color::Red, &submove));
    }

    #[test]
    fn test_can_do_submove_false_for_blocked_pos() {
        let mut game: Backgammon = Default::default();
        game.red_board.set(1, 1);
        let black_pos = game.get_opposite_pos(2);
        game.black_board.set(black_pos, 2);
        let submove = Submove { from: 1, to: 2 };
        assert!(!game.can_do_submove(Color::Red, &submove));
    }

    #[test]
    fn test_list_submoves_empty_for_empty() {
        let game: Backgammon = Default::default();
        let die = 1;
        let submoves = game.list_submoves(Color::Red, die);
        assert_eq!(submoves.len(), 0);
    }

    #[test]
    fn test_list_submoves_pushes_one_submove() {
        let mut game: Backgammon = Default::default();
        game.red_board.set(1, 1);
        let die = 1;
        let submoves = game.list_submoves(Color::Red, die);
        assert_eq!(submoves.len(), 1);
        assert_eq!(submoves[0].from, 1);
        assert_eq!(submoves[0].to, 2);
    }
}
