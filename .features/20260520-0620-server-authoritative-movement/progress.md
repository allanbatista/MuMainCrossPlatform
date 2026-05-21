# Server Authoritative Movement Progress

Status: OPEN

Current state: the world route now bridges movement through the live session
and applies authoritative movement replies back into the runtime pose.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0620-server-authoritative-movement/spec.md`, `.features/20260520-0620-server-authoritative-movement/plan.md`, `.features/20260520-0620-server-authoritative-movement/progress.md` | `.features/20260520-0620-server-authoritative-movement/spec.md`, `.features/20260520-0620-server-authoritative-movement/plan.md`, `.features/20260520-0620-server-authoritative-movement/progress.md` | feature-workflow audit output | feature-workflow audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_protocol/src/movement.rs` | `port_rust/crates/mu_protocol/src/movement.rs` | protocol helper tests | `cargo test -p mu_protocol` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/client_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_gameplay/src/entities.rs` | runtime / bridge tests | `cargo test -p mu_app`, `cargo test -p mu_gameplay` | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_motion.rs` | route / fallback tests | `cargo test -p mu_app`, `cargo test --workspace` | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/*`, `port_rust/crates/mu_protocol/src/*` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_gameplay/src/entities.rs`, `port_rust/crates/mu_gameplay/src/lib.rs`, `port_rust/crates/mu_protocol/src/movement.rs` | `cargo test` outputs | `cargo test --manifest-path port_rust/Cargo.toml --workspace` | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | docs updated for authoritative movement | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-0620-server-authoritative-movement/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | fmt/test/build/smoke logs | `cargo fmt --check`, `cargo test --workspace`, `mu_client --headless --control-http` | none |
| F3.S1.T2 | done | local | `.memory/TODO.md`, progress | `.memory/TODO.md` | memory diff | appended next follow-up entry | none |

## Done

- Concluí a slice de movimento autoritativo: o world route agora envia
  `walk_request` pela sessão viva, aplica updates autoritativos de posição no
  `ClientRuntime` e mantém a predicao local. Files changed:
  `port_rust/crates/mu_app/src/bootstrap_runtime.rs`,
  `port_rust/crates/mu_app/src/client_runtime.rs`,
  `port_rust/crates/mu_app/src/world_motion.rs`,
  `port_rust/crates/mu_gameplay/src/entities.rs`,
  `port_rust/crates/mu_gameplay/src/lib.rs`,
  `port_rust/crates/mu_protocol/src/movement.rs`,
  `docs/player-rust-client.md`, `port_rust/README.md`.

- Validei a slice com `cargo fmt --check`, `cargo test` nos crates afetados,
  `cargo test --workspace` e smoke `mu_client --headless --control-http`.

- Created the feature spec, plan, and progress documents for the
  server-authoritative movement slice. This sets the execution boundary before
  the runtime bridge is changed.
- Audited the new feature docs cleanly. The execution plan now has the right
  smoke and e2e gate shape for the movement authority slice.

## Files Touched

New:

- `.features/20260520-0620-server-authoritative-movement/spec.md`
- `.features/20260520-0620-server-authoritative-movement/plan.md`
- `.features/20260520-0620-server-authoritative-movement/progress.md`

## Validation Evidence

- `cargo fmt --manifest-path port_rust/Cargo.toml --all --check`
- `cargo test --manifest-path port_rust/Cargo.toml -p mu_protocol`
- `cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay`
- `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
