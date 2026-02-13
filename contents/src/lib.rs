//! Contents listing screen for VectorCade.

use glam::Vec2;
use vectorcade_shared::{
    Rgba,
    draw::DrawCmd,
    font::FontStyleId,
    game::{Game, GameCtx, GameMeta},
};

const WHITE: Rgba = Rgba::WHITE;

/// Static contents screen listing all available games.
pub struct Contents {
    font_style: FontStyleId,
}

impl Default for Contents {
    fn default() -> Self { Self::new() }
}

impl Contents {
    pub fn new() -> Self {
        Self { font_style: FontStyleId::ATARI }
    }
}

impl Game for Contents {
    fn metadata(&self) -> GameMeta {
        GameMeta { name: "Contents", preferred_aspect: Some(4.0 / 3.0) }
    }

    fn reset(&mut self, _ctx: &mut GameCtx) {}

    fn update(&mut self, _ctx: &mut GameCtx, _dt: f32) {}

    fn render(&mut self, _ctx: &mut GameCtx, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::Clear { color: Rgba::BLACK });

        let lines = [
            ("> CONTENTS <", 0.55),
            ("- CHESS BOARD IMAGE", 0.30),
            ("- PONG", 0.15),
            ("- ASTEROIDS", 0.00),
            ("- LUNAR LANDER", -0.15),
            ("- BATTLEZONE", -0.30),
            ("- TEMPEST", -0.45),
        ];

        for (text, y) in lines {
            out.push(DrawCmd::Text {
                pos: Vec2::new(-0.55, y),
                text: text.to_string(),
                size_px: 36.0,
                color: WHITE,
                style: self.font_style,
            });
        }
    }
}
