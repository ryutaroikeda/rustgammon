#[macro_use]
extern crate log;
extern crate log4rs;

extern crate rustgammon;

use rustgammon::rustgammon::Backgammon;
use rustgammon::rustgammon::Color;
use rustgammon::commandlineplayer::CommandLinePlayer;
use rustgammon::randomplayer::RandomPlayer;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("rustgammon - Backgammon implementation in Rust");

    let mut game: Backgammon = Default::default();
    game.init();

    let cmd_player = CommandLinePlayer { color: Color::Red };
    //let second_player = CommandLinePlayer { color: Color::White };
    let second_player = RandomPlayer { color: Color::White};

    game.run(&cmd_player, &second_player);
}
