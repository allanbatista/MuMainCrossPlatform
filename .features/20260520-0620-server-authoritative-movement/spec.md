# Server Authoritative Movement

Status: READY_FOR_PLAN

## Goal

Make world movement server-authoritative when the gameplay session is live.
WASD should queue movement through the session bridge, and the local avatar
pose should be corrected by authoritative move updates from the server. The
offline smoke path must remain deterministic when no live session is present.

## Users And Journeys

- Player/developer: loads a world, presses WASD, and sees the local avatar stay
  in sync with the committed server position.
- QA/developer: runs the graphical client against a fake server and confirms
  movement requests are sent and committed positions are applied.
- QA/developer: runs the headless/control-http smoke path and sees the same
  deterministic boot behavior when no gameplay session is connected.

## Requirements

- World-route movement can send a walk request through the live session bridge.
- Authoritative move-position and move-character updates apply to the local
  avatar pose.
- The rendered local marker follows the authoritative runtime pose.
- Movement stays ignored outside the world route and after reset/disconnect.
- Offline/headless boot remains deterministic when no session bridge exists.
- Every code change has automated tests.
- Usage docs mention the server-authoritative movement flow and the offline
  fallback.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| World movement authority | `UiRoute::World` | `world-movement-authority` | World | Bevy window + local avatar marker | Movement keys, session state, authoritative pose | Loaded world bundle, live session bridge, runtime projections | Movement is ignored outside the world route, during loading, and after reset/disconnect | Players move the avatar; QA scripts validate fake-server round trips; developers keep smoke stable |

## Acceptance Criteria

- AC-01. A live world session accepts movement requests from the world route.
- AC-02. Authoritative movement replies update the local player pose and the
  rendered marker.
- AC-03. Movement input is ignored outside the world route and after world
  reset/disconnect.
- AC-04. Automated tests cover request encoding, packet decode, runtime sync,
  and a fake-server round trip.
- AC-05. Player-facing docs describe the movement controls and note the
  authoritative server sync.
- AC-06. Existing headless/control-http smoke remains unchanged.

## Scope

In scope:

- Session-bridged world-route movement requests.
- Server move-position / move-character packet handling.
- Local avatar pose reconciliation after authoritative updates.
- Usage docs for the first network-backed movement loop.

Out of scope:

- Collision, pathfinding, combat, chat, inventory, trade, GameShop, and MU
  Helper.
- Camera follow polish beyond what is needed to verify movement.
- Network protocol expansion beyond the movement round-trip.

## Open Questions

None blocking. Assumption: the first authoritative slice can keep the existing
offline/local smoke path as a fallback when the live session bridge is absent.
