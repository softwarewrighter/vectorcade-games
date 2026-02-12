//! Physics and collision handling for Asteroids.

use glam::Vec2;
use vectorcade_shared::{GameRng, game::GameCtx, input::Key, wrap_position};

use crate::entities::{Asteroid, Bullet, Particle, Ship};
use crate::{Asteroids, MAX_ASTEROIDS, MAX_BULLETS, MAX_PARTICLES};

const ROTATION_SPEED: f32 = 4.0;
const THRUST_ACCEL: f32 = 1.0;
const MAX_SPEED: f32 = 0.8;
const FRICTION: f32 = 0.5;
const FIRE_COOLDOWN: f32 = 0.25;
const SHIP_RADIUS: f32 = 0.03;

pub fn update_ship(ship: &mut Ship, ctx: &GameCtx, dt: f32) {
    if ctx.input.key(Key::Left).is_down {
        ship.angle += ROTATION_SPEED * dt;
    }
    if ctx.input.key(Key::Right).is_down {
        ship.angle -= ROTATION_SPEED * dt;
    }

    let thrust = ctx.input.key(Key::Up).is_down || ctx.input.key(Key::W).is_down;
    ship.thrusting = thrust;
    if thrust {
        ship.vel += ship.direction() * THRUST_ACCEL * dt;
        if ship.vel.length() > MAX_SPEED {
            ship.vel = ship.vel.normalize() * MAX_SPEED;
        }
    } else {
        ship.vel *= 1.0 - FRICTION * dt;
    }
    ship.pos += ship.vel * dt;
    ship.pos = wrap_position(ship.pos);
}

pub fn update_bullets(bullets: &mut Vec<Bullet>, dt: f32) {
    for b in bullets.iter_mut() {
        b.update(dt);
    }
    bullets.retain(|b| b.is_alive());
}

pub fn update_asteroids(asteroids: &mut [Asteroid], dt: f32) {
    for a in asteroids.iter_mut() {
        a.update(dt);
    }
}

pub fn update_particles(particles: &mut Vec<Particle>, dt: f32) {
    for p in particles.iter_mut() {
        p.update(dt);
    }
    particles.retain(|p| p.is_alive());
}

static mut FIRE_TIMER: f32 = 0.0;

pub fn handle_shooting(ship: &Ship, bullets: &mut Vec<Bullet>, ctx: &GameCtx, dt: f32) {
    unsafe {
        FIRE_TIMER -= dt;
    }
    if ctx.input.key(Key::Space).is_down
        && unsafe { FIRE_TIMER <= 0.0 }
        && bullets.len() < MAX_BULLETS
    {
        bullets.push(Bullet::new(
            ship.pos + ship.direction() * 0.04,
            ship.direction(),
        ));
        unsafe {
            FIRE_TIMER = FIRE_COOLDOWN;
        }
    }
}

pub fn handle_collisions(game: &mut Asteroids, ctx: &mut GameCtx) {
    let mut splits: Vec<Asteroid> = Vec::new();
    game.bullets.retain(|bullet| {
        for (i, a) in game.asteroids.iter().enumerate() {
            if (bullet.pos - a.pos).length() < a.size.radius() {
                game.score += a.size.points();
                spawn_explosion(&mut game.particles, a.pos, ctx.rng);
                if a.size.split().is_some() {
                    splits.push(Asteroid::spawn_split(a, ctx.rng));
                    splits.push(Asteroid::spawn_split(a, ctx.rng));
                }
                game.asteroids.swap_remove(i);
                return false;
            }
        }
        true
    });
    for a in splits {
        if game.asteroids.len() < MAX_ASTEROIDS {
            game.asteroids.push(a);
        }
    }

    if game.respawn_timer > 0.0 {
        game.respawn_timer -= 1.0 / 60.0;
        return;
    }
    for a in &game.asteroids {
        if (game.ship.pos - a.pos).length() < a.size.radius() + SHIP_RADIUS {
            game.lives = game.lives.saturating_sub(1);
            spawn_explosion(&mut game.particles, game.ship.pos, ctx.rng);
            if game.lives == 0 {
                game.game_over = true;
            } else {
                game.ship = Ship::new();
                game.respawn_timer = 2.0;
            }
            break;
        }
    }
}

fn spawn_explosion(particles: &mut Vec<Particle>, pos: Vec2, rng: &mut dyn GameRng) {
    for _ in 0..8 {
        if particles.len() >= MAX_PARTICLES {
            break;
        }
        let angle = rng.range_f32(0.0, std::f32::consts::TAU);
        particles.push(Particle::new(
            pos,
            Vec2::new(angle.cos(), angle.sin()) * rng.range_f32(0.2, 0.5),
        ));
    }
}
