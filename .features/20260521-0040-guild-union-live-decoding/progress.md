# Guild Union Live Decoding Progress

Status: DONE

Current state: the guild union/alliance tab now overlays live roster data
decoded from `0xE9` and the slice is validated.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260521-0040-guild-union-live-decoding/spec.md`, `.features/20260521-0040-guild-union-live-decoding/plan.md`, `.features/20260521-0040-guild-union-live-decoding/progress.md` | `.features/20260521-0040-guild-union-live-decoding/spec.md`, `.features/20260521-0040-guild-union-live-decoding/plan.md`, `.features/20260521-0040-guild-union-live-decoding/progress.md` | feature-doc audit output | slice docs written and synchronized with the implementation | none |
| F0.S1.T2 | done | local | `.codexpotter/kb/friend-guild-response-decoding.md`, `.codexpotter/kb/README.md` | `.codexpotter/kb/friend-guild-response-decoding.md` | KB note with union packet layout and code locations | union packet layout and Rust touchpoints added to the existing KB note | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | unit tests for alliance-list packet decode helpers | decode helper tests cover live union roster packets | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | worker classifies alliance-list packets into signals | `0xE9` now classifies into `BootstrapSignal::GuildUnionRoster` | none |
| F1.S1.T3 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | union state resets on logout and disconnect | union roster snapshot now clears with the other social rosters | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/guild_shell.rs` | `port_rust/crates/mu_app/src/guild_shell.rs` | shell snapshot tests with live union roster overrides | live union roster overlays the guild shell and keeps selection stable | none |
| F2.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | shell rerenders when decoded union state changes | bootstrap snapshot feeds the guild shell key, so decoded union updates re-render the view | none |
| F3.S1.T1 | done | local | `docs/friend-guild.md`, `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | `docs/friend-guild.md`, `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | docs diff for live union sync | docs updated for live alliance-list overlay behavior | none |
| F3.S1.T2 | done | local | progress docs | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.features/20260521-0040-guild-union-live-decoding/progress.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, headless control-http smoke, and graphical boot smoke | none |

## Files Touched

New:

- `.features/20260521-0040-guild-union-live-decoding/spec.md`
- `.features/20260521-0040-guild-union-live-decoding/plan.md`
- `.features/20260521-0040-guild-union-live-decoding/progress.md`

Modified:

- `.codexpotter/kb/friend-guild-response-decoding.md`
- `port_rust/crates/mu_app/src/bootstrap_runtime.rs`
- `port_rust/crates/mu_app/src/guild_shell.rs`
- `docs/friend-guild.md`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `port_rust/docs/control-http.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`
- `.features/20260521-0040-guild-union-live-decoding/progress.md`

Removed:

- none
