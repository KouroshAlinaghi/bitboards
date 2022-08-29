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
pub trait Bitwise {
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
    fn from_indexes(indexes: Vec<u8>) -> Bitboard {
        let mut bb = Bitboard::new();
        
        for i in indexes {
            bb.set(i, true);
        }

        bb
    }

    /// Returns the bit (true or false) which is at 
    /// index 'i' in self. (index starting from 0)
    ///
    /// # Examples
    ///
    /// ```
    /// let b: Bitboard = 0b0000000000000000000000000000000000000000000000000000000000000001;
    ///
    /// assert_eq!(b.at(34), false);
    // assert_eq!(b.at(63), true);
    /// ```
    fn at(&self, i: u8) -> bool;

    /// Sets the value of bit in index 'place' of
    /// self to 'value'.
    /// 
    ///
    /// # Examples
    ///
    /// ```
    /// let b: Bitboard = 0b0000000000000000000000000000000000000000000000000000000000000001;
    /// 
    /// b.set(63, true)
    /// assert_eq!(b.at(63), true);
    /// 
    /// b.set(63, false)
    /// assert_eq!(b.at(63), false);
    /// ```
    fn set(&mut self, place: u8, value: bool);
}

impl Bitwise for Bitboard {
    fn at(&self, i: u8) -> bool {
        (self >> 63 - i & 1) != 0
    }

    fn set(&mut self, place: u8, value: bool) {
        if place <= 63 {
            if value {
                *self |= 1 << 63 - place;
            } else {
                *self &= !(1 << 63 - place);
            }
        }
	}
}


#[cfg(test)]
mod tests {
    use super::{Bitboard, Bitwise};

    #[test]
    fn at_fn() {
        let b1: Bitboard = 0b1101010111010101110101011101010111010101110101011101010111010101;
        let b2: Bitboard = 0b1010101110101011101010111010101110101011101010111010101110101011;
        assert_eq!(b1.at(0), true);
        assert_eq!(b1.at(7), true);
        assert_eq!(b2.at(1), false);
        assert_eq!(b2.at(6), true);
    }

    #[test]
    fn set_fn() {
        let mut b1: Bitboard = 0b0000000000000000000000000000000000000000000000000000000000000000;
        b1.set(0, true);
        assert_eq!(b1.at(0), true);
        b1.set(0, false);
        assert_eq!(b1.at(0), false);
        let mut b2: Bitboard = 0b1111111111111111111111111111111111111111111111111111111111111111;
        b2.set(63, true);
        assert_eq!(b2.at(63), true);
        b2.set(0, false);
        assert_eq!(b2.at(0), false);
    }
}
