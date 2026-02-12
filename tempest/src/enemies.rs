//! Enemy types for Tempest.

use crate::tube::Tube;

/// Enemy types matching original arcade.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyKind {
    Flipper,  // Moves up and can flip between segments
    Tanker,   // Moves up, spawns smaller enemies when hit
    Spiker,   // Leaves spikes on the tube
}

impl EnemyKind {
    pub fn points(self) -> u32 {
        match self {
            Self::Flipper => 150,
            Self::Tanker => 100,
            Self::Spiker => 50,
        }
    }

    pub fn speed(self) -> f32 {
        match self {
            Self::Flipper => 0.3,
            Self::Tanker => 0.2,
            Self::Spiker => 0.15,
        }
    }
}

/// An enemy climbing up the tube.
pub struct Enemy {
    pub segment: usize,
    pub depth: f32,
    pub kind: EnemyKind,
    pub alive: bool,
}

impl Enemy {
    pub fn new(segment: usize, kind: EnemyKind) -> Self {
        Self { segment, depth: 1.0, kind, alive: true }
    }
}

/// Update all enemies - move toward rim.
pub fn update_enemies(enemies: &mut [Enemy], _tube: &Tube, dt: f32) {
    for e in enemies.iter_mut() {
        if !e.alive { continue; }
        e.depth -= e.kind.speed() * dt;
    }
}
