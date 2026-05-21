# Login Character List Request Progress

Status: OPEN

Current state: the request-chain slice is scoped and the feature docs are in
place. The bootstrap worker now requests the character list immediately after
login success, the fake-server checks are aligned, and the docs plus
validation are recorded.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1116-login-character-list-request/spec.md`, `.features/20260520-1116-login-character-list-request/plan.md`, `.features/20260520-1116-login-character-list-request/progress.md` | `.features/20260520-1116-login-character-list-request/spec.md`, `.features/20260520-1116-login-character-list-request/plan.md`, `.features/20260520-1116-login-character-list-request/progress.md` | feature-workflow audit output | feature docs drafted and audited | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | unit test coverage for the request chain | login-success now triggers `request_character_list()` with the legacy locale byte, and the session-event snapshot is taken before receive so the request is not skipped | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | fake-server packet expectation | fake-server scripts now expect `request_character_list(0)` before character list/world handoff; bootstrap worker tests pass | none |
| F2.S1.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/login-world-bootstrap.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/login-world-bootstrap.md` | docs diff | docs now describe the automatic character-list request and the legacy locale byte mapping | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-1116-login-character-list-request/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime`; `cargo test --manifest-path port_rust/Cargo.toml --workspace`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0` with `control-http listening on http://127.0.0.1:33291` | none |

## Done

- Created the feature spec, plan, and progress documents for the login
  character-list request slice. This set the execution boundary before the
  bootstrap worker changes.
- Implemented the post-login character-list request in `bootstrap_runtime.rs`.
  The worker now snapshots the pre-receive session event, sends
  `request_character_list()` immediately after `LoginSuccess`, and maps the
  configured locale to the legacy request byte (`en`/`eng` -> `0`,
  `pt`/`por` -> `1`, `es`/`spn` -> `2`). Updated the fake-server tests to
  expect the request, added unit coverage for the legacy mapping, and
  refreshed the player guide, port README, and KB entry. Files changed:
  `port_rust/crates/mu_app/src/bootstrap_runtime.rs`,
  `docs/player-rust-client.md`, `port_rust/README.md`,
  `.codexpotter/kb/login-world-bootstrap.md`.
  Validation: `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`,
  `cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime`,
  `cargo test --manifest-path port_rust/Cargo.toml --workspace`,
  `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`,
  `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
  with `control-http listening on http://127.0.0.1:33291`.
