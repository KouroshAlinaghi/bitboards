use std::fmt::{self, Display};

fn main() {
    let bitboard = Board::default();
    println!("{}", bitboard);
}

struct Board {
    white_pieces: u64,
    black_pieces: u64,
    pawns: u64,
    rooks: u64,
    kings: u64,
    queens: u64,
    knights: u64,
    bishops: u64,
}

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

impl Board {
    fn default() -> Board {
        Board {
            // RANK 1 - RANK 2 - RANK 3 - RANK 4 - RANK 5 - RANK 6 - RANK 7 - RANK 8
            white_pieces: 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            black_pieces: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
            rooks: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            knights: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            bishops: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            queens: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            kings: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            pawns: 0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
            //ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH-ABCDEFGH
        }
    }

    fn at(binary: u64, i: i32) -> u64 {
        binary >> 63 - i & 1
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "".to_string();
        let mut item = " ";

        out.push_str("\n +---+---+---+---+---+---+---+---+\n");

        for r in (0..=7).rev() {
            for f in 0..=7 {
                out.push_str(" | ");

                let i = r * 8 + f;

                if Board::at(self.white_pieces, i) == 1 {
                    if Board::at(self.pawns, i) == 1 {
                        item = "♟";
                    } else if Board::at(self.bishops, i) == 1 {
                        item = "♝";
                    } else if Board::at(self.knights, i) == 1 {
                        item = "♞";
                    } else if Board::at(self.rooks, i) == 1 {
                        item = "♜";
                    } else if Board::at(self.kings, i) == 1 {
                        item = "♚";
                    } else if Board::at(self.queens, i) == 1 {
                        item = "♛";
                    }
                } else if Board::at(self.black_pieces, i) == 1 {
                    if Board::at(self.pawns, i) == 1 {
                        item = "♙";
                    } else if Board::at(self.bishops, i) == 1 {
                        item = "♗";
                    } else if Board::at(self.knights, i) == 1 {
                        item = "♘";
                    } else if Board::at(self.rooks, i) == 1 {
                        item = "♖";
                    } else if Board::at(self.kings, i) == 1 {
                        item = "♔";
                    } else if Board::at(self.queens, i) == 1 {
                        item = "♕";
                    }
                } else {
                    item = " ";
                }

                out.push_str(item);
            }
            out.push_str(" |\n +---+---+---+---+---+---+---+---+\n");
        }

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn build_at_fn() {
        let b1: u64 = 0b1101010111010101110101011101010111010101110101011101010111010101;
        let b2: u64 = 0b1010101110101011101010111010101110101011101010111010101110101011;
        assert_eq!(Board::at(b1, 0), 1);
        assert_eq!(Board::at(b1, 7), 1);
        assert_eq!(Board::at(b2, 1), 0);
        assert_eq!(Board::at(b2, 6), 1);
    }
}
