# World HUD Overlay Progress

Status: DONE_FOR_WORLD_HUD_OVERLAY

Current state: the world HUD overlay slice is implemented, documented, and
validated. The Bevy runtime now shows the legacy HUD main frame on the world
route and clears it again when the route leaves the world.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0615-world-hud-overlay/spec.md`, `.features/20260520-0615-world-hud-overlay/plan.md`, `.features/20260520-0615-world-hud-overlay/progress.md` | `.features/20260520-0615-world-hud-overlay/spec.md`, `.features/20260520-0615-world-hud-overlay/plan.md`, `.features/20260520-0615-world-hud-overlay/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/world_hud.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/world_hud.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | unit tests for overlay spawn/cleanup and body formatting | `world_hud::tests::world_hud_spawns_visible_overlay_when_world_route_is_ready`, `world_hud::tests::world_hud_clears_when_route_leaves_world`, `world_hud::tests::hud_body_lists_the_legacy_main_frame_state` | none |
| F1.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | world HUD overlay wording added to player guide and port README | none |
| F2.S1.T1 | done | local | progress docs | `.features/20260520-0615-world-hud-overlay/progress.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`, `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0` | none |
| F2.S1.T2 | done | local | `.memory/TODO.md`, `.codexpotter/kb/world-hud-overlay.md`, `.codexpotter/kb/README.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | `.memory/TODO.md`, `.codexpotter/kb/world-hud-overlay.md`, `.codexpotter/kb/README.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | memory + kb diff | pending-work memory cleaned up and HUD overlay facts captured in KB | none |

## Done

- Created and audited the feature spec, plan, and progress control docs for
  the world HUD overlay slice. Files changed:
  `.features/20260520-0615-world-hud-overlay/spec.md`,
  `.features/20260520-0615-world-hud-overlay/plan.md`,
  `.features/20260520-0615-world-hud-overlay/progress.md`.
- Implemented the route-gated world HUD overlay in `mu_app` and wired it into
  the graphical runtime. Files changed:
  `port_rust/crates/mu_app/src/world_hud.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/lib.rs`.
- Updated player-facing docs to describe the visible world HUD overlay. Files
  changed: `docs/player-rust-client.md`, `port_rust/README.md`.
- Recorded the slice in the workspace control docs and KB, and cleaned the
  stale pending-work entries. Files changed:
  `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.memory/TODO.md`,
  `.codexpotter/kb/world-hud-overlay.md`, `.codexpotter/kb/README.md`.

## Files Touched

New:

- `port_rust/crates/mu_app/src/world_hud.rs`
- `.features/20260520-0615-world-hud-overlay/spec.md`
- `.features/20260520-0615-world-hud-overlay/plan.md`
- `.features/20260520-0615-world-hud-overlay/progress.md`
- `.codexpotter/kb/world-hud-overlay.md`

Modified:

- `.codexpotter/projects/2026/05/20/1/MAIN.md`
- `.codexpotter/kb/README.md`
- `.memory/TODO.md`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `port_rust/crates/mu_app/src/lib.rs`

## Validation Evidence

- `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0615-world-hud-overlay`
- `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
