# Party Leave Control Plane Progress

Status: DONE

Current state: the `party-leave` control-plane slice is implemented and
validated.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260521-1705-party-leave-control-plane/spec.md`, `.features/20260521-1705-party-leave-control-plane/plan.md`, `.features/20260521-1705-party-leave-control-plane/progress.md` | `.features/20260521-1705-party-leave-control-plane/spec.md`, `.features/20260521-1705-party-leave-control-plane/plan.md`, `.features/20260521-1705-party-leave-control-plane/progress.md` | workflow docs for the slice | spec, plan, and progress created | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | control command parsing, snapshot payload, and runtime bridge | `party-leave` parses `member_number`, stores `party_member_number`, and the runtime queues the leave packet | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_protocol/src/social.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | bootstrap queue method and packet encoding | `BootstrapCommand::PartyLeave`, `queue_party_leave_request`, and `party_player_kick_request` send coverage | none |
| F2.S1.T1 | done | local | `docs/party-ui.md`, `docs/player-rust-client.md`, `port_rust/docs/control-http.md`, `port_rust/README.md`, `.codexpotter/kb/README.md`, `.codexpotter/kb/party-leave-control-plane.md` | `docs/party-ui.md`, `docs/player-rust-client.md`, `port_rust/docs/control-http.md`, `port_rust/README.md`, `.codexpotter/kb/README.md`, `.codexpotter/kb/party-leave-control-plane.md` | usage docs and KB note updated | docs and KB note are updated | none |
| F3.S1.T1 | done | local | validation logs | validation logs | fmt, tests, build, and smoke evidence | `cargo fmt --manifest-path port_rust/Cargo.toml --all`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app control_http`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app graphical_runtime`; `cargo test --manifest-path port_rust/Cargo.toml --workspace`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; headless smoke on `party-leave` and `exit` | none |

## Files Touched

New:

- `.features/20260521-1705-party-leave-control-plane/spec.md`
- `.features/20260521-1705-party-leave-control-plane/plan.md`
- `.features/20260521-1705-party-leave-control-plane/progress.md`

Modified:

- `port_rust/crates/mu_app/src/control_http.rs`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `port_rust/crates/mu_app/src/bootstrap_runtime.rs`
- `docs/party-ui.md`
- `docs/player-rust-client.md`
- `port_rust/docs/control-http.md`
- `port_rust/README.md`
- `.codexpotter/kb/party-leave-control-plane.md`
- `.codexpotter/kb/party-invite-control-plane.md`
- `.codexpotter/kb/README.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`

Removed:

- none

## Done

- Implemented and validated the party leave control-plane slice end to end.
