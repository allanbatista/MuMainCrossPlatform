# Local Movement Sync Progress

Status: DONE_FOR_LOCAL_MOVEMENT_SYNC

Current state: the local movement slice is implemented and validated locally.
The next concrete step is the server-authoritative follow-up tracked in
memory, while this slice keeps the local avatar seed, movement input, and scene
sync stable.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0530-local-movement-sync/spec.md`, `.features/20260520-0530-local-movement-sync/plan.md`, `.features/20260520-0530-local-movement-sync/progress.md` | `.features/20260520-0530-local-movement-sync/spec.md`, `.features/20260520-0530-local-movement-sync/plan.md`, `.features/20260520-0530-local-movement-sync/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_gameplay/src/entities.rs` | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_gameplay/src/entities.rs` | unit tests for the seed and pose helpers | `runtime_loads_world_bundle_and_render_assets`, `world_entities_manager_translates_local_player_pose` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | movement controller and scene sync tests | `world_motion_moves_the_local_avatar_in_the_world_route`, `world_motion_ignores_non_world_routes`, `world_scene_reconciles_the_local_player_marker_after_runtime_motion` | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/client_runtime.rs` | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_app/src/world_motion.rs` | route/reset guard tests | `world_motion_ignores_non_world_routes`, `world_motion_ignores_a_reset_world_projection` | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/*`, `port_rust/crates/mu_gameplay/src/*` | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_gameplay/src/entities.rs` | cargo test outputs | `cargo test -p mu_gameplay`, `cargo test -p mu_app`, `cargo test --workspace` | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | `rtk rg -n "movement|avatar|WASD" docs/player-rust-client.md port_rust/README.md` | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-0530-local-movement-sync/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`, `cargo test -p mu_gameplay`, `cargo test -p mu_app`, `cargo test --workspace`, `cargo build -p mu_client`, `cargo run -p mu_client -- --headless`, `timeout 5s cargo run -p mu_client` | none |
| F3.S1.T2 | done | local | `.memory/TODO.md`, `.codexpotter/kb/local-movement-sync.md`, `.codexpotter/kb/README.md`, progress | `.memory/TODO.md`, `.codexpotter/kb/local-movement-sync.md`, `.codexpotter/kb/README.md`, `.features/20260520-0530-local-movement-sync/progress.md` | memory + kb diff | server-authoritative movement follow-up recorded and slice facts captured in KB | none |

## Done

- Created the feature spec, plan, and progress documents for the local
  movement slice, then audited them cleanly. This establishes the next
  implementation step without leaving the controls or scope ambiguous.
- Implemented the local movement slice in `mu_app`: seeded a placeholder local
  avatar on world load, added a world-route movement controller, kept the
  rendered world markers in sync with the runtime pose, and documented the
  first interactive controls. Files changed:
  `port_rust/crates/mu_app/src/client_runtime.rs`,
  `port_rust/crates/mu_app/src/world_motion.rs`,
  `port_rust/crates/mu_app/src/world_scene.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/lib.rs`,
  `port_rust/crates/mu_gameplay/src/entities.rs`,
  `docs/player-rust-client.md`, `port_rust/README.md`, `.memory/TODO.md`,
  `.codexpotter/projects/2026/05/20/1/MAIN.md`.
- Validated the slice with `cargo fmt --manifest-path port_rust/Cargo.toml
  --all`, `cargo test -p mu_gameplay`, `cargo test -p mu_app`,
  `cargo test --workspace`, `cargo build -p mu_client`, `cargo run -p
  mu_client -- --headless`, `timeout 5s cargo run -p mu_client`, and the docs
  grep gate for `movement|avatar|WASD`.
- Captured the slice facts in `.codexpotter/kb/local-movement-sync.md` and
  indexed them in `.codexpotter/kb/README.md` for future movement work.

## Files Touched

New:

- `.codexpotter/kb/local-movement-sync.md`
- `.features/20260520-0530-local-movement-sync/spec.md`
- `.features/20260520-0530-local-movement-sync/plan.md`
- `.features/20260520-0530-local-movement-sync/progress.md`
- `port_rust/crates/mu_app/src/world_motion.rs`

Modified:

- `.codexpotter/projects/2026/05/20/1/MAIN.md`
- `.codexpotter/kb/README.md`
- `.memory/TODO.md`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `port_rust/crates/mu_app/src/client_runtime.rs`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `port_rust/crates/mu_app/src/lib.rs`
- `port_rust/crates/mu_app/src/world_scene.rs`
- `port_rust/crates/mu_gameplay/src/entities.rs`

## Validation Evidence

- `rtk node /home/allanbatista/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0530-local-movement-sync`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`
- `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk rg -n "movement|avatar|WASD" docs/player-rust-client.md port_rust/README.md`
- `rtk sed -n '1,220p' .codexpotter/kb/local-movement-sync.md`
