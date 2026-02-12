mod drawing;
mod physics;

use glam::Vec2;
use vectorcade_shared::{
    Rgba,
    draw::DrawCmd,
    font::FontStyleId,
    game::{Game, GameCtx, GameMeta},
    input::Key,
};

pub struct Pong {
    pub ball: Vec2,
    pub vel: Vec2,
    pub paddle_l: f32,
    pub paddle_r: f32,
    pub score_l: u32,
    pub score_r: u32,
    pub font_style: FontStyleId,
    pub showing_instructions: bool,
}

impl Default for Pong {
    fn default() -> Self {
        Self::new()
    }
}

impl Pong {
    pub fn new() -> Self {
        Self {
            ball: Vec2::ZERO,
            vel: Vec2::new(0.6, 0.25),
            paddle_l: 0.0,
            paddle_r: 0.0,
            score_l: 0,
            score_r: 0,
            font_style: FontStyleId::ATARI,
            showing_instructions: true,
        }
    }
}

impl Game for Pong {
    fn metadata(&self) -> GameMeta {
        GameMeta {
            name: "Pong",
            preferred_aspect: Some(4.0 / 3.0),
        }
    }

    fn reset(&mut self, _ctx: &mut GameCtx) {
        self.ball = Vec2::ZERO;
        self.vel = Vec2::new(0.6, 0.25);
        self.paddle_l = 0.0;
        self.paddle_r = 0.0;
        self.score_l = 0;
        self.score_r = 0;
        self.showing_instructions = true;
    }

    fn update(&mut self, ctx: &mut GameCtx, dt: f32) {
        if self.showing_instructions {
            if ctx.input.key(Key::Space).went_down {
                self.showing_instructions = false;
            }
            return;
        }
        physics::update_paddles(self, ctx, dt);
        physics::update_ball(self, dt);
    }

    fn render(&mut self, _ctx: &mut GameCtx, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::Clear { color: Rgba::BLACK });
        if self.showing_instructions {
            drawing::render_instructions(out, self.font_style);
        } else {
            drawing::render_court(out, self.paddle_l, self.paddle_r, self.ball);
            drawing::render_scores(out, self.score_l, self.score_r, self.font_style);
        }
    }
}
