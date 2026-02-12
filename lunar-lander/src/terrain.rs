//! Terrain generation for Lunar Lander.

use glam::Vec2;
use vectorcade_shared::GameRng;

/// Landing pad on the terrain.
pub struct LandingPad {
    pub x_min: f32,
    pub x_max: f32,
    pub y: f32,
    pub multiplier: u32,
}

/// Terrain with surface points and landing pads.
pub struct Terrain {
    pub points: Vec<Vec2>,
    pub pads: Vec<LandingPad>,
}

impl Terrain {
    pub fn new() -> Self {
        Self { points: Vec::new(), pads: Vec::new() }
    }

    pub fn generate(rng: &mut dyn GameRng) -> Self {
        let mut points = Vec::with_capacity(32);
        let mut pads = Vec::new();
        let mut x = -1.0_f32;
        let mut y = rng.range_f32(-0.7, -0.5);
        points.push(Vec2::new(x, y));

        while x < 1.0 {
            let step = rng.range_f32(0.08, 0.15);
            x += step;
            if x > 1.0 { x = 1.0; }
            // Chance for landing pad (flat section)
            if rng.range_f32(0.0, 1.0) < 0.2 && pads.len() < 3 {
                let pad_width = rng.range_f32(0.1, 0.18);
                let x_end = (x + pad_width).min(1.0);
                pads.push(LandingPad {
                    x_min: x - step,
                    x_max: x_end,
                    y,
                    multiplier: if pad_width < 0.12 { 3 } else { 2 },
                });
                points.push(Vec2::new(x_end, y));
                x = x_end;
            } else {
                y += rng.range_f32(-0.15, 0.15);
                y = y.clamp(-0.9, -0.3);
                points.push(Vec2::new(x, y));
            }
        }
        Self { points, pads }
    }

    pub fn surface_y_at(&self, x: f32) -> f32 {
        for i in 0..self.points.len().saturating_sub(1) {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];
            if x >= p1.x && x <= p2.x {
                let t = (x - p1.x) / (p2.x - p1.x);
                return p1.y + t * (p2.y - p1.y);
            }
        }
        -0.9
    }

    pub fn pad_at(&self, x: f32) -> Option<&LandingPad> {
        self.pads.iter().find(|p| x >= p.x_min && x <= p.x_max)
    }
}

impl Default for Terrain {
    fn default() -> Self { Self::new() }
}
