use crate::color_side::ColorSide;
use super::piece::*;
use super::square::*;
use super::color_side::*;

/// Bitboard type is a 64-bit binary which
/// each index of it represents following
/// squares on a chess board.
///
///
/// N | -A -B -C -D -E -F -G -H  | N
/// --------------------------------
/// 8 | 56 56 58 59 60 61 62 63 | 8
/// 7 | 48 49 50 51 52 53 54 55 | 7
/// 6 | 40 41 42 43 44 45 46 47 | 6
/// 5 | 32 33 34 35 36 37 38 39 | 5
/// 4 | 24 25 26 27 28 29 30 31 | 4
/// 3 | 16 17 18 19 20 21 22 23 | 3
/// 2 | 18 09 10 11 12 13 14 15 | 2
/// 1 | 00 01 02 03 04 05 06 07 | 0
/// --------------------------------
/// N | -A -B -C -D -E -F -G -H  | N

pub type Bitboard = u64;

/// defines functions to do bitwise operations 
/// and indexing on bits.
pub trait Bitboardable {
    // Prints a bitboard.
    fn draw(&self) {
        let mut out = "".to_string();

        out.push_str("\n +---+---+---+---+---+---+---+---+\n");

        for r in (0..=7).rev() {
            for f in 0..=7 {
                out.push_str(" | ");

                let i = r * 8 + f;
                if self.at(i) {
                    out.push_str("X");
                } else {
                    out.push_str(" ");
                }

            }
            out.push_str(" |\n +---+---+---+---+---+---+---+---+\n");
        }

        println!("{}", out);
    }

    /// Returns a new 64 bit Bitboard with just zeros.
    fn new() -> Bitboard {
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 as Bitboard
    }
    
    /// Returns a bitboard with ones in given indexes.
    fn from_squares(squares: Vec<Square>) -> Bitboard {
        let mut bb = Bitboard::new();
        
        for i in squares {
            bb.set(i, true);
        }

        bb
    }

    /// Returns the bit (true or false) which is at 
    /// index square in self.
    ///
    /// # Examples
    ///
    /// ```
    /// let b: Bitboard = 0b0000000000000000000000000000000000000000000000000000000000000001;
    ///
    /// assert_eq!(b.at(F4), false);
    /// assert_eq!(b.at(H8), true);
    /// ```
    fn at(&self, s: Square) -> bool;

    /// Sets the value of bit in index square of
    /// self to 'value'.
    /// 
    ///
    /// # Examples
    ///
    /// ```
    /// let b: Bitboard = 0b0000000000000000000000000000000000000000000000000000000000000001;
    /// 
    /// b.set(H8, true)
    /// assert_eq!(b.at(H8), true);
    /// 
    /// b.set(H8, false)
    /// assert_eq!(b.at(H8), false);
    /// ```
    fn set(&mut self, square: u8, value: bool);

    /// Returns bitboard for given piece kind (both colors) 
    /// from initial standard chess position.
    fn initial_from_piece_kind(kind: PieceKind) -> Bitboard {
        match kind {      // RANK 1 - RANK 2 - RANK 3 - RANK 4 - RANK 5 - RANK 6 - RANK 7 - RANK 8
            PAWN =>   0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
            KING =>   0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            ROOK =>   0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            QUEEN =>  0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            KNIGHT => 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            BISHOP => 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            _ =>          0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                          //ABCDEFGH_ABCDEFGH_ABCDEFGH_ABCDEFGH_ABCDEFGH_ABCDEFGH_ABCDEFGH_ABCDEFGH
        }
    }

    /// Returns bitboard for given color side (all pieces)
    /// from initial standard chess position.
    fn initial_from_piece_color(color: ColorSide) -> Bitboard {
        if color == WHITE {
            0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000
        } else if color == BLACK {
            0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111
        } else {
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
        }
    }
}

impl Bitboardable for Bitboard {
    fn at(&self, s: Square) -> bool {
        assert!(s < 64);

        (self >> 63 - s & 1) != 0
    }

    fn set(&mut self, square: u8, value: bool) {
        assert!(square < 64);
        if value {
            *self |= 1 << 63 - square;
        } else {
            *self &= !(1 << 63 - square);
        }
	}
}


#[cfg(test)]
mod tests {
    use super::{Bitboard, Bitboardable};
    use crate::square::*;

    #[test]
    fn at_fn() {
        let b1: Bitboard = 0b1101010111010101110101011101010111010101110101011101010111010101;
        let b2: Bitboard = 0b1010101110101011101010111010101110101011101010111010101110101011;
        assert_eq!(b1.at(H1), true);
        assert_eq!(b1.at(H1), true);
        assert_eq!(b2.at(B1), false);
        assert_eq!(b2.at(G1), true);
    }

    #[test]
    fn set_fn() {
        let mut b1: Bitboard = 0b0000000000000000000000000000000000000000000000000000000000000000;
        b1.set(A1, true);
        assert_eq!(b1.at(A1), true);
        b1.set(A1, false);
        assert_eq!(b1.at(A1), false);
        let mut b2: Bitboard = 0b1111111111111111111111111111111111111111111111111111111111111111;
        b2.set(H8, true);
        assert_eq!(b2.at(H8), true);
        b2.set(A1, false);
        assert_eq!(b2.at(A1), false);
    }
}
