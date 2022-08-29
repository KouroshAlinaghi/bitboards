use super::bitboard::{Bitboard, Bitwise};
use super::game::Game;
use super::position::Position;
use super::piece::PieceKind;

// N | -A -B -C -D -E -F -G -H  | N
// --------------------------------
// 8 | 56 56 58 59 60 61 62 63 | 8
// 7 | 48 49 50 51 52 53 54 55 | 7
// 6 | 40 41 42 43 44 45 46 47 | 6
// 5 | 32 33 34 35 36 37 38 39 | 5
// 4 | 24 25 26 27 28 29 30 31 | 4
// 3 | 16 17 18 19 20 21 22 23 | 3
// 2 | 18 09 10 11 12 13 14 15 | 2
// 1 | 00 01 02 03 04 05 06 07 | 0
// --------------------------------
// N | -A -B -C -D -E -F -G -H  | N

pub fn piece_attacks(piece_kind, PieceKind, game: Game, index: u8) {
    // true if it's white to move, false if it's
    // black to move.
    let side = game.played_moves & 2 == 0;

    return match piece_kind {
        King =>   king_atatcks(game, index, side),
        Rook =>   rook_attacks(game, index, side),
        Queen =>  queen_attacks(game, index, side),
        Knight => knight_attacks(game, index, side),
        Bishop => Bishop_attacks(game, index, side),
        Pawn =>   pawn_attacks(game, index, side),
    }
}

fn rook_attacks(game: Game, index: u8, side) -> Bitboard {
    horizontall_moves(&game.position, index, side) |
    verticall_moves(&game.position, index, side)
}

fn horizontall_moves(p: &Position, i: u8, side: bool) -> Bitboard {
    legal_moves_horizonetally(&p, i-1, side, true) |
    legal_moves_horizonetally(&p, i+1, side, false)
}

fn verticall_moves(p: &Position, i: u8, side: bool) -> Bitboard {
    legal_moves_vertically(&p, i+8, side, true) |
    legal_moves_vertically(&p, i.saturating_sub(8), side, false) 
}

fn legal_moves_vertically(p: &Position, i: u8, side: bool, dir: bool) -> Bitboard {
    let up_or_down: i8 = if dir { 8 } else { -8 };
    let (file, _) = get_coordinations(i);

    let mut c_i = i as i8;
    let mut legal_indexes: Vec<u8> = vec![];

    loop {
        // Break if a piece from same color is there or we've
        // crossed to end of file.
        if c_i < 0 || c_i > 63 || p.get_bitboard_from_side(side).at(c_i as u8) || c_i%8 != file as i8 {
            break 
        }

        legal_indexes.push(c_i as u8);
        
        // This is after the push because capture is legal.
        // NOTE: It is safe to ignore king capture here, since
        // is not possbile that enemy's king be in piece's
        // attacking squares. because then wont be this side
        // to move.
        if p.get_bitboard_from_side(!side).at(c_i as u8) { break }

        // go to up if dir (c_i += 8), otherwise go to
        // down (c_i -= 8)
        c_i = c_i + up_or_down;
    }

    Bitboard::from_indexes(legal_indexes)
}

fn legal_moves_horizonetally(p: &Position, i: u8, side: bool, dir: bool) -> Bitboard {
    let left_or_right: i8 = if dir { -1 } else { 1 };
    let (_, rank) = get_coordinations(i);

    let mut c_i = i as i8;
    let mut legal_indexes: Vec<u8> = vec![];

    loop {
        // Break if a piece from same color is there or we've
        // crossed to end of rank.
        if c_i < 0 || c_i > 63 || p.get_bitboard_from_side(side).at(c_i as u8) || (c_i)/8 != rank as i8 {
            break 
        }

        legal_indexes.push(c_i as u8);
        
        // This is after the push because capture is legal.
        // NOTE: It is safe to ignore king capture here, since
        // is not possbile that enemy's king be in piece's
        // attacking squares. because then wont be this side
        // to move.
        if p.get_bitboard_from_side(!side).at(c_i as u8) { break }

        // go to left if dir (c_i -= 1), otherwise go to
        // right (c_i += 1)
        c_i = c_i + left_or_right;
    }

    Bitboard::from_indexes(legal_indexes)
}

fn get_coordinations(index: u8) -> (u8, u8) {
    let file = index % 8;
    let rank = (index)/8;
    (file, rank)
}