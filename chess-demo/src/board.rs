//! Chess board rendering.

use glam::Vec2;
use vectorcade_shared::{Rgba, draw::{DrawCmd, Stroke}, font::FontStyleId};

const WHITE: Rgba = Rgba::WHITE;
const BOARD_SIZE: f32 = 1.3;
const SQUARE_SIZE: f32 = BOARD_SIZE / 8.0;
const BOARD_OFFSET: Vec2 = Vec2::new(-0.55, -0.75);

pub fn square_center(row: usize, col: usize) -> Vec2 {
    BOARD_OFFSET + Vec2::new((col as f32 + 0.5) * SQUARE_SIZE, (row as f32 + 0.5) * SQUARE_SIZE)
}

pub fn square_scale() -> f32 { SQUARE_SIZE }

pub fn render_board(out: &mut Vec<DrawCmd>) {
    // Board outline
    let corners = vec![
        BOARD_OFFSET, BOARD_OFFSET + Vec2::new(BOARD_SIZE, 0.0),
        BOARD_OFFSET + Vec2::new(BOARD_SIZE, BOARD_SIZE), BOARD_OFFSET + Vec2::new(0.0, BOARD_SIZE),
    ];
    out.push(DrawCmd::Polyline { pts: corners, closed: true, stroke: Stroke::new(WHITE, 2.0) });
    // Grid lines
    for i in 1..8 {
        let off = i as f32 * SQUARE_SIZE;
        out.push(DrawCmd::Polyline {
            pts: vec![BOARD_OFFSET + Vec2::new(off, 0.0), BOARD_OFFSET + Vec2::new(off, BOARD_SIZE)],
            closed: false, stroke: Stroke::new(WHITE, 1.0),
        });
        out.push(DrawCmd::Polyline {
            pts: vec![BOARD_OFFSET + Vec2::new(0.0, off), BOARD_OFFSET + Vec2::new(BOARD_SIZE, off)],
            closed: false, stroke: Stroke::new(WHITE, 1.0),
        });
    }
    // Dark square dot patterns
    for row in 0..8 {
        for col in 0..8 {
            if (row + col) % 2 == 1 {
                render_dots(out, row, col);
            }
        }
    }
}

fn render_dots(out: &mut Vec<DrawCmd>, row: usize, col: usize) {
    let base = square_center(row, col) - Vec2::splat(SQUARE_SIZE * 0.4);
    let spacing = SQUARE_SIZE * 0.2;
    for dy in 0..5 {
        for dx in 0..5 {
            let pos = base + Vec2::new(dx as f32 * spacing, dy as f32 * spacing);
            out.push(DrawCmd::Polyline {
                pts: vec![pos, pos + Vec2::new(0.003, 0.0)],
                closed: false, stroke: Stroke::new(WHITE, 1.5),
            });
        }
    }
}

pub fn render_labels(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    let files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    for (i, &f) in files.iter().enumerate() {
        out.push(DrawCmd::Text {
            pos: Vec2::new(BOARD_OFFSET.x + (i as f32 + 0.35) * SQUARE_SIZE, BOARD_OFFSET.y - 0.12),
            text: f.to_string(), size_px: 32.0, color: WHITE, style,
        });
    }
    for i in 0..8 {
        out.push(DrawCmd::Text {
            pos: Vec2::new(BOARD_OFFSET.x - 0.12, BOARD_OFFSET.y + (i as f32 + 0.25) * SQUARE_SIZE),
            text: (i + 1).to_string(), size_px: 32.0, color: WHITE, style,
        });
    }
}

pub fn render_title(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.70, 0.68),
        text: "IBM 2250 STATIC DEMO".to_string(),
        size_px: 40.0,
        color: WHITE,
        style,
    });
}
