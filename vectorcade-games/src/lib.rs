//! Registry / facade over available games.

use vectorcade_shared::game::Game;

pub fn all_games() -> Vec<Box<dyn Game + Send>> {
    vec![
        Box::new(contents::Contents::new()),
        Box::new(chess_demo::ChessDemo::new()),
        Box::new(pong::Pong::new()),
        Box::new(asteroids::Asteroids::new()),
        Box::new(lunar_lander::LunarLander::new()),
        Box::new(battlezone::Battlezone::new()),
        Box::new(tempest::Tempest::new()),
    ]
}
