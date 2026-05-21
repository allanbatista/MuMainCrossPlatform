# World Scene Shell Progress

Status: OPEN

Current state: the visible world-scene shell is implemented, documented, and
validated with unit tests plus local runtime smoke. The remaining full-parity
work stays tracked in the workspace TODO notes.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0318-world-scene-shell/spec.md`, `.features/20260520-0318-world-scene-shell/plan.md`, `.features/20260520-0318-world-scene-shell/progress.md` | `.features/20260520-0318-world-scene-shell/spec.md`, `.features/20260520-0318-world-scene-shell/plan.md`, `.features/20260520-0318-world-scene-shell/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | unit tests for scene bootstrap/cleanup | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app` passed, including scene bootstrap tests | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | route-based cleanup coverage | `world_scene::tests::world_scene_clears_when_route_leaves_world` passed | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | scene bootstrap and fallback tests | `world_scene::tests::world_scene_spawns_visible_shell_when_world_route_is_ready`, `world_scene::tests::world_scene_waits_for_world_projection` passed | none |
| F2.S1.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | visible world shell wording added to player guide and port README | none |
| F2.S2.T1 | done | local | progress/docs | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.memory/TODO.md`, `.codexpotter/kb/world-scene-shell.md`, `.codexpotter/kb/README.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`, `mu_client --headless`, `mu_client --headless --control-http 127.0.0.1:0`, docs grep gate | none |
| F2.S3.T1 | done | local | `.memory/TODO.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | `.memory/TODO.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | memory/progress diff | pending work notes updated for the world scene shell and the remaining parity slice | none |

## Done

- Created the feature spec, plan, and progress control docs for the world
  scene shell slice. Files changed:
  `.features/20260520-0318-world-scene-shell/spec.md`,
  `.features/20260520-0318-world-scene-shell/plan.md`,
  `.features/20260520-0318-world-scene-shell/progress.md`.
- Implemented the route-driven world scene shell in `mu_app` and wired it into
  the graphical runtime. Files changed:
  `port_rust/crates/mu_app/src/world_scene.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/lib.rs`.
- Updated player-facing docs to describe the visible world shell and its
  limits. Files changed: `docs/player-rust-client.md`,
  `port_rust/README.md`.
- Recorded the slice in the workspace control docs and KB:
  `.codexpotter/projects/2026/05/20/1/MAIN.md`,
  `.memory/TODO.md`, `.codexpotter/kb/world-scene-shell.md`,
  `.codexpotter/kb/README.md`.

## Files Touched

New:

- `port_rust/crates/mu_app/src/world_scene.rs`
- `.features/20260520-0318-world-scene-shell/spec.md`
- `.features/20260520-0318-world-scene-shell/plan.md`
- `.features/20260520-0318-world-scene-shell/progress.md`
- `.codexpotter/kb/world-scene-shell.md`

Modified:

- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `port_rust/crates/mu_app/src/lib.rs`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`
- `.memory/TODO.md`
- `.codexpotter/kb/README.md`

Removed:

- none

## Validation Evidence

- `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0318-world-scene-shell`
- `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`
- `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
- `rtk curl -s http://127.0.0.1:45561/state`
- `rtk curl -s -X POST 'http://127.0.0.1:45561/command?name=exit'`
- `rtk rg -n "world shell|world-scene-shell" docs/player-rust-client.md port_rust/README.md`
