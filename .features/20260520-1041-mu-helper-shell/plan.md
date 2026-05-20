# MU Helper Shell

Status: READY_FOR_PLAN

## Summary

Expose the existing MU Helper runtime and snapshot UI in `mu_app`, add the
local `mu-helper` control-plane smoke command, and document how to use the new
surface.

## Interfaces / Contracts

- `mu_gameplay::MuHelperRuntime` remains the source of truth for helper state.
- `mu_ui::mu_helper_screen()` remains the snapshot contract for the shell.
- `mu_app::graphical_runtime::configure_project_plugins` registers the helper
  runtime and shell plugins.
- `mu_app::control_http::ControlCommand` and `ControlSnapshot` drive the local
  smoke path.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|
| `mu-helper-shell` | `mu_app::MuHelperRuntimePlugin`, shell plugin, control HTTP | Bevy shell + snapshot | route visibility, helper execution state | config summary, status detail | local only | `mu_app` tests | Exposes an existing gameplay surface, not a new helper model |

## Parallelization

- Shell rendering and control-plane command wiring are closely coupled and
  should be implemented in one slice.
- Docs can be updated after the runtime wiring is stable.
- Validation can run after the runtime and docs are in place.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for this
  slice and audit them.

### F1. Runtime exposure

- F1.S1.T1: Register the helper runtime and render a Bevy shell when
  `UiRoute::MuHelper` is active.
- F1.S1.T2: Add the `mu-helper` control-plane command and keep the snapshot
  sync deterministic.

### F2. Tests and docs

- F2.S1.T1: Add automated tests for shell visibility and control-plane wiring.
- F2.S2.T1: Update usage documentation for the helper shell and smoke path.

### F3. Validation

- F3.S1.T1: Run the relevant Cargo tests, build the client, and execute the
  local smoke checks.
- F3.S1.T2: Capture any remaining blockers or follow-up work in the progress
  and memory files.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Required evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260520-1041-mu-helper-shell/spec.md`, `.features/20260520-1041-mu-helper-shell/plan.md`, `.features/20260520-1041-mu-helper-shell/progress.md` | none | docs exist and audit passes | feature-workflow audit output |
| F1.S1.T1 Shell plugin | local | `port_rust/crates/mu_app/src/mu_helper_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | F0.S1.T1 | runtime shows the MU Helper surface when the route is active | unit tests |
| F1.S1.T2 Control-plane command | local | `port_rust/crates/mu_app/src/control_http.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | F1.S1.T1 | `mu-helper` drives the visible shell and snapshot sync stays stable | control-plane smoke |
| F2.S1.T1 Automated tests | local | `port_rust/crates/mu_app/src/*` | F1 | tests cover helper shell visibility and command parsing | `cargo test` outputs |
| F2.S2.T1 Usage docs | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | F1 | docs explain how to smoke the MU Helper shell | docs diff |
| F3.S1.T1 Validation | local | progress docs | F1-F2 | fmt/test/build/smoke evidence is captured | command output / logs |
| F3.S1.T2 Follow-up tracking | local | `.memory/TODO.md`, progress | F3.S1.T1 | any remaining helper debt is recorded | memory diff |

## Validation Gates

- `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-1041-mu-helper-shell`
- `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- local graphical smoke opening `mu-helper`
- local headless `--control-http` smoke covering `mu-helper` and `exit`
- `rtk rg -n "mu-helper|MU Helper" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md`

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1 | shell visibility tests and runtime wiring |
| AC-02 | F1.S1.T2 | control-plane smoke |
| AC-03 | F1.S1.T1, F1.S1.T2 | snapshot assertions and smoke logs |
| AC-04 | F2.S1.T1, F3.S1.T1 | automated test output |
| AC-05 | F2.S2.T1 | docs diff |

## Risks

- The helper shell can drift from the gameplay runtime if the state is not
  sourced directly from `MuHelperRuntime`; keep the snapshot path one-way.
- Control-plane wiring must not introduce a new helper state machine.
- The new route should stay local and deterministic for smoke tests.

## Rollback

- Remove the helper shell plugin and control command if the runtime exposure
  disturbs existing gameplay routes.
- Keep the helper runtime in `mu_gameplay` and the UI snapshot in `mu_ui` so
  the slice can be retried without changing the underlying model.
