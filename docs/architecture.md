# Architecture

## Overview

`vectorcade-games` contains game logic crates for the VectorCade platform. Games are **renderer-agnostic**: they emit `DrawCmd` display lists and never touch rendering APIs directly.

## Dependency Graph

```
vectorcade-shared (root)
       │
       ├───────────────┐
       ▼               ▼
vectorcade-fonts   vectorcade-render-wgpu
       │
       ▼
vectorcade-games ──────► (depends on shared + fonts)
       │
       ▼
vectorcade-web-yew (integration)
```

## Crate Structure

```
vectorcade-games/
├── Cargo.toml           # Workspace root
├── vectorcade-games/    # Facade crate (re-exports all games)
│   ├── Cargo.toml
│   └── src/lib.rs       # all_games() registry
├── pong/                # Individual game crate
│   ├── Cargo.toml
│   ├── src/lib.rs
│   └── tests/
├── asteroids/           # (planned)
├── lunar-lander/        # (planned)
├── battlezone/          # (planned)
└── tempest/             # (planned)
```

## Game Trait Architecture

Every game implements the `Game` trait from `vectorcade-shared`:

```rust
pub trait Game {
    fn metadata(&self) -> GameMeta;
    fn reset(&mut self, ctx: &mut GameCtx);
    fn update(&mut self, ctx: &mut GameCtx, dt: f32);
    fn render(&mut self, ctx: &mut GameCtx, out: &mut Vec<DrawCmd>);
}
```

### Separation of Concerns

| Layer | Responsibility | Location |
|-------|----------------|----------|
| Input | Abstract input state (keys, axes, pointers) | `vectorcade-shared` |
| Game Logic | Physics, collision, scoring, state machines | `vectorcade-games/*` |
| Draw Commands | Display list primitives (lines, polylines, text) | `vectorcade-shared` |
| Rendering | Tessellation, GPU submission | `vectorcade-render-wgpu` |
| Integration | Event loop, canvas, UI | `vectorcade-web-yew` |

## Design Principles

1. **Pure Logic**: Game crates contain no platform-specific code
2. **Deterministic**: Fixed timestep + seeded RNG enables replays and testing
3. **Display List**: All rendering via `DrawCmd` enum—games don't know about wgpu/canvas
4. **Font Abstraction**: Text uses `FontStyleId` so each game can have distinct visual style
5. **Testable**: Headless smoke tests verify game logic without a renderer

## Data Flow

```
Input Events → InputState → Game::update() → Game::render() → Vec<DrawCmd>
                                                                    │
                                                                    ▼
                                                            VectorRenderer
                                                                    │
                                                                    ▼
                                                              GPU/Canvas
```

## Local Development

With sibling repos cloned into the same parent directory, `.cargo/config.toml` patches crates-io to use local paths:

```toml
[patch.crates-io]
vectorcade-shared = { path = "../vectorcade-shared/vectorcade-shared" }
vectorcade-fonts = { path = "../vectorcade-fonts/vectorcade-fonts" }
```
