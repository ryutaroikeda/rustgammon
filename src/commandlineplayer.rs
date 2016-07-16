/*
 * Rustgammon
 *
 * Command line player
 *
 * @author ryutaroikeda94@gmail.com
 *
 */

use rustgammon::Backgammon;
use rustgammon::Color;
use rustgammon::DiceRoll;
use rustgammon::Move;
use rustgammon::Player;
use rustgammon::Submove;

use std::cmp;
use std::io;
use std::io::Write;
use std::error::Error;

pub type CommandLineError = String;

pub struct CommandLinePlayer {
    pub color: Color,
}

impl Player for CommandLinePlayer {
    fn get_color(&self) -> Color {
        return self.color;
    }

    fn make_move(&self, game: &Backgammon, roll: DiceRoll) -> Move {
        loop {
            print!("rolled {}-{}, enter move: ", roll.0, roll.1);
            match io::stdout().flush() {
                Ok(_) => (),
                Err(e) => println!("error: {}", e),
            }
            let mov = match self.read_command(game, roll) {
                Ok(v) => v,
                Err(e) => { println!("{}", e); continue },
            };
            if game.can_do_move(self.color, roll, &mov) {
                return mov;
            }
            for submove in &mov.submoves {
                debug!("{}, ", submove);
            }
            println!("illegal move");
        }
    }
}

impl CommandLinePlayer {
    fn read_command(&self, game: &Backgammon, roll: DiceRoll) -> Result<Move, CommandLineError> {
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(_) => (),
            Err(e) => return Err(e.description().to_string()),
        }
        return self.parse_command(game, roll, &command.trim());
    }

    fn is_valid_pos(&self, pos: usize) -> bool {
        return pos <= 25;
    }

    // A submove is written from/to where
    // `from` is a position or "bar"
    // `to` is a position or "off"
    fn parse_submove(&self, game: &Backgammon, submove: &str)
        -> Result<Submove, CommandLineError> {
        let mut poss = submove.split("/");
        let from_str = try!(poss.next().ok_or("Invalid backgammon notation"));
        let from = match from_str {
            "bar" => 0,
            _ =>
                match from_str.parse::<usize>() {
                    Ok(pos) => 
                        match self.color {
                            Color::Red => game.get_opposite_pos(pos),
                            Color::White => pos,
                        },
                    Err(e) => return Err(e.description().to_string()),
                }
        };

        let to_str = try!(poss.next().ok_or("Invalid backgammon notation"));
        let to = match to_str {
            "off" =>
                match self.color {
                    Color::Red => ::rustgammon::BEARING_OFF_POS,
                    Color::White => ::rustgammon::BEARING_OFF_POS,
                },
            _ =>
                match to_str.parse::<usize>() {
                    Ok(pos) => 
                        match self.color {
                            Color::Red => game.get_opposite_pos(pos),
                            Color::White => pos,
                        },
                    Err(e) => return Err(e.description().to_string()),
                }
        };

        let die = if from < to {
            // The case for bearing off is handled in parse_move().
            to - from
        } else {
            return Err("move is in the wrong direction".to_string());
        };
        if !self.is_valid_pos(from) {
            return Err("position out of range".to_string());
        }
        if !self.is_valid_pos(to) {
            return Err("position out of range".to_string());
        }
        return Ok(Submove { from: from, die: die });
    }

    fn parse_move(&self, game: &Backgammon, roll: DiceRoll, command: &str)
    -> Result<Move, CommandLineError> {
        let submove_iter = command.split_whitespace();
        let mut submoves = vec!();
        for submove in submove_iter {
            submoves.push(try!(self.parse_submove(game, submove)));
        }
        // Handle die for bearing off.
        if roll.0 == roll.1 {
            submoves[0].die = roll.0;
            submoves[1].die = roll.0;
            submoves[2].die = roll.0;
            submoves[3].die = roll.0;
        } else if submoves.len() == 2 {
            let sub1 = submoves[0];
            let sub2 = submoves[1];
            let sub1_bearing_off = sub1.from + sub1.die == ::rustgammon::BEARING_OFF_POS;
            let sub2_bearing_off = sub2.from + sub2.die == ::rustgammon::BEARING_OFF_POS;
            if !sub1_bearing_off && sub2_bearing_off {
                if sub1.die == roll.0 {
                    submoves[1].die = roll.1;
                } else {
                    submoves[1].die = roll.0;
                }
            } else if sub1_bearing_off && !sub2_bearing_off {
                if sub2.die == roll.0 {
                    submoves[0].die = roll.1;
                } else {
                    submoves[0].die = roll.0;
                }
            } else if sub1_bearing_off && sub2_bearing_off {
                if sub1.die < sub2.die {
                    submoves[0].die = cmp::min(roll.0, roll.1);
                    submoves[1].die = cmp::max(roll.0, roll.1);
                } else {
                    submoves[0].die = cmp::max(roll.0, roll.1);
                    submoves[1].die = cmp::min(roll.0, roll.1);
                }
            }
        }
        // Submoves are treated as a stack, so reverse it.
        submoves.reverse();
        return Ok(Move { submoves: submoves });
    }

    pub fn parse_command(&self, game: &Backgammon, roll: DiceRoll, command: &str)
        -> Result<Move, CommandLineError> {
        return match command {
            "list" => {
                let moves = game.list_moves(self.get_color(), roll);
                for mov in &moves {
                    mov.print();
                }
                Err("".to_string())
            },
            "show" => {
                game.print();
                Err("".to_string())
            },
            _ => self.parse_move(game, roll, command),
        }
    }
}

