# Character Select Command Progress

Status: OPEN

Current state: the manual `select-character` path is implemented, documented,
and validated. The bootstrap no longer auto-selects the roster entry, the
control HTTP bridge forwards the explicit selection name, and the client now
waits on character select until that command arrives.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1146-character-select-command/spec.md`, `.features/20260520-1146-character-select-command/plan.md`, `.features/20260520-1146-character-select-command/progress.md` | `.features/20260520-1146-character-select-command/spec.md`, `.features/20260520-1146-character-select-command/plan.md`, `.features/20260520-1146-character-select-command/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | manual selection queue method and no auto-select shortcut | `queue_character_select_request`; worker no longer auto-selects on character list | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | fake-server bootstrap smoke waits for manual selection | `fake_server_packets_drive_the_bootstrap_worker`, `fake_server_applies_authoritative_movement_updates`, and `fake_server_sends_public_chat_messages` queue the manual selection request before world entry | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `select-character` payload parsing and runtime bridge | `ControlCommand::SelectCharacter`; snapshot payload storage; runtime bridge hook | none |
| F3.S1.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/login-world-bootstrap.md`, `.codexpotter/kb/control-http-session-flow.md`, `.codexpotter/kb/port-rust-usage-docs.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/login-world-bootstrap.md`, `.codexpotter/kb/control-http-session-flow.md`, `.codexpotter/kb/port-rust-usage-docs.md` | docs/KB diff | docs updated for the explicit selection command and wait-on-select behavior | none |
| F4.S1.T1 | done | local | progress docs, tests, validation logs | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`; `cargo test --manifest-path port_rust/Cargo.toml --workspace`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; control-http smoke with `login-success`, `select-character`, and `exit` | none |

## Files Touched

New:

- `.features/20260520-1146-character-select-command/spec.md`
- `.features/20260520-1146-character-select-command/plan.md`
- `.features/20260520-1146-character-select-command/progress.md`

Modified:

- `port_rust/crates/mu_app/src/bootstrap_runtime.rs`
- `port_rust/crates/mu_app/src/control_http.rs`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `port_rust/docs/control-http.md`
- `.codexpotter/kb/login-world-bootstrap.md`
- `.codexpotter/kb/control-http-session-flow.md`
- `.codexpotter/kb/port-rust-usage-docs.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`

Removed:

- none

## Done

- Replaced the character-select auto-advance shortcut with a manual
  `select-character` queue path in `bootstrap_runtime.rs`, so the worker waits
  on character select until automation sends the command. Files changed:
  `port_rust/crates/mu_app/src/bootstrap_runtime.rs`.
- Bridged the new `select-character` control command through the HTTP control
  snapshot and runtime sync path, including the explicit name payload and
  missing-name failure. Files changed:
  `port_rust/crates/mu_app/src/control_http.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`.
- Updated the player guide, port README, control-http docs, and KB notes to
  document the manual selection command and waiting behavior. Files changed:
  `docs/player-rust-client.md`, `port_rust/README.md`,
  `port_rust/docs/control-http.md`,
  `.codexpotter/kb/login-world-bootstrap.md`,
  `.codexpotter/kb/control-http-session-flow.md`,
  `.codexpotter/kb/port-rust-usage-docs.md`.
- Validated with `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`,
  `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`,
  `cargo test --manifest-path port_rust/Cargo.toml --workspace`,
  `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, and a
  control-http smoke that exercised `login-success`, `select-character`, and
  `exit`.
