# Fake-Server Login Bootstrap Progress

Status: OPEN

Current state: the fake-server login/world bootstrap slice is implemented,
tested, and documented. The Bevy runtime now drives login, server select,
character select, and world handoff from the bootstrap worker.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0400-fake-server-login-world-bootstrap/spec.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/plan.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/progress.md` | `.features/20260520-0400-fake-server-login-world-bootstrap/spec.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/plan.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, `port_rust/crates/mu_app/src/runtime.rs` | bootstrap resource/unit test | `bootstrap_runtime::tests::routes_progress_from_login_to_world`, `graphical_runtime::tests::graphical_app_starts_on_the_login_route` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/session_state.rs`, `port_rust/crates/mu_app/src/client_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/session_state.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | session/route transition tests | `session_state::tests::tracks_login_logout_and_disconnect_transitions`, `bootstrap_runtime::tests::login_failure_stays_on_the_login_surface` | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_app/src/runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/client_runtime.rs` | initial world bootstrap test | `bootstrap_runtime::tests::fake_server_packets_drive_the_bootstrap_worker` | none |
| F1.S2.T2 | done | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/state.rs` | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/control_http.rs` | headless/control-http smoke | `mu_client --headless --control-http 127.0.0.1:0`, `GET /state`, `POST /command?name=exit` | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/*`, `port_rust/crates/mu_network/src/*` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/session_state.rs`, `port_rust/Cargo.toml`, `port_rust/Cargo.lock` | cargo test output | `cargo test -p mu_app` (31 passed, 2 suites) | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | docs diff | user-facing bootstrap flow and control docs updated | none |
| F3.S1.T1 | done | local | progress/docs | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | fmt/test/build/smoke logs | `cargo fmt --all`, `cargo test -p mu_app`, `mu_client --headless --control-http 127.0.0.1:0` | none |
| F3.S1.T2 | done | local | `.memory/TODO.md`, progress | `.memory/TODO.md`, `.codexpotter/kb/login-world-bootstrap.md` | memory diff | pending-work memory entry and bootstrap KB note refreshed | none |

## Done

- Created and audited the feature spec, plan, and progress documents for the
  fake-server login/world bootstrap slice. This locks the next implementation
  step without leaving scope or validation ambiguous.
- Implemented the Bevy bootstrap runtime in `mu_app`, including login,
  server-select, character-select, and world handoff routing plus the
  disconnect-handling rule during map transfer. Files changed:
  `port_rust/crates/mu_app/src/bootstrap_runtime.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/runtime.rs`,
  `port_rust/crates/mu_app/src/lib.rs`,
  `port_rust/crates/mu_app/src/client_runtime.rs`,
  `port_rust/crates/mu_app/src/session_state.rs`,
  `port_rust/crates/mu_app/Cargo.toml`,
  `port_rust/Cargo.lock`.
- Validated the slice with `cargo test -p mu_app`, `cargo fmt --all`, and a
  live `mu_client --headless --control-http` smoke that returned
  `ready-for-login` and then exited cleanly.
- Documented the new bootstrap flow in `docs/player-rust-client.md`,
  `port_rust/README.md`, and `port_rust/docs/control-http.md`, and refreshed
  the KB note at `.codexpotter/kb/login-world-bootstrap.md`.
