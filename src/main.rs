use board::Player;
use game::Game;

mod board;
mod game;
mod utils;

fn main() {
    Game::new(Player::X).start();
}
