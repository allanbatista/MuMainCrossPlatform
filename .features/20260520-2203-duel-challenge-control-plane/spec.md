# Duel Challenge Control Plane

Status: READY_FOR_PLAN

## Goal

Expose the existing duel packet helpers as a local control HTTP slice so QA can
submit a duel start or stop action without touching the wider game UI. The
first slice should keep the duel shell unchanged while surfacing the live
request bridge through `mu_app` and the bootstrap worker.

## Users And Journeys

- QA opens the duel route and sends a duel start payload from `--control-http`.
- QA can cancel or stop the duel request through the same control surface.
- The runtime queues the matching duel packet when a live session is present.

## Requirements

- The control HTTP API accepts `duel-start` with a duel target player id and
  player name.
- The control HTTP API accepts `duel-stop` without extra payload.
- The control snapshot records the duel start parameters so the runtime bridge
  can queue the packet.
- The bootstrap worker sends the existing `duel_start_request()` and
  `duel_stop_request()` helpers.
- Every code change has automated tests.
- Usage docs mention the new duel control-plane commands.

## Non-Functional Requirements

- The duel commands stay local-control only; they do not add a remote duel API
  or new persistence.
- If the bootstrap sender is unavailable, the snapshot can still update but no
  packet should be emitted.
- The bridge should not block the runtime loop; packet send work stays on the
  bootstrap worker path.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/unavailable behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Duel smoke control | local control HTTP | `duel-start`, `duel-stop` | Duel challenge actions | HTTP route command | command name, duel target params | local smoke only | unavailable when control HTTP is disabled; commands update the runtime snapshot and queue the live packet when a bootstrap sender exists | QA and developers can drive the duel challenge bridge without a real UI click path |
| Duel shell | `UiRoute::Duel` | `duel` | Duel | Visible Bevy route shell | session connected, duel manager state | existing duel manager only | hidden outside the duel route or on disconnect; challenge/channel-list/watching/error states still come from the duel manager | Players and QA can still open the duel window |

## Acceptance Criteria

- AC-01. `POST /command?name=duel-start&player_id=...&player_name=...`
  updates the control snapshot and queues the duel start packet in the live
  bootstrap path.
- AC-02. `POST /command?name=duel-stop` updates the control snapshot and
  queues the duel stop packet in the live bootstrap path.
- AC-03. Automated tests cover command parsing, snapshot routing, and packet
  queue/send behavior.
- AC-04. Player-facing docs describe the duel challenge control commands.

## Scope

In scope:

- Local control HTTP command parsing for duel start/stop.
- Bootstrap runtime queue/send wiring for the existing duel packet helpers.
- Usage docs and KB notes for the control-plane slice.

Out of scope:

- New duel gameplay rules, matchmaking, or UI interaction flows.
- Duel channel join/quit or other duel menu actions.
- New persistence or protocol changes for duel state.

## Open Questions

None blocking. Assumption: the first slice exposes the simplest live duel
bridge first, not the full duel interaction matrix.
