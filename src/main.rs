mod board;
mod constants;
mod evaluation;
mod game;
mod game_history;
mod game_state;
mod interface;
mod move_generator;
mod zobrist;

fn main() {
    interface::UciInterface::new().run();
}
