# World Terrain Lightmap

Status: READY_FOR_EXEC

## Summary

Apply the bundled terrain lightmap to the visible world terrain, keep the
current route-driven scene lifecycle stable, and document the visible change
and its fallback.

## Interfaces / Contracts

- `mu_app::world_scene` remains the owner of the visible world shell.
- `mu_app::ClientRuntime` and `mu_gameplay::WorldManager` keep owning the
  loaded bundle data.
- `world_scene` reads the loaded bundle and the configured asset root to
  resolve the lightmap path used by the visible terrain.
- No CLI, protocol, storage, or control-http contract changes.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `world-terrain-lightmap` | `mu_app::world_scene`, `mu_app::GraphicalRuntimeConfig`, `mu_assets::TerrainWorldBundle`, `mu_assets::TerrainWorldConfig` | Bevy terrain surface with baked lighting overlay | current UI route, loaded world bundle, asset root, lightmap path resolution | route readiness, bundle availability, resolved asset path | local converted terrain bundle only; no auth or tenant data | n/a; single-client port | `mu_app` unit tests + graphical smoke | the lightmap is a startup-time visual enhancement only; fallback stays on the current terrain surface |

## Parallelization

- Lightmap path resolution and scene wiring can be implemented together in the
  world-scene module.
- Documentation and KB updates can happen after the runtime behavior lands.
- Validation runs after the code and docs are in place.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for the
  terrain lightmap slice and audit them.

### F1. Terrain lightmap implementation

#### F1.S1 Tasks

- F1.S1.T1: Add a helper in `mu_app::world_scene` that resolves the bundle
  lightmap path against the configured asset root and returns a usable asset
  path when the checked-in file exists.
- F1.S1.T2: Spawn the lightmap-backed terrain surface in the world scene when
  the path resolves, while preserving the current textured terrain fallback and
  route gating.

#### F1.S2 Tasks

- F1.S2.T1: Add unit tests for lightmap path resolution, overlay spawning, and
  scene cleanup on route changes.

### F2. Docs and tracking

#### F2.S1 Tasks

- F2.S1.T1: Update player-facing usage docs to describe the lightmap-backed
  terrain surface and its current limit.

#### F2.S2 Tasks

- F2.S2.T1: Add or refresh the KB note for the terrain lightmap slice and
  clean up any stale pending-work tracking that is no longer true after the
  slice lands.

### F3. Validation

#### F3.S1 Tasks

- F3.S1.T1: Run fmt, tests, build, and graphical/headless smoke validation,
  then record the evidence.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F3.S1.T1 | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app` covers lightmap path resolution and terrain surface generation, and `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client` reaches the world route |
| AC-02 | F1.S1.T2, F1.S2.T1 | `mu_app` unit tests cover terrain cleanup and reload on route changes |
| AC-03 | F3.S1.T1 | Validation Gate F3 headless smoke test: `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless` still prints `ready-for-login` |
| AC-04 | F3.S1.T1 | Validation Gate F3 control-http smoke test: `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0` still serves the same endpoints |
| AC-05 | F1.S2.T1, F3.S1.T1 | `mu_app` unit tests cover lightmap resolution and scene lifecycle |
| AC-06 | F2.S1.T1, F2.S2.T1, F3.S1.T1 | Validation Gate F2 docs grep validation test: `rg -n "lightmap|terrain lighting|world terrain" docs/player-rust-client.md port_rust/README.md` shows the usage docs mention the slice and its limit |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-1218-world-terrain-lightmap`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk rg -n "lightmap|terrain lighting|world terrain" docs/player-rust-client.md port_rust/README.md`
- Gate F3: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`; `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`; `e2e-validator` handoff for the visible terrain smoke when a windowed run is available

## Risks

- Resolving the wrong file name could make the lightmap invisible and regress
  the current terrain appearance if the fallback is not preserved.
- The new overlay must remain a startup-time cost only, not a per-frame
  rebuild.
- If the bundle lightmap cannot be resolved, the runtime should fall back
  safely rather than reintroducing an unintended exit path.

## Rollback

- Restore the current terrain surface-only path in `world_scene.rs`.
- Drop the lightmap helper and its tests.
- Revert the doc wording if the slice needs to be backed out.
