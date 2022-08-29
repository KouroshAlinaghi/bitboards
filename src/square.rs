#![allow(dead_code)]

use super::dir::{Dir, Dir::*};
use move_possible::*;

pub type Square = u8;

pub const A1: Square = 0;
pub const B1: Square = 1;
pub const C1: Square = 2;
pub const D1: Square = 3;
pub const E1: Square = 4;
pub const F1: Square = 5;
pub const G1: Square = 6;
pub const H1: Square = 7;
pub const A2: Square = 8;
pub const B2: Square = 9;
pub const C2: Square = 10;
pub const D2: Square = 11;
pub const E2: Square = 12;
pub const F2: Square = 13;
pub const G2: Square = 14;
pub const H2: Square = 15;
pub const A3: Square = 16;
pub const B3: Square = 17;
pub const C3: Square = 18;
pub const D3: Square = 19;
pub const E3: Square = 20;
pub const F3: Square = 21;
pub const G3: Square = 22;
pub const H3: Square = 23;
pub const A4: Square = 24;
pub const B4: Square = 25;
pub const C4: Square = 26;
pub const D4: Square = 27;
pub const E4: Square = 28;
pub const F4: Square = 29;
pub const G4: Square = 30;
pub const H4: Square = 31;
pub const A5: Square = 32;
pub const B5: Square = 33;
pub const C5: Square = 34;
pub const D5: Square = 35;
pub const E5: Square = 36;
pub const F5: Square = 37;
pub const G5: Square = 38;
pub const H5: Square = 39;
pub const A6: Square = 40;
pub const B6: Square = 41;
pub const C6: Square = 42;
pub const D6: Square = 43;
pub const E6: Square = 44;
pub const F6: Square = 45;
pub const G6: Square = 46;
pub const H6: Square = 47;
pub const A7: Square = 48;
pub const B7: Square = 49;
pub const C7: Square = 50;
pub const D7: Square = 51;
pub const E7: Square = 52;
pub const F7: Square = 53;
pub const G7: Square = 54;
pub const H7: Square = 55;
pub const A8: Square = 56;
pub const B8: Square = 57;
pub const C8: Square = 58;
pub const D8: Square = 59;
pub const E8: Square = 60;
pub const F8: Square = 61;
pub const G8: Square = 62;
pub const H8: Square = 63;

pub struct Coordination(u8, u8);

pub trait Squarable {
    /// Returns a new Coordination(file, rank). (zero-indexed)
    /// NOTE: I recommend using file() and rank() methods
    /// sice they don't require working with Coordination struct
    /// anymore.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let Coordination(file, rank) = B8.coord();
    /// 
    /// assert_eq!(file, 1);
    /// assert_eq!(rank, 7);
    /// ```
    fn coord(&self) -> Coordination;

    /// Returns file of the square. again, zero-indexed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// assert_eq!(A7.file(), 0);
    /// ```
    fn file(&self) -> u8;

    /// Returns rank of the square.
    /// 
    /// # Examples
    /// 
    /// ```
    /// assert_eq!(A7.rank(), 6);
    /// ```
    fn rank(&self) -> u8;

    /// Returns file of the square with capital name
    /// of the file.
    /// 
    /// # Examples
    /// 
    /// ```
    /// assert_eq!(A7.file_as_char(), 'A');
    /// ```
    fn file_as_char(&self) -> char;

    /// Returns Some(Square) if a square in dir: dir of
    /// self is not out of board, if it's out of board
    /// returns None.
    /// 
    /// # Examples
    /// 
    /// ```
    /// assert_eq!(A2.get(Left), None);
    /// assert_eq!(A2.get(Up), Some(A3));
    /// ```
    fn get(&self, dir: &Dir) -> Option<Square>;
}

impl Squarable for Square {
    fn coord(&self) -> Coordination {
        assert!(*self < 64);
        
        Coordination(self.file(), self.rank())
    }

    fn file(&self) -> u8 {
        assert!(*self < 64);

        self % 8
    }

    fn rank(&self) -> u8 {
        assert!(*self < 64);

        (self)/8
    }

