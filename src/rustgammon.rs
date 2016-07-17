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

extern crate rand;

use std;
use std::io;
use std::io::Write;
use std::fmt;
use rand::Rng;

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

// The `from` in Submove is relative to the player.
// 0 is the bar and 25 the bearing-off point for either player.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Submove {
    pub from: Position,
    pub die: Die,
}

#[derive(PartialEq, Eq)]
pub struct Move {
    pub submoves: Vec<Submove>,
}

#[derive(Default, Copy, Clone)]
pub struct Backgammon {
    pub red_board: Board,
    pub white_board: Board,
}

pub trait Player {
    fn get_color(&self) -> Color;

    fn make_move(&self, game: &Backgammon, roll: DiceRoll) -> Move;
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Red => write!(f, "red"),
            Color::White =>write!(f, "white"),
        }
    }
}

impl fmt::Display for Submove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "from: {}, die: {}", self.from, self.die)
    }
}

impl Move {
    pub fn print(&self) {
        for submove in &self.submoves {
            print!("{}, ", submove);
        }
        print!("\n");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => println!("error: {}", e),
        }
    }

}

impl Board {

    pub fn get(&self, pos: Position) -> Checker {
        return self.board[pos];
    }

    pub fn set(&mut self, pos: Position, checkers: Checker) {
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

    pub fn get_board(&self, color: Color, pos: Position) -> Checker {
        return match color {
            Color::Red => self.red_board.get(pos),
            Color::White=> self.white_board.get(pos),
        }
    }

    pub fn set_board(&mut self, color: Color, pos: Position, checkers: Checker) {
        match color {
            Color::Red => self.red_board.set(pos, checkers),
            Color::White => self.white_board.set(pos, checkers),
        }
    }

    pub fn get_opposite_pos(&self, pos: Position) -> Position {
        return BEARING_OFF_POS - pos;
    }

    pub fn is_blocked(&self, color: Color, pos: Position) -> bool {
        // The bearing off position should never be blocked.
        if pos == BEARING_OFF_POS {
            return false;
        }
        // Reverse the position to look at the opponent's board from your point of view.
        let opposite_pos = self.get_opposite_pos(pos);
        if 1 < self.get_board(color.opposite(), opposite_pos) {
            return true;
        }
        return false;
    }

    pub fn is_all_home(&self, color: Color) -> bool {
        let end_of_outer_board = 19;
        for pos in 0..end_of_outer_board {
            if 0 < self.get_board(color, pos) {
                return false;
            }
        }
        return true;
    }

    pub fn can_do_submove(&self, color: Color, submove: &Submove) -> bool {
        // Make sure there is a checker to move.
        if 0 == self.get_board(color, submove.from) {
            return false;
        }
        // Don't move checkers that were beared off.
        if BEARING_OFF_POS == submove.from {
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
    pub fn list_submoves(&self, color: Color, die: Die) -> Vec<Submove> {
        let mut submoves: Vec<Submove> = Vec::new();
        for pos in 0..BEARING_OFF_POS {
            let submove = Submove { from: pos, die: die };
            if self.can_do_submove(color, &submove) {
                submoves.push(submove);
            }
        }
        return submoves;
    }

    pub fn do_submove(&mut self, color: Color, submove: &Submove) {
        debug_assert!(self.can_do_submove(color, &submove));
        let destination = submove.destination();
        let checkers_from = self.get_board(color, submove.from);
        let checkers_to = self.get_board(color, destination);
        // Check if we hit an opposing checker and move it to the bar.
        let opposite_pos = self.get_opposite_pos(destination);
        let opposite_color = color.opposite();
        let is_blot = 0 < self.get_board(opposite_color, opposite_pos);
        // Hit the blot, but not if we're bearing off.
        if is_blot && (destination != BEARING_OFF_POS) {
            let checkers_in_bar = self.get_board(opposite_color, BAR_POS);
            self.set_board(opposite_color, BAR_POS, checkers_in_bar + 1);
            self.set_board(opposite_color, opposite_pos, 0);
        }
        self.set_board(color, submove.from, checkers_from - 1);
        self.set_board(color, destination, checkers_to + 1);
    }

    // List the moves for the given order of playing the dice.
    // Move.submoves is a stack of submoves.
    pub fn list_moves_with_ordered_dice_r(&self, color: Color, dice: &[Die]) -> Vec<Move> {
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
    pub fn list_moves(&self, color: Color, roll: DiceRoll) -> Vec<Move> {
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

    pub fn can_do_move(&self, color: Color, roll: DiceRoll, mov: &Move) -> bool {
        let legal_moves = self.list_moves(color, roll);
        for legal_move in &legal_moves {
            if mov == legal_move {
                return true;
            }
        }
        return false;
    }

    pub fn do_move(&mut self, color: Color, mov: &Move) {
        let mut move_idx = mov.submoves.len();
        while move_idx > 0 {
            move_idx -= 1;
            self.do_submove(color, &mov.submoves[move_idx]);
        }
    }

    // Return true if the move is legal.
    pub fn play_move<T: Player>(&mut self, roll: DiceRoll, player: &T) -> bool {
        let color = player.get_color();
        let player_move = player.make_move(self, roll);
        if self.can_do_move(color, roll, &player_move) {
            self.do_move(color, &player_move);
            return true;
        }
        return false;
    }

    fn is_game_over(&self) -> bool {
        let red_checkers = self.get_board(Color::Red, BEARING_OFF_POS);
        if 15 <= red_checkers {
            return true;
        }
        let white_checkers = self.get_board(Color::White, BEARING_OFF_POS);
        if 15 <= white_checkers {
            return true;
        }
        return false;
    }

    pub fn print(&self) {
        for pos in 13..19 {
            print!(" {}", pos);
        }
        print!("   ");
        for pos in 19..25 {
            print!(" {}", pos);
        }
        print!("\n");

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

        let mut pos = 13;
        let end_pos = 1;
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

        for pos in 0..6 {
            print!(" {number:>width$}", number=12-pos, width=2);
        }
        print!("   ");
        for pos in 6..12 {
            print!(" {number:>width$}", number=12-pos, width=2);
        }
        print!("\n");
    }

    fn roll_dice(&self) -> DiceRoll {
        let a = rand::thread_rng().gen_range(1, 7);
        let b = rand::thread_rng().gen_range(1, 7);
        return (a, b);
    }

    pub fn run<S: Player, T: Player>(&mut self, first: &S, second: &T) {
        loop {
            self.print();
            if self.is_game_over() {
                println!("player {} won", second.get_color());
                break;
            }
            println!("player {} to play", first.get_color());
            let roll = self.roll_dice();
            if self.list_moves(first.get_color(), roll).is_empty() {
                println!("rolled {}-{}, no legal moves", roll.0, roll.1);
            } else { 
                println!("rolled {}-{}", roll.0, roll.1);
                loop {
                    if self.play_move(roll, first) {
                        break;
                    }
                }
            }

            self.print();
            if self.is_game_over() {
                println!("player {} won", first.get_color());
                break;
            }
            println!("player {} to play", second.get_color());
            let roll = self.roll_dice();
            if self.list_moves(second.get_color(), roll).is_empty() {
                println!("rolled {}-{}, no legal moves", roll.0, roll.1);
            } else {
                println!("rolled {}-{}", roll.0, roll.1);
                loop {
                    if self.play_move(roll, second) {
                        break;
                    }
                }
            }
        }
    }
}


