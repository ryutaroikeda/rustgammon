/*
 * Rustgammon
 *
 * Random player
 * Play a random legal move.
 *
 * @author ryutaroikeda94@gmail.com
 */

extern crate rand;

use rand::Rng;

use rustgammon::Backgammon;
use rustgammon::Color;
use rustgammon::DiceRoll;
use rustgammon::Move;
use rustgammon::Player;

pub struct RandomPlayer {
    pub color: Color,
}

impl Player for RandomPlayer {
    fn get_color(&self) -> Color {
        return self.color;
    }

    fn make_move(&self, game: &Backgammon, roll: DiceRoll) -> Move {
        let moves = game.list_moves(self.color, roll);
        let random_index = rand::thread_rng().gen_range(0, moves.len()) as usize;
        let mov = Move { submoves: moves[random_index].submoves.clone() };
        return mov;
    }
}

