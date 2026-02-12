//! Physics and collision handling for Lunar Lander.

use vectorcade_shared::{game::GameCtx, input::Key};

use crate::lander::Lander;
use crate::terrain::Terrain;
use crate::GameState;

const GRAVITY: f32 = 0.15;
const THRUST: f32 = 0.35;
const ROTATION_SPEED: f32 = 2.5;
const FUEL_BURN_RATE: f32 = 15.0;
const SAFE_VELOCITY: f32 = 0.25;
const SAFE_ANGLE: f32 = 0.3;
const LANDER_HEIGHT: f32 = 0.05;

pub fn update_lander(lander: &mut Lander, ctx: &GameCtx, dt: f32) {
    // Rotation
    if ctx.input.key(Key::Left).is_down {
        lander.angle -= ROTATION_SPEED * dt;
    }
    if ctx.input.key(Key::Right).is_down {
        lander.angle += ROTATION_SPEED * dt;
    }
    // Thrust
    let thrust = ctx.input.key(Key::Up).is_down || ctx.input.key(Key::Space).is_down;
    lander.thrusting = thrust && lander.fuel > 0.0;
    if lander.thrusting {
        lander.vel += lander.direction() * THRUST * dt;
        lander.fuel -= FUEL_BURN_RATE * dt;
        lander.fuel = lander.fuel.max(0.0);
    }
    // Gravity
    lander.vel.y -= GRAVITY * dt;
    // Position update
    lander.pos += lander.vel * dt;
    // Horizontal bounds
    lander.pos.x = lander.pos.x.clamp(-0.95, 0.95);
}

pub fn check_landing(lander: &Lander, terrain: &Terrain) -> GameState {
    let ground_y = terrain.surface_y_at(lander.pos.x);
    let lander_bottom = lander.pos.y - LANDER_HEIGHT;
    if lander_bottom > ground_y {
        return GameState::Playing;
    }
    // Check if on landing pad with safe velocity and angle
    let on_pad = terrain.pad_at(lander.pos.x).is_some();
    let safe_vel = lander.vel.length() < SAFE_VELOCITY;
    let safe_angle = lander.angle.abs() < SAFE_ANGLE;
    if on_pad && safe_vel && safe_angle {
        GameState::Landed
    } else {
        GameState::Crashed
    }
}

pub fn calculate_score(lander: &Lander) -> u32 {
    let fuel_bonus = (lander.fuel * 10.0) as u32;
    100 + fuel_bonus
}
