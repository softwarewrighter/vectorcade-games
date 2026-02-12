# Project Status

> **Implementation status for the `vectorcade-games` repository.**
> This document tracks progress on game crates within this repo.

## Current State

| Aspect | Status | Notes |
|--------|--------|-------|
| **Dependencies** | ⬜ Blocked | Requires `vectorcade-shared` and `vectorcade-fonts` |
| **Pong** | 🔄 In Progress | Core logic complete, needs polish |
| **Asteroids** | ⬜ Not Started | Planned |
| **Lunar Lander** | ⬜ Not Started | Planned |
| **Battlezone** | ⬜ Not Started | Planned |
| **Tempest** | ⬜ Not Started | Planned |

Legend: ✅ Complete | 🔄 In Progress | ⬜ Not Started

## Dependency Status

This repo cannot compile until these sibling repos exist:

| Dependency | Path | Status |
|------------|------|--------|
| vectorcade-shared | `../vectorcade-shared/vectorcade-shared/` | ⬜ Needed |
| vectorcade-fonts | `../vectorcade-fonts/vectorcade-fonts/` | ⬜ Needed |

## Crate Inventory

| Crate | Lines | Purpose | Status |
|-------|-------|---------|--------|
| vectorcade-games | ~10 | Registry facade (`all_games()`) | 🔄 Stub |
| pong | ~130 | Pong game logic | 🔄 Core done |

## Game Implementation Progress

### Pong

| Feature | Status | Notes |
|---------|--------|-------|
| Ball movement | ✅ | Velocity-based with dt |
| Paddle movement | ✅ | W/S and Up/Down keys |
| Collision detection | ✅ | Ball↔paddle, ball↔walls |
| Scoring | ✅ | Tracks left/right scores |
| DrawCmd rendering | ✅ | Emits lines, polylines, text |
| Speed increase | ⬜ | On paddle hits |
| Win condition | ⬜ | First to 11 |
| Sound hooks | ⬜ | Via AudioOut trait |
| Smoke test | ✅ | pong_smoke.rs |
| Determinism test | ⬜ | Same inputs → same state |

### Asteroids
*Not started*

### Lunar Lander
*Not started*

### Battlezone
*Not started*

### Tempest
*Not started*

## Test Coverage

| Game | Smoke | Determinism | Edge Cases | Integration |
|------|-------|-------------|------------|-------------|
| Pong | ✅ | ⬜ | ⬜ | ⬜ |
| Asteroids | ⬜ | ⬜ | ⬜ | ⬜ |
| Lunar Lander | ⬜ | ⬜ | ⬜ | ⬜ |
| Battlezone | ⬜ | ⬜ | ⬜ | ⬜ |
| Tempest | ⬜ | ⬜ | ⬜ | ⬜ |

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
| 2026-02-12 | Project initialized with Pong stub |

---

*Updated as development progresses.*
