# Local Movement Sync

Status: READY_FOR_EXEC

## Summary

Implement the first interactive world slice after the bootstrap path: seed a
default local avatar, wire world-route movement input into the runtime, keep
the rendered marker in sync with the local pose, and document the controls.

## Interfaces / Contracts

- `mu_app::ClientRuntime::load_world_bundle` seeds a local player when the
  world loads and keeps the render/world projections in sync.
- `mu_gameplay::WorldEntitiesManager` remains the canonical owner of local and
  remote world player spawns, with explicit pose mutation helpers for the
  local avatar.
- `mu_app::world_motion::WorldMotionPlugin` captures the world-route movement
  input and applies local pose updates.
- `mu_app::WorldScenePlugin` keeps the rendered local marker aligned with the
  runtime pose data.
- `mu_app::graphical_runtime` registers the movement plugin in the Bevy app.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `local-avatar-seed` | `mu_app::ClientRuntime`, `mu_gameplay::WorldEntitiesManager`, `mu_gameplay::WorldPlayerSpawn` | local player spawn | world bundle load | missing local avatar | local world bundle only | no split | `mu_app` tests | Seeds a visible local avatar at world entry |
| `world-motion-input` | `mu_app::world_motion`, `bevy::input::ButtonInput<KeyCode>`, `mu_ui::UiShellState` | pose updates | movement keys, delta time | world route active only | world route and runtime ready | no split | `mu_app` tests | Moves the local avatar with the current bindings layout |
| `scene-marker-sync` | `mu_app::world_scene`, `WorldSceneMarker`, `Transform` | visible marker movement | local player pose | world scene active | local runtime projection only | no split | `mu_app` tests | Keeps the Bevy marker aligned with the runtime pose |
| `player-controls-docs` | `docs/player-rust-client.md`, `port_rust/README.md` | usage docs | movement controls | local-only note | player-facing documentation only | no split | docs grep | Documents the first interactive controls |

## Parallelization

- The local avatar seed and the world-motion controller can be built as one
  tightly coupled runtime slice.
- Tests can be added after the seed and motion helper shape is stable.
- Docs can be updated after the control scheme is fixed.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write and audit the feature spec, plan, and progress documents for
  the local movement slice.

### F1. Runtime movement

- F1.S1.T1: Seed a default local avatar during world load and add pose mutation
  helpers on the world entity/runtime side.
- F1.S1.T2: Add a Bevy movement controller for the world route and keep the
  rendered local marker aligned with the updated pose.
- F1.S2.T1: Keep movement ignored outside the world route and ensure the local
  avatar is cleared with the rest of the world projection.

### F2. Tests and docs

- F2.S1.T1: Add automated tests for avatar seeding, motion updates, and scene
  sync.
- F2.S2.T1: Update usage documentation with the first interactive movement
  controls and the local-only scope.

### F3. Validation

- F3.S1.T1: Run the relevant Cargo tests, build the client, and smoke the
  graphical/headless boot paths.
- F3.S1.T2: Track any remaining movement or network-sync debt in the progress
  and memory files.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Required evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260520-0530-local-movement-sync/spec.md`, `.features/20260520-0530-local-movement-sync/plan.md`, `.features/20260520-0530-local-movement-sync/progress.md` | none | docs exist and audit passes | feature-workflow audit output |
| F1.S1.T1 Local avatar seed | local | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_gameplay/src/entities.rs` | F0.S1.T1 | world load seeds a visible local avatar and exposes pose mutation helpers | unit tests for the seed and pose helpers |
| F1.S1.T2 World motion controller | local | `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | F1.S1.T1 | WASD movement updates the local avatar and the visible marker stays aligned | movement controller and scene sync tests |
| F1.S2.T1 Route and reset guards | local | `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/client_runtime.rs` | F1.S1.T1 | movement is ignored outside the world route and clears with world reset/disconnect | route/clear guard tests |
| F2.S1.T1 Automated tests | local | `port_rust/crates/mu_app/src/*`, `port_rust/crates/mu_gameplay/src/*` | F1 | tests cover seeding, pose updates, and scene sync | `cargo test` outputs |
| F2.S2.T1 Usage docs | local | `docs/player-rust-client.md`, `port_rust/README.md` | F1 | docs explain the movement controls and local-only scope | docs diff |
| F3.S1.T1 Validation | local | progress docs | F1-F2 | fmt/test/build/smoke evidence is captured | command output / logs |
| F3.S1.T2 Follow-up tracking | local | `.memory/TODO.md`, progress | F3.S1.T1 | remaining movement or network-sync debt is recorded | memory diff |

## Validation Gates

- Validation Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0530-local-movement-sync`
- Validation Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Validation Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Validation Gate F3: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Validation Gate F4: graphical and headless local smoke for `mu_client`
- Validation Gate F5: `e2e-validator` handoff for the visible movement path once the local avatar is wired
- Validation Gate F6: `rtk rg -n "movement|avatar|WASD" docs/player-rust-client.md port_rust/README.md`

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2 | local avatar seed and scene sync tests |
| AC-02 | F1.S1.T2, F2.S1.T1 | movement controller and scene sync tests |
| AC-03 | F1.S2.T1, F2.S1.T1 | route/reset guard tests |
| AC-04 | F2.S1.T1, F3.S1.T1 | `cargo test` output |
| AC-05 | F2.S2.T1, F3.S1.T1 | validation evidence: Validation Gate F6 |
| AC-06 | F3.S1.T1 | Validation Gate F4 headless/control smoke evidence |

## Risks

- Movement can drift if the runtime pose and the rendered marker are updated in
  different code paths; keep the motion driver and scene sync close together.
- The placeholder local avatar can be confused with real server-owned player
  state later; keep the seed helper isolated so later networked spawn logic can
  replace it cleanly.
- Reset/disconnect handling must keep the local avatar out of the world scene
  when the route changes.

## Rollback

- Revert the movement controller and local-avatar seed as a single slice if
  the world route becomes unstable.
- Keep the world bootstrap changes isolated so login/world flow can remain
  intact if movement needs to be backed out.
