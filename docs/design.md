# Design Document

## Design Goals

1. **Faithful Aesthetics**: Black backgrounds, vector lines, phosphor glow feel
2. **Clean Separation**: Game logic isolated from rendering and platform
3. **Extensibility**: Easy to add new games following established patterns
4. **Testability**: All game logic verifiable without graphics

## Game State Pattern

Each game follows a consistent state pattern:

```rust
pub struct GameName {
    // Simulation state
    pub position: Vec2,
    pub velocity: Vec2,

    // Game state
    pub score: u32,
    pub lives: u8,

    // Configuration
    pub font_style: FontStyleId,
}
```

### State Categories

| Category | Mutated By | Example |
|----------|------------|---------|
| Simulation | `update()` | positions, velocities, timers |
| Game | `update()` | score, lives, level |
| Configuration | `new()` / `reset()` | font style, difficulty |
| Transient | `render()` only | draw commands (not stored) |

## Update/Render Split

The `update()` and `render()` methods are strictly separated:

```
update(ctx, dt):
  - Read input from ctx.input
  - Advance physics by dt
  - Handle collisions
  - Update score/state
  - NO drawing

render(ctx, out):
  - Read current state
  - Emit DrawCmd to out vec
  - NO state mutation
```

This enables:
- Running multiple update steps per render (frame skip)
- Headless testing without renderer
- Replay recording (only need to record inputs)

## Coordinate System

Games operate in normalized world coordinates:

```
        +1.0
          |
   -1.0 --+-- +1.0
          |
        -1.0
```

- Origin at center
- X: -1 (left) to +1 (right)
- Y: -1 (bottom) to +1 (top)
- Aspect ratio handled by renderer via letterboxing

## Collision Detection

Simple collision approaches per game type:

### Pong
- AABB for paddle/ball (rectangle intersection)
- Point-line for boundary checks

### Asteroids
- Circle-circle for ship/asteroid/bullet
- Radius stored per entity

### Lunar Lander
- Point-polyline for ship/terrain
- AABB for landing pad zones

### Battlezone / Tempest
- 3D collision in world space before projection
- View frustum culling for rendering

## DrawCmd Usage Patterns

### Lines and Polylines
```rust
// Single line
out.push(DrawCmd::Line(Line2 { a, b, stroke }));

// Connected line strip
out.push(DrawCmd::Polyline { pts, closed: false, stroke });

// Closed polygon outline
out.push(DrawCmd::Polyline { pts, closed: true, stroke });
```

### Rectangles (Helper)
```rust
// Wire rectangle (via helper function)
out.push(rect_wire(min, max, stroke));
```

### Text
```rust
out.push(DrawCmd::Text {
    pos: Vec2::new(x, y),
    text: format!("{}", score),
    size_px: 18.0,
    color: Rgba::WHITE,
    style: FontStyleId::ATARI,
});
```

## Input Mapping

### Keyboard Conventions
| Game | Left/CCW | Right/CW | Action1 | Action2 |
|------|----------|----------|---------|---------|
| Pong (L) | W | S | - | - |
| Pong (R) | Up | Down | - | - |
| Asteroids | Left | Right | Space (fire) | Up (thrust) |
| Lunar Lander | Left | Right | Space (thrust) | - |
| Battlezone | A/D | Left/Right | Space (fire) | W (forward) |

### Touch Mapping (Future)
- Virtual joystick on left side
- Action buttons on right side
- Tilt controls (optional)

## 3D Projection (Battlezone/Tempest)

For pseudo-3D games, use the projection helpers from `vectorcade-shared`:

```rust
pub struct Camera3 {
    pub pos: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y_rad: f32,
}

pub fn project_polyline(cam: &Camera3, pts3: &[[f32;3]]) -> Vec<[f32;2]>;
```

Games maintain 3D world state but render via 2D DrawCmd after projection.

## Error Handling

Games should be resilient:
- Clamp values to valid ranges (paddle positions, velocities)
- Reset on invalid state rather than panic
- Log warnings via `GameCtx` (future) for debugging

## Performance Considerations

1. **Reuse allocations**: Clear and reuse draw command vectors
2. **Avoid cloning**: Pass references where possible
3. **Batch similar draws**: Group DrawCmd by stroke for renderer efficiency
4. **Limit particle count**: Cap explosion particles to ~50
