# Product Requirements Document

## Product Vision

Provide a collection of classic vector arcade game implementations that demonstrate the VectorCade platform's capabilities while remaining faithful to the original aesthetic.

## Target Games

| Game | Status | Complexity | Key Features |
|------|--------|------------|--------------|
| Pong | In Progress | Low | 2-player, paddle physics, scoring |
| Asteroids | Planned | Medium | Screen wrap, rotation, bullets, particles |
| Lunar Lander | Planned | Medium | Gravity, thrust, fuel management, landing |
| Battlezone | Planned | High | 3D-to-2D projection, tank combat |
| Tempest | Planned | High | 3D tubes, perspective projection, depth |

## Functional Requirements

### FR-1: Game Trait Compliance
All games MUST implement the `Game` trait:
- `metadata()` -> name, aspect ratio hints
- `reset()` -> return to initial state
- `update(dt)` -> advance simulation by fixed timestep
- `render()` -> emit `DrawCmd` display list

### FR-2: Input Abstraction
Games MUST use abstract input via `GameCtx.input`:
- Keyboard keys mapped to `Key` enum
- Analog axes for touch/gamepad support
- No direct DOM/browser input handling

### FR-3: Deterministic Updates
Games SHOULD support deterministic playback:
- Fixed timestep (typically 1/60s)
- Optional seeded RNG via `GameCtx`
- No reliance on system time for game state

### FR-4: Vector Font Integration
Score displays and labels MUST use `DrawCmd::Text` with `FontStyleId`:
- Each game can specify its preferred font style
- Renderer handles stroke-based text rendering

### FR-5: Testability
Each game MUST include smoke tests that:
- Run headless (no renderer required)
- Verify basic state advancement
- Test collision/physics edge cases

## Non-Functional Requirements

### NFR-1: Performance
- Games should support 60fps on modest hardware
- Avoid per-frame allocations in hot paths
- Target <500 draw commands per frame for 2D games

### NFR-2: Portability
- No wasm-bindgen, web-sys, or platform dependencies
- Pure Rust with `no_std`-compatible math (via glam)

### NFR-3: Maintainability
- Each game in its own crate
- Shared utilities in `vectorcade-shared`
- Clear separation from rendering

## Game-Specific Requirements

### Pong
- Two-player support (keyboard: W/S vs Up/Down)
- Ball speed increases slightly on paddle hits (optional)
- Score display with vector font
- Center dividing line

### Asteroids (Planned)
- Ship rotation and thrust
- Wraparound screen edges
- Asteroid splitting (large -> medium -> small)
- Bullet lifetime and cooldown
- Particle effects for explosions

### Lunar Lander (Planned)
- Gravity simulation
- Rotational controls (left/right)
- Main thruster with fuel consumption
- Terrain collision detection
- Safe landing zones with scoring

### Battlezone (Planned)
- First-person 3D wireframe view
- Tank movement and rotation
- Enemy tanks with basic AI
- Obstacles (pyramids, blocks)
- Radar display

### Tempest (Planned)
- Tube/web level geometry
- Player moves around tube rim
- Enemies climb up from depth
- Superzapper weapon
- Level progression with different tube shapes

## Success Criteria

1. All five games playable in browser via vectorcade-web-yew
2. Each game visually faithful to original vector aesthetic
3. Smooth 60fps performance on mid-range devices
4. Touch controls functional for mobile play
5. All games pass headless test suites
