# Project Status

> **Implementation status for the `vectorcade-games` repository.**
> This document tracks progress on game crates within this repo.

## Current State

| Aspect | Status | Notes |
|--------|--------|-------|
| **Dependencies** | Ready | `vectorcade-shared` and `vectorcade-fonts` available |
| **Pong** | Complete | Core logic complete, smoke test passing |
| **Asteroids** | Complete | Full implementation, smoke test passing |
| **Lunar Lander** | Not Started | Planned |
| **Battlezone** | Not Started | Planned |
| **Tempest** | Not Started | Planned |

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
*Not started*

### Battlezone
*Not started*

### Tempest
*Not started*

## Test Coverage

| Game | Smoke | Determinism | Edge Cases | Integration |
|------|-------|-------------|------------|-------------|
| Pong | Done | Todo | Todo | Todo |
| Asteroids | Done | Todo | Todo | Todo |
| Lunar Lander | Todo | Todo | Todo | Todo |
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
| 2026-02-12 | Asteroids implementation complete |
| 2026-02-12 | Dependencies (shared, fonts) now available |
| 2026-02-12 | Project initialized with Pong stub |

---

*Updated as development progresses.*
