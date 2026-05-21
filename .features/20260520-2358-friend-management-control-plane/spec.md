# Friend Management Control Plane

Status: READY_FOR_PLAN

## Goal

Add local control-http commands for friend management so QA can drive
`friend-add` and `friend-delete` through the live session bridge while the
existing friend shell, headless smoke, and route gating stay unchanged.

## Users And Journeys

- QA/developer opens the graphical client with `--control-http`, switches to
  the friend route, posts `friend-add`, and sees the bootstrap queue a
  `friend_add_request`.
- QA/developer posts `friend-delete` with a friend name and sees the matching
  delete request go out through the live session.
- QA/developer sends an empty friend action request and gets a bad-request
  response instead of an unintended packet.
- Maintainer keeps the existing headless/control smoke deterministic.

## Requirements

- `friend-add` and `friend-delete` must accept a friend name payload.
- Empty friend action requests must return bad-request and not queue packets.
- The bootstrap worker must send the matching friend packet helper through
  the live session bridge.
- The existing friend shell, route ladder, and headless smoke behavior must
  remain unchanged.
- Automated tests must cover command parsing, request validation, and packet
  queueing.
- Usage docs must mention the new friend-management control commands.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Friend management control plane | `POST /command?name=friend-add|friend-delete` | `friend-management-control-plane` | Friend | Local HTTP command + live session packet | friend name | local runtime only | empty payload returns bad-request; packet queue failure stays safe | QA/dev use the local command; players only see the route shell |

## Scope

In scope:

- friend-add/delete request plumbing;
- control-http request parsing and runtime bridge;
- tests, docs, and KB updates for the new local commands.

Out of scope:

- guild interaction commands;
- new on-screen friend UI widgets;
- network protocol changes.

## Acceptance Criteria

- AC-01. `friend-add` queues `friend_add_request` with the requested friend
  name.
- AC-02. `friend-delete` queues `friend_delete` with the requested friend
  name.
- AC-03. Empty friend action requests return bad-request and do not queue a
  packet.
- AC-04. Automated tests cover the control-plane parsing and queueing path.
- AC-05. Usage docs mention the new friend-management commands.

## Open Questions

None blocking. Assumption: the request body or `friend=` query parameter
contains the friend name, whichever arrives first.
