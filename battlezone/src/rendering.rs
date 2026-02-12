//! 3D vector rendering for Battlezone.

use glam::{Vec2, Vec3};
use vectorcade_shared::{
    draw::{DrawCmd, Stroke},
    font::FontStyleId,
    project_line_3d, project_persp, rotate_point_y,
    projectile::Projectile3D,
};

use crate::{GREEN, RED, enemies::Enemy, world::{Obstacle, ObstacleKind}};

const FOV: f32 = 1.0;
const ASPECT: f32 = 4.0 / 3.0;

/// Render instruction screen.
pub fn render_instructions(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    let lines = [
        ("BATTLEZONE", -0.42, 0.55, 96.0),
        ("LEFT/RIGHT: ROTATE", -0.65, 0.25, 48.0),
        ("UP/DOWN: MOVE", -0.48, 0.05, 48.0),
        ("SPACE: FIRE", -0.38, -0.15, 48.0),
        ("DESTROY ENEMY TANKS", -0.70, -0.4, 48.0),
        ("PRESS SPACE TO START", -0.70, -0.7, 56.0),
    ];
    for (text, x, y, size) in lines {
        out.push(DrawCmd::Text {
            pos: Vec2::new(x, y), text: text.to_string(), size_px: size, color: GREEN, style,
        });
    }
}

/// Render the horizon line.
pub fn render_horizon(out: &mut Vec<DrawCmd>) {
    out.push(DrawCmd::Polyline {
        pts: vec![Vec2::new(-1.0, 0.0), Vec2::new(1.0, 0.0)],
        closed: false,
        stroke: Stroke::new(GREEN, 1.5),
    });
}

/// Render crosshair in center of screen.
pub fn render_crosshair(out: &mut Vec<DrawCmd>) {
    let s = 0.05;
    out.push(DrawCmd::Polyline {
        pts: vec![Vec2::new(-s, 0.0), Vec2::new(s, 0.0)],
        closed: false,
        stroke: Stroke::new(GREEN, 2.0),
    });
    out.push(DrawCmd::Polyline {
        pts: vec![Vec2::new(0.0, -s), Vec2::new(0.0, s)],
        closed: false,
        stroke: Stroke::new(GREEN, 2.0),
    });
}

/// Render HUD with score and lives.
pub fn render_hud(out: &mut Vec<DrawCmd>, score: u32, lives: u8, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.95, 0.85),
        text: format!("SCORE: {}", score),
        size_px: 14.0,
        color: RED,
        style,
    });
    out.push(DrawCmd::Text {
        pos: Vec2::new(0.55, 0.85),
        text: format!("LIVES: {}", lives),
        size_px: 14.0,
        color: RED,
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

/// Render the 3D world - obstacles and enemies.
pub fn render_world(out: &mut Vec<DrawCmd>, obstacles: &[Obstacle], enemies: &[Enemy], player_pos: Vec3, player_angle: f32) {
    for obs in obstacles {
        render_obstacle(out, obs, player_pos, player_angle);
    }
    for e in enemies {
        if e.alive { render_enemy(out, e, player_pos, player_angle); }
    }
}

fn draw_line_3d(out: &mut Vec<DrawCmd>, a: Vec3, b: Vec3) {
    if let Some((sa, sb)) = project_line_3d(a, b, FOV, ASPECT) {
        out.push(DrawCmd::Polyline {
            pts: vec![sa, sb],
            closed: false,
            stroke: Stroke::new(GREEN, 1.5),
        });
    }
}

fn render_obstacle(out: &mut Vec<DrawCmd>, obs: &Obstacle, player_pos: Vec3, player_angle: f32) {
    let rel = obs.pos - player_pos;
    let rotated = rotate_point_y(rel, -player_angle);
    if rotated.z > -0.5 { return; } // Behind camera
    match obs.kind {
        ObstacleKind::Cube => render_cube(out, rotated, 1.5),
        ObstacleKind::Pyramid => render_pyramid(out, rotated, 1.5),
        ObstacleKind::Block => render_cube(out, rotated, 2.5),
    }
}

fn render_cube(out: &mut Vec<DrawCmd>, center: Vec3, size: f32) {
    let h = size / 2.0;
    let verts = [
        Vec3::new(-h, 0.0, -h), Vec3::new(h, 0.0, -h), Vec3::new(h, 0.0, h), Vec3::new(-h, 0.0, h),
        Vec3::new(-h, size, -h), Vec3::new(h, size, -h), Vec3::new(h, size, h), Vec3::new(-h, size, h),
    ];
    let edges = [(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(6,7),(7,4),(0,4),(1,5),(2,6),(3,7)];
    for (a, b) in edges {
        draw_line_3d(out, verts[a] + center, verts[b] + center);
    }
}

fn render_pyramid(out: &mut Vec<DrawCmd>, center: Vec3, size: f32) {
    let h = size / 2.0;
    let base = [Vec3::new(-h, 0.0, -h), Vec3::new(h, 0.0, -h), Vec3::new(h, 0.0, h), Vec3::new(-h, 0.0, h)];
    let apex = Vec3::new(0.0, size, 0.0);
    for i in 0..4 {
        draw_line_3d(out, base[i] + center, base[(i + 1) % 4] + center);
        draw_line_3d(out, base[i] + center, apex + center);
    }
}

fn render_enemy(out: &mut Vec<DrawCmd>, enemy: &Enemy, player_pos: Vec3, player_angle: f32) {
    let rel = enemy.pos - player_pos;
    let rotated = rotate_point_y(rel, -player_angle);
    if rotated.z > -0.5 { return; } // Behind camera
    // Simple tank shape - hull
    let h = 0.8;
    let hull = [
        Vec3::new(-h, 0.0, -h), Vec3::new(h, 0.0, -h), Vec3::new(h, 0.0, h), Vec3::new(-h, 0.0, h),
        Vec3::new(-h, 0.6, -h), Vec3::new(h, 0.6, -h), Vec3::new(h, 0.6, h), Vec3::new(-h, 0.6, h),
    ];
    let edges = [(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(6,7),(7,4),(0,4),(1,5),(2,6),(3,7)];
    for (a, b) in edges {
        draw_line_3d(out, hull[a] + rotated, hull[b] + rotated);
    }
    // Turret gun barrel
    let turret_base = rotated + Vec3::new(0.0, 0.6, 0.0);
    let gun_end = turret_base + Vec3::new(0.0, 0.3, 1.2);
    draw_line_3d(out, turret_base, gun_end);
}

/// Render player shots as small bright points.
pub fn render_shots(out: &mut Vec<DrawCmd>, shots: &[Projectile3D], player_pos: Vec3, player_angle: f32) {
    for shot in shots {
        if !shot.alive { continue; }
        let rel = shot.pos - player_pos;
        let rotated = rotate_point_y(rel, -player_angle);
        if rotated.z > -0.5 { continue; } // Behind camera
        if let Some(screen_pos) = project_persp(rotated, FOV, ASPECT) {
            // Draw shot as a small cross
            let size = 0.02;
            out.push(DrawCmd::Polyline {
                pts: vec![screen_pos - Vec2::new(size, 0.0), screen_pos + Vec2::new(size, 0.0)],
                closed: false,
                stroke: Stroke::new(GREEN, 3.0),
            });
            out.push(DrawCmd::Polyline {
                pts: vec![screen_pos - Vec2::new(0.0, size), screen_pos + Vec2::new(0.0, size)],
                closed: false,
                stroke: Stroke::new(GREEN, 3.0),
            });
        }
    }
}
