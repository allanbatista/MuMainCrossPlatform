# Chat Shell Progress

Status: DONE_FOR_CHAT_SHELL

Current state: the `Chat` route now has a visible Bevy shell and a `chat`
control-http smoke path.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1500-chat-shell/spec.md`, `.features/20260520-1500-chat-shell/plan.md`, `.features/20260520-1500-chat-shell/progress.md` | `.features/20260520-1500-chat-shell/spec.md`, `.features/20260520-1500-chat-shell/plan.md`, `.features/20260520-1500-chat-shell/progress.md` | feature boundary docs | feature docs drafted and tracked | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/chat_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/chat_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | shell spawn/cleanup tests | chat shell renders from `mu_ui::chat_screen()` and clears on route/session loss | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/docs/control-http.md` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/docs/control-http.md` | control-http smoke | `chat` command exposed and smoke-documented | none |
| F1.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | player-facing docs now mention the chat shell smoke path | none |
| F2.S1.T1 | done | local | KB + progress docs | `.codexpotter/kb/chat-shell-visual.md`, `.codexpotter/kb/README.md`, `.features/20260520-1500-chat-shell/progress.md` | fmt/test/build/smoke logs | validation recorded and the chat slice facts captured | none |

## Done

- Created the feature spec, plan, and progress documents for the chat shell
  slice, then implemented the route-gated Bevy shell in `mu_app`, wired the
  `chat` control-http command, updated the usage docs, and captured the slice
  facts in KB. Files changed:
  `.features/20260520-1500-chat-shell/spec.md`,
  `.features/20260520-1500-chat-shell/plan.md`,
  `.features/20260520-1500-chat-shell/progress.md`,
  `port_rust/crates/mu_app/src/chat_shell.rs`,
  `port_rust/crates/mu_app/src/control_http.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/lib.rs`,
  `docs/player-rust-client.md`, `port_rust/README.md`,
  `port_rust/docs/control-http.md`,
  `.codexpotter/kb/chat-shell-visual.md`, `.codexpotter/kb/README.md`.
- Validated with `cargo fmt --manifest-path port_rust/Cargo.toml --all
  --check`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`,
  `cargo test --manifest-path port_rust/Cargo.toml --workspace`,
  `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, and the
  runtime smoke `mu_client --control-http 127.0.0.1:34567` driven through
  `chat` and `exit`.
