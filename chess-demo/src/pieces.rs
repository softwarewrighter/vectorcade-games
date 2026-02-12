//! Vector chess piece definitions.

use glam::Vec2;
use vectorcade_shared::{Rgba, draw::{DrawCmd, Stroke}};

const WHITE: Rgba = Rgba::WHITE;

pub const KING: u8 = 0;
pub const QUEEN: u8 = 1;
pub const ROOK: u8 = 2;
pub const BISHOP: u8 = 3;
pub const KNIGHT: u8 = 4;
pub const PAWN: u8 = 5;

pub fn draw_piece(out: &mut Vec<DrawCmd>, center: Vec2, piece: u8, scale: f32) {
    let pts = piece_points(piece);
    let scaled: Vec<Vec2> = pts.iter().map(|p| center + *p * scale).collect();
    out.push(DrawCmd::Polyline { pts: scaled, closed: true, stroke: Stroke::new(WHITE, 1.5) });
    if piece == KING {
        let h = vec![center + Vec2::new(-0.15, 0.85) * scale, center + Vec2::new(0.15, 0.85) * scale];
        let v = vec![center + Vec2::new(0.0, 0.7) * scale, center + Vec2::new(0.0, 1.0) * scale];
        out.push(DrawCmd::Polyline { pts: h, closed: false, stroke: Stroke::new(WHITE, 1.5) });
        out.push(DrawCmd::Polyline { pts: v, closed: false, stroke: Stroke::new(WHITE, 1.5) });
    }
}

fn piece_points(piece: u8) -> Vec<Vec2> {
    match piece {
        KING => vec![
            Vec2::new(0.0, 0.7), Vec2::new(-0.3, 0.3), Vec2::new(-0.4, 0.3),
            Vec2::new(-0.4, -0.2), Vec2::new(-0.5, -0.2), Vec2::new(-0.5, -0.5),
            Vec2::new(-0.6, -0.7), Vec2::new(0.6, -0.7), Vec2::new(0.5, -0.5),
            Vec2::new(0.5, -0.2), Vec2::new(0.4, -0.2), Vec2::new(0.4, 0.3), Vec2::new(0.3, 0.3),
        ],
        QUEEN => vec![
            Vec2::new(0.0, 0.8), Vec2::new(-0.15, 0.4), Vec2::new(-0.35, 0.7),
            Vec2::new(-0.35, 0.3), Vec2::new(-0.5, -0.2), Vec2::new(-0.6, -0.7),
            Vec2::new(0.6, -0.7), Vec2::new(0.5, -0.2), Vec2::new(0.35, 0.3),
            Vec2::new(0.35, 0.7), Vec2::new(0.15, 0.4),
        ],
        ROOK => vec![
            Vec2::new(-0.4, 0.6), Vec2::new(-0.4, 0.4), Vec2::new(-0.2, 0.4),
            Vec2::new(-0.2, 0.6), Vec2::new(0.0, 0.6), Vec2::new(0.0, 0.4),
            Vec2::new(0.2, 0.4), Vec2::new(0.2, 0.6), Vec2::new(0.4, 0.6), Vec2::new(0.4, 0.4),
            Vec2::new(0.3, 0.4), Vec2::new(0.3, 0.1), Vec2::new(0.4, 0.1), Vec2::new(0.4, -0.5),
            Vec2::new(0.5, -0.7), Vec2::new(-0.5, -0.7), Vec2::new(-0.4, -0.5),
            Vec2::new(-0.4, 0.1), Vec2::new(-0.3, 0.1), Vec2::new(-0.3, 0.4),
        ],
        BISHOP => vec![
            Vec2::new(0.0, 0.8), Vec2::new(-0.15, 0.5), Vec2::new(-0.3, 0.4),
            Vec2::new(-0.25, 0.1), Vec2::new(-0.4, -0.3), Vec2::new(-0.5, -0.7),
            Vec2::new(0.5, -0.7), Vec2::new(0.4, -0.3), Vec2::new(0.25, 0.1),
            Vec2::new(0.3, 0.4), Vec2::new(0.15, 0.5),
        ],
        KNIGHT => vec![
            Vec2::new(-0.1, 0.7), Vec2::new(-0.3, 0.5), Vec2::new(-0.4, 0.6),
            Vec2::new(-0.35, 0.3), Vec2::new(-0.5, 0.1), Vec2::new(-0.3, -0.1),
            Vec2::new(-0.4, -0.5), Vec2::new(-0.5, -0.7), Vec2::new(0.5, -0.7),
            Vec2::new(0.4, -0.5), Vec2::new(0.3, -0.1), Vec2::new(0.4, 0.2),
            Vec2::new(0.3, 0.4), Vec2::new(0.15, 0.5), Vec2::new(0.2, 0.7), Vec2::new(0.0, 0.6),
        ],
        PAWN | _ => vec![
            Vec2::new(0.0, 0.6), Vec2::new(-0.2, 0.4), Vec2::new(-0.15, 0.2),
            Vec2::new(-0.25, 0.0), Vec2::new(-0.2, -0.3), Vec2::new(-0.4, -0.7),
            Vec2::new(0.4, -0.7), Vec2::new(0.2, -0.3), Vec2::new(0.25, 0.0),
            Vec2::new(0.15, 0.2), Vec2::new(0.2, 0.4),
        ],
    }
}
