use crate::constants::Pieces;

#[derive(Clone, Copy)]
pub struct Board {
    // a1 -> bit 0, h8 -> bit 63
    white_pieces: u64,
    black_pieces: u64,
    occupied_squares: u64,

    kings: u64,
    queens: u64,
    rooks: u64,
    bishops: u64,
    knights: u64,
    pawns: u64,

    pub piece_list: [Pieces; 64], // Maps square index to piece type
}

impl Board {
    fn new() -> Self {
        Board {
            white_pieces: 0,
            black_pieces: 0,
            occupied_squares: 0,
            kings: 0,
            queens: 0,
            rooks: 0,
            bishops: 0,
            knights: 0,
            pawns: 0,
            piece_list: [Pieces::Empty; 64],
        }
    }

    fn init_piece_list(&mut self) {
        for i in 0..64 {
            let field: u64 = 1 << i;
            if self.occupied_squares & field == 0 {
                self.piece_list[i] = Pieces::Empty;
            } else if self.kings & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Pieces::WhiteKing;
                } else {
                    self.piece_list[i] = Pieces::BlackKing;
                }
            } else if self.queens & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Pieces::WhiteQueen;
                } else {
                    self.piece_list[i] = Pieces::BlackQueen;
                }
            } else if self.rooks & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Pieces::WhiteRook;
                } else {
                    self.piece_list[i] = Pieces::BlackRook;
                }
            } else if self.bishops & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Pieces::WhiteBishop;
                } else {
                    self.piece_list[i] = Pieces::BlackBishop;
                }
            } else if self.knights & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Pieces::WhiteKnight;
                } else {
                    self.piece_list[i] = Pieces::BlackKnight;
                }
            } else if self.pawns & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Pieces::WhitePawn;
                } else {
                    self.piece_list[i] = Pieces::BlackPawn;
                }
            }
        }
    }

    pub fn starting_position() -> Self {
        let mut board = Board {
            white_pieces: 0x000000000000FFFF,
            black_pieces: 0xFFFF000000000000,
            occupied_squares: 0xFFFF00000000FFFF,
            kings: 0x1000000000000010,
            queens: 0x0800000000000008,
            rooks: 0x8100000000000081,
            bishops: 0x2400000000000024,
            knights: 0x4200000000000042,
            pawns: 0x00FF00000000FF00,
            piece_list: [Pieces::Empty; 64],
        };
        board.init_piece_list();
        board
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        // Implementation to parse FEN string and initialize the board
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("Invalid FEN string".to_string());
        }

        let pieces: Vec<&str> = parts[0].split('/').collect();
        if pieces.len() != 8 {
            return Err("Invalid FEN string".to_string());
        }

        // Initialize the board
        let mut board = Board::new();

        // Parse piece placement
        for (rank, piece_row) in pieces.iter().rev().enumerate() {
            if piece_row.len() > 8 {
                return Err("Invalid FEN string".to_string());
            }
            let mut square = rank * 8;
            for piece in piece_row.chars() {
                if piece.is_digit(10) {
                    let empty_squares = piece.to_digit(10).unwrap() as usize;
                    square += empty_squares;
                    continue;
                }

                board.occupied_squares |= 1 << square;
                // match color
                match piece {
                    'R' | 'B' | 'N' | 'Q' | 'K' | 'P' => board.white_pieces |= 1 << square,
                    'r' | 'b' | 'n' | 'q' | 'k' | 'p' => board.black_pieces |= 1 << square,
                    _ => return Err("Invalid FEN string".to_string()),
                }
                // match piece type
                match piece {
                    'R' | 'r' => board.rooks |= 1 << square,
                    'B' | 'b' => board.bishops |= 1 << square,
                    'N' | 'n' => board.knights |= 1 << square,
                    'Q' | 'q' => board.queens |= 1 << square,
                    'K' | 'k' => board.kings |= 1 << square,
                    'P' | 'p' => board.pawns |= 1 << square,
                    _ => return Err("Invalid FEN string".to_string()),
                }
                square += 1;
            }
        }
        board.init_piece_list();

        Ok(board)
    }
}

pub fn print_bitboard(bitboard: u64) -> String {
    let mut board = String::new();
    for rank in (0..8).rev() {
        for file in 0..8 {
            let mask = 1u64 << (rank * 8 + file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            board.push(char);
        }
        board.push('\n');
    }
    println!("{}", board);
    board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position() {
        let board = Board::starting_position();
        assert_eq!(board.white_pieces, 0x000000000000FFFF);
        assert_eq!(board.black_pieces, 0xFFFF000000000000);
        assert_eq!(board.kings, 0x1000000000000010);
        assert_eq!(board.queens, 0x0800000000000008);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.pawns, 0x00FF00000000FF00);
    }

    #[test]
    fn test_from_fen() {
        // Test parsing starting position FEN
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen).unwrap();
        assert_eq!(board.white_pieces, 0x000000000000FFFF);
        assert_eq!(board.black_pieces, 0xFFFF000000000000);
        assert_eq!(board.occupied_squares, 0xFFFF00000000FFFF);
        assert_eq!(board.kings, 0x1000000000000010);
        assert_eq!(board.queens, 0x0800000000000008);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.pawns, 0x00FF00000000FF00);
        assert_eq!(board.piece_list[0], Pieces::WhiteRook);
        assert_eq!(board.piece_list[60], Pieces::BlackKing);
    }

    #[test]
    fn test_from_fen_e4e5_opening() {
        // Test parsing a FEN string after 1. e4 e5
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2";
        let board = Board::from_fen(fen).unwrap();
        assert_eq!(board.white_pieces, 0x000000001000EFFF);
        assert_eq!(board.black_pieces, 0xFFEF001000000000);
        assert_eq!(board.kings, 0x1000000000000010);
        assert_eq!(board.queens, 0x0800000000000008);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.pawns, 0x00EF00101000EF00); // e4 and e5 pawns
        assert_eq!(board.piece_list[28], Pieces::WhitePawn); // e4
        assert_eq!(board.piece_list[36], Pieces::BlackPawn); // e5
    }

    #[test]
    fn test_from_fen_bongcloud_opening() {
        // Test parsing a FEN string after 1. e4 e5 2. Ke2
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 1 2";
        let board = Board::from_fen(fen).unwrap();
        assert_eq!(board.white_pieces, 0x000000001000FFEF);
        assert_eq!(board.black_pieces, 0xFFEF001000000000);
        assert_eq!(board.occupied_squares, 0xFFEF00101000FFEF);
        assert_eq!(board.kings, 0x1000000000001000);
        assert_eq!(board.queens, 0x0800000000000008);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.pawns, 0x00EF00101000EF00); // e4 and e5 pawns
        assert_eq!(board.piece_list[28], Pieces::WhitePawn); // e4
        assert_eq!(board.piece_list[36], Pieces::BlackPawn); // e5
        assert_eq!(board.piece_list[12], Pieces::WhiteKing); // e2
    }

    #[test]
    fn test_print_bitboard() {
        let bitboard: u64 = 0xFFEF00101000FFEF;
        let expected_output =
            "11111111\n11110111\n00000000\n00001000\n00001000\n00000000\n11111111\n11110111\n";
        let output = print_bitboard(bitboard);
        assert_eq!(output, expected_output);
    }
}
