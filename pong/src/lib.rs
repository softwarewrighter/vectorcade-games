use glam::{Vec2};
use vectorcade_shared::{
    color::Rgba,
    draw::{DrawCmd, Stroke, rect_wire},
    font::FontStyleId,
    game::{Game, GameCtx, GameMeta},
    input::Key,
    math::clamp,
};

pub struct Pong {
    pub ball: Vec2,
    pub vel: Vec2,
    pub paddle_l: f32,
    pub paddle_r: f32,
    pub score_l: u32,
    pub score_r: u32,
    pub font_style: FontStyleId,
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
        }
    }
}

impl Game for Pong {
    fn metadata(&self) -> GameMeta {
        GameMeta { name: "Pong", preferred_aspect: Some(4.0/3.0) }
    }

    fn reset(&mut self, _ctx: &mut GameCtx) {
        self.ball = Vec2::ZERO;
        self.vel = Vec2::new(0.6, 0.25);
        self.paddle_l = 0.0;
        self.paddle_r = 0.0;
        self.score_l = 0;
        self.score_r = 0;
    }

    fn update(&mut self, ctx: &mut GameCtx, dt: f32) {
        // Player controls (keyboard stub). Web shell should map touch → axes later.
        let left_up = ctx.input.key(Key::W).is_down;
        let left_dn = ctx.input.key(Key::S).is_down;
        let right_up = ctx.input.key(Key::Up).is_down;
        let right_dn = ctx.input.key(Key::Down).is_down;

        let spd = 1.2;
        if left_up { self.paddle_l += spd * dt; }
        if left_dn { self.paddle_l -= spd * dt; }
        if right_up { self.paddle_r += spd * dt; }
        if right_dn { self.paddle_r -= spd * dt; }

        self.paddle_l = clamp(self.paddle_l, -0.8, 0.8);
        self.paddle_r = clamp(self.paddle_r, -0.8, 0.8);

        self.ball += self.vel * dt;

        // bounce on top/bottom
        if self.ball.y > 0.95 { self.ball.y = 0.95; self.vel.y = -self.vel.y; }
        if self.ball.y < -0.95 { self.ball.y = -0.95; self.vel.y = -self.vel.y; }

        // paddles
        let paddle_half = 0.18;
        let paddle_x_l = -0.9;
        let paddle_x_r = 0.9;

        let hit_l = (self.ball.x - paddle_x_l).abs() < 0.03 && (self.ball.y - self.paddle_l).abs() < paddle_half;
        let hit_r = (self.ball.x - paddle_x_r).abs() < 0.03 && (self.ball.y - self.paddle_r).abs() < paddle_half;
        if hit_l && self.vel.x < 0.0 { self.vel.x = -self.vel.x; }
        if hit_r && self.vel.x > 0.0 { self.vel.x = -self.vel.x; }

        // score
        if self.ball.x < -1.05 {
            self.score_r += 1;
            self.ball = Vec2::ZERO;
            self.vel = Vec2::new(0.6, 0.25);
        }
        if self.ball.x > 1.05 {
            self.score_l += 1;
            self.ball = Vec2::ZERO;
            self.vel = Vec2::new(-0.6, 0.25);
        }
    }

    fn render(&mut self, _ctx: &mut GameCtx, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::Clear { color: Rgba::BLACK });

        let white = Stroke::new(Rgba::WHITE, 2.0);
        // center line
        out.push(DrawCmd::Polyline {
            pts: vec![Vec2::new(0.0, -1.0), Vec2::new(0.0, 1.0)],
            closed: false,
            stroke: Stroke { color: Rgba::WHITE.with_a(0.6), width_px: 2.0, glow: 0.0 },
        });

        // paddles
        let paddle_half = 0.18;
        out.push(rect_wire(Vec2::new(-0.92, self.paddle_l - paddle_half), Vec2::new(-0.88, self.paddle_l + paddle_half), white));
        out.push(rect_wire(Vec2::new(0.88, self.paddle_r - paddle_half), Vec2::new(0.92, self.paddle_r + paddle_half), white));

        // ball (tiny square)
        out.push(rect_wire(self.ball - Vec2::splat(0.02), self.ball + Vec2::splat(0.02), white));

        // score (placeholder using Text cmd; web renderer will implement via vector fonts)
        out.push(DrawCmd::Text {
            pos: Vec2::new(-0.2, 0.85),
            text: format!("{}", self.score_l),
            size_px: 18.0,
            color: Rgba::WHITE,
            style: self.font_style,
        });
        out.push(DrawCmd::Text {
            pos: Vec2::new(0.15, 0.85),
            text: format!("{}", self.score_r),
            size_px: 18.0,
            color: Rgba::WHITE,
            style: self.font_style,
        });
    }
}
