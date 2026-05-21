# World Remote Player Sync

Status: READY_FOR_PLAN

## Goal

Movement packets for non-local player keys should create or update remote
player markers in the world projection instead of being ignored. The world
scene should keep rendering those markers in place, and logout/disconnect
should clear the remote roster so stale markers do not survive session loss.

## Users And Journeys

- QA drives the fake-server smoke and sees remote markers appear and move when
  movement packets arrive for keys other than the local player.
- Player/developer keeps the existing local movement behavior and world scene
  intact while remote markers now track live packets instead of staying
  fixture-only.
- When the session logs out or disconnects, the remote marker roster is cleared
  without tearing down the loaded world bundle.

## Requirements

- The bootstrap worker must route non-local movement updates into the runtime
  world projection.
- `WorldEntitiesManager` must support keyed remote-player lookup/update so
  repeated packets update the same marker instead of duplicating it.
- New remote markers should use a stable placeholder label/model when no roster
  packet has provided richer metadata yet.
- Local movement reconciliation must keep working exactly as before.
- Logout/disconnect must clear remote markers while leaving the loaded world
  state alone.
- Automated tests must cover keyed remote upserts, bootstrap packet handling,
  and remote-roster cleanup.
- Usage docs and KB notes must describe the live remote-player sync path.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| World remote markers | `UiRoute::World` | `remote-player-movement-sync` | World | Bevy window with remote player markers | Remote movement packet key, world route, session state | Loaded world bundle, live session bridge, runtime projections | Remote markers appear only after a non-local movement packet arrives; logout/disconnect clears the remote roster while leaving the world bundle intact | Players see live remote movement in the world; QA fake-server smokes the packet path; developers keep local movement and boot behavior unchanged |

## Interfaces / Contracts

- `mu_gameplay::entities::WorldEntitiesManager` gains keyed remote-player
  helpers for lookup and replacement.
- `mu_app::client_runtime::ClientRuntime` gains remote-player update and
  cleanup helpers that keep `render_entities` synced.
- `mu_app::bootstrap_runtime::apply_movement_update()` routes non-local
  movement packets into the remote-player update path.
- `mu_app::bootstrap_runtime` clears remote markers on logout/disconnect.
- `docs/player-rust-client.md`, `port_rust/README.md`, and the KB notes
  describe the new live remote-player behavior.

## Scope

In scope:

- Keyed remote-player update helpers in gameplay/runtime state.
- Bootstrap routing for remote movement packets.
- Remote-roster cleanup on logout/disconnect.
- Tests, docs, and KB updates.

Out of scope:

- New roster packets or character metadata sync.
- Manual remote-player editing controls.
- Changes to the world scene renderer beyond using the updated catalog.

## Acceptance Criteria

- AC-01. A movement packet for a non-local key creates a remote marker when
  no existing marker is present.
- AC-02. Repeated movement packets for the same remote key update the existing
  marker instead of adding duplicates.
- AC-03. Local movement reconciliation continues to update the local player as
  before.
- AC-04. Logout/disconnect clears remote markers while preserving the loaded
  world bundle.
- AC-05. Automated tests cover the keyed update path, the bootstrap packet
  path, and the cleanup path.
- AC-06. Usage docs and KB notes mention the live remote-player sync behavior.

## Open Questions

Assumption: a stable placeholder such as `Remote <key>` plus the existing
`remote-player` model label is sufficient until a future roster packet supplies
real remote-player metadata.
