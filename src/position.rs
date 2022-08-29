use super::bitboard::*;
use super::piece::*;
use super::square::*;
use super::color_side::*;


/// Position is an array with 8 bitboards:
/// 
/// First 6 items for each piece kind.
/// Last 2 items for each color.
/// 
/// The order for piece kinds should follow the order
/// used in piece.rs. (index: 0 to 5)
/// 
/// The order for color sides is WHITE (index: 6) and 
/// then BLACK (index: 7), by convention.

pub type Position = [Bitboard; 8];

pub trait Positionable {
    fn initial() -> Position {
        [
            Bitboard::initial_from_piece_kind(PAWN),
            Bitboard::initial_from_piece_kind(KING),
            Bitboard::initial_from_piece_kind(ROOK),
            Bitboard::initial_from_piece_kind(QUEEN),
            Bitboard::initial_from_piece_kind(KNIGHT),
            Bitboard::initial_from_piece_kind(BISHOP),
            Bitboard::initial_from_piece_color(WHITE),
            Bitboard::initial_from_piece_color(BLACK),
        ] as Position
    }
    fn from_piece_color(&self, color: ColorSide) -> Bitboard;
    fn from_piece_kind(&self, kind: PieceKind) -> Bitboard;
    fn from_piece(&self, piece: Piece) -> Bitboard;
    fn from_square(&self, square: Square) -> Option<Piece>;
}

impl Positionable for Position {
    fn from_piece_color(&self, color: ColorSide) -> Bitboard {
        if color == WHITE {
            self[6]
        } else {
            self[7]
        }
    }

    fn from_piece_kind(&self, kind: PieceKind) -> Bitboard {
        self[kind as usize]
    }

    fn from_piece(&self, piece: Piece) -> Bitboard {
        self.from_piece_color(piece.color()) & self.from_piece_kind(piece.kind())
    }

    fn from_square(&self, square: Square) -> Option<Piece> {
        let mut kind: Option<PieceKind> = None;
        for p in 0..=5 {
            if self.from_piece_kind(p as PieceKind).at(square) {
                kind = Some(p as PieceKind);
                break
            }
        };

        if kind == None {
            return None
        } else if self.from_piece_color(WHITE).at(square) {
            Some(Piece::new(kind.unwrap(), WHITE))
        } else {
            Some(Piece::new(kind.unwrap(), BLACK))
        }
    }
}