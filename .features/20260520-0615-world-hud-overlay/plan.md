# World HUD Overlay

Status: READY_FOR_EXEC

## Summary

Implement the first visible world HUD slice in `mu_app`: a compact Bevy
overlay that appears with the world route, renders the legacy HUD snapshot
data, clears with the world projection, and keeps the current headless/control
paths intact.

## Interfaces / Contracts

- New `mu_app::world_hud` module owns the world HUD plugin and visible overlay
  lifecycle.
- `build_graphical_app` registers the new HUD plugin alongside the existing
  bootstrap, world scene, auth shell, and movement plugins.
- `mu_ui::hud_screen` remains the source model for the visible HUD content.
- No CLI flags change.
- No protocol or storage shapes change.
- Existing headless and control-http code paths remain intact.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters / state | Datasets / permissions | Renderer / test target | Retailer/industry compatibility | Compatibility expectations |
|---|---|---|---|---|---|---|---|---|
| `world-hud-overlay` | `mu_app::world_hud`, `mu_app::graphical_runtime`, `mu_app::ClientRuntime`, `mu_ui::UiShellState`, `mu_ui::hud_screen` | Bevy world overlay card | route state plus world-ready projection | current world route, world-ready state, HUD snapshot | local converted assets and UI snapshot data only | `mu_app` unit tests + `mu_client` graphical smoke | not applicable; no retailer/industry split | no change to headless/control-http, no network contract change |

## Phases

### F0. Workflow docs

#### F0.S1 Task

- F0.S1.T1 Create and audit the feature workflow docs for the world HUD
  overlay slice.

### F1. Runtime overlay

#### F1.S1 Tasks

- F1.S1.T1 Add a world HUD plugin module that can spawn and tear down a visible
  overlay from the HUD snapshot data.
- F1.S1.T2 Wire the new plugin into the graphical runtime bootstrap.

#### F1.S2 Tasks

- F1.S2.T1 Add unit tests for overlay spawn, route cleanup, and HUD body
  formatting.

### F2. Usage and validation

#### F2.S1 Tasks

- F2.S1.T1 Update usage docs to describe the visible HUD overlay and its
  current limits.

#### F2.S2 Tasks

- F2.S2.T1 Run fmt, tests, build, and graphical smoke validation, then record
  the evidence.

#### F2.S3 Tasks

- F2.S3.T1 Update the pending-work memory and progress notes after validation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F2.S2.T1 | Validation evidence: graphical smoke reaches the world route and shows the HUD overlay. |
| AC-02 | F1.S1.T1, F1.S2.T1 | Validation evidence: `mu_app` tests cover overlay cleanup when the route leaves the world or the world projection is cleared. |
| AC-03 | F1.S2.T1 | Validation evidence: `mu_app` tests cover the HUD body formatting helpers and snapshot-driven content. |
| AC-04 | F2.S1.T1 | Validation evidence: player-facing docs mention the visible HUD overlay and its current limits. |
| AC-05 | F2.S2.T1 | Validation evidence: headless/control-http smoke still reports the same deterministic state behavior. |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0615-world-hud-overlay`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`; `e2e-validator` handoff for the visible HUD overlay when the environment allows a windowed smoke run
- Gate F3: `rtk rg -n "HUD overlay|world HUD" docs/player-rust-client.md port_rust/README.md`

## Risks

- The HUD overlay can obscure the 3D world if it is positioned too aggressively;
  keep it compact and anchored.
- The first slice intentionally reuses the static HUD snapshot data instead of
  rebuilding the full interactive HUD stack.
- If the graphical smoke cannot open a window in the current environment, keep
  the unit-test evidence and record the blocker explicitly.

## Parallelization

- The overlay body formatting and the cleanup tests can be built together once
  the plugin shape is stable.
- Docs and validation are sequential after the implementation lands.

## Rollback

- Remove the world HUD plugin registration from `graphical_runtime.rs`.
- Delete the new overlay module and its tests.
- Restore the previous docs wording if the visible HUD needs to be backed out.
