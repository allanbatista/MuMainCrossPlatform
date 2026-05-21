# Chat Input Progress

Status: DONE_FOR_CHAT_INPUT

Current state: the chat composer/send slice is implemented, documented, and
validated. Chat now accepts typed text on the visible shell, shows draft and
feedback state, and sends public chat packets through the live session on
Enter.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1600-chat-input/spec.md`, `.features/20260520-1600-chat-input/plan.md`, `.features/20260520-1600-chat-input/progress.md` | `.features/20260520-1600-chat-input/spec.md`, `.features/20260520-1600-chat-input/plan.md`, `.features/20260520-1600-chat-input/progress.md` | feature boundary docs | feature docs drafted and tracked | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/chat_composer.rs`, `port_rust/crates/mu_app/src/chat_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/chat_composer.rs`, `port_rust/crates/mu_app/src/chat_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | draft editing tests | `composer_appends_text_and_limits_length`, `composer_backspace_escape_and_enter_match_keyboard_events`, `chat_shell_view_renders_expected_bodies`, `plugin_spawns_and_clears_the_visible_shell` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/chat_composer.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/chat_composer.rs` | fake-server packet send test | `fake_server_sends_public_chat_messages`, `BootstrapRuntime::queue_chat_message_request` | none |
| F1.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | docs diff | chat draft/send flow added to the player guide, README, and control-http docs | none |
| F2.S1.T1 | done | local | progress docs, KB | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.codexpotter/kb/chat-input-compose-send.md`, `.codexpotter/kb/README.md`, `.memory/TODO.md`, `.features/20260520-1600-chat-input/progress.md` | fmt/test/build/smoke logs | `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all`, `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`, `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, `rtk timeout 10s cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --control-http 127.0.0.1:0` | none |

## Done

- Implemented the chat composer and send path in `mu_app`, keeping the Chat
  route snapshot-driven while the composer owns typed input, draft limits,
  status feedback, and Enter-to-send behavior. Files changed:
  `port_rust/crates/mu_app/src/chat_composer.rs`,
  `port_rust/crates/mu_app/src/chat_shell.rs`,
  `port_rust/crates/mu_app/src/bootstrap_runtime.rs`,
  `port_rust/crates/mu_app/src/client_runtime.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/lib.rs`.
- Updated the usage docs and KB tracking for the new chat flow. Files changed:
  `docs/player-rust-client.md`, `port_rust/README.md`,
  `port_rust/docs/control-http.md`,
  `.codexpotter/kb/chat-input-compose-send.md`, `.codexpotter/kb/README.md`.
- Validated the slice with `cargo fmt`, `cargo test -p mu_app`,
  `cargo test --workspace`, `cargo build -p mu_client`, and the fake-server
  public-chat smoke test.
