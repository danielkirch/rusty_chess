use crate::board::Board;
use crate::game_history::GameHistory;

#[derive(Clone, Copy)]
pub struct Game {
    pub board: Board,
    pub history: GameHistory,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::starting_position(),
            history: GameHistory::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Game {
        Game {
            board: Board::from_fen(fen).unwrap(),
            history: GameHistory::from_fen(fen),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::Pieces;

    #[test]
    fn test_starting_position() {
        let game = Game::new();
        let board = game.board;
        assert_eq!(board.piece_list[0], Pieces::WhiteRook);
        assert_eq!(board.piece_list[60], Pieces::BlackKing);
    }

    #[test]
    fn test_from_fen() {
        // Test parsing starting position FEN
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game = Game::from_fen(fen);
        let board = game.board;
        assert_eq!(board.piece_list[0], Pieces::WhiteRook);
        assert_eq!(board.piece_list[60], Pieces::BlackKing);
    }

    #[test]
    fn test_from_fen_e4e5_opening() {
        // Test parsing a FEN string after 1. e4 e5
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2";
        let game = Game::from_fen(fen);
        let board = game.board;
        assert_eq!(board.piece_list[28], Pieces::WhitePawn); // e4
        assert_eq!(board.piece_list[36], Pieces::BlackPawn); // e5
    }

    #[test]
    fn test_from_fen_bongcloud_opening() {
        // Test parsing a FEN string after 1. e4 e5 2. Ke2
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 1 2";
        let game = Game::from_fen(fen);
        let board = game.board;
        assert_eq!(board.piece_list[28], Pieces::WhitePawn); // e4
        assert_eq!(board.piece_list[36], Pieces::BlackPawn); // e5
        assert_eq!(board.piece_list[12], Pieces::WhiteKing); // e2
    }
}
