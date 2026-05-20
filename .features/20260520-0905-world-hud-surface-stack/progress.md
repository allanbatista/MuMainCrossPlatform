# World HUD Surface Stack Progress

Status: DONE_FOR_WORLD_HUD_SURFACE_STACK

Current state: the world HUD overlay currently renders the main frame only.
This slice will extend it to surface the chat, minimap, and hotkey snapshots in
the world route while keeping the route-gated cleanup behavior intact.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0905-world-hud-surface-stack/spec.md`, `.features/20260520-0905-world-hud-surface-stack/plan.md`, `.features/20260520-0905-world-hud-surface-stack/progress.md` | `.features/20260520-0905-world-hud-surface-stack/spec.md`, `.features/20260520-0905-world-hud-surface-stack/plan.md`, `.features/20260520-0905-world-hud-surface-stack/progress.md` | feature-workflow audit output | feature docs audited cleanly | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/world_hud.rs` | `port_rust/crates/mu_app/src/world_hud.rs` | runtime overlay tests | `world_hud_spawns_visible_overlay_when_world_route_is_ready` and the stacked overlay cards render from the world snapshots | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/world_hud.rs` | `port_rust/crates/mu_app/src/world_hud.rs` | route/cleanup tests | `world_hud_clears_when_route_leaves_world` and `world_hud_waits_for_world_projection` | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/world_hud.rs` | `port_rust/crates/mu_app/src/world_hud.rs` | body formatting and lifecycle tests | `world_hud_body_lists_the_world_surface_stack_state` and `format_helpers_keep_labels_readable` | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | world HUD stack wording added to the player guide and port README | none |
| F2.S3.T1 | done | local | progress docs | `.features/20260520-0905-world-hud-surface-stack/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`, `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`, `rg -n "HUD stack|chat|minimap|hotkey" docs/player-rust-client.md port_rust/README.md` | none |
| F2.S3.T2 | done | local | `.memory/TODO.md`, progress | `.codexpotter/kb/world-hud-surface-stack.md`, `.codexpotter/kb/README.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.features/20260520-0905-world-hud-surface-stack/progress.md` | memory diff | slice facts captured in KB and the central tracker was updated; no new pending debt was needed | none |

## Done

- Created the feature spec, plan, and progress documents for the world HUD
  surface stack slice. This locks the slice scope before implementation.
- Audited the docs cleanly with the feature-workflow checker, so the slice is
  ready for the runtime implementation phase.
- Implemented the stacked world HUD overlay in `mu_app`: the world route now
  renders HUD, chat, minimap, and hotkey snapshot cards together and clears
  them when the world route or projection is no longer active. Files changed:
  `port_rust/crates/mu_app/src/world_hud.rs`.
- Updated the player-facing docs to describe the world HUD stack. Files
  changed: `docs/player-rust-client.md`, `port_rust/README.md`.
- Captured the slice facts in KB and refreshed the tracker. Files changed:
  `.codexpotter/kb/world-hud-surface-stack.md`, `.codexpotter/kb/README.md`,
  `.codexpotter/projects/2026/05/20/1/MAIN.md`.
- Validated with `cargo fmt --manifest-path port_rust/Cargo.toml --all`,
  `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`,
  `cargo test --manifest-path port_rust/Cargo.toml --workspace`,
  `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`,
  graphical smoke `timeout 5s cargo run --manifest-path port_rust/Cargo.toml
  -p mu_client`, headless smoke `timeout 5s cargo run --manifest-path
  port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`,
  and the docs grep gate for `HUD stack|chat|minimap|hotkey`.
