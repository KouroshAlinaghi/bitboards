#![allow(dead_code)]

use super::color_side::*;

pub mod piece_attacks;

pub type PieceKind = u8;
pub type Piece     = u8; // first bit shows color, other other 3
                         // bits represent piece kind.

// Same order should be used for a Position array;                         
pub const PAWN:   PieceKind = 0b0000; // 0
pub const KING:   PieceKind = 0b0001; // 1
pub const ROOK:   PieceKind = 0b0010; // 2
pub const QUEEN:  PieceKind = 0b0011; // 3
pub const KNIGHT: PieceKind = 0b0100; // 4
pub const BISHOP: PieceKind = 0b0101; // 5

pub trait Pieceable {
    /// Returns a new Piece.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let p = Piece.new(ROOK, WHITE);
    /// 
    /// assert_eq!(p, 0b1010);
    /// ```
    fn new(kind: PieceKind, color: ColorSide) -> Piece;

    /// Returns chess symbol for the given piece.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let p = Piece.new(QUEEN, BLACK);
    /// 
    /// assert_eq!(p.to_char(), '♕');
    /// ```
    fn to_char(&self) -> char;

    /// Returns the color of the piece.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let p = Piece.new(PAWN, WHITE);
    /// 
    /// assert_eq!(p.color(), WHITE);
    /// ```
    fn color(&self) -> ColorSide;

    /// Returns the kind of the piece.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let p = Piece.new(PAWN, WHITE);
    /// 
    /// assert_eq!(p.kind(), PAWN);
    /// ```
    fn kind(&self) -> PieceKind;
}

impl Pieceable for Piece {
    fn new(kind: PieceKind, color: ColorSide) -> Piece {
        color | kind
    }

    fn color(&self) -> ColorSide {
        self & 0b1000
    }

    fn kind(&self) -> PieceKind {
        self & 0b0111
    }

    fn to_char(&self) -> char {
        match self.color() {
            WHITE => {
                match self.kind() {
                    KING => '♚',
                    QUEEN => '♛',
                    ROOK => '♜',
                    BISHOP => '♝',
                    KNIGHT => '♞',
                    PAWN => '♟',
                    _ => ' ',
                }
            }
            BLACK => {
                match self.kind() {
                    KING => '♔',
                    QUEEN => '♕',
                    ROOK => '♖',
                    BISHOP => '♗',
                    KNIGHT => '♘',
                    PAWN => '♙',
                    _ => ' ',
                }
            }
            _ => ' ',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p1 = Piece::new(QUEEN, WHITE);
        let p2 = Piece::new(PAWN, BLACK);
        assert_eq!(p1, 0b1011);
        assert_eq!(p2, 0b0000);
    }

    #[test]
    fn test_color() {
        let p1 = Piece::new(QUEEN, WHITE);
        let p2 = Piece::new(PAWN, BLACK);
        assert_eq!(p1.color(), WHITE);
        assert_eq!(p2.color(), BLACK);
    }

    #[test]
    fn test_kind() {
        let p1 = Piece::new(QUEEN, WHITE);
        let p2 = Piece::new(PAWN, BLACK);
        assert_eq!(p1.kind(), QUEEN);
        assert_eq!(p2.kind(), PAWN);
    }
}