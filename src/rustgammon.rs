/*
 * Rustgammon
 *
 * Backgammon implementation in Rust.
 * 
 * @note Ignore doubling cube for now.
 *
 * @author ryutaroikeda94@gmail.com
 *
 */

use std;

pub const BOARD_SIZE: usize = 26;
pub const BAR_POS: usize = 0;
pub const BEARING_OFF_POS: usize = BOARD_SIZE - 1;

pub type Position = usize;

pub type Checker = i8;

pub type InternalBoard = [Checker; BOARD_SIZE];

#[derive(Default, Copy, Clone)]
pub struct Board {
    board: InternalBoard,
}

#[derive(Copy, Clone)]
pub enum Color {
    Red,
    White,
}

pub type Die = usize;
pub type DiceRoll = (Die, Die);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Submove {
    from: Position,
    die: Die,
}

#[derive(PartialEq, Eq)]
pub struct Move {
    pub submoves: Vec<Submove>,
}

pub trait Player {
    fn make_move(&self, color: Color, roll: DiceRoll) -> Move;
}

#[derive(Default, Copy, Clone)]
pub struct Backgammon {
    red_board: Board,
    white_board: Board,
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
            Color::Red => Color::White,
            Color::White => Color::Red,
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
    pub fn init(&mut self) {
        self.red_board = Board { board: INITIAL_BOARD };
        self.white_board = Board { board: INITIAL_BOARD };
    }

    fn get_board(&self, color: Color, pos: Position) -> Checker {
        return match color {
            Color::Red => self.red_board.get(pos),
            Color::White=> self.white_board.get(pos),
        }
    }

    fn set_board(&mut self, color: Color, pos: Position, checkers: Checker) {
        match color {
            Color::Red => self.red_board.set(pos, checkers),
            Color::White => self.white_board.set(pos, checkers),
        }
    }

    fn get_opposite_pos(&self, pos: Position) -> Position {
        return BEARING_OFF_POS - pos;
    }

    fn is_blocked(&self, color: Color, pos: Position) -> bool {
        // Reverse the position to look at the opponent's board from your point of view.
        let opposite_pos = self.get_opposite_pos(pos);
        if 1 < self.get_board(color.opposite(), opposite_pos) {
            return true;
        }
        return false;
    }

    fn is_all_home(&self, color: Color) -> bool {
        let end_of_outer_board = 19;
        for pos in 0..end_of_outer_board {
            if 0 < self.get_board(color, pos) {
                return false;
            }
        }
        return true;
    }

