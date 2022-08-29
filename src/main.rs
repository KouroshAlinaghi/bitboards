mod game;
use game::Game;

use crate::position::Position;
use crate::bitboard::*;
mod bitboard;
mod position;
mod attacks;

fn main() {
    // let game = Game::default();
    // println!("{}", game);

    let position = 
        Position {        // RANK 1 - RANK 2 - RANK 3 - RANK 4 - RANK 5 - RANK 6 - RANK 7 - RANK 8
            white_pieces: 0b01000000_00000000_01000000_00000000_00000000_00000000_00000000_00000000,
            black_pieces: 0b00000000_00000000_00000000_00000000_00000000_01000000_00000000_00000000,
            rooks:        0b00000000_00000000_01000000_00000000_00000000_00000000_00000000_00000000,
            knights:      0b01000000_00000000_00000000_00000000_00000000_01000000_00000000_00000000,
            bishops:      0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            queens:       0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            kings:        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            pawns:        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        };

    let rook_game = Game {
        played_moves: 0,
        position,
    };

    println!("{}", rook_game);

    attacks::rook_attacks(rook_game, 17).draw();
}