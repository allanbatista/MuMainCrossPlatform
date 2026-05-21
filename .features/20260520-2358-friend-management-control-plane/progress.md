# Friend Management Control Plane Progress

Status: DONE_FOR_FRIEND_MANAGEMENT_CONTROL_PLANE

Current state: the friend add/delete control-http commands, runtime bridge,
tests, docs, KB, and workflow bookkeeping are complete. The slice is closed.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-2358-friend-management-control-plane/spec.md`, `.features/20260520-2358-friend-management-control-plane/plan.md`, `.features/20260520-2358-friend-management-control-plane/progress.md` | `.features/20260520-2358-friend-management-control-plane/spec.md`, `.features/20260520-2358-friend-management-control-plane/plan.md`, `.features/20260520-2358-friend-management-control-plane/progress.md` | feature-doc audit | docs drafted and audited | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | friend add/delete queue methods and packet send handling | `bootstrap_runtime::tests::friend_management_requests_queue_commands`, `bootstrap_runtime::tests::fake_server_packets_drive_the_friend_management_request_worker` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | command parsing and runtime bridge | `control_http::tests::friend_actions_require_a_name`, `control_http::tests::server_exposes_state_and_applies_commands`, `graphical_runtime::tests::control_http_snapshot_queues_friend_actions` | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/*`, `port_rust/crates/mu_protocol/src/*` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | request validation and queueing tests | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace` | none |
| F2.S1.T2 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/*` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/friend-management-control-plane.md`, `.codexpotter/kb/README.md` | docs and KB updates | docs updated for `friend-add`/`friend-delete`, KB note and index updated | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-2358-friend-management-control-plane/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.memory/TODO.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --control-http 127.0.0.1:0`, `curl -sS http://127.0.0.1:36893/state`, `curl -sS -X POST http://127.0.0.1:36893/command?name=friend-add\&friend=Astra`, `curl -sS -X POST http://127.0.0.1:36893/command?name=friend-delete\&friend=Astra`, `curl -sS -X POST http://127.0.0.1:36893/command?name=exit` | none |

## Files Touched

New:

- `.features/20260520-2358-friend-management-control-plane/spec.md`
- `.features/20260520-2358-friend-management-control-plane/plan.md`
- `.features/20260520-2358-friend-management-control-plane/progress.md`
- `.codexpotter/kb/friend-management-control-plane.md`

Modified:

- `port_rust/crates/mu_app/src/bootstrap_runtime.rs`
- `port_rust/crates/mu_app/src/control_http.rs`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `port_rust/docs/control-http.md`
- `.codexpotter/kb/README.md`
- `.memory/TODO.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`

Removed:

- none

## Validation Evidence

- `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --control-http 127.0.0.1:0`
- `rtk curl -sS http://127.0.0.1:36893/state`
- `rtk curl -sS -X POST http://127.0.0.1:36893/command?name=friend-add\&friend=Astra`
- `rtk curl -sS -X POST http://127.0.0.1:36893/command?name=friend-delete\&friend=Astra`
- `rtk curl -sS -X POST http://127.0.0.1:36893/command?name=exit`
- `rtk rg -n "friend-add|friend-delete|friend management" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md`

## Done

- Created and audited the feature workflow docs for the friend-management
  control-plane slice. Files changed:
  `.features/20260520-2358-friend-management-control-plane/spec.md`,
  `.features/20260520-2358-friend-management-control-plane/plan.md`,
  `.features/20260520-2358-friend-management-control-plane/progress.md`.

- Added friend-add/friend-delete control-http commands and the live-session
  bridge so the bootstrap worker queues the matching friend packets after
  validating an explicit friend name. Files changed:
  `port_rust/crates/mu_app/src/bootstrap_runtime.rs`,
  `port_rust/crates/mu_app/src/control_http.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`.

- Updated the player-facing and port docs for the friend-management commands
  and recorded the slice in KB. Files changed:
  `docs/player-rust-client.md`, `port_rust/README.md`,
  `port_rust/docs/control-http.md`,
  `.codexpotter/kb/friend-management-control-plane.md`,
  `.codexpotter/kb/README.md`.

- Synced the pending-work memory and workflow progress bookkeeping, then
  validated the slice with fmt, tests, build, and live control-http smoke.
  Files changed: `.memory/TODO.md`,
  `.codexpotter/projects/2026/05/20/1/MAIN.md`.
