use std::fmt::{self, Display};
use super::position::*;
use super::piece::*;
use super::square::*;

pub struct Game {
    pub position: Position,
    pub played_moves:  usize,
}

impl Game {
    /// Returns a new Game {} with inital chess board defaults
    pub fn default() -> Game {
        Game {
            position: Position::initial(),
            played_moves: 0,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "".to_string();
        let mut item: char;

        out.push_str("\n +---+---+---+---+---+---+---+---+\n");

        for r in (0..=7).rev() {
            for f in 0..=7 {
                out.push_str(" | ");

                let square: Square = r * 8 + f;

                item = match self.position.from_square(square) {
                    Some(p) => p.to_char(),
                    None => ' ',
                };

                out.push_str(item.to_string().as_str());
            }

            out.push_str(" |\n +---+---+---+---+---+---+---+---+\n");
        }

        out.push_str(format!(" {} Moves played", self.played_moves).as_str());

        write!(f, "{}", out)
    }
}