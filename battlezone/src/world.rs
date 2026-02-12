//! World obstacles - geometric shapes on the battlefield.

use glam::Vec3;
use vectorcade_shared::GameRng;

/// Obstacle types matching original arcade.
#[derive(Clone, Copy)]
pub enum ObstacleKind {
    Cube,
    Pyramid,
    Block,
}

/// A world obstacle that blocks movement and shots.
pub struct Obstacle {
    pub pos: Vec3,
    pub kind: ObstacleKind,
}

impl Obstacle {
    pub fn new(pos: Vec3, kind: ObstacleKind) -> Self {
        Self { pos, kind }
    }
}

/// Spawn obstacles around the battlefield.
pub fn spawn_obstacles(obstacles: &mut Vec<Obstacle>, rng: &mut dyn GameRng) {
    obstacles.clear();
    let kinds = [ObstacleKind::Cube, ObstacleKind::Pyramid, ObstacleKind::Block];
    for _ in 0..12 {
        let angle = rng.range_f32(0.0, std::f32::consts::TAU);
        let dist = rng.range_f32(10.0, 40.0);
        let pos = Vec3::new(angle.cos() * dist, 0.0, angle.sin() * dist);
        let kind_idx = rng.range_i32(0, 3) as usize;
        let kind = kinds[kind_idx];
        obstacles.push(Obstacle::new(pos, kind));
    }
}
