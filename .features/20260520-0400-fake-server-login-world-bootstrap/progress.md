# Fake-Server Login Bootstrap Progress

Status: OPEN

Current state: the next slice is defined and documented; the feature docs were
created and audited, and implementation has not started yet. The next concrete
step is to wire the bootstrap state machine into `mu_app`, then add tests and
docs for the fake-server login-to-world path.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0400-fake-server-login-world-bootstrap/spec.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/plan.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/progress.md` | `.features/20260520-0400-fake-server-login-world-bootstrap/spec.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/plan.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | todo | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, maybe `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | pending | bootstrap resource/unit test | pending | none |
| F1.S1.T2 | todo | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/session_state.rs`, `port_rust/crates/mu_app/src/client_runtime.rs` | pending | session/route transition tests | pending | none |
| F1.S2.T1 | todo | local | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_app/src/runtime.rs` | pending | initial world bootstrap test | pending | none |
| F1.S2.T2 | todo | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/state.rs` | pending | headless/control-http smoke | pending | none |
| F2.S1.T1 | todo | local | `port_rust/crates/mu_app/src/*`, `port_rust/crates/mu_network/src/*` | pending | cargo test output | pending | none |
| F2.S2.T1 | todo | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | pending | docs diff | pending | none |
| F3.S1.T1 | todo | local | progress/docs | pending | fmt/test/build/smoke logs | pending | none |
| F3.S1.T2 | todo | local | `.memory/TODO.md`, progress | pending | memory diff | pending | none |

## Done

- Created and audited the feature spec, plan, and progress documents for the
  fake-server login/world bootstrap slice. This locks the next implementation
  step without leaving scope or validation ambiguous.
