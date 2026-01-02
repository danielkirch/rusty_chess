struct Board {
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

    white_to_move: bool,
    allow_castling: u8,
    en_passant_square: Option<u8>,
    reversible_move_counter: u8,
    full_move_counter: u16,
}

impl Board {
    pub fn new() -> Self {
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
            white_to_move: true,
            allow_castling: 0,
            en_passant_square: None,
            reversible_move_counter: 0,
            full_move_counter: 1,
        }
    }

    pub fn starting_position() -> Self {
        Board {
            white_pieces: 0x000000000000FFFF,
            black_pieces: 0xFFFF000000000000,
            occupied_squares: 0xFFFF00000000FFFF,
            kings: 0x1000000000000010,
            queens: 0x0800000000000080,
            rooks: 0x8100000000000081,
            bishops: 0x2400000000000024,
            knights: 0x4200000000000042,
            pawns: 0x00FF00000000FF00,
            white_to_move: true,
            allow_castling: 0b1111,
            en_passant_square: None,
            reversible_move_counter: 0,
            full_move_counter: 1,
        }
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

        // Part 1: Parse piece placement
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

        // Part 2: Active color
        board.white_to_move = match parts[1] {
            "w" => true,
            "b" => false,
            _ => return Err("Invalid FEN string".to_string()),
        };

        // Part 3: Castling availability
        board.allow_castling = 0;
        for c in parts[2].chars() {
            match c {
                'K' => board.allow_castling |= 0b0001,
                'Q' => board.allow_castling |= 0b0010,
                'k' => board.allow_castling |= 0b0100,
                'q' => board.allow_castling |= 0b1000,
                '-' => (),
                _ => return Err("Invalid FEN string".to_string()),
            }
        }

        // Part 4: En passant target square
        if parts[3] != "-" {
            let file = parts[3].chars().next().unwrap() as u8 - 'a' as u8;
            let rank = parts[3].chars().nth(1).unwrap() as u8 - '1' as u8;
            board.en_passant_square = Some(file + rank * 8);
        }

        // Part 5: Halfmove clock
        board.reversible_move_counter = parts[4].parse().unwrap();

        // Part 6: Fullmove number
        board.full_move_counter = parts[5].parse().unwrap();

        Ok(board)
    }
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
        assert_eq!(board.queens, 0x0800000000000080);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.pawns, 0x00FF00000000FF00);
        assert!(board.white_to_move);
        assert_eq!(board.allow_castling, 0b1111);
        assert_eq!(board.en_passant_square, None);
        assert_eq!(board.reversible_move_counter, 0);
        assert_eq!(board.full_move_counter, 1);
    }

    #[test]
    fn test_from_fen() {
        // Test parsing starting position FEN
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen).unwrap();
        assert_eq!(board.white_pieces, 0x000000000000FFFF);
        assert_eq!(board.black_pieces, 0xFFFF000000000000);
        assert_eq!(board.kings, 0x1000000000000010);
        assert_eq!(board.queens, 0x0800000000000008);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.pawns, 0x00FF00000000FF00);
        assert!(board.white_to_move);
        assert_eq!(board.allow_castling, 0b1111);
        assert_eq!(board.en_passant_square, None);
        assert_eq!(board.reversible_move_counter, 0);
        assert_eq!(board.full_move_counter, 1);
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
        assert!(board.white_to_move);
        assert_eq!(board.allow_castling, 0b1111);
        assert_eq!(board.en_passant_square, Some(44)); // e6
        assert_eq!(board.reversible_move_counter, 0);
        assert_eq!(board.full_move_counter, 2);
    }

    #[test]
    fn test_from_fen_bongcloud_opening() {
        // Test parsing a FEN string after 1. e4 e5 2. Ke2
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 1 2";
        let board = Board::from_fen(fen).unwrap();
        assert_eq!(board.white_pieces, 0x000000001000FFEF);
        assert_eq!(board.black_pieces, 0xFFEF001000000000);
        assert_eq!(board.kings, 0x1000000000001000);
        assert_eq!(board.queens, 0x0800000000000008);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.pawns, 0x00EF00101000EF00); // e4 and e5 pawns
        assert!(!board.white_to_move);
        assert_eq!(board.allow_castling, 0b1100); // only black can castle
        assert_eq!(board.en_passant_square, None);
        assert_eq!(board.reversible_move_counter, 1);
        assert_eq!(board.full_move_counter, 2);
    }
}
