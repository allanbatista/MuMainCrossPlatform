# Game Shop Shell Progress

Status: OPEN

Current state: the GameShop shell slice is implemented, documented, and
validated. The route-gated shell now renders the existing GameShop snapshot
and the local smoke path uses `game-shop`.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1024-game-shop-shell/spec.md`, `.features/20260520-1024-game-shop-shell/plan.md`, `.features/20260520-1024-game-shop-shell/progress.md` | `.features/20260520-1024-game-shop-shell/spec.md`, `.features/20260520-1024-game-shop-shell/plan.md`, `.features/20260520-1024-game-shop-shell/progress.md` | feature-workflow audit output | docs drafted and audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/game_shop_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/game_shop_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | shell visibility tests | `game_shop_shell_view_renders_expected_bodies`; `plugin_spawns_and_clears_the_visible_shell`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app` passed | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | control-plane smoke | `command_parser_recognizes_game_shop_aliases`; `server_exposes_state_and_applies_commands`; `game-shop` smoke returned `ui_route":"game-shop"` and `session_phase":"logged-in"` | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_app/src/*` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/game_shop_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `cargo test` outputs | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app` passed | none |
| F2.S1.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | docs diff | docs now mention the GameShop shell and the `game-shop` smoke command | none |
| F2.S2.T1 | done | local | `.codexpotter/kb/game-shop-runtime-ui.md`, `.codexpotter/kb/README.md`, `.memory/TODO.md` | `.codexpotter/kb/game-shop-runtime-ui.md`, `.codexpotter/kb/README.md` | KB and pending-work updates | KB note updated to include the new shell and smoke path; pending-work notes already cover remaining port parity | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-1024-game-shop-shell/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`; `cargo test --manifest-path port_rust/Cargo.toml --workspace`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`; `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`; `curl` smoke for `/state`, `game-shop`, and `exit`; `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client` | none |

## Files Touched

New:

- `port_rust/crates/mu_app/src/game_shop_shell.rs`
- `.features/20260520-1024-game-shop-shell/spec.md`
- `.features/20260520-1024-game-shop-shell/plan.md`
- `.features/20260520-1024-game-shop-shell/progress.md`

Modified:

- `docs/player-rust-client.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`
- `.codexpotter/kb/game-shop-runtime-ui.md`
- `port_rust/README.md`
- `port_rust/crates/mu_app/src/control_http.rs`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `port_rust/crates/mu_app/src/lib.rs`
- `port_rust/docs/control-http.md`

Removed:

- none

## Done

- Created, audited, and then implemented the GameShop shell slice. The shell
  now uses the existing `mu_ui::game_shop_screen()` snapshot, the runtime
  registers the `GameShopPlugin` and `GameShopShellPlugin`, and the control
  plane exposes a dedicated `game-shop` command.