    // @cleanup emit user friendly messages somewhere.
    fn can_do_submove(&self, color: Color, submove: &Submove) -> bool {
        // Make sure there is a checker to move.
        if 0 == self.get_board(color, submove.from) {
            return false;
        }
        // If there are checkers in the bar, they must be moved first.
        let bar_position = 0;
        if 0 < self.get_board(color, bar_position) && submove.from != bar_position {
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
                if 0 < self.get_board(color, pos) {
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

    fn do_submove(&mut self, color: Color, submove: &Submove) {
        debug_assert!(self.can_do_submove(color, &submove));
        let destination = submove.destination();
        let checkers_from = self.get_board(color, submove.from);
        let checkers_to = self.get_board(color, destination);
        // Check if we hit an opposing checker and move it to the bar.
        let opposite_pos = self.get_opposite_pos(destination);
        let opposite_color = color.opposite();
        let is_blot = 0 < self.get_board(opposite_color, opposite_pos);
        if is_blot {
            self.set_board(opposite_color, BAR_POS, 1);
            self.set_board(opposite_color, opposite_pos, 0);
        }
        self.set_board(color, submove.from, checkers_from - 1);
        self.set_board(color, destination, checkers_to + 1);
    }

    // List the moves for the given order of playing the dice.
    // Move.submoves is a stack of submoves.
    fn list_moves_with_ordered_dice_r(&self, color: Color, dice: &[Die]) -> Vec<Move> {
        let mut moves = Vec::new();
        let (die, dice_tail) = match dice.split_first() {
            Some((head, tail)) => (*head, tail),
            None => return moves,
        };
        let submoves = self.list_submoves(color, die);
        for submove in &submoves {
            let mut game = self.clone();
            game.do_submove(color, submove);
            let mut next_moves = game.list_moves_with_ordered_dice_r(color, dice_tail);
            // If we found no moves, create an empty move so we can put the current submove.
            if next_moves.is_empty() {
                next_moves.push(Move { submoves: Vec::new() });
            }
            for next_move in &mut next_moves {
                next_move.submoves.push(*submove);
            }
            moves.extend(next_moves);
        }
        return moves;
    }

    // List all legal moves.
    // Worst case is about 15 ^4 ~= 2 ^ 16
    // @fixme What do we do about duplicate moves? Can play the moves and compare board positions.
    // Do we need to get rid of duplicates?
    // Rules:
    // You must play all dice if possible.
    // If only one die can be played, the highest possible must be played.
    fn list_moves(&self, color: Color, roll: DiceRoll) -> Vec<Move> {
        let is_double = roll.0 == roll.1;
        if is_double {
            let dice = vec!(roll.0, roll.0, roll.0, roll.0);
            return self.list_moves_with_ordered_dice_r(color, &dice);
        }
        // We didn't roll a double.
        let high = std::cmp::max(roll.0, roll.1);
        let low  = std::cmp::min(roll.0, roll.1);
        // @cleanup dry
        let high_moves = self.list_moves_with_ordered_dice_r(color, &[high, low]);
        let low_moves  = self.list_moves_with_ordered_dice_r(color, &[low, high]);
        let mut can_play_both_dice = false;
        let mut both_dice_moves: Vec<Move> = Vec::new();
        for high_move in &high_moves {
            if high_move.submoves.len() == 2 {
                can_play_both_dice = true;
                // @cleanup This doesn't look idiomatic.
                let submoves = high_move.submoves.clone();
                both_dice_moves.push(Move { submoves: submoves });
            }
        }
        for low_move in &low_moves {
            if low_move.submoves.len() == 2 {
                can_play_both_dice = true;
                let submoves = low_move.submoves.clone();
                both_dice_moves.push(Move { submoves: submoves });
            }
        }
        if can_play_both_dice {
            // Allow only moves that use both dice.
            return both_dice_moves;
        } 
        // We can only play one die. Make sure we play the highest possible die.
        if 0 < high_moves.len() {
            return high_moves;
        } else {
            return low_moves;
        }
    }

    fn can_do_move(&self, color: Color, roll: DiceRoll, mov: &Move) -> bool {
        let legal_moves = self.list_moves(color, roll);
        for legal_move in &legal_moves {
            if mov == legal_move {
                return true;
            }
        }
        return false;
    }

    fn do_move(&mut self, color: Color, mov: &Move) {
        let mut move_idx = mov.submoves.len();
        while move_idx > 0 {
            move_idx -= 1;
            self.do_submove(color, &mov.submoves[move_idx]);
        }
    }

    fn play_move<T: Player>(&mut self, color: Color, roll: DiceRoll, player: &T) {
        let player_move = player.make_move(color, roll);
        if self.can_do_move(color, roll, &player_move) {
            self.do_move(color, &player_move);
        }
    }

    pub fn print(&self) {
        let mut pos = 13;
        let end_pos = 1;
        let print_checker = |pos: Position| {
            let red_checker = self.get_board(Color::Red, pos);
            if 0 < red_checker {
                print!(" R{}", red_checker);
                return;
            }
            let white_pos = self.get_opposite_pos(pos);
            let white_checker = self.get_board(Color::White, white_pos);
            if 0 < white_checker {
                print!(" W{}", white_checker);
                return;
            }
            print!(" ..");
        };
        // @cleanup Wait for a more idiomatic way to iterate.
        while pos > end_pos {
            pos -= 1;
            if pos == 6 {
                print!(" ||");
            }
            print_checker(pos);
        }
        print!("\tRed bar: {}\tRed off: {}",
               self.get_board(Color::Red, 0),
               self.get_board(Color::Red, BEARING_OFF_POS));
        print!("\n");
        let mut pos = 12;
        let end_pos = 24;
        while pos < end_pos {
            pos += 1;
            if pos == 19 {
                print!(" ||");
            }
            print_checker(pos);
        }
        print!("\tWhite bar: {}\tWhite off: {}",
               self.get_board(Color::White, 0),
               self.get_board(Color::White, BEARING_OFF_POS));
        print!("\n");
    }

}

/*
fn roll_dice() -> [i8; 2] {
    let a = rand::thread_rng().gen_range(1, 7);
    let b = rand::thread_rng().gen_range(1, 7);
    return [a, b];
}
*/

#[cfg(test)]
mod tests {
    use super::*;

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
        game.red_board.set(0, 2);
        assert!(game.is_blocked(Color::White, game.get_opposite_pos(0)));
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

}
