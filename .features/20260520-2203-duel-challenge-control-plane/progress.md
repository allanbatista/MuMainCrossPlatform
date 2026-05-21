# Duel Challenge Control Plane Progress

Status: OPEN

Current state: the duel control-plane slice is implemented, documented, and
validated locally. Next duel work is broader parity if full legacy behavior is
still required.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-2203-duel-challenge-control-plane/spec.md`, `.features/20260520-2203-duel-challenge-control-plane/plan.md`, `.features/20260520-2203-duel-challenge-control-plane/progress.md` | `.features/20260520-2203-duel-challenge-control-plane/spec.md`, `.features/20260520-2203-duel-challenge-control-plane/plan.md`, `.features/20260520-2203-duel-challenge-control-plane/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | duel-start/duel-stop command parsing and snapshot routing | `control_http::tests::control_http_route_accepts_duel_start_payload`; `control_http::tests::snapshot_tracks_duel_start_payload`; `graphical_runtime::tests::control_http_snapshot_queues_duel_actions` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | bootstrap queue/send bridge | `bootstrap_runtime::tests::duel_requests_queue_commands`; `bootstrap_runtime::tests::duel_packets_send_the_expected_bytes`; `bootstrap_runtime::tests::duel_stop_packets_send_the_expected_bytes` | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | unit tests for parser, snapshot, and queue/send | parser, snapshot, runtime-bridge, and packet-byte tests passed; headless HTTP smoke covered `duel-start`, `duel-stop`, and `exit` | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/duel-challenge-control-plane.md`, `.codexpotter/kb/README.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/duel-challenge-control-plane.md`, `.codexpotter/kb/README.md` | usage docs and KB updates | docs now describe `duel-start`/`duel-stop`; KB note and index updated | none |
| F3.S1.T1 | done | local | progress docs | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.features/20260520-2203-duel-challenge-control-plane/progress.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`; `cargo test --manifest-path port_rust/Cargo.toml --workspace`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; headless control-http smoke on `duel-start`, `duel-stop`, and `exit` | none |

## Files Touched

New:

- `.features/20260520-2203-duel-challenge-control-plane/spec.md`
- `.features/20260520-2203-duel-challenge-control-plane/plan.md`
- `.features/20260520-2203-duel-challenge-control-plane/progress.md`

Modified:

- `port_rust/crates/mu_app/src/control_http.rs`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `port_rust/crates/mu_app/src/bootstrap_runtime.rs`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `port_rust/docs/control-http.md`
- `docs/quests-events-duel-gens.md`
- `.codexpotter/kb/duel-challenge-control-plane.md`
- `.codexpotter/kb/README.md`
- `.memory/TODO.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`

Removed:

- none

## Done

- Created the duel challenge feature workflow docs and implemented the
  control-plane bridge, runtime sync, packet queue, tests, docs, KB note, and
  validation.
