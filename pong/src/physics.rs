use glam::Vec2;
use vectorcade_shared::{clamp, game::GameCtx, input::Key};

use crate::Pong;

const PADDLE_SPEED: f32 = 1.2;
const PADDLE_HALF: f32 = 0.18;
const PADDLE_X_L: f32 = -0.9;
const PADDLE_X_R: f32 = 0.9;

pub fn update_paddles(pong: &mut Pong, ctx: &GameCtx, dt: f32) {
    let left_up = ctx.input.key(Key::Up).is_down;
    let left_dn = ctx.input.key(Key::Down).is_down;
    let right_up = ctx.input.key(Key::W).is_down;
    let right_dn = ctx.input.key(Key::S).is_down;

    if left_up {
        pong.paddle_l += PADDLE_SPEED * dt;
    }
    if left_dn {
        pong.paddle_l -= PADDLE_SPEED * dt;
    }
    if right_up {
        pong.paddle_r += PADDLE_SPEED * dt;
    }
    if right_dn {
        pong.paddle_r -= PADDLE_SPEED * dt;
    }

    pong.paddle_l = clamp(pong.paddle_l, -0.8, 0.8);
    pong.paddle_r = clamp(pong.paddle_r, -0.8, 0.8);
}

pub fn update_ball(pong: &mut Pong, dt: f32) {
    pong.ball += pong.vel * dt;
    bounce_walls(pong);
    bounce_paddles(pong);
    check_scoring(pong);
}

fn bounce_walls(pong: &mut Pong) {
    if pong.ball.y > 0.95 {
        pong.ball.y = 0.95;
        pong.vel.y = -pong.vel.y;
    }
    if pong.ball.y < -0.95 {
        pong.ball.y = -0.95;
        pong.vel.y = -pong.vel.y;
    }
}

fn bounce_paddles(pong: &mut Pong) {
    let hit_l = (pong.ball.x - PADDLE_X_L).abs() < 0.03
        && (pong.ball.y - pong.paddle_l).abs() < PADDLE_HALF;
    let hit_r = (pong.ball.x - PADDLE_X_R).abs() < 0.03
        && (pong.ball.y - pong.paddle_r).abs() < PADDLE_HALF;

    if hit_l && pong.vel.x < 0.0 {
        pong.vel.x = -pong.vel.x;
    }
    if hit_r && pong.vel.x > 0.0 {
        pong.vel.x = -pong.vel.x;
    }
}

fn check_scoring(pong: &mut Pong) {
    if pong.ball.x < -1.05 {
        pong.score_r += 1;
        pong.ball = Vec2::ZERO;
        pong.vel = Vec2::new(0.6, 0.25);
    }
    if pong.ball.x > 1.05 {
        pong.score_l += 1;
        pong.ball = Vec2::ZERO;
        pong.vel = Vec2::new(-0.6, 0.25);
    }
}
