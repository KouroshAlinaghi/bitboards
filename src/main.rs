mod game;
use game::Game;
use bitboard::*;

mod bitboard;
mod position;
mod piece;
mod square;
mod color_side;
mod dir;

fn main() {
    // let game = Game::default();
    // println!("{}", game);


    // Same order should be used for a Position array;                         
    // pub const PAWN:   PieceKind = 0b0000; // 0
    // pub const KING:   PieceKind = 0b0001; // 1
    // pub const ROOK:   PieceKind = 0b0010; // 2
    // pub const QUEEN:  PieceKind = 0b0011; // 3
    // pub const KNIGHT: PieceKind = 0b0100; // 4
    // pub const BISHOP: PieceKind = 0b0101; // 5
    
    let position = 
        [   // RANK 1 - RANK 2 - RANK 3 - RANK 4 - RANK 5 - RANK 6 - RANK 7 - RANK 8
            0b00000000_00000000_00000100_00000000_00000000_00000000_00000000_00000000,
            0b00000000_00000000_00000000_00000000_00000000_00100000_00000000_00000000,
            0b00000000_00100000_00000000_00000000_00000000_00001000_00000000_00000000,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            0b00000000_00000000_00000000_00010000_00000000_00000000_00000000_00000000,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            0b00100000_00100000_00000000_00010000_00000000_00110000_00000000_00000000,
            0b00000000_00000000_00000100_00000000_00000000_00001000_00000000_00000000,
        ];

    let rook_game = Game {
        played_moves: 0,
        position,
    };

    println!("{}", rook_game);

    piece::piece_attacks::piece_attacks(piece::KNIGHT, rook_game, 27).draw();
}