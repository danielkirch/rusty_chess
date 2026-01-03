#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Pieces {
    Empty = 0,
    WhiteKing = 1,
    WhiteQueen = 2,
    WhiteRook = 3,
    WhiteBishop = 4,
    WhiteKnight = 5,
    WhitePawn = 6,
    BlackKing = 7,
    BlackQueen = 8,
    BlackRook = 9,
    BlackBishop = 10,
    BlackKnight = 11,
    BlackPawn = 12,
}

pub const MAX_GAME_HISTORY_LENGTH: usize = 1024;
