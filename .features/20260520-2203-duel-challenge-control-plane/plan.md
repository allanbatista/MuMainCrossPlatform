# Duel Challenge Control Plane

Status: READY_FOR_EXEC

## Summary

Wire `duel-start` and `duel-stop` through `mu_app`, bridge them to the
bootstrap worker, and document the control-plane slice.

## Interfaces / Contracts

- `mu_app::control_http` accepts `duel-start` and `duel-stop`.
- `mu_app::graphical_runtime` mirrors the new control snapshot state into the
  runtime and queues the live bootstrap commands.
- `mu_app::bootstrap_runtime` owns the queue/send path for the duel packet
  helpers.
- `docs/player-rust-client.md`, `port_rust/README.md`, and
  `port_rust/docs/control-http.md` describe the new commands.

## Technical Inventory

| Slug/ID | Components | Output type | Queries | Filters / URL state | Datasets / permissions | Renderer / test target | Compatibility expectations | Retailer / industry compatibility |
|---|---|---|---|---|---|---|---|---|
| `duel-start-stop-control-plane` | `mu_app::control_http`, `mu_app::graphical_runtime`, `mu_app::bootstrap_runtime`, `mu_protocol::events` | control-plane command bridge | `duel-start`, `duel-stop` | HTTP command string, duel target id/name | local smoke only | `mu_app` unit tests and headless control HTTP smoke | route the duel command into the live packet bridge without changing the visible duel shell | n/a; local QA/dev control plane only |

## Phases

### F0. Workflow docs

#### F0.S1 Task

- F0.S1.T1 Create and audit the feature workflow docs for the duel challenge
  control-plane slice.

### F1. Runtime bridge

#### F1.S1 Tasks

- F1.S1.T1 Extend the control snapshot and command parser for `duel-start`
  and `duel-stop`.
- F1.S1.T2 Add bootstrap queue methods and live send wiring for the duel
  packet helpers.

### F2. Tests and docs

#### F2.S1 Tasks

- F2.S1.T1 Add unit tests for command parsing, snapshot routing, and queue
  behavior.

#### F2.S2 Tasks

- F2.S2.T1 Update usage docs to describe the duel control commands.
- F2.S2.T2 Add a KB note and update the KB index.

### F3. Validation

#### F3.S1 Tasks

- F3.S1.T1 Run fmt, tests, build, and local control HTTP smoke validation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | Unit tests and runtime bridge tests cover `duel-start`. |
| AC-02 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | Unit tests and runtime bridge tests cover `duel-stop`. |
| AC-03 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | Automated tests cover parser, queue, and send behavior. |
| AC-04 | F2.S2.T1, F3.S1.T1 | Validation Gate F0 docs grep evidence: `rtk rg -n "duel-start|duel-stop" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md` plus `.codexpotter/kb/duel-challenge-control-plane.md` and `.codexpotter/kb/README.md` updates. |

## Validation Gates

- Gate F0: feature doc audit for `.features/20260520-2203-duel-challenge-control-plane`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`;
  `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`;
  `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`;
  headless control HTTP smoke for `duel-start`, `duel-stop`, and `exit`;
  `e2e-validator`: not applicable because this slice is headless control HTTP
  only and has no new rendered UI surface

## Risks

- The snapshot bridge must not break the existing `duel` route shell.
- The new commands should stay local-control only and not require a live duel
  server for validation.

## Rollback

- Remove the new command variants, bridge methods, and docs together if the
  slice destabilizes the graphical runtime or control HTTP parsing.
