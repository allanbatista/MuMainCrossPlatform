---
status: READY_FOR_EXEC
---

# World Camera Follow

## Summary

Make the world camera follow the local avatar inside the existing Bevy world
scene so the playable world stays centered while preserving headless and
control-http behavior.

## Interfaces / Contracts

- `mu_app::world_scene` remains the owner of the visible world shell.
- `mu_app::ClientRuntime` keeps owning the local player projection.
- The new work only changes the internal world-scene camera behavior.
- No CLI, protocol, storage, or control-http contract changes.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `world-camera-follow` | `mu_app::world_scene`, `mu_app::ClientRuntime`, `mu_ui::UiShellState` | Bevy follow camera in the world route | current UI route, local player pose | world-route readiness, local avatar availability | local converted world bundle only | n/a; single-client port | `mu_app` unit tests + graphical smoke | fixed follow offset is enough for this slice; orbital zoom remains follow-up |

## Parallelization

- Camera-spawn and camera-follow code can be implemented in the world-scene
  module together.
- Documentation and KB updates can happen after the runtime behavior lands.
- Validation runs after the code and docs are in place.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for the
  world-camera follow slice and audit them.

### F1. Camera follow implementation

#### F1.S1 Tasks

- F1.S1.T1: Add a camera follow marker/state in `mu_app::world_scene` that
  stores the world-camera offset and can be updated from the local avatar.
- F1.S1.T2: Update the world-scene spawn path to keep the camera centered on
  the local avatar while preserving route cleanup and reload behavior.

#### F1.S2 Tasks

- F1.S2.T1: Add unit tests for the camera follow behavior and the world-scene
  lifecycle.

### F2. Docs and tracking

#### F2.S1 Tasks

- F2.S1.T1: Update player-facing usage docs to describe the follow camera and
  its current limits.

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
| AC-01 | F1.S1.T1, F1.S1.T2, F3.S1.T1 | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app` covers the camera follow helper and the world route smoke reaches the scene |
| AC-02 | F1.S1.T2, F1.S2.T1 | `mu_app` unit tests cover scene spawn, cleanup, and camera tracking on route changes |
| AC-03 | F3.S1.T1 | Validation evidence: Validation Gate F3 headless smoke test: `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless` still prints `ready-for-login` |
| AC-04 | F3.S1.T1 | Validation evidence: Validation Gate F3 control-http smoke test: `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0` still serves the same endpoints |
| AC-05 | F1.S2.T1, F3.S1.T1 | `mu_app` tests cover the camera follow path and the scene lifecycle |
| AC-06 | F2.S1.T1, F2.S2.T1, F3.S1.T1 | Validation evidence: Validation Gate F2 docs grep test: `rg -n "follow camera|world camera|world route" docs/player-rust-client.md port_rust/README.md` shows the usage docs mention the slice and its limits |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-2100-world-camera-follow`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk rg -n "follow camera|world camera|world route" docs/player-rust-client.md port_rust/README.md`
- Gate F3: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`; `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`; `e2e-validator` handoff for the visible camera smoke when a windowed run is available

## Risks

- Camera updates can drift from the legacy client if the follow offset is
  misread; keep the helper narrow and test the local-avatar delta.
- The new follow behavior must remain a scene-time cost only, not a per-frame
  world rebuild.
- If bundle data or the local avatar is missing, the runtime should keep the
  existing world shell stable rather than reintroducing an exit path.

## Rollback

- Remove the camera follow component/system from `world_scene.rs`.
- Drop the follow-camera tests.
- Revert the doc wording if the follow camera needs to be backed out.
