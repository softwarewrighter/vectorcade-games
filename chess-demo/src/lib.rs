//! Vector chess board demo.

mod board;
mod pieces;

use vectorcade_shared::{
    Rgba,
    draw::DrawCmd,
    font::FontStyleId,
    game::{Game, GameCtx, GameMeta},
};

/// Chess demo - displays a static chess board with pieces.
pub struct ChessDemo {
    font_style: FontStyleId,
}

impl Default for ChessDemo {
    fn default() -> Self { Self::new() }
}

impl ChessDemo {
    pub fn new() -> Self {
        Self { font_style: FontStyleId::ATARI }
    }
}

impl Game for ChessDemo {
    fn metadata(&self) -> GameMeta {
        GameMeta { name: "Chess Image", preferred_aspect: Some(1.0) }
    }

    fn reset(&mut self, _ctx: &mut GameCtx) {}

    fn update(&mut self, _ctx: &mut GameCtx, _dt: f32) {}

    fn render(&mut self, _ctx: &mut GameCtx, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::Clear { color: Rgba::BLACK });
        board::render_title(out, self.font_style);
        board::render_board(out);
        render_pieces(out);
        board::render_labels(out, self.font_style);
    }
}

fn render_pieces(out: &mut Vec<DrawCmd>) {
    let scale = board::square_scale();
    let back_row = [pieces::ROOK, pieces::KNIGHT, pieces::BISHOP, pieces::QUEEN,
                    pieces::KING, pieces::BISHOP, pieces::KNIGHT, pieces::ROOK];
    // White pieces (rows 0-1)
    for (col, &piece) in back_row.iter().enumerate() {
        pieces::draw_piece(out, board::square_center(0, col), piece, scale * 0.35);
    }
    for col in 0..8 {
        pieces::draw_piece(out, board::square_center(1, col), pieces::PAWN, scale * 0.3);
    }
    // Black pieces (rows 6-7)
    for (col, &piece) in back_row.iter().enumerate() {
        pieces::draw_piece(out, board::square_center(7, col), piece, scale * 0.35);
    }
    for col in 0..8 {
        pieces::draw_piece(out, board::square_center(6, col), pieces::PAWN, scale * 0.3);
    }
}
