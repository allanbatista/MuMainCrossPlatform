# Session-Backed UI Flow Progress

Status: DONE_FOR_SESSION_BACKED_UI_FLOW

Current state: the session-backed UI/control-plane slice is implemented and
validated. `--control-http` now works in graphical mode, mirrors
`UiRoute`/`SessionPhase`, and the local `exit` command shuts down the Bevy app
cleanly.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0422-session-backed-ui-flow/spec.md`, `.features/20260520-0422-session-backed-ui-flow/plan.md`, `.features/20260520-0422-session-backed-ui-flow/progress.md` | `.features/20260520-0422-session-backed-ui-flow/spec.md`, `.features/20260520-0422-session-backed-ui-flow/plan.md`, `.features/20260520-0422-session-backed-ui-flow/progress.md` | feature-workflow audit output | audit passed for the feature docs | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/session_state.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/session_state.rs` | parser + state tests | command parser, snapshot fields, and session sync tests passed | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/runtime.rs` | runtime/bridge tests | graphical runtime mirrors control-http, applies commands, and now exits cleanly on `exit` | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/state.rs` | `port_rust/crates/mu_app/src/runtime.rs` | headless smoke | headless `--control-http` smoke stayed deterministic | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/*` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `cargo test` outputs | `cargo test -p mu_app` and workspace tests passed | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | docs diff | usage docs now describe the graphical control plane and shutdown behavior | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-0422-session-backed-ui-flow/progress.md` | fmt/test/build/smoke logs | fmt, mu_app tests, mu_client build, and graphical/headless smoke passed | none |
| F3.S1.T2 | done | local | `.memory/TODO.md`, progress | `.memory/TODO.md`, `.codexpotter/kb/control-http-session-flow.md`, `.features/20260520-0422-session-backed-ui-flow/progress.md` | memory diff | next visible UI-shell follow-up remains tracked and the KB note now includes the graceful shutdown behavior | none |

## Done

- Implemented the session-backed UI/control-plane slice in the graphical
  client. `--control-http` now mirrors `UiRoute` and `SessionPhase`, supports
  the auth/bootstrap commands, and exits cleanly when `exit` is posted.
- Validated the slice with `cargo fmt`, `cargo test -p mu_app`, `cargo build -p
  mu_client`, and graphical/headless control-http smoke.
