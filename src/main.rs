#[macro_use]
extern crate log;
extern crate log4rs;

extern crate rustgammon;

use rustgammon::rustgammon::Backgammon;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("rustgammon - Backgammon implementation in Rust");

    let mut game: Backgammon = Default::default();
    game.init();
}
