//! Lunar Lander game implementation.

mod lander;
mod physics;
mod rendering;
mod terrain;

use vectorcade_shared::{
    Rgba,
    draw::DrawCmd,
    font::FontStyleId,
    game::{Game, GameCtx, GameMeta},
    input::Key,
};

use lander::Lander;
use terrain::Terrain;

/// Game state enumeration.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Instructions,
    Playing,
    Landed,
    Crashed,
}

/// Main game struct.
pub struct LunarLander {
    pub lander: Lander,
    pub terrain: Terrain,
    pub state: GameState,
    pub score: u32,
    pub font_style: FontStyleId,
}

impl Default for LunarLander {
    fn default() -> Self { Self::new() }
}

impl LunarLander {
    pub fn new() -> Self {
        Self {
            lander: Lander::new(),
            terrain: Terrain::new(),
            state: GameState::Instructions,
            score: 0,
            font_style: FontStyleId::ATARI,
        }
    }
}

impl Game for LunarLander {
    fn metadata(&self) -> GameMeta {
        GameMeta { name: "Lunar Lander", preferred_aspect: Some(4.0 / 3.0) }
    }

    fn reset(&mut self, ctx: &mut GameCtx) {
        self.lander = Lander::new();
        self.terrain = Terrain::generate(ctx.rng);
        self.state = GameState::Instructions;
        self.score = 0;
    }

    fn update(&mut self, ctx: &mut GameCtx, dt: f32) {
        if self.state == GameState::Instructions {
            if ctx.input.key(Key::Space).went_down {
                self.state = GameState::Playing;
            }
            return;
        }
        if self.state != GameState::Playing { return; }
        physics::update_lander(&mut self.lander, ctx, dt);
        self.state = physics::check_landing(&self.lander, &self.terrain);
        if self.state == GameState::Landed {
            self.score = physics::calculate_score(&self.lander);
        }
    }

    fn render(&mut self, _ctx: &mut GameCtx, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::Clear { color: Rgba::BLACK });
        if self.state == GameState::Instructions {
            rendering::render_instructions(out, self.font_style);
            return;
        }
        rendering::render_terrain(out, &self.terrain);
        rendering::render_lander(out, &self.lander);
        rendering::render_hud(out, &self.lander, self.font_style);
        if self.state == GameState::Landed {
            rendering::render_landed(out, self.score, self.font_style);
        } else if self.state == GameState::Crashed {
            rendering::render_crashed(out, self.font_style);
        }
    }
}
