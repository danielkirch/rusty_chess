#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameState {
    pub white_to_move: bool,
    pub castling_rights: u8,
    pub en_passant_square: Option<u8>,

    pub reversible_move_counter: u8,
    pub full_move_counter: u16,

    pub zobrist_hash: u64, // TODO: add zobrist hash to game state
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            white_to_move: true,
            castling_rights: 0b1111,
            en_passant_square: None,
            reversible_move_counter: 0,
            full_move_counter: 1,
            zobrist_hash: 0,
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
                'K' => state.castling_rights |= 0b0001,
                'Q' => state.castling_rights |= 0b0010,
                'k' => state.castling_rights |= 0b0100,
                'q' => state.castling_rights |= 0b1000,
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
}
