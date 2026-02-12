//! Rendering functions for Lunar Lander.

use glam::Vec2;
use vectorcade_shared::{
    Rgba,
    draw::{DrawCmd, Stroke},
    font::FontStyleId,
};

use crate::lander::Lander;
use crate::terrain::Terrain;

const WHITE: Rgba = Rgba::WHITE;

pub fn render_terrain(out: &mut Vec<DrawCmd>, terrain: &Terrain) {
    // Draw terrain surface
    out.push(DrawCmd::Polyline {
        pts: terrain.points.clone(),
        closed: false,
        stroke: Stroke::new(WHITE, 1.5),
    });
    // Draw landing pads (thicker line to stand out)
    for pad in &terrain.pads {
        out.push(DrawCmd::Polyline {
            pts: vec![Vec2::new(pad.x_min, pad.y), Vec2::new(pad.x_max, pad.y)],
            closed: false,
            stroke: Stroke::new(WHITE, 3.0),
        });
        // Multiplier indicator
        let mid_x = (pad.x_min + pad.x_max) / 2.0;
        out.push(DrawCmd::Text {
            pos: Vec2::new(mid_x - 0.02, pad.y - 0.05),
            text: format!("x{}", pad.multiplier),
            size_px: 10.0,
            color: WHITE,
            style: FontStyleId::ATARI,
        });
    }
}

pub fn render_lander(out: &mut Vec<DrawCmd>, lander: &Lander) {
    let (s, c) = (lander.angle.sin(), lander.angle.cos());
    let rotate = |v: Vec2| Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c);
    // Lander body (triangular with legs)
    let body = vec![
        lander.pos + rotate(Vec2::new(0.0, 0.04)),
        lander.pos + rotate(Vec2::new(-0.025, -0.02)),
        lander.pos + rotate(Vec2::new(0.025, -0.02)),
    ];
    out.push(DrawCmd::Polyline { pts: body, closed: true, stroke: Stroke::new(WHITE, 2.0) });
    // Landing legs
    let leg_l = vec![
        lander.pos + rotate(Vec2::new(-0.02, -0.02)),
        lander.pos + rotate(Vec2::new(-0.035, -0.05)),
    ];
    let leg_r = vec![
        lander.pos + rotate(Vec2::new(0.02, -0.02)),
        lander.pos + rotate(Vec2::new(0.035, -0.05)),
    ];
    out.push(DrawCmd::Polyline { pts: leg_l, closed: false, stroke: Stroke::new(WHITE, 1.5) });
    out.push(DrawCmd::Polyline { pts: leg_r, closed: false, stroke: Stroke::new(WHITE, 1.5) });
    // Thrust flame
    if lander.thrusting {
        let flame = vec![
            lander.pos + rotate(Vec2::new(-0.012, -0.02)),
            lander.pos + rotate(Vec2::new(0.0, -0.07)),
            lander.pos + rotate(Vec2::new(0.012, -0.02)),
        ];
        out.push(DrawCmd::Polyline { pts: flame, closed: false, stroke: Stroke::new(WHITE, 2.0) });
    }
}

pub fn render_hud(out: &mut Vec<DrawCmd>, lander: &Lander, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.95, 0.9),
        text: format!("FUEL: {:3.0}", lander.fuel),
        size_px: 14.0,
        color: WHITE,
        style,
    });
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.95, 0.8),
        text: format!("VEL: {:.2}", lander.vel.length()),
        size_px: 12.0,
        color: WHITE,
        style,
    });
    out.push(DrawCmd::Text {
        pos: Vec2::new(0.5, 0.9),
        text: format!("ALT: {:.2}", lander.pos.y + 0.9),
        size_px: 12.0,
        color: WHITE,
        style,
    });
}

pub fn render_landed(out: &mut Vec<DrawCmd>, score: u32, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.25, 0.1),
        text: "LANDED!".to_string(),
        size_px: 24.0,
        color: WHITE,
        style,
    });
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.2, -0.05),
        text: format!("SCORE: {}", score),
        size_px: 18.0,
        color: WHITE,
        style,
    });
}

pub fn render_crashed(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.25, 0.0),
        text: "CRASHED!".to_string(),
        size_px: 24.0,
        color: WHITE,
        style,
    });
}

pub fn render_instructions(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    let lines = [
        ("LUNAR LANDER", -0.50, 0.65, 96.0),
        ("LEFT/RIGHT - ROTATE", -0.70, 0.35, 48.0),
        ("UP OR SPACE - THRUST", -0.72, 0.15, 48.0),
        ("LAND ON FLAT PADS", -0.62, -0.05, 48.0),
        ("LAND SLOWLY AND LEVEL", -0.75, -0.25, 48.0),
        ("CONSERVE FUEL FOR BONUS", -0.82, -0.45, 48.0),
        ("PRESS SPACE TO START", -0.70, -0.75, 56.0),
    ];
    for (text, x, y, size) in lines {
        out.push(DrawCmd::Text {
            pos: Vec2::new(x, y), text: text.to_string(), size_px: size, color: WHITE, style,
        });
    }
}
