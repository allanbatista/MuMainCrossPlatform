# World Remote Player Sync Progress

Status: DONE_FOR_WORLD_REMOTE_PLAYER_SYNC

Current state: remote movement packets now create/update keyed remote players, and logout/disconnect clears the remote roster while preserving the world bundle. Local validation passed; the e2e-validator handoff remains a follow-up because that tool is not available in this session.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260521-2330-world-remote-player-sync/spec.md`, `.features/20260521-2330-world-remote-player-sync/plan.md`, `.features/20260521-2330-world-remote-player-sync/progress.md` | `.features/20260521-2330-world-remote-player-sync/spec.md`, `.features/20260521-2330-world-remote-player-sync/plan.md`, `.features/20260521-2330-world-remote-player-sync/progress.md` | feature-workflow audit output | docs drafted and audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_gameplay/src/entities.rs` | `port_rust/crates/mu_gameplay/src/entities.rs` | keyed remote-player helper tests | `world_entities_manager_upserts_remote_players_by_key` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/client_runtime.rs` | `port_rust/crates/mu_app/src/client_runtime.rs` | runtime remote update/cleanup tests | `runtime_tracks_remote_players_and_clears_them` | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | bootstrap packet-path test | `fake_server_applies_remote_and_local_movement_updates` | none |
| F2.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | session cleanup coverage | `social_rosters_clear_on_logout_and_disconnect` | none |
| F3.S1.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/world-entity-spawns.md`, `.codexpotter/kb/server-authoritative-movement.md`, `.codexpotter/kb/README.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/world-entity-spawns.md`, `.codexpotter/kb/server-authoritative-movement.md`, `.codexpotter/kb/README.md` | docs/KB updated | docs grep gate passed | none |
| F3.S1.T2 | done | local | none | none | audit/test/build/smoke output | `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260521-2330-world-remote-player-sync`, `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all`, `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay entities`, `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app client_runtime`, `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime`, `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`, `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`, `rtk rg -n "remote player|movement packets|logout|disconnect" docs/player-rust-client.md port_rust/README.md .codexpotter/kb` | e2e-validator follow-up deferred |

## Files Touched

New:

- `.features/20260521-2330-world-remote-player-sync/spec.md`
- `.features/20260521-2330-world-remote-player-sync/plan.md`
- `.features/20260521-2330-world-remote-player-sync/progress.md`

Modified:

- `.codexpotter/kb/README.md`
- `.codexpotter/kb/server-authoritative-movement.md`
- `.codexpotter/kb/world-entity-spawns.md`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `port_rust/crates/mu_app/src/bootstrap_runtime.rs`
- `port_rust/crates/mu_app/src/client_runtime.rs`
- `port_rust/crates/mu_gameplay/src/entities.rs`

Removed:

- none

## Done

- Wrote and audited the feature docs for the live remote-player sync slice.
- Implemented keyed remote-player upsert helpers, runtime projection updates,
  bootstrap routing for non-local movement packets, and remote-roster cleanup
  on logout/disconnect. Decision: use a stable placeholder label/model for
  newly seen remote keys until richer roster metadata exists. Files changed:
  `port_rust/crates/mu_gameplay/src/entities.rs`,
  `port_rust/crates/mu_app/src/client_runtime.rs`,
  `port_rust/crates/mu_app/src/bootstrap_runtime.rs`.
- Updated the player docs and KB notes to match the live remote-player
  behavior, then validated with `cargo fmt`, targeted `cargo test` passes,
  `cargo test --workspace`, `cargo build -p mu_client`, the headless
  control-http smoke, and the docs grep gate. The `e2e-validator` handoff
  remains a follow-up because that tool is not available in this session.

## Validation Evidence

- `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260521-2330-world-remote-player-sync`
- `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay entities`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app client_runtime`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime`
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
- `rtk rg -n "remote player|movement packets|logout|disconnect" docs/player-rust-client.md port_rust/README.md .codexpotter/kb`
