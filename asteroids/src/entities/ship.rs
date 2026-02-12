//! Ship entity for Asteroids.

use glam::Vec2;

/// The player's ship.
pub struct Ship {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f32,
    pub thrusting: bool,
}

impl Ship {
    pub fn new() -> Self {
        Self { pos: Vec2::ZERO, vel: Vec2::ZERO, angle: 0.0, thrusting: false }
    }

    pub fn direction(&self) -> Vec2 {
        Vec2::new(self.angle.cos(), self.angle.sin())
    }
}

impl Default for Ship {
    fn default() -> Self { Self::new() }
}
