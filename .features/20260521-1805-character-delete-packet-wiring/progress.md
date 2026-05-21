---
status: DONE_FOR_CHARACTER_DELETE_PACKET_WIRING
---

# Character Delete Packet Wiring Progress

Current state: the delete route shell, control-plane routing, packet send,
docs, and validation are complete. The Rust client now exposes
`character-delete`, queues `delete-character` with the selected roster name
and local security code, and mirrors submitting/error states through the
control plane.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260521-1805-character-delete-packet-wiring/spec.md`, `.features/20260521-1805-character-delete-packet-wiring/plan.md`, `.features/20260521-1805-character-delete-packet-wiring/progress.md` | `.features/20260521-1805-character-delete-packet-wiring/spec.md`, `.features/20260521-1805-character-delete-packet-wiring/plan.md`, `.features/20260521-1805-character-delete-packet-wiring/progress.md` | feature-workflow audit output | docs drafted and tracked | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_ui/src/character_delete.rs`, `port_rust/crates/mu_ui/src/lib.rs`, `port_rust/crates/mu_app/src/auth_shell.rs` | `port_rust/crates/mu_ui/src/character_delete.rs`, `port_rust/crates/mu_ui/src/lib.rs`, `port_rust/crates/mu_app/src/auth_shell.rs` | delete route shell and auth-shell rendering | visible delete shell and auth-shell route exist | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | control command, packet send, and response handling | `character-delete` and `delete-character` route into bootstrap with selected name + security code | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_ui/src/character_delete.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/control_http.rs` | `port_rust/crates/mu_ui/src/character_delete.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/control_http.rs` | unit coverage for route shell, packet send, and response handling | `cargo test --manifest-path port_rust/Cargo.toml -p mu_ui character_delete`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime` | none |
| F2.S1.T2 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/README.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/character-delete-packet-wiring.md`, `.codexpotter/kb/README.md` | docs and KB updates | user docs and KB describe the delete route and smoke flow | none |
| F3.S1.T1 | done | local | progress docs | progress docs | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, headless control-http smoke on `select-character`, `character-delete`, and `delete-character` | none |
| F3.S1.T2 | done | local | `.codexpotter/projects/2026/05/20/1/MAIN.md` | `.codexpotter/projects/2026/05/20/1/MAIN.md` | progress sync | main tracker updated to reflect the completed delete slice | none |

## Files Touched

New:

- `.features/20260521-1805-character-delete-packet-wiring/spec.md`
- `.features/20260521-1805-character-delete-packet-wiring/plan.md`
- `.features/20260521-1805-character-delete-packet-wiring/progress.md`

Modified:

- `.features/20260521-1805-character-delete-packet-wiring/progress.md`

Removed:

- none

## Done

- Created the feature workflow docs for the character-delete packet wiring
  slice.
- Shipped the visible character-delete shell, control-plane wiring, packet
  submit path, docs, KB note, and smoke coverage.
