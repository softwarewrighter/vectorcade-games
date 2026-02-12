//! Enemy tanks and other hostile units.

use glam::Vec3;

/// Types of enemies in the game.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyKind {
    Tank,
    SuperTank,
}

/// An enemy unit.
pub struct Enemy {
    pub pos: Vec3,
    pub angle: f32,
    pub kind: EnemyKind,
    pub alive: bool,
    pub fire_timer: f32,
}

impl Enemy {
    pub fn new(pos: Vec3, kind: EnemyKind) -> Self {
        Self { pos, angle: 0.0, kind, alive: true, fire_timer: 2.0 }
    }
}

/// Update all enemies - move toward player and fire.
pub fn update_enemies(enemies: &mut [Enemy], player_pos: Vec3, dt: f32) {
    for e in enemies.iter_mut() {
        if !e.alive { continue; }
        let to_player = player_pos - e.pos;
        let dist = to_player.length();
        if dist > 0.1 {
            e.angle = to_player.z.atan2(-to_player.x);
            let speed = match e.kind {
                EnemyKind::Tank => 2.0,
                EnemyKind::SuperTank => 3.5,
            };
            if dist > 8.0 {
                let dir = to_player.normalize();
                e.pos += dir * speed * dt;
            }
        }
        e.fire_timer -= dt;
    }
}
