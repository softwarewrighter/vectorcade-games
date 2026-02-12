//! Rendering functions for Asteroids.

use glam::Vec2;
use vectorcade_shared::{
    Rgba,
    draw::{DrawCmd, Stroke},
    font::FontStyleId,
};

use crate::entities::{Asteroid, Bullet, Particle, Ship};

const WHITE: Rgba = Rgba::WHITE;

pub fn render_ship(out: &mut Vec<DrawCmd>, ship: &Ship) {
    let (s, c) = (ship.angle.sin(), ship.angle.cos());
    let rotate = |v: Vec2| Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c);
    let pts = vec![
        ship.pos + rotate(Vec2::new(0.04, 0.0)),
        ship.pos + rotate(Vec2::new(-0.025, 0.02)),
        ship.pos + rotate(Vec2::new(-0.025, -0.02)),
    ];
    out.push(DrawCmd::Polyline {
        pts,
        closed: true,
        stroke: Stroke::new(WHITE, 2.0),
    });

    if ship.thrusting {
        let pts = vec![
            ship.pos + rotate(Vec2::new(-0.025, 0.01)),
            ship.pos + rotate(Vec2::new(-0.045, 0.0)),
            ship.pos + rotate(Vec2::new(-0.025, -0.01)),
        ];
        out.push(DrawCmd::Polyline {
            pts,
            closed: false,
            stroke: Stroke::new(Rgba::rgb(1.0, 0.6, 0.2), 2.0),
        });
    }
}

pub fn render_asteroid(out: &mut Vec<DrawCmd>, asteroid: &Asteroid) {
    let (sides, radius) = (8, asteroid.size.radius());
    let mut pts = Vec::with_capacity(sides);
    let mut seed = asteroid.shape_seed;
    for i in 0..sides {
        let angle = asteroid.rot + (i as f32 / sides as f32) * std::f32::consts::TAU;
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let r = radius * (0.7 + 0.3 * ((seed % 100) as f32 / 100.0));
        pts.push(Vec2::new(
            asteroid.pos.x + r * angle.cos(),
            asteroid.pos.y + r * angle.sin(),
        ));
    }
    out.push(DrawCmd::Polyline {
        pts,
        closed: true,
        stroke: Stroke::new(WHITE, 1.5),
    });
}

pub fn render_bullet(out: &mut Vec<DrawCmd>, bullet: &Bullet) {
    let size = 0.008;
    out.push(DrawCmd::Polyline {
        pts: vec![
            bullet.pos + Vec2::new(-size, 0.0),
            bullet.pos + Vec2::new(size, 0.0),
        ],
        closed: false,
        stroke: Stroke::new(WHITE, 2.0),
    });
}

pub fn render_particle(out: &mut Vec<DrawCmd>, particle: &Particle) {
    out.push(DrawCmd::Polyline {
        pts: vec![particle.pos, particle.pos + particle.vel.normalize() * 0.01],
        closed: false,
        stroke: Stroke::new(WHITE.with_a(particle.alpha()), 1.0),
    });
}

pub fn render_hud(out: &mut Vec<DrawCmd>, score: u32, lives: u8, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.9, 0.85),
        text: format!("{:05}", score),
        size_px: 16.0,
        color: WHITE,
        style,
    });
    for i in 0..lives {
        let pos = Vec2::new(-0.9 + i as f32 * 0.05, 0.75);
        out.push(DrawCmd::Polyline {
            pts: vec![
                pos + Vec2::new(0.015, 0.0),
                pos + Vec2::new(-0.01, 0.008),
                pos + Vec2::new(-0.01, -0.008),
            ],
            closed: true,
            stroke: Stroke::new(WHITE, 1.0),
        });
    }
}

pub fn render_game_over(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.25, 0.0),
        text: "GAME OVER".to_string(),
        size_px: 24.0,
        color: WHITE,
        style,
    });
}
