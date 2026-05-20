---
status: OPEN
---

# World Camera Follow Progress

Current state: the world camera follow slice is complete. The follow-camera
system now keeps the local avatar framed in the world route, the docs/KB are
updated, and the validation gates passed.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-2100-world-camera-follow/spec.md`, `.features/20260520-2100-world-camera-follow/plan.md`, `.features/20260520-2100-world-camera-follow/progress.md` | `.features/20260520-2100-world-camera-follow/spec.md`, `.features/20260520-2100-world-camera-follow/plan.md`, `.features/20260520-2100-world-camera-follow/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | unit tests for camera tracking | `world_scene_camera_follows_the_local_player_marker` passed in `cargo test --manifest-path port_rust/Cargo.toml -p mu_app` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | camera follows local avatar in the world route | follow-camera spawn/update wiring now keeps the camera offset anchored to the local avatar | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | scene cleanup and camera-follow coverage | existing route cleanup/reload tests plus the new follow-camera test passed | none |
| F2.S1.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | docs now mention the follow camera in the visible world shell | none |
| F2.S2.T1 | done | local | `.codexpotter/kb/world-camera-follow.md`, `.codexpotter/kb/README.md`, `.memory/TODO.md` | `.codexpotter/kb/world-camera-follow.md`, `.codexpotter/kb/README.md`, `.memory/TODO.md` | KB and pending-work updates | KB note added and camera zoom follow-up recorded | none |
| F3.S1.T1 | done | local | progress/docs | `.features/20260520-2100-world-camera-follow/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:45575` with `/state` and `exit`; `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client` | none |

## Files Touched

New:

- `.features/20260520-2100-world-camera-follow/spec.md`
- `.features/20260520-2100-world-camera-follow/plan.md`
- `.features/20260520-2100-world-camera-follow/progress.md`
- `.codexpotter/kb/world-camera-follow.md`

Modified:

- `.codexpotter/projects/2026/05/20/1/MAIN.md`
- `port_rust/crates/mu_app/src/world_scene.rs`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `.codexpotter/kb/README.md`
- `.memory/TODO.md`

Removed:

- none
