# MU Helper Shell Progress

Status: OPEN

Current state: the MU Helper runtime/UI slice is implemented, documented, and
validated. The helper runtime and snapshot model already exist in
`mu_gameplay` and `mu_ui`; the Bevy client now exposes them in the runtime and
control plane.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1041-mu-helper-shell/spec.md`, `.features/20260520-1041-mu-helper-shell/plan.md`, `.features/20260520-1041-mu-helper-shell/progress.md` | `.features/20260520-1041-mu-helper-shell/spec.md`, `.features/20260520-1041-mu-helper-shell/plan.md`, `.features/20260520-1041-mu-helper-shell/progress.md` | feature-workflow audit output | docs drafted and audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/mu_helper_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/mu_helper_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | shell visibility tests | `mu_helper_shell::tests::mu_helper_shell_view_renders_expected_bodies`, `mu_helper_shell::tests::plugin_spawns_and_clears_the_visible_shell` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | control-plane smoke | `command_parser_recognizes_game_shop_and_mu_helper_aliases`, `server_exposes_state_and_applies_commands`, graphical smoke on `mu-helper` | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/*` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, `port_rust/crates/mu_app/src/mu_helper_shell.rs` | `cargo test` outputs | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace` | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | docs diff | MU Helper smoke path added to player guide, README, and control-http docs | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-1041-mu-helper-shell/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, graphical control-http smoke on `mu-helper`, headless control-http smoke on `mu-helper` | none |
| F3.S1.T2 | done | local | `.memory/TODO.md`, progress | `.codexpotter/kb/mu-helper-runtime-ui.md`, `.codexpotter/kb/README.md` | memory diff | KB note refreshed with the mu_app shell and control-plane locations | none |

## Done

- Created the feature spec, plan, and progress documents for the MU Helper
  runtime/UI exposure slice. This locks the next runtime step before code
  changes begin.
- Implemented the MU Helper runtime shell in `mu_app`, registered the helper
  runtime in the graphical runtime, and exposed the `mu-helper` control-plane
  command. Files changed:
  `port_rust/crates/mu_app/src/mu_helper_shell.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/control_http.rs`,
  `port_rust/crates/mu_app/src/lib.rs`.
- Updated the player guide, repo README, control-http docs, and KB note to
  describe the new smoke path. Files changed:
  `docs/player-rust-client.md`, `port_rust/README.md`,
  `port_rust/docs/control-http.md`, `.codexpotter/kb/mu-helper-runtime-ui.md`.
- Validated the slice with `cargo fmt --manifest-path port_rust/Cargo.toml --all
  --check`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`,
  `cargo test --manifest-path port_rust/Cargo.toml --workspace`,
  `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, and local
  control-http smoke on `mu-helper` plus `exit`.
