# Friend/Guild Response Decoding Progress

Status: OPEN

Current state: live friend/guild roster decode, shell overlay, docs, and
validation are complete. The doc stays OPEN because the worktree still has
unrelated untracked feature-doc directories; the remaining social parity work
is tracked separately in `.memory/TODO.md`.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1755-friend-guild-response-decoding/spec.md`, `.features/20260520-1755-friend-guild-response-decoding/plan.md`, `.features/20260520-1755-friend-guild-response-decoding/progress.md` | `.features/20260520-1755-friend-guild-response-decoding/spec.md`, `.features/20260520-1755-friend-guild-response-decoding/plan.md`, `.features/20260520-1755-friend-guild-response-decoding/progress.md` | feature-workflow audit output | audit passed before the slice was marked complete; rerun clean after this sync | none |
| F0.S1.T2 | done | local | `.codexpotter/kb/friend-guild-response-decoding.md`, `.codexpotter/kb/README.md` | `.codexpotter/kb/friend-guild-response-decoding.md`, `.codexpotter/kb/README.md` | KB note with packet layouts and code locations | KB note records the legacy friend/guild layouts and Rust touchpoints | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | unit tests for friend/guild packet decode helpers | decode tests cover `0xC0` friend packets and `0xD1:0x52` guild packets | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | worker classifies friend and guild roster packets into signals | bootstrap runtime stores decoded friend/guild roster snapshots | none |
| F1.S1.T3 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | roster state resets on logout and disconnect | logout/disconnect clears the decoded social rosters | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/friend_shell.rs` | `port_rust/crates/mu_app/src/friend_shell.rs` | shell snapshot tests with live friend roster overrides | friend shell overlays the live roster and rerenders on snapshot changes | none |
| F2.S1.T2 | done | local | `port_rust/crates/mu_app/src/guild_shell.rs` | `port_rust/crates/mu_app/src/guild_shell.rs` | shell snapshot tests with live guild roster overrides | guild shell overlays the live roster and rerenders on snapshot changes | none |
| F2.S1.T3 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | shell rerenders when decoded roster state changes | live roster packets repaint the friend and guild shells without route changes | none |
| F3.S1.T1 | done | local | `docs/friend-guild.md`, `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | `docs/friend-guild.md`, `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | docs diff for live roster/member sync | usage docs now call out the live friend/guild roster overlays | none |
| F3.S1.T2 | done | local | progress docs | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.features/20260520-1755-friend-guild-response-decoding/progress.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`; `cargo test --manifest-path port_rust/Cargo.toml --workspace`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`; `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`; feature-doc audit passed | none |

## Files Touched

New:

- `.features/20260520-1755-friend-guild-response-decoding/spec.md`
- `.features/20260520-1755-friend-guild-response-decoding/plan.md`
- `.features/20260520-1755-friend-guild-response-decoding/progress.md`

Modified:

- none

Removed:

- none
