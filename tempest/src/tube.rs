//! Tube geometry for Tempest levels.

use glam::Vec2;
use std::f32::consts::TAU;

/// A tube made of segments forming a playfield.
pub struct Tube {
    /// Rim points (outer edge where player moves).
    pub rim: Vec<Vec2>,
    /// Center point of the tube (vanishing point).
    pub center: Vec2,
    /// Number of segments.
    pub segments: usize,
}

impl Tube {
    /// Create a circular tube with n segments.
    pub fn circle(n: usize) -> Self {
        let mut rim = Vec::with_capacity(n);
        for i in 0..n {
            let angle = (i as f32 / n as f32) * TAU - TAU / 4.0;
            rim.push(Vec2::new(angle.cos() * 0.7, angle.sin() * 0.7));
        }
        Self { rim, center: Vec2::ZERO, segments: n }
    }

    /// Get interpolated point at depth (0=rim, 1=center) in segment.
    pub fn point_at(&self, segment: usize, depth: f32) -> Vec2 {
        let rim_pt = self.rim[segment];
        rim_pt.lerp(self.center, depth)
    }

    /// Get the two rim points for a segment's edges.
    pub fn segment_edges(&self, segment: usize) -> (Vec2, Vec2) {
        let a = self.rim[segment];
        let b = self.rim[(segment + 1) % self.segments];
        (a, b)
    }
}
