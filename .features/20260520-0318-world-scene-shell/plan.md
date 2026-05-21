# World Scene Shell

Status: READY_FOR_EXEC

## Summary

Implement the first visible world slice after bootstrap: a Bevy 3D shell that
appears once the client reaches the world route, uses the existing runtime
projections for terrain and scene markers, preserves headless/control-http
behavior, and documents the new visible world state.

## Interfaces / Contracts

- New `mu_app::world_scene` module owns the world-scene plugin and route
  lifecycle.
- `build_graphical_app` registers the world-scene plugin alongside the existing
  bootstrap and UI/runtime plugins.
- No CLI flags change.
- No protocol or storage shapes change.
- Existing headless and control-http code paths remain intact.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters / state | Datasets / permissions | Renderer / test target | Retailer/industry compatibility | Compatibility expectations |
|---|---|---|---|---|---|---|---|---|
| `world-scene-shell` | `mu_app::world_scene`, `mu_app::graphical_runtime`, `mu_app::ClientRuntime`, `mu_ui::UiShellState`, `mu_render::RenderEntities` | Bevy 3D scene window | route state only | current UI route, loaded world summary, optional `--asset-root` | local converted assets only | `mu_app` unit tests + `mu_client` graphical smoke | not applicable; no retailer/industry split | no change to headless/control-http, no network contract change |

## Phases

### F0. Workflow docs

#### F0.S1 Task

- F0.S1.T1 Create and audit the feature workflow docs for the world-scene
  shell slice.

### F1. Scene implementation

#### F1.S1 Tasks

- F1.S1.T1 Add a world-scene plugin module that can spawn and tear down a
  visible 3D shell from the existing runtime projections.
- F1.S1.T2 Wire the new plugin into the graphical runtime bootstrap.

#### F1.S2 Tasks

- F1.S2.T1 Add unit tests for scene bootstrap, route cleanup, and the
  world-ready fallback path.

### F2. Usage and validation

#### F2.S1 Tasks

- F2.S1.T1 Update usage docs to describe the visible world shell and its
  current limits.

#### F2.S2 Tasks

- F2.S2.T1 Run fmt, tests, build, and graphical smoke validation, then record
  the evidence.

#### F2.S3 Tasks

- F2.S3.T1 Update the pending-work memory and progress notes after validation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F2.S2.T1 | Validation evidence: Validation Gate F2 graphical smoke: `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client` stays alive in graphical mode and reaches the world shell. |
| AC-02 | F1.S1.T1, F1.S2.T1 | Validation Gate F1 `mu_app` tests: `cargo test --manifest-path port_rust/Cargo.toml -p mu_app` covers scene bootstrap and verifies the world route builds a visible shell. |
| AC-03 | F2.S2.T1 | Validation evidence: Validation Gate F2 headless/control-http smoke: `mu_client --headless` still prints `ready-for-login`, and `mu_client --headless --control-http 127.0.0.1:0` still serves the same state/control endpoints. |
| AC-04 | F1.S2.T1, F2.S2.T1 | Validation Gate F1/F2 `mu_app` tests: scene bootstrap and cleanup unit tests pass in `mu_app`. |
| AC-05 | F2.S1.T1 | Validation evidence: Validation Gate F3 docs diff: `docs/player-rust-client.md` and `port_rust/README.md` mention the visible world shell and its limits. |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0318-world-scene-shell`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`; `e2e-validator` handoff for the visible world shell when the environment allows a windowed smoke run
- Gate F3: `rtk rg -n "world shell|world-scene-shell" docs/player-rust-client.md port_rust/README.md`

## Risks

- Bevy API drift may require small adjustments to primitive mesh or camera
  APIs.
- The first slice intentionally uses a visible shell and sampled markers, not
  full asset-faithful world rendering.
- If the graphical smoke cannot open a window in the current environment, keep
  the unit-test evidence and record the blocker explicitly.

## Parallelization

- F1.S1.T1 and F1.S2.T1 can proceed in parallel only after the plugin
  interface is stable.
- Docs and validation are sequential after the implementation lands.

## Rollback

- Remove the world-scene plugin registration from `graphical_runtime.rs`.
- Delete the new scene module and its tests.
- Restore the previous docs wording if the visible shell needs to be backed out.
