# Statistics & Metrics

## Codebase Stats

| Metric | Value | Notes |
|--------|-------|-------|
| Games Implemented | 1 | Pong (in progress) |
| Games Planned | 4 | Asteroids, Lunar Lander, Battlezone, Tempest |
| Total Crates | 2 | vectorcade-games (facade), pong |
| Test Count | 1 | pong_smoke.rs |

## Lines of Code

| Crate | Lines | Status |
|-------|-------|--------|
| pong/src/lib.rs | ~130 | Core game logic |
| vectorcade-games/src/lib.rs | ~10 | Registry facade |
| pong/tests/pong_smoke.rs | ~30 | Smoke test |
| **Total** | ~170 | |

## DrawCmd Budget (Per Frame)

Target: <500 DrawCmd per frame for smooth performance.

| Game | Estimated DrawCmd | Breakdown |
|------|-------------------|-----------|
| Pong | ~10 | 2 paddles, 1 ball, 1 center line, 2 scores |
| Asteroids | ~100-200 | Ship + bullets + asteroids + particles |
| Lunar Lander | ~50-100 | Ship + terrain + UI gauges |
| Battlezone | ~200-400 | 3D geometry + HUD + radar |
| Tempest | ~300-500 | Tube geometry + enemies + player |

## Game Complexity Estimates

| Game | Entities | Physics | 3D | AI |
|------|----------|---------|----|----|
| Pong | Low (~5) | Simple | No | No |
| Asteroids | Medium (~50) | Medium | No | No |
| Lunar Lander | Low (~10) | Complex | No | No |
| Battlezone | Medium (~30) | Medium | Yes | Simple |
| Tempest | High (~100) | Simple | Yes | Simple |

## Test Coverage Goals

| Game | Smoke | Determinism | Edge Cases | Integration |
|------|-------|-------------|------------|-------------|
| Pong | ✅ | ⬜ | ⬜ | ⬜ |
| Asteroids | ⬜ | ⬜ | ⬜ | ⬜ |
| Lunar Lander | ⬜ | ⬜ | ⬜ | ⬜ |
| Battlezone | ⬜ | ⬜ | ⬜ | ⬜ |
| Tempest | ⬜ | ⬜ | ⬜ | ⬜ |

Legend: ✅ Complete | 🔄 In Progress | ⬜ Not Started

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Frame Rate | 60 fps | Fixed timestep at 16.67ms |
| Update Time | <2ms | Game logic per frame |
| DrawCmd Generation | <1ms | Render method |
| Memory (per game) | <10MB | Including entity pools |

## Dependency Versions

| Crate | Version | Purpose |
|-------|---------|---------|
| glam | 0.27 | Vector math (Vec2, Vec3) |
| vectorcade-shared | 0.1.0 | Core traits and types |
| vectorcade-fonts | 0.1.0 | Vector font rendering |

## Historical Progress

| Date | Milestone | Notes |
|------|-----------|-------|
| 2026-02-12 | Project Init | Repo structure, Pong stub |
| - | - | - |

---

*This document will be updated as development progresses.*
