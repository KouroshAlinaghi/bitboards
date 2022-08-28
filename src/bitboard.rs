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
/// and indexing on bits
pub trait Bitwise {
    /// Returns the bit (0 or 1) which is at 
    /// index i in self. (index starting from 0)
    ///
    /// # Examples
    ///
    /// ```
    /// let b: Bitboard = 0b0000000000000000000000000000000000000000000000000000000000000001;
    ///
    /// assert_eq!(b.at(34), 0);
    // assert_eq!(b.at(63), 1);
    /// ```
    fn at(&self, i: i32) -> Bitboard;
}

impl Bitwise for Bitboard {
    fn at(&self, i: i32) -> Self {
        self >> 63 - i & 1
    }
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, Bitwise};

    #[test]
    fn build_at_fn() {
        let b1: Bitboard = 0b1101010111010101110101011101010111010101110101011101010111010101;
        let b2: Bitboard = 0b1010101110101011101010111010101110101011101010111010101110101011;
        assert_eq!(b1.at(0), 1);
        assert_eq!(b1.at(7), 1);
        assert_eq!(b2.at(1), 0);
        assert_eq!(b2.at(6), 1);
    }
}
