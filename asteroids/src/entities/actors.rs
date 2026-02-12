//! Asteroid entities.

use glam::Vec2;
use vectorcade_shared::{GameRng, wrap_position};

/// Size variants for asteroids.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}

impl AsteroidSize {
    pub fn radius(self) -> f32 {
        match self {
            Self::Large => 0.12,
            Self::Medium => 0.06,
            Self::Small => 0.03,
        }
    }

    pub fn points(self) -> u32 {
        match self {
            Self::Large => 20,
            Self::Medium => 50,
            Self::Small => 100,
        }
    }

    pub fn split(self) -> Option<AsteroidSize> {
        match self {
            Self::Large => Some(Self::Medium),
            Self::Medium => Some(Self::Small),
            Self::Small => None,
        }
    }
}

/// An asteroid floating in space.
pub struct Asteroid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: AsteroidSize,
    pub rot: f32,
    pub rot_speed: f32,
    pub shape_seed: u32,
}

impl Asteroid {
    pub fn spawn_random(rng: &mut dyn GameRng) -> Self {
        let edge = rng.pick_index(4).unwrap_or(0);
        let pos = match edge {
            0 => Vec2::new(rng.range_f32(-1.0, 1.0), 1.0),
            1 => Vec2::new(rng.range_f32(-1.0, 1.0), -1.0),
            2 => Vec2::new(-1.0, rng.range_f32(-1.0, 1.0)),
            _ => Vec2::new(1.0, rng.range_f32(-1.0, 1.0)),
        };
        let angle = rng.range_f32(0.0, std::f32::consts::TAU);
        Self {
            pos,
            vel: Vec2::new(angle.cos(), angle.sin()) * rng.range_f32(0.1, 0.3),
            size: AsteroidSize::Large,
            rot: rng.range_f32(0.0, std::f32::consts::TAU),
            rot_speed: rng.range_f32(-2.0, 2.0),
            shape_seed: rng.next_u32(),
        }
    }

    pub fn spawn_split(parent: &Asteroid, rng: &mut dyn GameRng) -> Self {
        let offset_angle = rng.range_f32(0.0, std::f32::consts::TAU);
        let angle = rng.range_f32(0.0, std::f32::consts::TAU);
        Self {
            pos: parent.pos + Vec2::new(offset_angle.cos(), offset_angle.sin()) * 0.05,
            vel: Vec2::new(angle.cos(), angle.sin()) * parent.vel.length() * 1.2,
            size: parent.size.split().unwrap_or(AsteroidSize::Small),
            rot: rng.range_f32(0.0, std::f32::consts::TAU),
            rot_speed: rng.range_f32(-3.0, 3.0),
            shape_seed: rng.next_u32(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.pos = wrap_position(self.pos);
        self.rot += self.rot_speed * dt;
    }
}
