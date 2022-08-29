use std::fmt::{self, Display};
use super::position::Position;
use super::bitboard::*;

pub struct Game {
    pub position: Position,
    pub played_moves:  usize,
}

impl Game {
    /// Returns a new Game {} with inital chess board defaults
    pub fn default() -> Game {
        Game {
            position: 
                Position {        // RANK 1 - RANK 2 - RANK 3 - RANK 4 - RANK 5 - RANK 6 - RANK 7 - RANK 8
                    white_pieces: 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
                    black_pieces: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
                    rooks:        0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
                    knights:      0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
                    bishops:      0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
                    queens:       0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
                    kings:        0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
                    pawns:        0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
                },                //ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH
            played_moves: 0,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "".to_string();
        let mut item = " ";

        out.push_str("\n +---+---+---+---+---+---+---+---+\n");

        // ugly code goes brrrrrrrrrrrrrrrrrrrrrrrrr
        //
        // but it doesn't matter, this function is
        // going to be called when printing the game 
        for r in (0..=7).rev() {
            for f in 0..=7 {
                out.push_str(" | ");

                let i = r * 8 + f;

                if self.position.white_pieces.at(i) {
                    if self.position.pawns.at(i) {
                        item = "♟";
                    } else if self.position.bishops.at(i) {
                        item = "♝";
                    } else if self.position.knights.at(i) {
                        item = "♞";
                    } else if self.position.rooks.at(i) {
                        item = "♜";
                    } else if self.position.queens.at(i) {
                        item = "♛";
                    } else if self.position.kings.at(i) {
                        item = "♚";
                    }
                } else if self.position.black_pieces.at(i) {
                    if self.position.pawns.at(i) {
                        item = "♙";
                    } else if self.position.bishops.at(i) {
                        item = "♗";
                    } else if self.position.knights.at(i) {
                        item = "♘";
                    } else if self.position.rooks.at(i) {
                        item = "♖";
                    } else if self.position.queens.at(i) {
                        item = "♕";
                    } else if self.position.kings.at(i) {
                        item = "♔";
                    }
                } else {
                    item = " ";
                }

                out.push_str(item);
            }
            out.push_str(" |\n +---+---+---+---+---+---+---+---+\n");
        }
        out.push_str(format!(" {} Moves played", self.played_moves).as_str());

        write!(f, "{}", out)
    }
}