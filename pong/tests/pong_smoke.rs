use pong::Pong;
use vectorcade_shared::Xorshift64;
use vectorcade_shared::game::{AudioOut, Game, GameCtx, ScreenInfo};
use vectorcade_shared::input::{Axis, Button, InputState, Key};

struct NoInput;
impl InputState for NoInput {
    fn key(&self, _k: Key) -> Button { Button::UP }
    fn axis(&self, _a: Axis) -> f32 { 0.0 }
    fn pointer(&self) -> Option<vectorcade_shared::input::Pointer> { None }
}

struct NoAudio;
impl AudioOut for NoAudio {}

#[test]
fn pong_advances() {
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
    let mut g = Pong::new();
    g.showing_instructions = false; // Skip instructions for test
    let x0 = g.ball.x;
    g.update(&mut ctx, 0.016);
    assert!(g.ball.x != x0);
}
