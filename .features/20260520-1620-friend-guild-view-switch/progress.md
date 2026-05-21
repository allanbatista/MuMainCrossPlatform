# Friend and Guild View Switch Progress

Status: OPEN

Current state: the friend and guild shells are already visible in the Rust
runtime. This slice adds control-http-driven view overrides so QA can smoke
all legacy subviews without changing gameplay networking.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1620-friend-guild-view-switch/spec.md`, `.features/20260520-1620-friend-guild-view-switch/plan.md`, `.features/20260520-1620-friend-guild-view-switch/progress.md` | `.features/20260520-1620-friend-guild-view-switch/spec.md`, `.features/20260520-1620-friend-guild-view-switch/plan.md`, `.features/20260520-1620-friend-guild-view-switch/progress.md` | feature-doc audit | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs` | `port_rust/crates/mu_app/src/control_http.rs` | control snapshot fields and command variants | friend/guild view state stored in control snapshot and parsed from HTTP commands | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | shell override wiring | friend and guild shells now prefer the selected control-http view when present | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | unit tests for parsing, serialization, and override behavior | parser, JSON, shell fallback, and control-http override tests now cover the new states | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `docs/friend-guild.md`, `.codexpotter/kb/*` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `docs/friend-guild.md`, `.codexpotter/kb/friend-guild-view-switch.md`, `.codexpotter/kb/README.md` | usage docs and KB updates | docs now mention the friend/guild subview smoke commands; KB index and note updated | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-1620-friend-guild-view-switch/progress.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`; `cargo test --manifest-path port_rust/Cargo.toml --workspace`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; headless control-http smoke with `friend-compose`, `guild-members`, and `exit`; headless client run via `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0` | none |
| F3.S1.T2 | done | local | `.memory/TODO.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | `.memory/TODO.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | workflow sync | pending-work memory now records the remaining live packet wiring; main progress updated for the finished slice | none |
