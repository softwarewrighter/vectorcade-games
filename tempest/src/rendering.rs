//! Rendering for Tempest.

use glam::Vec2;
use vectorcade_shared::{
    draw::{DrawCmd, Stroke},
    font::FontStyleId,
};

use crate::{BLUE, CYAN, GREEN, MAGENTA, RED, YELLOW, Blaster, Shot, enemies::{Enemy, EnemyKind}, tube::Tube};

/// Render instruction screen.
pub fn render_instructions(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    let lines = [
        ("TEMPEST", -0.20, 0.5, 24.0, YELLOW),
        ("LEFT/RIGHT: MOVE", -0.35, 0.2, 12.0, BLUE),
        ("SPACE: FIRE", -0.22, 0.05, 12.0, BLUE),
        ("DESTROY ALL ENEMIES", -0.40, -0.2, 12.0, BLUE),
        ("PRESS SPACE TO START", -0.42, -0.5, 14.0, YELLOW),
    ];
    for (text, x, y, size, color) in lines {
        out.push(DrawCmd::Text {
            pos: Vec2::new(x, y), text: text.to_string(), size_px: size, color, style,
        });
    }
}

/// Render the tube structure.
pub fn render_tube(out: &mut Vec<DrawCmd>, tube: &Tube) {
    // Draw segments from rim to center
    for i in 0..tube.segments {
        let rim_pt = tube.rim[i];
        out.push(DrawCmd::Polyline {
            pts: vec![rim_pt, tube.center],
            closed: false,
            stroke: Stroke::new(BLUE, 1.0),
        });
    }
    // Draw rim outline
    let mut rim_pts = tube.rim.clone();
    rim_pts.push(tube.rim[0]); // Close the loop
    out.push(DrawCmd::Polyline {
        pts: rim_pts,
        closed: false,
        stroke: Stroke::new(BLUE, 2.0),
    });
}

/// Render the player's blaster.
pub fn render_blaster(out: &mut Vec<DrawCmd>, tube: &Tube, blaster: &Blaster) {
    let (a, b) = tube.segment_edges(blaster.segment);
    let mid = (a + b) * 0.5;
    let inward = (tube.center - mid).normalize() * 0.05;
    // Claw shape
    let pts = vec![a, mid + inward, b];
    out.push(DrawCmd::Polyline { pts, closed: false, stroke: Stroke::new(YELLOW, 3.0) });
}

/// Render player shots.
pub fn render_shots(out: &mut Vec<DrawCmd>, tube: &Tube, shots: &[Shot]) {
    for shot in shots {
        let pos = tube.point_at(shot.segment, shot.depth);
        let size = 0.02 * (1.0 - shot.depth * 0.5);
        out.push(DrawCmd::Polyline {
            pts: vec![pos - Vec2::new(size, 0.0), pos + Vec2::new(size, 0.0)],
            closed: false,
            stroke: Stroke::new(YELLOW, 2.0),
        });
    }
}

/// Render enemies.
pub fn render_enemies(out: &mut Vec<DrawCmd>, tube: &Tube, enemies: &[Enemy]) {
    for enemy in enemies {
        if !enemy.alive { continue; }
        let pos = tube.point_at(enemy.segment, enemy.depth);
        let size = 0.03 * (1.0 - enemy.depth * 0.5);
        let color = match enemy.kind {
            EnemyKind::Flipper => RED,
            EnemyKind::Tanker => GREEN,
            EnemyKind::Spiker => MAGENTA,
        };
        // Simple diamond shape for enemies
        let pts = vec![
            pos + Vec2::new(0.0, size),
            pos + Vec2::new(size, 0.0),
            pos + Vec2::new(0.0, -size),
            pos + Vec2::new(-size, 0.0),
        ];
        out.push(DrawCmd::Polyline { pts, closed: true, stroke: Stroke::new(color, 2.0) });
    }
}

/// Render HUD with score, lives, level.
pub fn render_hud(out: &mut Vec<DrawCmd>, score: u32, lives: u8, level: u8, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.95, 0.85),
        text: format!("{:06}", score),
        size_px: 14.0,
        color: CYAN,
        style,
    });
    out.push(DrawCmd::Text {
        pos: Vec2::new(0.55, 0.85),
        text: format!("L{} x{}", level, lives),
        size_px: 14.0,
        color: CYAN,
        style,
    });
}

/// Render game over text.
pub fn render_game_over(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.28, 0.0),
        text: "GAME OVER".to_string(),
        size_px: 24.0,
        color: RED,
        style,
    });
}
