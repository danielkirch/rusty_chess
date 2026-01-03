use crate::constants::MAX_GAME_HISTORY_LENGTH;
use crate::game_state::GameState;

#[derive(Clone, Copy)]
pub struct GameHistory {
    pub list: [GameState; MAX_GAME_HISTORY_LENGTH],
    pub length: usize,
}

impl GameHistory {
    pub fn new() -> GameHistory {
        GameHistory {
            list: [GameState::new(); MAX_GAME_HISTORY_LENGTH],
            length: 1,
        }
    }

    pub fn from_fen(fen: &str) -> GameHistory {
        GameHistory {
            list: [GameState::from_fen(fen).unwrap(); MAX_GAME_HISTORY_LENGTH],
            length: 1,
        }
    }
}
