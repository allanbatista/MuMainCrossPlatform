# Chat Shell

Status: READY_FOR_EXEC

## Summary

Implement a route-gated Bevy shell for `UiRoute::Chat`, wire the deterministic
`chat` control-http smoke command, and update the usage docs and validation
notes.

## Interfaces / Contracts

- `UiRoute::Chat` becomes a visible Bevy shell in the runtime, gated on the
  current route and session state.
- `ControlCommand::Chat` maps to the chat route and logged-in session in the
  local control snapshot.
- The shell body is derived from `mu_ui::chat_screen()` so the port keeps the
  legacy snapshot shape.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260520-1500-chat-shell/spec.md`, `.features/20260520-1500-chat-shell/plan.md`, `.features/20260520-1500-chat-shell/progress.md` | none | slice boundary is documented | feature docs present |
| F1.S1.T1 Runtime shell | local | `port_rust/crates/mu_app/src/chat_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | F0 | chat route shell renders and clears correctly | unit tests |
| F1.S1.T2 Control smoke | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/docs/control-http.md` | F1.S1.T1 | `chat` command is exposed | control-http tests/docs |
| F1.S2.T1 Usage docs | local | `docs/player-rust-client.md`, `port_rust/README.md` | F1.S1.T2 | player-facing docs mention chat smoke | docs diff |
| F2.S1.T1 Validation | local | progress docs, KB | F1 | fmt/test/build/smoke pass or blockers recorded | command logs |

## Validation Gates

- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`
- local control-http smoke covering `chat`

## Assumptions

- This slice keeps chat route rendering snapshot-driven.
- Interactive chat text entry remains a later slice.
