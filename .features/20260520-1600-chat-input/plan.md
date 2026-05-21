# Chat Input

Status: READY_FOR_EXEC

## Summary

Implement a chat composer for the Chat route, wire a live-session send path for
public chat packets, and update the visible shell and usage docs to match.

## Interfaces / Contracts

- `ChatShellPlugin` renders the current draft and send state from a chat
  composer resource.
- `BootstrapRuntime` accepts a chat send command alongside movement commands.
- The send command uses the local player label as the public chat sender and
  the draft text as the message payload.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260520-1600-chat-input/spec.md`, `.features/20260520-1600-chat-input/plan.md`, `.features/20260520-1600-chat-input/progress.md` | none | slice boundary is documented | feature docs present |
| F1.S1.T1 Chat composer | local | `port_rust/crates/mu_app/src/chat_composer.rs`, `port_rust/crates/mu_app/src/chat_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | F0 | typing updates the draft and the shell renders it | unit tests |
| F1.S1.T2 Chat send path | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/chat_composer.rs` | F1.S1.T1 | Enter queues a public chat packet through the live session | fake-server test |
| F1.S2.T1 Usage docs | local | `docs/player-rust-client.md`, `port_rust/README.md` | F1.S1.T2 | docs explain how to use the chat composer | docs diff |
| F2.S1.T1 Validation | local | progress docs, KB | F1 | fmt/test/build/smoke pass or blockers recorded | command logs |

## AC Traceability

| AC ID | Task IDs | Evidence |
|---|---|---|
| AC1 | F1.S1.T1 | draft editing tests and shell body diff |
| AC2 | F1.S1.T1 | unit tests for backspace/escape handling |
| AC3 | F1.S1.T2, F2.S1.T1 | fake-server smoke and validation logs |
| AC4 | F1.S2.T1 | docs diff |
| AC5 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | test output and smoke logs |

## Validation Gates

- `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- local graphical smoke opening `chat` and typing a message
- fake-server smoke covering the chat send packet
- e2e-validator handoff: if the graphical smoke shows a layout or focus issue,
  replay the Chat route with the validator before closing the slice.

## Risks

- The composer can starve movement only if it leaks outside the Chat route;
  keep the route gate strict.
- The send path depends on the live bootstrap session; keep the draft intact if
  the queue fails.
