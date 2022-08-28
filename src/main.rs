mod game;
use game::Game;
mod bitboard;
mod position;

fn main() {
    let game = Game::default();
    println!("{}", game);
}