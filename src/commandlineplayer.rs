/*
 * Rustgammon
 *
 * Command line player
 *
 * @author ryutaroikeda94@gmail.com
 *
 */

use rustgammon::Backgammon;
use rustgammon::Player;
use rustgammon::Color;
use rustgammon::DiceRoll;
use rustgammon::Move;

pub struct CommandLinePlayer {
    game: Backgammon,
}

impl Player for CommandLinePlayer {
    fn make_move(&self, color: Color, roll: DiceRoll) -> Move {
        return Move { submoves: vec!() };
    }
}

