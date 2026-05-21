---
status: OPEN
---

# Siege Shell Progress

Current state: the feature docs are drafted. Next step is to audit the docs,
wire the siege shell into the graphical runtime and control HTTP path, then
validate the route and smoke commands.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-2134-siege-shell/spec.md`, `.features/20260520-2134-siege-shell/plan.md`, `.features/20260520-2134-siege-shell/progress.md` | `.features/20260520-2134-siege-shell/spec.md`, `.features/20260520-2134-siege-shell/plan.md`, `.features/20260520-2134-siege-shell/progress.md` | feature-workflow audit output | audit passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/siege_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/siege_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | visible siege shell and plugin wiring | shell module compiled and plugin registered | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/control_http.rs` | `port_rust/crates/mu_app/src/control_http.rs` | siege command parsing and runtime route routing | siege commands now route to the local siege snapshot | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/siege_shell.rs`, `port_rust/crates/mu_app/src/control_http.rs` | `port_rust/crates/mu_app/src/siege_shell.rs`, `port_rust/crates/mu_app/src/control_http.rs` | unit tests for shell visibility, mode mapping, and command parsing | unit tests passed for shell visibility, mode mapping, and command parsing | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/README.md`, `.codexpotter/kb/siege-shell.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/README.md`, `.codexpotter/kb/siege-shell.md` | usage docs and KB note updates | siege shell usage and smoke commands documented | none |
| F3.S1.T1 | done | local | validation logs | n/a | fmt, tests, build, and smoke logs | `cargo fmt --all`; `cargo test -p mu_app control_http`; `cargo test -p mu_app siege_shell`; `cargo build -p mu_client`; headless `siege`/`exit` smoke | none |
| F3.S1.T2 | done | local | `.codexpotter/kb/README.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | `.codexpotter/kb/README.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | KB and workflow progress sync | KB index and main progress synced | none |

## Files Touched

New:

- `port_rust/crates/mu_app/src/siege_shell.rs`
- `.codexpotter/kb/siege-shell.md`

Modified:

- `port_rust/crates/mu_app/src/control_http.rs`
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
- `port_rust/crates/mu_app/src/lib.rs`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `port_rust/docs/control-http.md`
- `.codexpotter/kb/README.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`
- `.features/20260520-2134-siege-shell/progress.md`

Removed:

- none

## Done

- Created the feature workflow docs for the siege shell slice.
- Implemented the siege shell, control-http mode routing, and runtime plugin
  wiring. Chose to drive the visible shell from `ControlHttpState` plus the
  shared guild cache so the local smoke path stays aligned with the runtime
  snapshot. Files changed:
  `port_rust/crates/mu_app/src/siege_shell.rs`,
  `port_rust/crates/mu_app/src/control_http.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/lib.rs`.
- Updated the usage docs, KB note, and KB index for the siege smoke path.
  Files changed: `docs/player-rust-client.md`, `port_rust/README.md`,
  `port_rust/docs/control-http.md`, `.codexpotter/kb/siege-shell.md`,
  `.codexpotter/kb/README.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md`.
