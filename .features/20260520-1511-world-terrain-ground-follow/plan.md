# World Terrain Ground Follow

Status: READY_FOR_EXEC

## Summary

Ground the local and remote player markers to the visible terrain surface in
the world scene while leaving the rest of the world shell unchanged.

## Interfaces / Contracts

- No CLI, protocol, or persisted storage shapes change.
- `mu_app::world_scene` owns the grounding helper and marker transform update.
- `mu_app::graphical_runtime` keeps the same plugin registration and startup
  path.
- `mu_app::ClientRuntime` continues to provide the loaded world bundle.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters / state | Datasets / permissions | Renderer / test target | Retailer/industry compatibility | Compatibility expectations |
|---|---|---|---|---|---|---|---|---|
| `world-terrain-ground-follow` | `mu_app::world_scene`, `mu_app::ClientRuntime`, loaded terrain bundle | Bevy 3D world scene with grounded player markers | terrain height lookup, marker transform sync | current world route, ready world/render projections | local converted world bundle only | `mu_app` unit tests + graphical smoke | not applicable; no split | keep objects/NPCs/monsters and headless/control smoke unchanged |

## Parallelization

- Marker grounding and terrain sampling stay in one file and one slice.
- Docs can update once the grounding behavior is stable.
- Validation can run after implementation lands.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for this
  slice and audit them.

### F1. World marker grounding

- F1.S1.T1: Add a terrain surface sampler in `mu_app::world_scene` that reads
  the loaded world bundle and returns a render-space ground height.
- F1.S1.T2: Apply the terrain height to local and remote player markers while
  preserving the existing marker families and cleanup behavior.

### F2. Tests and docs

- F2.S1.T1: Add automated tests for terrain sampling and grounded marker
  transforms.
- F2.S2.T1: Update usage documentation and KB notes to describe the grounded
  player markers and the slice limit.

### F3. Validation

- F3.S1.T1: Run the relevant Cargo tests, build the client, and execute the
  local smoke checks.
- F3.S1.T2: Capture any remaining blockers or follow-up work in the progress
  and memory files.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F3.S1.T1 | Validation evidence: Gate F3 graphical smoke (`timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`) reaches the world shell and the grounded player markers remain visible there |
| AC-02 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | `mu_app` tests prove the terrain sampler and marker transform update keep the rest of the scene unchanged |
| AC-03 | F3.S1.T1 | Validation evidence: Gate F3 headless smoke (`cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`) still prints `ready-for-login` |
| AC-04 | F3.S1.T1 | Validation evidence: Gate F3 control-http smoke (`cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`) still serves the same endpoints |
| AC-05 | F2.S1.T1, F3.S1.T1 | Validation evidence: unit tests cover the sampler and grounded marker transforms |
| AC-06 | F2.S2.T1 | Validation evidence: docs mention grounded player markers and the out-of-scope collision limit |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-1511-world-terrain-ground-follow`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk rg -n "grounded|terrain surface|player markers" docs/player-rust-client.md port_rust/README.md`
- Gate F3: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`; `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`
- Gate F4: `e2e-validator` handoff for the grounded world markers when a windowed smoke run is available

## Risks

- If the terrain sample math drifts from the visible mesh, markers can float
  or sink. Keep the sampler aligned with the existing world terrain build.
- The slice should not touch object/NPC/monster rendering paths.
- Grounding must remain visual only; collision work is a separate follow-up.

## Rollback

- Remove the marker-grounding helper and restore the previous fixed marker
  height if the new placement disrupts the world scene.
- Leave the world bundle loader and headless smoke paths intact.
