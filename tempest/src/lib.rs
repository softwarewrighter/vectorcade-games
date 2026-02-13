//! Tempest tube-shooter game implementation.
//!
//! Player moves around the rim of a tube and shoots enemies climbing up.
//! Multi-colored vector graphics matching the 1981 arcade original.

mod enemies;
mod rendering;
mod tube;

use vectorcade_shared::{
    Rgba,
    draw::DrawCmd,
    font::FontStyleId,
    game::{Game, GameCtx, GameMeta},
    input::Key,
};

use enemies::{Enemy, EnemyKind};
use tube::Tube;

/// Colors matching original Tempest arcade.
pub const YELLOW: Rgba = Rgba::YELLOW;
pub const BLUE: Rgba = Rgba::BLUE;
pub const RED: Rgba = Rgba::RED;
pub const GREEN: Rgba = Rgba::GREEN;
pub const CYAN: Rgba = Rgba::CYAN;
pub const MAGENTA: Rgba = Rgba::MAGENTA;

/// Game state.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState { Instructions, Playing, GameOver, LevelComplete }

/// Player's blaster on the tube rim.
pub struct Blaster {
    pub segment: usize,
    pub fire_cooldown: f32,
}

/// Player shot traveling down the tube.
pub struct Shot {
    pub segment: usize,
    pub depth: f32,
    pub alive: bool,
}

/// Main game struct.
pub struct Tempest {
    pub tube: Tube,
    pub blaster: Blaster,
    pub shots: Vec<Shot>,
    pub enemies: Vec<Enemy>,
    pub score: u32,
    pub lives: u8,
    pub level: u8,
    pub state: GameState,
    pub spawn_timer: f32,
    pub font_style: FontStyleId,
    pub blink_timer: f32,
}

impl Default for Tempest {
    fn default() -> Self { Self::new() }
}

impl Tempest {
    pub fn new() -> Self {
        Self {
            tube: Tube::circle(16),
            blaster: Blaster { segment: 0, fire_cooldown: 0.0 },
            shots: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            lives: 3,
            level: 1,
            state: GameState::Instructions,
            spawn_timer: 0.0,
            font_style: FontStyleId::ATARI,
            blink_timer: 0.0,
        }
    }

    fn spawn_enemy(&mut self, ctx: &mut GameCtx) {
        let segment = ctx.rng.range_i32(0, self.tube.segments as i32) as usize;
        let kind = match ctx.rng.range_i32(0, 3) {
            0 => EnemyKind::Flipper,
            1 => EnemyKind::Tanker,
            _ => EnemyKind::Spiker,
        };
        self.enemies.push(Enemy::new(segment, kind));
    }
}

impl Game for Tempest {
    fn metadata(&self) -> GameMeta {
        GameMeta { name: "Tempest", preferred_aspect: Some(4.0 / 3.0) }
    }

    fn reset(&mut self, _ctx: &mut GameCtx) {
        self.tube = Tube::circle(16);
        self.blaster = Blaster { segment: 0, fire_cooldown: 0.0 };
        self.shots.clear();
        self.enemies.clear();
        self.score = 0;
        self.lives = 3;
        self.level = 1;
        self.state = GameState::Instructions;
        self.spawn_timer = 0.0;
        self.blink_timer = 0.0;
    }

    fn update(&mut self, ctx: &mut GameCtx, dt: f32) {
        if self.state == GameState::Instructions {
            self.blink_timer += dt;
            if ctx.input.key(Key::Space).went_down { self.state = GameState::Playing; }
            return;
        }
        if self.state == GameState::GameOver || self.state == GameState::LevelComplete { return; }
        update_player(self, ctx, dt);
        enemies::update_enemies(&mut self.enemies, &self.tube, dt);
        update_shots(self, dt);
        check_collisions(self);
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 && self.enemies.len() < 6 {
            self.spawn_enemy(ctx);
            self.spawn_timer = 1.5 - (self.level as f32 * 0.1).min(1.0);
        }
        if self.enemies.is_empty() && self.spawn_timer <= 0.0 {
            self.state = GameState::LevelComplete;
        }
    }

    fn render(&mut self, _ctx: &mut GameCtx, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::Clear { color: Rgba::BLACK });
        if self.state == GameState::Instructions {
            rendering::render_instructions(out, self.font_style, self.blink_timer);
            return;
        }
        rendering::render_tube(out, &self.tube);
        rendering::render_blaster(out, &self.tube, &self.blaster);
        rendering::render_shots(out, &self.tube, &self.shots);
        rendering::render_enemies(out, &self.tube, &self.enemies);
        rendering::render_hud(out, self.score, self.lives, self.level, self.font_style);
        if self.state == GameState::GameOver {
            rendering::render_game_over(out, self.font_style);
        }
    }
}

fn update_player(game: &mut Tempest, ctx: &GameCtx, dt: f32) {
    let left = ctx.input.key(Key::Left).went_down;
    let right = ctx.input.key(Key::Right).went_down;
    let segs = game.tube.segments;
    if left { game.blaster.segment = (game.blaster.segment + segs - 1) % segs; }
    if right { game.blaster.segment = (game.blaster.segment + 1) % segs; }
    game.blaster.fire_cooldown -= dt;
    if ctx.input.key(Key::Space).is_down && game.blaster.fire_cooldown <= 0.0 {
        game.blaster.fire_cooldown = 0.15;
        game.shots.push(Shot { segment: game.blaster.segment, depth: 0.0, alive: true });
    }
}

fn update_shots(game: &mut Tempest, dt: f32) {
    for shot in &mut game.shots {
        shot.depth += 2.0 * dt;
        if shot.depth > 1.0 { shot.alive = false; }
    }
    game.shots.retain(|s| s.alive);
}

fn check_collisions(game: &mut Tempest) {
    // Shots hitting enemies
    for shot in &mut game.shots {
        for enemy in &mut game.enemies {
            if shot.segment == enemy.segment && (shot.depth - enemy.depth).abs() < 0.1 {
                shot.alive = false;
                enemy.alive = false;
                game.score += enemy.kind.points();
            }
        }
    }
    // Enemies reaching the rim (hitting player)
    for enemy in &mut game.enemies {
        if enemy.depth <= 0.0 && enemy.segment == game.blaster.segment {
            enemy.alive = false;
            game.lives = game.lives.saturating_sub(1);
            if game.lives == 0 { game.state = GameState::GameOver; }
        }
    }
    game.shots.retain(|s| s.alive);
    game.enemies.retain(|e| e.alive);
}
