//! Lander entity for Lunar Lander.

use glam::Vec2;

/// The lunar lander spacecraft.
pub struct Lander {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f32,
    pub fuel: f32,
    pub thrusting: bool,
}

impl Lander {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.7),
            vel: Vec2::new(0.1, 0.0),
            angle: 0.0,
            fuel: 100.0,
            thrusting: false,
        }
    }

    pub fn direction(&self) -> Vec2 {
        Vec2::new(self.angle.sin(), self.angle.cos())
    }

    pub fn is_out_of_fuel(&self) -> bool {
        self.fuel <= 0.0
    }
}

impl Default for Lander {
    fn default() -> Self { Self::new() }
}
