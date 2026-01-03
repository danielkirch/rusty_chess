use crate::constants::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameState {
    pub white_to_move: bool,
    // castling_rights uses 4 bits:
    //  white kingside (bit 0),
    //  white queenside (bit 1),
    //  black kingside (bit 2),
    //  black queenside (bit 3)
    pub castling_rights: u8,
    pub en_passant_square: Option<u8>,

    pub current_move: Option<u16>,
    pub captured_piece: Piece,
    pub reversible_move_counter: u8,
    pub full_move_counter: u16,

    pub zobrist_hash: u64, // TODO: add zobrist hash to game state
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            white_to_move: true,
            castling_rights: 0x0F,
            en_passant_square: None,
            reversible_move_counter: 0,
            full_move_counter: 1,
            zobrist_hash: 0,
            current_move: None,
            captured_piece: Piece::Empty,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("Invalid FEN string".to_string());
        }

        let mut state = GameState::new();
        // Part 1: Piece placement (we skip this as it's handled in Board)
        // Part 2: Active color
        state.white_to_move = match parts[1] {
            "w" => true,
            "b" => false,
            _ => return Err("Invalid FEN string".to_string()),
        };

        // Part 3: Castling availability
        state.castling_rights = 0;
        for c in parts[2].chars() {
            match c {
                'K' => state.castling_rights |= 0x01,
                'Q' => state.castling_rights |= 0x02,
                'k' => state.castling_rights |= 0x04,
                'q' => state.castling_rights |= 0x08,
                '-' => (),
                _ => return Err("Invalid FEN string".to_string()),
            }
        }

        // Part 4: En passant target square
        if parts[3] != "-" {
            let file = parts[3].chars().next().unwrap() as u8 - 'a' as u8;
            let rank = parts[3].chars().nth(1).unwrap() as u8 - '1' as u8;
            state.en_passant_square = Some(file + rank * 8);
        }

        // Part 5: Halfmove clock
        state.reversible_move_counter = parts[4].parse().unwrap();

        // Part 6: Fullmove number
        state.full_move_counter = parts[5].parse().unwrap();
        Ok(state)
    }

    pub fn after_move(&self, mv: u16, captured_piece: Piece) -> GameState {
        let mut new_state = *self;
        new_state.current_move = Some(mv);
        new_state.captured_piece = captured_piece;
        new_state.white_to_move = !self.white_to_move;
        new_state.reversible_move_counter += 1;
        new_state.full_move_counter += if !self.white_to_move { 1 } else { 0 };

        // Reset reversible move counter on pawn move or capture
        if mv & 0xC000 != 0 || mv & 0xF000 == 0x1000 {
            // TODO: This only checks captures and promotions and double pawn push,
            // need to check for pawn moves as well
            new_state.reversible_move_counter = 0;
        }

        // Check double pawn push
        if mv & 0xF000 == 0x1000 {
            let to = ((mv >> 6) & 0x003F) as u8;
            if to <= 32 {
                // White pawn double push
                new_state.en_passant_square = Some(to - 8);
            } else {
                // Black pawn double push
                new_state.en_passant_square = Some(to + 8);
            }
        } else {
            new_state.en_passant_square = None;
        }

        // Update castling rights if a rook or king moves
        let from = (mv & 0x003F) as usize;
        match from {
            0 => new_state.castling_rights &= !0x02, // White queenside rook
            4 => new_state.castling_rights &= !0x03, // White king
            7 => new_state.castling_rights &= !0x01, // White kingside rook
            56 => new_state.castling_rights &= !0x08, // Black queenside rook
            60 => new_state.castling_rights &= !0x0C, // Black king
            63 => new_state.castling_rights &= !0x04, // Black kingside rook
            _ => {}
        }
        new_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let state = GameState::from_fen(fen).unwrap();
        assert_eq!(state.white_to_move, true);
        assert_eq!(state.castling_rights, 0x0F);
        assert_eq!(state.en_passant_square, None);
        assert_eq!(state.reversible_move_counter, 0);
        assert_eq!(state.full_move_counter, 1);
    }

    #[test]
    fn test_after_move() {
        let state = GameState::new();
        // Move: e2e4 (double pawn push)
        let mv: u16 = 0x1000 | (28 << 6) | 12;
        let new_state = state.after_move(mv, Piece::Empty);
        assert_eq!(new_state.white_to_move, false);
        assert_eq!(new_state.en_passant_square, Some(20));
        assert_eq!(new_state.reversible_move_counter, 0);
        assert_eq!(new_state.full_move_counter, 1);
    }

    #[test]
    fn test_after_move_castling_rights() {
        let state = GameState::new();
        // Move: e1g1 (white kingside castle)
        let mv: u16 = 0x2000 | (6 << 6) | 4;
        let new_state = state.after_move(mv, Piece::Empty);
        assert_eq!(new_state.castling_rights & 0x03, 0); // White castling rights removed
    }
}
