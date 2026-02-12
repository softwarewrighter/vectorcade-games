//! Registry / facade over available games.

use vectorcade_shared::game::Game;

pub fn all_games() -> Vec<Box<dyn Game + Send>> {
    vec![
        Box::new(pong::Pong::new()),
    ]
}
