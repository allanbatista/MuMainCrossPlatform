# Party Leave Control Plane Spec

Status: READY_FOR_PLAN

## Goal

Expose the legacy party leave/kick packet through the Rust control plane so
QA can drive the current party row from HTTP and the runtime can queue the
matching `0x43` request.

## Users And Journeys

- QA/developer opens the graphical client while logged in and inspects the
  party route.
- The control plane accepts a `party-leave` command with the party member
  number to target.
- The graphical runtime forwards that selection to the bootstrap worker.
- The bootstrap worker sends the legacy party leave/kick packet for that
  member.
- If the server answers with the party leave signal, the party snapshot
  clears on the next bootstrap pass as before.

## Requirements

- `ControlCommand` must accept a party leave command name and payload.
- The control snapshot must carry the chosen party member number so `/state`
  reflects the request target.
- The graphical runtime must queue the legacy party leave packet through the
  bootstrap worker.
- The bootstrap worker must encode the existing social packet helper.
- The command must be documented in the player docs, control-http docs, and
  party UI notes.
- Tests must cover request parsing, runtime queueing, and packet bytes.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Party leave control | `UiRoute::Party` | `party-leave-control-plane` | Party Leave | HTTP control command + legacy packet bridge | logged-in party target member number | live session only | missing payload returns 400; successful request leaves/kicks the chosen party member | QA/dev can drive the existing leave control without the legacy UI |

## Scope

In scope:

- `party-leave` control-http command and payload parsing;
- runtime queueing for the party leave packet;
- bootstrap packet encoding and request tests;
- docs and KB updates.

Out of scope:

- new party selection widgets;
- invite-response handling;
- broader party chat or loot flows;
- server-side protocol changes.

## Acceptance Criteria

- AC-01. `POST /command?name=party-leave&member_number=<u8>` is accepted when
  logged in and updates the control snapshot target.
- AC-02. The graphical runtime queues the legacy party leave packet for the
  requested member number.
- AC-03. The bootstrap worker sends the expected `0x43` packet bytes.
- AC-04. Usage docs mention the new control-http command.
- AC-05. Automated tests cover request parsing, runtime queueing, and packet
  encoding.

## Open Questions

None blocking. Assumption: the legacy UI semantics stay the same, so the
control-plane command targets the party member number shown in the roster.
