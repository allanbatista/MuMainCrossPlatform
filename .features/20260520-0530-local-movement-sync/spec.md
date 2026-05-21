# Local Movement Sync

Status: READY_FOR_PLAN

## Goal

Make the loaded world interactive by seeding a visible local avatar and moving
it with the core movement bindings in the Bevy world. This is the first
client-side gameplay loop after login/world bootstrap. It keeps the runtime
state and rendered scene aligned, but it does not add game-server authority
yet.

## Users And Journeys

- Player/developer: loads a world, sees a local avatar marker at spawn, and
  presses WASD to move it in the visible world.
- QA/developer: runs the graphical client, confirms movement is ignored outside
  the world route, and checks that reset/disconnect returns to a safe static
  state.
- QA/developer: runs headless/control-http smoke as before; movement does not
  alter the deterministic boot path.

## Requirements

- World load seeds a default local player when no server-driven avatar exists.
- Movement input only affects the local avatar while the world route is active
  and the world projection is ready.
- The rendered local marker must follow local pose updates.
- Reset/disconnect clears the local avatar with the rest of the world
  projection.
- Headless and control-http behavior remain deterministic.
- Every code change has automated tests.
- Usage docs mention the first interactive world controls.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Local world movement | `UiRoute::World` | `world-movement` | World | Bevy window + local avatar marker | Movement keys, local avatar pose | Loaded world bundle, seeded local avatar, runtime projections | Movement is ignored outside the world route, during loading, and after reset/disconnect | Players move the avatar; QA scripts deterministic key/input state; developers keep the smoke path stable |

## Acceptance Criteria

- AC-01. A loaded world shows a visible local avatar marker.
- AC-02. Holding the movement keys updates the local player pose and the
  rendered marker moves accordingly.
- AC-03. Movement input is ignored outside the world route and after world
  reset/disconnect.
- AC-04. Automated tests cover avatar seeding, movement pose updates, and scene
  sync.
- AC-05. Player-facing docs describe the movement controls and note that the
  slice is local-only.
- AC-06. Existing headless/control-http smoke remains unchanged.

## Scope

In scope:

- Default local avatar seeding on world entry.
- World-route movement input and local pose updates.
- Scene marker reconciliation for the local avatar.
- Usage docs for the first interactive controls.

Out of scope:

- Game-server authoritative movement.
- Collision, pathfinding, combat, chat, inventory, trade, GameShop, and MU
  Helper.
- Camera follow polish beyond what is needed to verify movement.
- Network protocol expansion beyond later server sync slices.

## Open Questions

None blocking. Assumption: until game-server movement packets exist, the local
avatar uses a placeholder spawn at the world origin and client-side motion
only.
