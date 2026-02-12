//! Bullet and particle entities.

use glam::Vec2;
use vectorcade_shared::wrap_position;

/// A bullet fired by the ship.
pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub lifetime: f32,
}

impl Bullet {
    pub const SPEED: f32 = 1.5;
    pub const LIFETIME: f32 = 1.2;

    pub fn new(pos: Vec2, direction: Vec2) -> Self {
        Self {
            pos,
            vel: direction * Self::SPEED,
            lifetime: Self::LIFETIME,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.pos = wrap_position(self.pos);
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

/// A particle for explosion effects.
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub lifetime: f32,
}

impl Particle {
    pub const MAX_LIFETIME: f32 = 0.6;

    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos,
            vel,
            lifetime: Self::MAX_LIFETIME,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn alpha(&self) -> f32 {
        (self.lifetime / Self::MAX_LIFETIME).clamp(0.0, 1.0)
    }
}
