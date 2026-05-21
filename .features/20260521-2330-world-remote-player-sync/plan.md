# World Remote Player Sync

Status: READY_FOR_EXEC

## Summary

Teach the runtime to keep remote player markers in the world projection by
key, feed non-local movement packets into that path, clear the remote roster on
logout/disconnect, and update the docs/tests to match.

## Interfaces / Contracts

- `mu_gameplay::entities::WorldEntitiesManager` owns the keyed remote-player
  storage and update helpers.
- `mu_app::client_runtime::ClientRuntime` exposes remote-player update and
  cleanup helpers that always resync the render projection.
- `mu_app::bootstrap_runtime::apply_movement_update()` differentiates local
  and remote movement keys.
- `mu_app::bootstrap_runtime` session handlers clear remote players on
  logout/disconnect without clearing the loaded world bundle.
- `docs/player-rust-client.md` and `port_rust/README.md` document the live
  remote-player sync behavior.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `remote-player-keyed-upsert` | `mu_gameplay::WorldEntitiesManager`, `mu_gameplay::WorldPlayerSpawn` | keyed remote-player list | movement packet key, existing remote key | duplicate key / replace vs insert | world projection only | no split | `mu_gameplay` tests | Keeps repeated packets on the same remote marker |
| `remote-player-runtime-bridge` | `mu_app::ClientRuntime`, `mu_app::bootstrap_runtime` | world projection sync | local key, remote key, movement packet payload | local vs remote player key | live session / world route | no split | `mu_app` tests | Routes non-local movement into the remote roster and preserves local movement behavior |
| `remote-player-cleanup` | `mu_app::bootstrap_runtime`, `mu_app::ClientRuntime` | cleared remote roster | logout / disconnect events | session reset only | world bundle stays loaded | no split | `mu_app` tests | Removes stale remote markers without tearing down the world |
| `remote-player-docs` | `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/*` | usage docs / KB notes | remote movement and cleanup behavior | docs grep | documentation only | no split | docs grep | Records the live remote-player sync flow for later movement work |

## Parallelization

- The gameplay manager helpers and the client-runtime bridge can be developed
  independently once the keyed update contract is fixed.
- Bootstrap packet routing can land after the runtime helper is in place.
- Docs and KB updates can follow the code changes.

## Phases

### F0. Workflow docs

- F0.S1.T1: Write the feature spec, plan, and progress documents for this
  slice and audit them.

### F1. Gameplay/runtime helpers

- F1.S1.T1: Add keyed remote-player lookup/update helpers to
  `WorldEntitiesManager` and cover them with unit tests.
- F1.S1.T2: Add `ClientRuntime` helpers that create/update remote players by
  key, keep `render_entities` in sync, and clear the remote roster on demand.

### F2. Bootstrap routing and cleanup

- F2.S1.T1: Route non-local movement packets through the remote-player update
  path and extend the bootstrap integration test to prove remote markers move.
- F2.S1.T2: Clear remote markers on logout/disconnect and extend the session
  cleanup coverage.

### F3. Docs and validation

- F3.S1.T1: Update the player guide, repo README, and KB notes to mention live
  remote-player sync.
- F3.S1.T2: Run the targeted Cargo tests, the workspace build/test pass, the
  client build, and the local smoke checks.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Required Evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260521-2330-world-remote-player-sync/spec.md`, `.features/20260521-2330-world-remote-player-sync/plan.md`, `.features/20260521-2330-world-remote-player-sync/progress.md` | none | docs exist and audit passes | feature-workflow audit output |
| F1.S1.T1 Remote-player helpers | local | `port_rust/crates/mu_gameplay/src/entities.rs` | F0.S1.T1 | keyed remote-player lookup/update helpers exist and are unit tested | `cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay entities` |
| F1.S1.T2 Runtime bridge helpers | local | `port_rust/crates/mu_app/src/client_runtime.rs` | F1.S1.T1 | `ClientRuntime` can create/update/clear remote players and resync projection | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app client_runtime` |
| F2.S1.T1 Bootstrap routing | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | F1.S1.T2 | non-local movement packets update remote markers and the integration test proves it | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime` |
| F2.S1.T2 Session cleanup | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | F1.S1.T2 | logout/disconnect clears the remote roster without clearing the world bundle | bootstrap cleanup test output |
| F3.S1.T1 Docs and KB | local | `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/world-entity-spawns.md`, `.codexpotter/kb/server-authoritative-movement.md`, `.codexpotter/kb/README.md` | F2 | docs and KB notes mention live remote-player sync | docs grep / KB diff |
| F3.S1.T2 Validation | local | none | F1-F3.S1.T1 | audit, test, build, and smoke evidence captured | command output / logs |

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F2.S1.T1 | keyed update unit tests and bootstrap packet smoke |
| AC-02 | F1.S1.T1, F2.S1.T1 | duplicate-key unit test and bootstrap packet assertions |
| AC-03 | F2.S1.T1 | existing local movement tests plus the remote packet extension |
| AC-04 | F2.S1.T2 | session cleanup assertions and runtime cleanup test |
| AC-05 | F1.S1.T1, F1.S1.T2, F2.S1.T1, F2.S1.T2 | targeted `cargo test` output |
| AC-06 | F3.S1.T1 | validation evidence: Validation Gate F7 |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260521-2330-world-remote-player-sync`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay entities`
- Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app client_runtime`
- Gate F3: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime`
- Gate F4: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Gate F5: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Gate F6: `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
- Gate F7: `rtk rg -n \"remote player|movement packets|logout|disconnect\" docs/player-rust-client.md port_rust/README.md .codexpotter/kb`
- Gate F8: `e2e-validator` handoff for the visible remote-marker movement path once the bootstrap routing is wired

## Risks

- The runtime must avoid duplicating remote players when the same key appears
  in multiple packets.
- Remote markers should remain cheap to update so the world scene does not
  regress into unnecessary respawns.
- Logout/disconnect cleanup must not clear the loaded world bundle or local
  player state.

## Rollback

- Remove the remote-player insertion/update helper and restore the prior
  local-only movement handling if the new packet path destabilizes the world
  projection.
- Keep cleanup scoped to the remote roster so the world bundle can survive a
  rollback of this slice.
