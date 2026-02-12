use vectorcade_shared::game::{Game, GameCtx, ScreenInfo, AudioOut};
use vectorcade_shared::input::{InputState, Key, Axis, Button};
use pong::Pong;

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
    let mut ctx = GameCtx {
        input: &input,
        audio: &audio,
        screen: ScreenInfo { width_px: 800, height_px: 600, dpi_scale: 1.0 },
        now_s: 0.0,
    };
    let mut g = Pong::new();
    let x0 = g.ball.x;
    g.update(&mut ctx, 0.016);
    assert!(g.ball.x != x0);
}
