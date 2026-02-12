//! Asteroids game implementation.

mod entities;
mod physics;
mod rendering;

use vectorcade_shared::{
    Rgba,
    draw::DrawCmd,
    font::FontStyleId,
    game::{Game, GameCtx, GameMeta},
};

use entities::{Asteroid, Bullet, Particle, Ship};

const MAX_ASTEROIDS: usize = 20;
const MAX_BULLETS: usize = 8;
const MAX_PARTICLES: usize = 50;

pub struct Asteroids {
    pub ship: Ship,
    pub asteroids: Vec<Asteroid>,
    pub bullets: Vec<Bullet>,
    pub particles: Vec<Particle>,
    pub score: u32,
    pub lives: u8,
    pub level: u8,
    pub font_style: FontStyleId,
    pub game_over: bool,
    pub respawn_timer: f32,
}

impl Default for Asteroids {
    fn default() -> Self {
        Self::new()
    }
}

impl Asteroids {
    pub fn new() -> Self {
        Self {
            ship: Ship::new(),
            asteroids: Vec::with_capacity(MAX_ASTEROIDS),
            bullets: Vec::with_capacity(MAX_BULLETS),
            particles: Vec::with_capacity(MAX_PARTICLES),
            score: 0,
            lives: 3,
            level: 1,
            font_style: FontStyleId::ATARI,
            game_over: false,
            respawn_timer: 0.0,
        }
    }

    fn spawn_level_asteroids(&mut self, ctx: &mut GameCtx) {
        let count = 3 + self.level as usize;
        for _ in 0..count.min(MAX_ASTEROIDS) {
            self.asteroids.push(Asteroid::spawn_random(ctx.rng));
        }
    }
}

impl Game for Asteroids {
    fn metadata(&self) -> GameMeta {
        GameMeta {
            name: "Asteroids",
            preferred_aspect: Some(4.0 / 3.0),
        }
    }

    fn reset(&mut self, ctx: &mut GameCtx) {
        self.ship = Ship::new();
        self.asteroids.clear();
        self.bullets.clear();
        self.particles.clear();
        self.score = 0;
        self.lives = 3;
        self.level = 1;
        self.game_over = false;
        self.respawn_timer = 0.0;
        self.spawn_level_asteroids(ctx);
    }

    fn update(&mut self, ctx: &mut GameCtx, dt: f32) {
        if self.game_over {
            return;
        }

        physics::update_ship(&mut self.ship, ctx, dt);
        physics::update_bullets(&mut self.bullets, dt);
        physics::update_asteroids(&mut self.asteroids, dt);
        physics::update_particles(&mut self.particles, dt);

        physics::handle_shooting(&self.ship, &mut self.bullets, ctx, dt);
        physics::handle_collisions(self, ctx);

        // Check for level complete
        if self.asteroids.is_empty() {
            self.level += 1;
            self.spawn_level_asteroids(ctx);
        }
    }

    fn render(&mut self, _ctx: &mut GameCtx, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::Clear { color: Rgba::BLACK });

        if !self.game_over && self.respawn_timer <= 0.0 {
            rendering::render_ship(out, &self.ship);
        }

        for asteroid in &self.asteroids {
            rendering::render_asteroid(out, asteroid);
        }

        for bullet in &self.bullets {
            rendering::render_bullet(out, bullet);
        }

        for particle in &self.particles {
            rendering::render_particle(out, particle);
        }

        rendering::render_hud(out, self.score, self.lives, self.font_style);

        if self.game_over {
            rendering::render_game_over(out, self.font_style);
        }
    }
}
