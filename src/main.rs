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
pub const BEARING_OFF_POS: usize = BOARD_SIZE - 1;

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
    die: Die,
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

impl Submove {
    fn destination(&self) -> Position {
        if self.from + self.die >= BOARD_SIZE {
            return BEARING_OFF_POS;
        }
        return self.from + self.die;
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

    fn get_mut_board(&mut self, color: Color) -> &mut Board {
        return match color {
            Color::Red => &mut self.red_board,
            Color::Black => &mut self.black_board,
        }
    }

    fn get_opposite_pos(&self, pos: Position) -> Position {
        return BEARING_OFF_POS - pos;
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

    // @todo We need to report user errors.
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
        // We're bearing off a checker.
        if submove.destination() == BEARING_OFF_POS {
            // Make sure all checkers are on the home board.
            if !self.is_all_home(color) {
                return false;
            }
            // A die may not be used to bear off a lower numbered point unless there are no 
            // checkers on any higher points.
            let start_pos = BEARING_OFF_POS - submove.die;
            for pos in start_pos..submove.from {
                if 0 < board.get(pos) {
                    return false;
                }
            }
        }

        // Make sure the destination isn't blocked.
        if self.is_blocked(color, submove.destination()) {
            return false;
        }
        return true;
    }

    // List the submoves for a die.
    fn list_submoves(&self, color: Color, die: Die) -> Vec<Submove> {
        let mut submoves: Vec<Submove> = Vec::new();
        for pos in 0..BOARD_SIZE {
            let submove = Submove { from: pos, die: die };
            if self.can_do_submove(color, &submove) {
                submoves.push(submove);
            }
        }
        return submoves;
    }

    fn play_submove(&mut self, color: Color, submove: &Submove) {
        debug_assert!(self.can_do_submove(color, &submove));
        let mut board = self.get_mut_board(color);
        let checkers_from = board.get(submove.from);
        let checkers_to   = board.get(submove.destination());
        board.set(submove.from, checkers_from - 1);
        board.set(submove.destination(), checkers_to + 1);
    }

    // List all legal moves.
    // @fixme What do we do about duplicate moves?
    // 1. play out and compare board position
    // 2. Order matters when we're not home yet and one move is bearing off. It also matters when
    //    we're home.
    //
    // Rules:
    // You must play all dice if possible.
    // If only one die can be played, the largest possible must be played.
    // What happens when one die is legal and the other is not? Play the legal die. If the other
    // die becomes legal, can it be moved?
    //
    //
    /*
    fn list_moves(&self, color: Color, roll: DiceRoll) -> Vec<Move> {
        // There are three cases:
        // 1. first move with first die,
        // 2. first move with second die,
        // 3. double.
        let mut dies = Vec::new();
        dies.push(roll.0);
        dies.push(roll.1);
        let is_double = roll.0 == roll.1;
        if is_double {
            dies.push(roll.0);
            dies.push(roll.1);
        }
    }
    */
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
        let black_pos = game.get_opposite_pos(2);
        game.black_board.set(black_pos, 2);
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
    fn test_play_submove_does_submove() {
        let mut game: Backgammon = Default::default();
        game.red_board.set(1, 1);
        let submove = Submove { from: 1, die: 1 };
        game.play_submove(Color::Red, &submove);
        assert_eq!(game.red_board.get(1), 0);
        assert_eq!(game.red_board.get(2), 1);
    }
}
