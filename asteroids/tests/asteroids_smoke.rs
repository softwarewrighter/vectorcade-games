use asteroids::Asteroids;
use vectorcade_shared::Xorshift64;
use vectorcade_shared::game::{AudioOut, Game, GameCtx, ScreenInfo};
use vectorcade_shared::input::{Axis, Button, InputState, Key};

struct NoInput;
impl InputState for NoInput {
    fn key(&self, _k: Key) -> Button {
        Button::UP
    }
    fn axis(&self, _a: Axis) -> f32 {
        0.0
    }
    fn pointer(&self) -> Option<vectorcade_shared::input::Pointer> {
        None
    }
}

struct NoAudio;
impl AudioOut for NoAudio {}

#[test]
fn asteroids_advances() {
    let input = NoInput;
    let audio = NoAudio;
    let mut rng = Xorshift64::new(12345);
    let mut ctx = GameCtx {
        input: &input,
        audio: &audio,
        rng: &mut rng,
        screen: ScreenInfo::default(),
        now_s: 0.0,
    };
    let mut g = Asteroids::new();
    g.reset(&mut ctx);
    let initial_asteroids = g.asteroids.len();
    assert!(initial_asteroids > 0, "should spawn asteroids on reset");
    g.update(&mut ctx, 0.016);
}
