use glam::Vec2;
use vectorcade_shared::{
    Rgba,
    draw::{DrawCmd, Stroke, rect_wire},
    font::FontStyleId,
};

const PADDLE_HALF: f32 = 0.18;

pub fn render_court(out: &mut Vec<DrawCmd>, paddle_l: f32, paddle_r: f32, ball: Vec2) {
    let white = Stroke::new(Rgba::WHITE, 2.0);

    // Center line
    out.push(DrawCmd::Polyline {
        pts: vec![Vec2::new(0.0, -1.0), Vec2::new(0.0, 1.0)],
        closed: false,
        stroke: Stroke {
            color: Rgba::WHITE.with_a(0.6),
            width_px: 2.0,
            glow: 0.0,
        },
    });

    // Paddles
    out.push(rect_wire(
        Vec2::new(-0.92, paddle_l - PADDLE_HALF),
        Vec2::new(-0.88, paddle_l + PADDLE_HALF),
        white,
    ));
    out.push(rect_wire(
        Vec2::new(0.88, paddle_r - PADDLE_HALF),
        Vec2::new(0.92, paddle_r + PADDLE_HALF),
        white,
    ));

    // Ball
    out.push(rect_wire(
        ball - Vec2::splat(0.02),
        ball + Vec2::splat(0.02),
        white,
    ));
}

pub fn render_scores(out: &mut Vec<DrawCmd>, score_l: u32, score_r: u32, style: FontStyleId) {
    out.push(DrawCmd::Text {
        pos: Vec2::new(-0.2, 0.85),
        text: format!("{}", score_l),
        size_px: 18.0,
        color: Rgba::WHITE,
        style,
    });
    out.push(DrawCmd::Text {
        pos: Vec2::new(0.15, 0.85),
        text: format!("{}", score_r),
        size_px: 18.0,
        color: Rgba::WHITE,
        style,
    });
}

pub fn render_instructions(out: &mut Vec<DrawCmd>, style: FontStyleId) {
    let white = Rgba::WHITE;
    let lines = [
        ("PONG", -0.12, 0.6, 96.0),
        ("2 PLAYER GAME", -0.50, 0.35, 56.0),
        ("LEFT PADDLE: W/S", -0.50, 0.1, 48.0),
        ("RIGHT PADDLE: UP/DOWN", -0.65, -0.15, 48.0),
        ("PRESS SPACE TO START", -0.70, -0.6, 56.0),
    ];
    for (text, x, y, size) in lines {
        out.push(DrawCmd::Text {
            pos: Vec2::new(x, y), text: text.to_string(), size_px: size, color: white, style,
        });
    }
}
