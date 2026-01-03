mod board;
mod game;
mod game_history;
mod game_state;
mod move_generator;
mod zobrist;
use crate::game::Game;
use crate::zobrist::Zobrist;

pub mod constants;

fn main() {
    println!("Hello, KyrkaChess!");
    let zobrist = Zobrist::new();

    let game = Game::new();
    println!("Starting Position Hash: {}", zobrist.zobrist_hash(game));

    let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("FEN Position Hash: {}", zobrist.zobrist_hash(game));

    board::print_bitboard(0xFFEF00101000FFEF);
}
