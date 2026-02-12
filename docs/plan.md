# Implementation Plan

## Phase 1: Pong Completion (Current)

### 1.1 Core Gameplay (Done)
- [x] Ball movement and bouncing
- [x] Paddle movement (keyboard)
- [x] Collision detection
- [x] Scoring system
- [x] Basic rendering (DrawCmd emission)

### 1.2 Polish (In Progress)
- [ ] Ball speed increase on paddle hits
- [ ] Serve direction alternates
- [ ] Win condition (first to 11)
- [ ] Game over / restart flow
- [ ] Sound hooks (via AudioOut trait)

### 1.3 Testing
- [x] Basic smoke test
- [ ] Determinism test (same inputs -> same state)
- [ ] Edge case tests (corner bounces, simultaneous hits)

## Phase 2: Asteroids

### 2.1 Ship
- [ ] Rotation (Left/Right keys)
- [ ] Thrust (Up key)
- [ ] Velocity with friction
- [ ] Screen wraparound

### 2.2 Asteroids
- [ ] Large/medium/small variants
- [ ] Random initial positions and velocities
- [ ] Split on hit (large->medium->small)
- [ ] Screen wraparound

### 2.3 Combat
- [ ] Bullet firing (Space key)
- [ ] Bullet lifetime
- [ ] Fire rate limiting
- [ ] Collision: bullet<->asteroid, ship<->asteroid

### 2.4 Effects
- [ ] Explosion particles (polyline bursts)
- [ ] Ship destruction animation
- [ ] Hyperspace (random teleport)

### 2.5 Scoring
- [ ] Points per asteroid size
- [ ] Lives system
- [ ] High score tracking

## Phase 3: Lunar Lander

### 3.1 Physics
- [ ] Gravity constant
- [ ] Thrust force
- [ ] Angular momentum (rotation)
- [ ] Fuel consumption

### 3.2 Terrain
- [ ] Procedural terrain generation
- [ ] Landing pad placement
- [ ] Collision detection (ship hull vs terrain)

### 3.3 Landing
- [ ] Safe velocity threshold
- [ ] Safe angle threshold
- [ ] Landing pad bonus multiplier
- [ ] Crash detection

### 3.4 UI
- [ ] Fuel gauge
- [ ] Altitude indicator
- [ ] Horizontal/vertical velocity display

## Phase 4: Battlezone

### 4.1 3D Foundation
- [ ] Camera3 integration
- [ ] First-person view projection
- [ ] Horizon line rendering

### 4.2 Player Tank
- [ ] Forward/backward movement
- [ ] Rotation
- [ ] Cannon firing
- [ ] Damage/destruction

### 4.3 Environment
- [ ] Ground plane grid
- [ ] Obstacle geometry (pyramids, cubes)
- [ ] Volcano (distant landmark)

### 4.4 Enemies
- [ ] Enemy tank spawning
- [ ] Basic pursuit AI
- [ ] Enemy firing
- [ ] Hit detection

### 4.5 HUD
- [ ] Radar display
- [ ] Score
- [ ] Damage indicator

## Phase 5: Tempest

### 5.1 Tube Geometry
- [ ] Level shape definitions (circle, square, plus, etc.)
- [ ] Perspective projection (depth)
- [ ] Player position on rim

### 5.2 Player
- [ ] Move around tube rim
- [ ] Fire down tube
- [ ] Superzapper (screen clear)

### 5.3 Enemies
- [ ] Flipper (basic enemy)
- [ ] Tanker (splits into flippers)
- [ ] Spiker (leaves spikes)
- [ ] Fuseball (moves on rim)

### 5.4 Mechanics
- [ ] Enemy climbing
- [ ] Level completion (clear all enemies)
- [ ] Level progression (tube shape changes)
- [ ] Depth-based brightness

## Phase 6: Integration and Polish

### 6.1 Game Registry
- [ ] `all_games()` returns all five games
- [ ] Game selection metadata
- [ ] Difficulty settings

### 6.2 Cross-Game Features
- [ ] Pause/resume
- [ ] High score persistence (via web shell)
- [ ] Touch input mapping

### 6.3 Testing Suite
- [ ] All games have smoke tests
- [ ] Replay-based regression tests
- [ ] Performance benchmarks

## Dependencies

```
Phase 1 (Pong) ---------------------------------+
                                               |
Phase 2 (Asteroids) ---------------------------+
                                               +--> Phase 6 (Integration)
Phase 3 (Lunar Lander) ------------------------+
                                               |
Phase 4 (Battlezone) ----+---------------------+
                         |                     |
Phase 5 (Tempest) -------+                     |
        (shares 3D projection work)            |
```

## Milestones

| Milestone | Deliverable | Criteria |
|-----------|-------------|----------|
| M1 | Pong Complete | Playable, tested, with scoring |
| M2 | Asteroids Complete | All mechanics, particles, lives |
| M3 | Lunar Lander Complete | Physics, terrain, landing |
| M4 | 3D Games Complete | Battlezone + Tempest playable |
| M5 | Full Suite | All 5 games integrated, tested |
