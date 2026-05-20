---
status: READY_FOR_EXEC
---

# World Camera Zoom

## Summary

Wire the world-route follow camera to the persisted camera zoom setting, add
mouse-wheel zoom adjustments, and persist the updated value back to
`config/client.toml` on graphical exit.

## Interfaces / Contracts

- `mu_app::graphical_runtime` loads the shared `Config` resource from the
  CLI-selected config path and saves it back after the graphical app exits.
- `mu_app::world_scene` remains the owner of the visible world-route camera
  and reads the active camera zoom from `Config`.
- The feature does not change control-http commands, network packets, or the
  headless runtime contract.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `world-camera-zoom` | `mu_app::graphical_runtime`, `mu_app::world_scene`, `mu_app::config`, `docs/player-rust-client.md`, `port_rust/README.md` | Bevy world camera + persisted config zoom | world route, mouse wheel, saved `camera.zoom` | world-route readiness, local avatar availability | local config file only | n/a; single-client port | `mu_app` unit tests + graphical smoke | keep the follow angle fixed; only the distance changes |

## Parallelization

- Config load/save wiring and the world-scene zoom math can be implemented in
  the same working set.
- The wheel-input handling can land alongside the camera sync helper once the
  config resource exists.
- Docs and KB updates can follow the runtime behavior.
- Validation runs after the code and docs are in place.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for the
  world-camera-zoom slice and audit them.

### F1. Camera zoom implementation

#### F1.S1 Tasks

- F1.S1.T1: Load the shared `Config` resource in the graphical runtime and
  persist it after the app exits.
- F1.S1.T2: Update the world-scene camera to scale from `camera.zoom` instead
  of a fixed follow offset.
- F1.S1.T3: Handle mouse-wheel zoom changes on the world route and clamp the
  saved zoom to the legacy range.

#### F1.S2 Tasks

- F1.S2.T1: Add unit tests for the zoom scale helper, the clamp behavior, and
  the world-camera follow distance.

### F2. Docs and tracking

#### F2.S1 Tasks

- F2.S1.T1: Update player-facing usage docs to describe the world-route zoom
  control and persistence.

#### F2.S2 Tasks

- F2.S2.T1: Refresh the KB note and any pending-work tracking that is still
  relevant after the slice lands.

### F3. Validation

#### F3.S1 Tasks

- F3.S1.T1: Run fmt, tests, build, and graphical/headless smoke validation,
  then record the evidence.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F3.S1.T1 | `mu_app` tests cover the zoom-scaled follow offset when the world scene loads |
| AC-02 | F1.S1.T2, F1.S1.T3, F1.S2.T1 | `mu_app` tests cover mouse-wheel zoom changes and the camera follow distance update |
| AC-03 | F1.S1.T1, F3.S1.T1 | Validation evidence: graphical exit saves the updated config and the saved zoom survives a reload |
| AC-04 | F3.S1.T1 | Validation evidence: `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless` still prints `ready-for-login` |
| AC-05 | F3.S1.T1 | Validation evidence: `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0` still serves the same endpoints |
| AC-06 | F1.S2.T1, F3.S1.T1 | `mu_app` tests cover the zoom math, clamp, and config round-trip |
| AC-07 | F2.S1.T1, F2.S2.T1, F3.S1.T1 | Validation evidence: docs mention the world-route zoom control and persistence |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-2200-world-camera-zoom`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk rg -n "mouse wheel|zoom|client.toml" docs/player-rust-client.md port_rust/README.md`
- Gate F3: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`; `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`; `e2e-validator` handoff for the visible camera smoke when a windowed run is available

## Risks

- If config load/save is wired incorrectly, the world camera may reset to the
  default zoom between runs.
- Wheel input must stay route-gated so it does not affect login, character
  select, or other shell routes.
- The camera follow math should preserve the current angle instead of
  accidentally introducing a new orbital behavior.

## Rollback

- Remove the config-backed zoom handling from `graphical_runtime.rs`.
- Revert the zoom-aware world-camera follow offset in `world_scene.rs`.
- Drop the zoom tests and docs wording if the slice needs to be backed out.
