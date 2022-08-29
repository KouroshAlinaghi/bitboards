use super::super::bitboard::*;
use super::super::game::Game;
use super::super::position::*;
use super::super::square::*;
use super::super::piece::*;
use super::super::dir::{Dir, Dir::*};
use super::super::color_side::*;


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

/// Generates attacking squares bitboard (captures) for
/// given piece type in a given game and square. Note that
/// this is equal to legal moves for all of the pieces, except
/// for pawns, which have different attacking squares and 
/// squares they can be pushed to.
pub fn piece_attacks(piece_kind: PieceKind, game: Game, square: Square) -> Bitboard {
    let side = if game.played_moves % 2 == 0 { WHITE } else { BLACK };

    return match piece_kind {
        KING =>   {
            neighborhood_attacks(&game, square, side, vec![Up, Right, Down, Left, UpLeft, UpRight, DownLeft, DownRight])
        }
        ROOK =>   {
            slider_attacks(&game, square, side, vec![Up, Right, Down, Left])
        }
        QUEEN =>  {
            slider_attacks(&game, square, side, vec![Up, Right, Down, Left, UpLeft, UpRight, DownRight, DownLeft])
        }
        BISHOP => {
            slider_attacks(&game, square, side, vec![UpLeft, UpRight, DownRight, DownLeft])
        }
        KNIGHT => knight_attacks(&game, square, side),
        PAWN =>   pawn_attacks(&game, square, side),
        _ => Bitboard::new(),
    }
}

fn knight_attacks(game: &Game, s: Square, side: ColorSide) -> Bitboard {
    let mut legal_squares: Vec<Square> = vec![];
    for dir in [HorseUpLeft, HorseUpRight, HorseDownRight, HorseDownLeft, HorseLeftDown, HorseLeftUp, HorseRightDown, HorseRightUp] {
        match s.get(&dir) {
            Some(target) => {
                match game.position.from_square(target) {
                    None => legal_squares.push(target),
                    Some(piece) => {
                        if piece.color() != side {
                            legal_squares.push(target)
                        }
                    }
                }
            }
            None => {},
        }
    }

    Bitboard::from_squares(legal_squares)   
}

fn pawn_attacks(game: &Game, s: Square, side: ColorSide) -> Bitboard {
    if side == WHITE {
        neighborhood_attacks(game, s, side, vec![UpLeft, UpRight])
    } else {
        neighborhood_attacks(game, s, side, vec![DownLeft, DownRight])
    }
}

fn slider_attacks(
    game: &Game,
    s: Square,
    side: ColorSide,
    dirs: Vec<Dir>,
) -> Bitboard {
    let mut legal_squares: Vec<Square> = vec![];
    let mut square = s;
    for dir in dirs {
        loop {
            match square.get(&dir) {
                Some(target) => {
                    match game.position.from_square(target) {
                        None => {
                            legal_squares.push(target);
                            square = target;
                        }
                        Some(piece) => {
                            if piece.color() != side {
                                legal_squares.push(target);
                                // TODO: explain why
                                if piece.kind() != KING {
                                    break;
                                }
                                square = target;
                            } else {
                                break;
                            }
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        square = s;
    }

    Bitboard::from_squares(legal_squares)

}

fn neighborhood_attacks(
    game: &Game,
    s: Square,
    side: ColorSide,
    dirs: Vec<Dir>,
) -> Bitboard {
    let mut legal_squares: Vec<Square> = vec![];
    for dir in dirs {
        match s.get(&dir) {
            Some(target) => {
                match game.position.from_square(target) {
                    None => legal_squares.push(target),
                    Some(piece) => {
                        if piece.color() != side {
                            legal_squares.push(target)
                        }
                    }
                }
            }
            None => {},
        }
    }

    Bitboard::from_squares(legal_squares)
}