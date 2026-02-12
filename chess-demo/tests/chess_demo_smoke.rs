use chess_demo::ChessDemo;
use vectorcade_shared::Xorshift64;
use vectorcade_shared::draw::DrawCmd;
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
fn chess_demo_renders() {
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
    let mut g = ChessDemo::new();
    let mut cmds = Vec::new();
    g.render(&mut ctx, &mut cmds);
    assert!(!cmds.is_empty(), "should generate draw commands");
    // Should have Clear, board lines, dots, pieces, labels
    assert!(cmds.len() > 100, "should have many draw commands for full board");
}
