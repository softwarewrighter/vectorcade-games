//! Battlezone game implementation.
//!
//! First-person tank combat with 3D vector graphics.
//! Uses green for gameplay area, red for score/radar (matching original arcade overlay).

mod enemies;
mod rendering;
mod world;

use glam::Vec3;
use vectorcade_shared::{
    Rgba,
    draw::DrawCmd,
    font::FontStyleId,
    game::{Game, GameCtx, GameMeta},
    input::Key,
    normalize_angle,
    projectile::{Projectile3D, update_projectiles_3d},
};

use enemies::{Enemy, EnemyKind};
use world::Obstacle;

/// Colors matching the original arcade overlay.
pub const GREEN: Rgba = Rgba::GREEN;
pub const RED: Rgba = Rgba::RED;

/// Game state.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState { Instructions, Playing, GameOver }

/// Main game struct.
pub struct Battlezone {
    pub pos: Vec3,
    pub angle: f32,
    pub score: u32,
    pub lives: u8,
    pub state: GameState,
    pub enemies: Vec<Enemy>,
    pub obstacles: Vec<Obstacle>,
    pub shots: Vec<Projectile3D>,
    pub fire_cooldown: f32,
    pub font_style: FontStyleId,
}

impl Default for Battlezone {
    fn default() -> Self { Self::new() }
}

impl Battlezone {
    pub fn new() -> Self {
        Self {
            pos: Vec3::ZERO,
            angle: 0.0,
            score: 0,
            lives: 3,
            state: GameState::Instructions,
            enemies: Vec::new(),
            obstacles: Vec::new(),
            shots: Vec::new(),
            fire_cooldown: 0.0,
            font_style: FontStyleId::ATARI,
        }
    }

    fn spawn_enemy(&mut self, ctx: &mut GameCtx) {
        let angle = ctx.rng.range_f32(0.0, std::f32::consts::TAU);
        let dist = ctx.rng.range_f32(15.0, 25.0);
        let pos = Vec3::new(angle.cos() * dist, 0.0, angle.sin() * dist);
        self.enemies.push(Enemy::new(pos, EnemyKind::Tank));
    }
}

impl Game for Battlezone {
    fn metadata(&self) -> GameMeta {
        GameMeta { name: "Battlezone", preferred_aspect: Some(4.0 / 3.0) }
    }

    fn reset(&mut self, ctx: &mut GameCtx) {
        self.pos = Vec3::ZERO;
        self.angle = 0.0;
        self.score = 0;
        self.lives = 3;
        self.state = GameState::Instructions;
        self.enemies.clear();
        self.obstacles.clear();
        self.shots.clear();
        self.fire_cooldown = 0.0;
        world::spawn_obstacles(&mut self.obstacles, ctx.rng);
    }

    fn update(&mut self, ctx: &mut GameCtx, dt: f32) {
        if self.state == GameState::Instructions {
            if ctx.input.key(Key::Space).went_down { self.state = GameState::Playing; }
            return;
        }
        if self.state == GameState::GameOver { return; }
        update_player(self, ctx, dt);
        update_projectiles_3d(&mut self.shots, dt);
        check_shot_collisions(self);
        enemies::update_enemies(&mut self.enemies, self.pos, dt);
        if self.enemies.is_empty() { self.spawn_enemy(ctx); }
    }

    fn render(&mut self, _ctx: &mut GameCtx, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::Clear { color: Rgba::BLACK });
        if self.state == GameState::Instructions {
            rendering::render_instructions(out, self.font_style);
            return;
        }
        rendering::render_horizon(out);
        rendering::render_world(out, &self.obstacles, &self.enemies, self.pos, self.angle);
        rendering::render_shots(out, &self.shots, self.pos, self.angle);
        rendering::render_crosshair(out);
        rendering::render_hud(out, self.score, self.lives, self.font_style);
        if self.state == GameState::GameOver {
            rendering::render_game_over(out, self.font_style);
        }
    }
}

fn update_player(game: &mut Battlezone, ctx: &GameCtx, dt: f32) {
    let left = ctx.input.key(Key::Left).is_down;
    let right = ctx.input.key(Key::Right).is_down;
    let fwd = ctx.input.key(Key::Up).is_down || ctx.input.key(Key::W).is_down;
    let back = ctx.input.key(Key::Down).is_down || ctx.input.key(Key::S).is_down;
    if left { game.angle += 2.0 * dt; }
    if right { game.angle -= 2.0 * dt; }
    game.angle = normalize_angle(game.angle);
    let speed = 5.0;
    let dir = Vec3::new(-game.angle.sin(), 0.0, -game.angle.cos());
    if fwd { game.pos += dir * speed * dt; }
    if back { game.pos -= dir * speed * dt * 0.5; }
    game.fire_cooldown -= dt;
    if ctx.input.key(Key::Space).went_down && game.fire_cooldown <= 0.0 {
        game.fire_cooldown = 0.5;
        // Fire a projectile from gun barrel height, slightly in front of player
        let barrel_height = Vec3::new(0.0, 0.8, 0.0);
        let start_pos = game.pos + barrel_height + dir * 1.0;
        let shot = Projectile3D::new(start_pos, dir, 30.0, 50.0);
        game.shots.push(shot);
    }
}

fn check_shot_collisions(game: &mut Battlezone) {
    for shot in &mut game.shots {
        for enemy in &mut game.enemies {
            if enemy.alive && shot.hits_sphere(enemy.pos, 1.5) {
                shot.alive = false;
                enemy.alive = false;
                game.score += 1000;
                break;
            }
        }
    }
    game.shots.retain(|s| s.alive);
    game.enemies.retain(|e| e.alive);
}
