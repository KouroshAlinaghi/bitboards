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
