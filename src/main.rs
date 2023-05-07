mod player;
mod card;
mod market;
mod game;

use crate::game::Game;


fn main() {
    let mut game:Game = Game::new();
    game.run();
}
