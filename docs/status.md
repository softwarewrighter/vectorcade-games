# Project Status

> **Implementation status for the `vectorcade-games` repository.**
> This document tracks progress on game crates within this repo.

## Current State

| Aspect | Status | Notes |
|--------|--------|-------|
| **Dependencies** | Ready | `vectorcade-shared` and `vectorcade-fonts` available |
| **Pong** | Complete | Core logic complete, smoke test passing |
| **Asteroids** | Complete | Full implementation, smoke test passing |
| **Lunar Lander** | Complete | Gravity/thrust physics, smoke test passing |
| **Battlezone** | Not Started | Planned |
| **Tempest** | Not Started | Planned |
| **Chess Demo** | Complete | Static vector chess board display |

Legend: [x] Complete | [~] In Progress | [ ] Not Started

## Dependency Status

Dependencies are available in sibling repos:

| Dependency | Path | Status |
|------------|------|--------|
| vectorcade-shared | `../vectorcade-shared/vectorcade-shared/` | Ready |
| vectorcade-fonts | `../vectorcade-fonts/vectorcade-fonts/` | Ready |

## Crate Inventory

| Crate | Lines | Purpose | Status |
|-------|-------|---------|--------|
| vectorcade-games | ~15 | Registry facade (`all_games()`) | Complete |
| pong | ~200 | Pong game logic | Complete |
| asteroids | ~630 | Asteroids game logic | Complete |
| lunar-lander | ~280 | Lunar Lander game logic | Complete |
| chess-demo | ~180 | Vector chess board demo | Complete |

## Game Implementation Progress

### Pong

| Feature | Status | Notes |
|---------|--------|-------|
| Ball movement | Done | Velocity-based with dt |
| Paddle movement | Done | W/S and Up/Down keys |
| Collision detection | Done | Ball<->paddle, ball<->walls |
| Scoring | Done | Tracks left/right scores |
| DrawCmd rendering | Done | Emits lines, polylines, text |
| Speed increase | Todo | On paddle hits |
| Win condition | Todo | First to 11 |
| Sound hooks | Todo | Via AudioOut trait |
| Smoke test | Done | pong_smoke.rs |
| Determinism test | Todo | Same inputs -> same state |

### Asteroids

| Feature | Status | Notes |
|---------|--------|-------|
| Ship movement | Done | Thrust, rotation, friction |
| Shooting | Done | Cooldown, bullet limit (8 max) |
| Asteroid spawning | Done | Edge spawning, 3 sizes |
| Asteroid splitting | Done | Large->Medium->Small |
| Collision detection | Done | Bullets/asteroids, ship/asteroids |
| Scoring | Done | 20/50/100 points by size |
| Lives system | Done | 3 lives, respawn invulnerability |
| Level progression | Done | More asteroids per level |
| Particle effects | Done | Explosion particles |
| Screen wraparound | Done | All entities wrap |
| HUD | Done | Score, lives display |
| Game over | Done | End screen on 0 lives |
| Smoke test | Done | asteroids_smoke.rs |
| Determinism test | Todo | Same inputs -> same state |

### Lunar Lander

| Feature | Status | Notes |
|---------|--------|-------|
| Lander physics | Done | Gravity, thrust, rotation |
| Fuel management | Done | Burns with thrust, HUD display |
| Terrain generation | Done | Procedural mountains with landing pads |
| Landing pads | Done | Multiple pads with score multipliers |
| Collision detection | Done | Surface collision, safe landing check |
| Landing conditions | Done | Velocity and angle requirements |
| Score system | Done | Based on fuel remaining |
| HUD | Done | Fuel, velocity, altitude display |
| Game states | Done | Playing, Landed, Crashed |
| Smoke test | Done | lunar_lander_smoke.rs |

### Battlezone
*Not started*

### Tempest
*Not started*

### Chess Demo

| Feature | Status | Notes |
|---------|--------|-------|
| Board rendering | Done | 8x8 grid with dot-pattern dark squares |
| Piece vectors | Done | King, Queen, Rook, Bishop, Knight, Pawn |
| Starting position | Done | Valid chess starting setup |
| File/rank labels | Done | A-H, 1-8 labels |
| Smoke test | Done | chess_demo_smoke.rs |

## Test Coverage

| Game | Smoke | Determinism | Edge Cases | Integration |
|------|-------|-------------|------------|-------------|
| Pong | Done | Todo | Todo | Todo |
| Asteroids | Done | Todo | Todo | Todo |
| Lunar Lander | Done | Todo | Todo | Todo |
| Battlezone | Todo | Todo | Todo | Todo |
| Tempest | Todo | Todo | Todo | Todo |

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Frame Rate | 60 fps | Fixed timestep at 16.67ms |
| Update Time | <2ms | Game logic per frame |
| DrawCmd Generation | <1ms | Render method |
| DrawCmd Count | <500/frame | Per game |

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| glam | 0.27 | Vector math (Vec2, Vec3) |
| vectorcade-shared | 0.1.0 | Core traits and types |
| vectorcade-fonts | 0.1.0 | Vector font rendering |

## Changelog

| Date | Change |
|------|--------|
| 2026-02-12 | Chess Demo implementation complete |
| 2026-02-12 | Lunar Lander implementation complete |
| 2026-02-12 | Asteroids implementation complete |
| 2026-02-12 | Dependencies (shared, fonts) now available |
| 2026-02-12 | Project initialized with Pong stub |

---

*Updated as development progresses.*
