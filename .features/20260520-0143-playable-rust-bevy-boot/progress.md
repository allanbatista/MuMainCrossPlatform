# Playable Rust Bevy Boot Progress

Status: DONE_FOR_BOOT_SLICE

Current state: the Rust client now enters a Bevy graphical runtime in normal mode, headless behavior is preserved, Rust cross-platform GitHub Actions were removed, and full gameplay parity is tracked as pending follow-up work.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0143-playable-rust-bevy-boot/*` | `.features/20260520-0143-playable-rust-bevy-boot/spec.md`, `.features/20260520-0143-playable-rust-bevy-boot/plan.md`, `.features/20260520-0143-playable-rust-bevy-boot/progress.md` | feature-workflow audit | audit passed before implementation; rerun pending after this progress sync | none |
| F0.S2.T1 | done | local | `.memory/RULES_AND_DEFINITION.md` | `.memory/RULES_AND_DEFINITION.md` | file diff | user gates and no-Rust-Actions rule appended | none |
| F1.S1.T1 | done | local | `.github/workflows/rust-client.yml`, `.github/workflows/rust-client-windows.yml` | `.github/workflows/rust-client.yml`, `.github/workflows/rust-client-windows.yml` | workflow grep | `rtk rg -n "Rust Client|rust-client" .github/workflows` returned no matches | none |
| F1.S2.T1 | done | local | `docs/build-guide.md`, `port_rust/README.md` | `docs/build-guide.md`, `port_rust/README.md` | docs diff | local Rust validation and graphical boot usage documented | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, `port_rust/crates/mu_app/Cargo.toml`, `port_rust/Cargo.toml` | `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, `port_rust/crates/mu_app/Cargo.toml`, `port_rust/Cargo.toml`, `port_rust/Cargo.lock` | unit test | `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app` passed | none |
| F2.S2.T1 | done | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/state.rs` | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/state.rs` | unit tests + smoke | headless smoke passed; invalid asset-root smoke exited `3`; graphical smoke stayed alive until timeout after creating the window | none |
| F3.S1.T1 | done | local | progress/docs | `.features/20260520-0143-playable-rust-bevy-boot/progress.md`, `port_rust/crates/mu_gameplay/src/lib.rs` | fmt/test/build/smoke logs | fmt, clippy, workspace tests, client build, headless smoke, invalid asset smoke, graphical smoke passed | none |
| F3.S2.T1 | done | local | `.memory/TODO.md`, progress | `.memory/TODO.md`, `.features/20260520-0143-playable-rust-bevy-boot/progress.md` | TODO entry | full playable parity follow-up recorded | none |

## Files Touched

New:

- `.features/20260520-0143-playable-rust-bevy-boot/spec.md`
- `.features/20260520-0143-playable-rust-bevy-boot/plan.md`
- `.features/20260520-0143-playable-rust-bevy-boot/progress.md`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`

Modified:

- `.memory/RULES_AND_DEFINITION.md`
- `.memory/TODO.md`
- `docs/build-guide.md`
- `port_rust/Cargo.lock`
- `port_rust/Cargo.toml`
- `port_rust/README.md`
- `port_rust/crates/mu_app/Cargo.toml`
- `port_rust/crates/mu_app/src/lib.rs`
- `port_rust/crates/mu_app/src/runtime.rs`
- `port_rust/crates/mu_app/src/state.rs`
- `port_rust/crates/mu_gameplay/src/lib.rs`

Removed:

- `.github/workflows/rust-client.yml`
- `.github/workflows/rust-client-windows.yml`

## Validation Evidence

- `rtk rg -n "Rust Client|rust-client" .github/workflows` returned no matches.
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app` passed: 27 tests.
- `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check` passed.
- `rtk cargo clippy --manifest-path port_rust/Cargo.toml --workspace --all-targets -- -D warnings` passed.
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace` passed: 410 tests.
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client` passed.
- `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless` printed `ready-for-login`.
- `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --asset-root __missing_mu_asset_root__` printed `asset-check-failed` and exited `3`.
- `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client` reached Bevy window creation and was killed by timeout, proving the process no longer exits immediately.

## Remaining Scope

Full playable and C++ parity are not complete in this slice. Login UI, fake-server login/world flow, terrain/player/entity rendering parity, movement/network sync, chat, inventory, GameShop, MU Helper, editor/admin tools, OpenMU compatibility, and full parity evidence remain pending.