    fn file_as_char(&self) -> char {
        match self.file() {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            _ => ' ',
        }
    }

    fn get(&self, dir: &Dir) -> Option<Square> {
        let i: i8 = *self as i8;
        let target: i8 = match dir {
            Up => i + 8,
            Down => i - 8,
            Left => i - 1,
            Right => i + 1,
            UpLeft => i + 7,
            UpRight => i + 9,
            DownLeft => i - 9,
            DownRight => i - 7,
            HorseUpLeft => i + 15,
            HorseUpRight => i + 17,
            HorseRightUp => i + 10,
            HorseRightDown => i - 6,
            HorseDownRight => i - 15,
            HorseDownLeft => i - 17,
            HorseLeftDown => i - 10,
            HorseLeftUp => i + 6,
        };

        if is_possible(i, &dir) {
            Some(target as u8)
        } else {
            None
        }
    }
}

mod move_possible {
    use super::super::{square::*, dir::{Dir, Dir::*}};

    pub fn is_possible(index: i8, dir: &Dir) -> bool {
        if index < 0 || index > 63 {
            return false
        }
        
        let square: Square = index as Square;
        let Coordination(file, rank) = square.coord();

        match dir {
            Up => rank != 7,
            Down => rank != 0,
            Left => file != 0,
            Right => file != 7,
            UpLeft => file != 0 && rank != 7,
            UpRight => file != 7 && rank != 7,
            DownLeft => file != 0 && rank != 0,
            DownRight => file != 7 && rank != 0,
            HorseUpLeft => rank < 6 && file != 0,
            HorseUpRight => rank < 6 && file != 7,
            HorseRightUp => file < 6 && file != 7,
            HorseRightDown => file < 6 && rank != 0,
            HorseDownRight => file != 7 && rank > 1,
            HorseDownLeft => file != 0 && rank > 1,
            HorseLeftDown => file > 1 && rank != 0,
            HorseLeftUp => file > 1 && rank != 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn coord() {
        let Coordination(file, rank) = B8.coord();

        assert_eq!(file, 1);
        assert_eq!(rank, 7);
    }

    #[test]
    fn file() {
        assert_eq!(A7.file(), 0);
        assert_eq!(E4.file(), 4);
        assert_eq!(H2.file(), 7);
    }

    #[test]
    fn rank() {
        assert_eq!(A1.rank(), 0);
        assert_eq!(C3.rank(), 2);
        assert_eq!(F8.rank(), 7);
    }

    #[test]
    fn get() {
        assert_eq!(A2.get(&Dir::Left), None);
        assert_eq!(A2.get(&Dir::Up), Some(A3));
        assert_eq!(A2.get(&Dir::UpLeft), None);
        assert_eq!(A2.get(&Dir::UpRight), Some(B3));

        assert_eq!(A2.get(&Dir::HorseUpLeft), None);
        assert_eq!(B6.get(&Dir::HorseUpLeft), Some(A8));

        assert_eq!(H1.get(&Dir::HorseUpRight), None);
        assert_eq!(A2.get(&Dir::HorseUpRight), Some(B4));

        assert_eq!(G1.get(&Dir::HorseRightUp), None);
        assert_eq!(A2.get(&Dir::HorseRightUp), Some(C3));

        assert_eq!(G1.get(&Dir::HorseRightDown), None);
        assert_eq!(F8.get(&Dir::HorseRightDown), Some(H7));

        assert_eq!(C1.get(&Dir::HorseDownRight), None);
        assert_eq!(G3.get(&Dir::HorseDownRight), Some(H1));

        assert_eq!(C2.get(&Dir::HorseDownLeft), None);
        assert_eq!(E8.get(&Dir::HorseDownLeft), Some(D6));

        assert_eq!(H1.get(&Dir::HorseLeftDown), None);
        assert_eq!(C3.get(&Dir::HorseLeftDown), Some(A2));

        assert_eq!(G8.get(&Dir::HorseLeftUp), None);
        assert_eq!(H1.get(&Dir::HorseLeftUp), Some(F2));
    }
}