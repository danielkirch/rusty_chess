use crate::constants::Piece;

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

    pub piece_list: [Piece; 64], // Maps square index to piece type
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
            piece_list: [Piece::Empty; 64],
        }
    }

    fn init_piece_list(&mut self) {
        for i in 0..64 {
            let field: u64 = 1 << i;
            if self.occupied_squares & field == 0 {
                self.piece_list[i] = Piece::Empty;
            } else if self.kings & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Piece::WhiteKing;
                } else {
                    self.piece_list[i] = Piece::BlackKing;
                }
            } else if self.queens & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Piece::WhiteQueen;
                } else {
                    self.piece_list[i] = Piece::BlackQueen;
                }
            } else if self.rooks & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Piece::WhiteRook;
                } else {
                    self.piece_list[i] = Piece::BlackRook;
                }
            } else if self.bishops & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Piece::WhiteBishop;
                } else {
                    self.piece_list[i] = Piece::BlackBishop;
                }
            } else if self.knights & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Piece::WhiteKnight;
                } else {
                    self.piece_list[i] = Piece::BlackKnight;
                }
            } else if self.pawns & field != 0 {
                if self.white_pieces & field != 0 {
                    self.piece_list[i] = Piece::WhitePawn;
                } else {
                    self.piece_list[i] = Piece::BlackPawn;
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
            piece_list: [Piece::Empty; 64],
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

    pub fn make_move(&mut self, mv: u16) -> Piece {
        let (from, to, promotion, en_passant, castle) = self.parse_move_bitmap(mv);

        let moving_piece = self.piece_list[from as usize];
        let captured_piece = self.piece_list[to as usize];

        // Remove piece from 'from' square
        self.remove_piece(from);

        // Handle captures
        let to_mask: u64 = 1 << to;
        if self.occupied_squares & to_mask != 0 {
            self.remove_piece(to);
        }

        // Handle en passant
        if en_passant {
            let ep_capture_square = if moving_piece == Piece::WhitePawn {
                to - 8
            } else {
                to + 8
            };
            self.remove_piece(ep_capture_square);
        }

        // Promote piece if needed and add to 'to' square
        let moving_piece = if let Some(promoted_piece) = promotion {
            if to >= 56 {
                // White promotion
                match promoted_piece {
                    0 => Piece::WhiteQueen,
                    1 => Piece::WhiteRook,
                    2 => Piece::WhiteKnight,
                    3 => Piece::WhiteBishop,
                    _ => Piece::Empty,
                }
            } else {
                // Black promotion
                match promoted_piece {
                    0 => Piece::BlackQueen,
                    1 => Piece::BlackRook,
                    2 => Piece::BlackKnight,
                    3 => Piece::BlackBishop,
                    _ => Piece::Empty,
                }
            }
        } else {
            moving_piece
        };
        self.add_piece(to, moving_piece);

        // Handle castling - move the rook as well
        if castle {
            match to {
                2 => {
                    // white queenside
                    self.remove_piece(0);
                    self.add_piece(3, Piece::WhiteRook);
                }
                6 => {
                    // white kingside
                    self.remove_piece(7);
                    self.add_piece(5, Piece::WhiteRook);
                }
                58 => {
                    // black queenside
                    self.remove_piece(56);
                    self.add_piece(59, Piece::BlackRook);
                }
                62 => {
                    // black kingside
                    self.remove_piece(63);
                    self.add_piece(61, Piece::BlackRook);
                }
                _ => {}
            }
        }

        captured_piece
    }

    pub fn undo_move(&mut self, mv: u16, captured_piece: Piece) {
        let (from, to, promotion, en_passant, castle) = self.parse_move_bitmap(mv);
        let moving_piece = if promotion != None {
            if to >= 56 {
                Piece::WhitePawn
            } else {
                Piece::BlackPawn
            }
        } else {
            self.piece_list[to as usize]
        };

        // Add piece back to 'from' square
        self.add_piece(from, moving_piece);

        // Remove piece from 'to' square
        self.remove_piece(to);

        // Handle captures
        if captured_piece != Piece::Empty {
            self.add_piece(to, captured_piece);
        }

        // Handle en passant
        if en_passant {
            let ep_capture_square = if moving_piece == Piece::WhitePawn {
                to - 8
            } else {
                to + 8
            };
            let captured_pawn = if moving_piece == Piece::WhitePawn {
                Piece::BlackPawn
            } else {
                Piece::WhitePawn
            };
            self.add_piece(ep_capture_square, captured_pawn);
        }

        // Handle castling - move the rook back as well
        if castle {
            match to {
                2 => {
                    // white queenside
                    self.remove_piece(3);
                    self.add_piece(0, Piece::WhiteRook);
                }
                6 => {
                    // white kingside
                    self.remove_piece(5);
                    self.add_piece(7, Piece::WhiteRook);
                }
                58 => {
                    // black queenside
                    self.remove_piece(59);
                    self.add_piece(56, Piece::BlackRook);
                }
                62 => {
                    // black kingside
                    self.remove_piece(61);
                    self.add_piece(63, Piece::BlackRook);
                }
                _ => {}
            }
        }
    }

    fn parse_move_bitmap(&self, mv: u16) -> (usize, usize, Option<u16>, bool, bool) {
        // mv bitmap:
        // bits 0..5 -> from square
        // bits 6..11 -> to square
        // bit 15 -> promotion
        // bit 14 -> capture
        // bit 12/13:
        // if promotion:
        //   0 = queen,
        //   1 = rook,
        //   2 = bishop,
        //   3 = knight
        // else if capture:
        //   0 = normal,
        //   1 = en passant
        // else:
        //   0 = quiet move,
        //   1 = double pawn push,
        //   2 = kingside castle,
        //   3 = queenside castle
        let from = (mv & 0x003F) as usize;
        let to = ((mv >> 6) & 0x003F) as usize;

        // is promotion (bit 15), check bits 12-13 for piece type
        let promotion = if mv & 0x8000 != 0 {
            Some((mv >> 12) & 0x0003)
        } else {
            None
        };
        // is not promotion (bit 15) and is capture (bit 14) and has en passant flag (bit 12)
        let en_passant = mv & 0xD000 == 0x5000;
        // is not promotion (bit 15) and is not capture (bit 14) and is castle (bits 13)
        let castle = mv & 0xE000 == 0x2000;

        (from, to, promotion, en_passant, castle)
    }

    // remove piece from a square (used for moving pieces and captures)
    fn remove_piece(&mut self, square: usize) {
        let mask: u64 = 1 << square;
        let piece = self.piece_list[square];
        match piece {
            Piece::WhiteKing | Piece::BlackKing => self.kings &= !mask,
            Piece::WhiteQueen | Piece::BlackQueen => self.queens &= !mask,
            Piece::WhiteRook | Piece::BlackRook => self.rooks &= !mask,
            Piece::WhiteBishop | Piece::BlackBishop => self.bishops &= !mask,
            Piece::WhiteKnight | Piece::BlackKnight => self.knights &= !mask,
            Piece::WhitePawn | Piece::BlackPawn => self.pawns &= !mask,
            _ => {}
        }
        if piece as u8 <= Piece::WhitePawn as u8 {
            self.white_pieces &= !mask;
        } else {
            self.black_pieces &= !mask;
        }
        self.occupied_squares &= !mask;
        self.piece_list[square] = Piece::Empty;
    }

    // add piece to a square (used for moving pieces and undoing captures)
    fn add_piece(&mut self, square: usize, piece: Piece) {
        let mask: u64 = 1 << square;
        self.occupied_squares |= mask;
        match piece {
            Piece::WhiteKing | Piece::BlackKing => self.kings |= mask,
            Piece::WhiteQueen | Piece::BlackQueen => self.queens |= mask,
            Piece::WhiteRook | Piece::BlackRook => self.rooks |= mask,
            Piece::WhiteBishop | Piece::BlackBishop => self.bishops |= mask,
            Piece::WhiteKnight | Piece::BlackKnight => self.knights |= mask,
            Piece::WhitePawn | Piece::BlackPawn => self.pawns |= mask,
            _ => {}
        }
        if piece as u8 <= Piece::WhitePawn as u8 {
            self.white_pieces |= mask;
        } else {
            self.black_pieces |= mask;
        }
        self.piece_list[square] = piece;
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
        assert_eq!(board.piece_list[0], Piece::WhiteRook);
        assert_eq!(board.piece_list[60], Piece::BlackKing);
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
        assert_eq!(board.piece_list[28], Piece::WhitePawn); // e4
        assert_eq!(board.piece_list[36], Piece::BlackPawn); // e5
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
        assert_eq!(board.piece_list[28], Piece::WhitePawn); // e4
        assert_eq!(board.piece_list[36], Piece::BlackPawn); // e5
        assert_eq!(board.piece_list[12], Piece::WhiteKing); // e2
    }

    #[test]
    fn test_add_piece() {
        let mut board = Board::new();
        board.add_piece(0, Piece::WhiteRook);
        assert_eq!(board.white_pieces, 0x0000000000000001);
        assert_eq!(board.rooks, 0x0000000000000001);
        assert_eq!(board.occupied_squares, 0x0000000000000001);
        assert_eq!(board.piece_list[0], Piece::WhiteRook);
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::new();
        board.add_piece(0, Piece::WhiteRook);
        board.remove_piece(0);
        assert_eq!(board.white_pieces, 0x0000000000000000);
        assert_eq!(board.rooks, 0x0000000000000000);
        assert_eq!(board.occupied_squares, 0x0000000000000000);
        assert_eq!(board.piece_list[0], Piece::Empty);
    }

    #[test]
    fn test_move_piece_normal() {
        let mut board = Board::new();
        board.add_piece(0, Piece::WhiteRook);
        // Move rook from a1 (0) to a4 (24)
        let mv: u16 = 0x0600; // from 0 to 24
        board.make_move(mv);
        assert_eq!(board.white_pieces, 0x0000000001000000);
        assert_eq!(board.rooks, 0x0000000001000000);
        assert_eq!(board.occupied_squares, 0x0000000001000000);
        assert_eq!(board.piece_list[0], Piece::Empty);
        assert_eq!(board.piece_list[24], Piece::WhiteRook);
    }

    #[test]
    fn test_move_piece_capture() {
        let mut board = Board::new();
        board.add_piece(0, Piece::WhiteRook);
        board.add_piece(24, Piece::BlackPawn);
        // Move rook from a1 (0) to a4 (24) capturing black pawn
        let mv: u16 = 0x4600; // from 0 to 24 with capture flag
        board.make_move(mv);
        assert_eq!(board.white_pieces, 0x0000000001000000);
        assert_eq!(board.black_pieces, 0x0000000000000000);
        assert_eq!(board.rooks, 0x0000000001000000);
        assert_eq!(board.pawns, 0x0000000000000000);
        assert_eq!(board.occupied_squares, 0x0000000001000000);
        assert_eq!(board.piece_list[0], Piece::Empty);
        assert_eq!(board.piece_list[24], Piece::WhiteRook);
    }

    #[test]
    fn test_move_piece_promotion() {
        let mut board = Board::new();
        board.add_piece(48, Piece::WhitePawn);
        // Move pawn from a7 (48) to a8 (56) promoting to queen
        let mv: u16 = 0x8000 | 0x0E00 | 0x0030; // promotion | to | from
        board.make_move(mv);
        assert_eq!(board.white_pieces, 0x0100000000000000);
        assert_eq!(board.queens, 0x0100000000000000);
        assert_eq!(board.pawns, 0x0000000000000000);
        assert_eq!(board.occupied_squares, 0x0100000000000000);
        assert_eq!(board.piece_list[48], Piece::Empty);
        assert_eq!(board.piece_list[56], Piece::WhiteQueen);
    }

    #[test]
    fn test_move_piece_en_passant() {
        let mut board = Board::new();
        board.add_piece(36, Piece::WhitePawn); // e5
        board.add_piece(35, Piece::BlackPawn); // d5
        // Move pawn from e5 (36) to d6 (43) en passant
        let mv: u16 = 0x5000 | 0x0AC0 | 0x0024; // en passant | to | from
        board.make_move(mv);
        assert_eq!(board.white_pieces, 0x0000080000000000);
        assert_eq!(board.black_pieces, 0x0000000000000000);
        assert_eq!(board.pawns, 0x0000080000000000);
        assert_eq!(board.occupied_squares, 0x0000080000000000);
        assert_eq!(board.piece_list[36], Piece::Empty);
        assert_eq!(board.piece_list[35], Piece::Empty);
        assert_eq!(board.piece_list[43], Piece::WhitePawn);
    }

    #[test]
    fn test_move_piece_castling() {
        let mut board = Board::new();
        board.add_piece(4, Piece::WhiteKing); // e1
        board.add_piece(7, Piece::WhiteRook); // h1
        // Move king from e1 (4) to g1 (6) castling kingside
        let mv: u16 = 0x2000 | 0x0180 | 0x0004; // castle | to | from
        board.make_move(mv);
        assert_eq!(board.white_pieces, 0x0000000000000060);
        assert_eq!(board.kings, 0x0000000000000040);
        assert_eq!(board.rooks, 0x0000000000000020);
        assert_eq!(board.occupied_squares, 0x0000000000000060);
        assert_eq!(board.piece_list[4], Piece::Empty);
        assert_eq!(board.piece_list[7], Piece::Empty);
        assert_eq!(board.piece_list[6], Piece::WhiteKing);
        assert_eq!(board.piece_list[5], Piece::WhiteRook);
    }

    #[test]
    fn test_move_piece_castling_queenside() {
        let mut board = Board::new();
        board.add_piece(4, Piece::WhiteKing); // e1
        board.add_piece(0, Piece::WhiteRook); // a1
        // Move king from e1 (4) to c1 (2) castling queenside
        let mv: u16 = 0x3000 | 0x0080 | 0x0004; // castle | to | from
        board.make_move(mv);
        assert_eq!(board.white_pieces, 0x000000000000000C);
        assert_eq!(board.kings, 0x0000000000000004);
        assert_eq!(board.rooks, 0x0000000000000008);
        assert_eq!(board.occupied_squares, 0x000000000000000C);
        assert_eq!(board.piece_list[4], Piece::Empty);
        assert_eq!(board.piece_list[0], Piece::Empty);
        assert_eq!(board.piece_list[2], Piece::WhiteKing);
        assert_eq!(board.piece_list[3], Piece::WhiteRook);
    }

    #[test]
    fn test_move_piece_castling_black() {
        let mut board = Board::new();
        board.add_piece(60, Piece::BlackKing); // e8
        board.add_piece(63, Piece::BlackRook); // h8
        // Move king from e8 (60) to g8 (62) castling kingside
        let mv: u16 = 0x2000 | 0x0F80 | 0x003C; // castle | to | from
        board.make_move(mv);
        assert_eq!(board.black_pieces, 0x6000000000000000);
        assert_eq!(board.kings, 0x4000000000000000);
        assert_eq!(board.rooks, 0x2000000000000000);
        assert_eq!(board.occupied_squares, 0x6000000000000000);
        assert_eq!(board.piece_list[60], Piece::Empty);
        assert_eq!(board.piece_list[63], Piece::Empty);
        assert_eq!(board.piece_list[62], Piece::BlackKing);
        assert_eq!(board.piece_list[61], Piece::BlackRook);
    }

    #[test]
    fn test_move_piece_castling_black_queenside() {
        let mut board = Board::new();
        board.add_piece(60, Piece::BlackKing); // e8
        board.add_piece(56, Piece::BlackRook); // a8
        // Move king from e8 (60) to c8 (58) castling queenside
        let mv: u16 = 0x3000 | 0x0E80 | 0x003C; // castle | to | from
        board.make_move(mv);
        assert_eq!(board.black_pieces, 0x0C00000000000000);
        assert_eq!(board.kings, 0x0400000000000000);
        assert_eq!(board.rooks, 0x0800000000000000);
        assert_eq!(board.occupied_squares, 0x0C00000000000000);
        assert_eq!(board.piece_list[60], Piece::Empty);
        assert_eq!(board.piece_list[56], Piece::Empty);
        assert_eq!(board.piece_list[58], Piece::BlackKing);
        assert_eq!(board.piece_list[59], Piece::BlackRook);
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
