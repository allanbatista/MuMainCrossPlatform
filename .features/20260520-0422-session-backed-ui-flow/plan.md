# Session-Backed UI Flow

Status: READY_FOR_EXEC

## Summary

Extend the existing control HTTP surface so the graphical Bevy client can be
driven and inspected over loopback while it runs. The slice keeps the headless
smoke path stable and adds route/session awareness to the control plane.

## Interfaces / Contracts

- `mu_app::control_http` owns the command parser, HTTP snapshot, and local-only
  command set.
- `mu_app::graphical_runtime` registers the control plane when `--control-http`
  is present and keeps the Bevy runtime in sync with it.
- `mu_app::runtime::run` preserves the existing headless branch and forwards the
  graphical branch into the new control-plane path.
- `mu_app::session_state::SessionState` remains the canonical session tracker.
- `mu_ui::UiRoute` and `mu_ui::UiShellState` remain the route contract for the
  bootstrap surfaces.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `control-snapshot` | `mu_app::control_http`, `mu_app::session_state`, `mu_ui::UiShellState` | HTTP state snapshot | current route/session/app state | loopback-only control requests | local-only | no split | `mu_app` tests | Exposes the control plane to QA |
| `graphical-control-bridge` | `mu_app::graphical_runtime`, `mu_app::runtime` | Bevy runtime wiring | command queue from HTTP | graphical runtime only | local-only | no split | `mu_app` tests + smoke | Applies scripted route/session changes while the client runs |
| `control-command-set` | `mu_app::control_http::ControlCommand` | local HTTP commands | login/server/character/world steps | unknown commands rejected | local-only | no split | parser tests | Covers the flow used for automation |
| `control-docs` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | usage docs | control-plane flow | local-only note | docs only | no split | grep gate | Explains the automation surface |

## Parallelization

- Command parsing and snapshot shape can be implemented before the graphical
  bridge is wired.
- Runtime wiring can land after the command semantics are stable.
- Docs can be updated after the command names and exposed state fields settle.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write and audit the feature spec, plan, and progress documents for
  the session-backed UI flow slice.

### F1. Control plane runtime

- F1.S1.T1: Extend the control snapshot and command set so the session/bootstrap
  flow can be driven by HTTP.
- F1.S1.T2: Register the control plane in the graphical runtime and keep the
  Bevy session/UI state in sync with the HTTP queue.
- F1.S2.T1: Keep the existing headless `--control-http` smoke path stable while
  the graphical control plane is active.

### F2. Tests and docs

- F2.S1.T1: Add automated tests for command parsing, snapshot updates, and
  route/session progression.
- F2.S2.T1: Update usage documentation for the graphical control plane and the
  local-only command set.

### F3. Validation

- F3.S1.T1: Run the relevant Cargo tests, build the client, and smoke both the
  graphical and headless control-http paths.
- F3.S1.T2: Capture any remaining control-plane or UI-flow debt in the progress
  and memory files.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Required evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260520-0422-session-backed-ui-flow/spec.md`, `.features/20260520-0422-session-backed-ui-flow/plan.md`, `.features/20260520-0422-session-backed-ui-flow/progress.md` | none | docs exist and audit passes | feature-workflow audit output |
| F1.S1.T1 Control snapshot/commands | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/session_state.rs` | F0.S1.T1 | HTTP commands can model the auth/bootstrap flow and snapshot the route/session state | parser + state tests |
| F1.S1.T2 Graphical bridge | local | `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | F1.S1.T1 | graphical runtime exposes the control plane and stays in sync with it | runtime/bridge tests |
| F1.S2.T1 Headless parity | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/state.rs` | F1.S1.T1 | headless `--control-http` behavior stays deterministic | headless smoke |
| F2.S1.T1 Automated tests | local | `port_rust/crates/mu_app/src/*` | F1 | tests cover command parsing, state updates, and route progression | `cargo test` outputs |
| F2.S2.T1 Usage docs | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | F1 | docs explain the graphical control plane and local-only command set | docs diff |
| F3.S1.T1 Validation | local | progress docs | F1-F2 | fmt/test/build/smoke evidence is captured | command output / logs |
| F3.S1.T2 Follow-up tracking | local | `.memory/TODO.md`, progress | F3.S1.T1 | any remaining control-plane debt is recorded | memory diff |

## Validation Gates

- Validation Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0422-session-backed-ui-flow`
- Validation Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Validation Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Validation Gate F3: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Validation Gate F4: graphical `mu_client --control-http 127.0.0.1:0` smoke with live state probe output
- Validation Gate F5: headless `mu_client --headless --control-http 127.0.0.1:0` smoke remains deterministic
- Validation Gate F6: `rtk rg -n "control-http|login|server select|character select" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md`
- Validation Gate F7: `e2e-validator` handoff for the graphical control-plane flow once the runtime bridge is wired

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T2, F3.S1.T1 | Validation evidence: Validation Gate F4 graphical `--control-http` smoke with live state output |
| AC-02 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | command parsing and route/session tests |
| AC-03 | F1.S1.T1, F2.S1.T1 | snapshot/state tests |
| AC-04 | F1.S2.T1, F3.S1.T1 | Validation evidence: Validation Gate F5 headless `--control-http` smoke remains deterministic |
| AC-05 | F2.S1.T1, F3.S1.T1 | `cargo test` output |
| AC-06 | F2.S2.T1, F3.S1.T1, F7.S1.T1 | Validation Gate F6 docs grep and F7 e2e handoff evidence |

## Risks

- The control snapshot can drift from the real runtime if the bridge only
  updates on sparse events; keep the runtime sync path centralized.
- Adding control commands can accidentally affect headless behavior; keep the
  existing headless branch isolated.
- Route/state commands must stay local-only so the control plane does not
  become an accidental production API.

## Rollback

- Revert the graphical control-plane bridge if it destabilizes the runtime.
- Keep the headless `--control-http` branch available so smoke tests can still
  probe the client if the graphical path needs to be backed out.
