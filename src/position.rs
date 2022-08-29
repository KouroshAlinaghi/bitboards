use super::bitboard::Bitboard;

/// Position struct contains 8 Bitboards:
/// 6 for each pieces type and
/// 2 for each color
pub struct Position {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub pawns: Bitboard,
    pub rooks: Bitboard,
    pub kings: Bitboard,
    pub queens: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
}

impl Position {
    pub fn get_bitboard_from_side(&self, side: bool) -> Bitboard {
        return if side { self.white_pieces } else { self.black_pieces }
    }
}
