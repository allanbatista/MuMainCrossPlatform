# Guild Union Live Decoding

Status: READY_FOR_PLAN

## Goal

Make the guild union tab reflect the live alliance-list response from the
server instead of the static placeholder unions.

## Users / Journeys

- A logged-in player opens `guild-union`. After the live alliance list arrives,
  the union panel shows the decoded allied guilds and member counts.
- If the session logs out or disconnects before data arrives, the shell clears
  as it does today.

## Scope In

- Decode alliance-list responses from the live session.
- Persist decoded union roster snapshots in runtime state.
- Re-render the guild shell when decoded union state changes.
- Reset decoded union state on logout and disconnect.

## Scope Out

- Friend list and letter inbox payload decoding.
- New control-http commands.
- Guild member roster changes beyond the existing live guild list.

## Product Inventory

| Route / page | Slug / ID | User-visible label | Visual / output type | Filters / state | Required dataset / permission | Empty / loading / error / unavailable behavior | Persona differences |
|---|---|---|---|---|---|---|---|
| `guild` union tab | `guild-union` | Guild unions | union roster panel | no extra filters; unions are shown as received | live alliance-list response for the active session; logged-in player session required | before the first packet, the shell stays on the existing placeholder unions; logout/disconnect clears the decoded roster; malformed packets are ignored | none |

## Non-Functional Requirements

- Preserve the current boot/login/world flow.
- No new external dependencies.
- Cross-platform Rust implementation only.
- Keep the current shell smoke commands working.

## Acceptance Criteria

- AC-01: Alliance-list packets update the visible union roster using the
  legacy packet layout.
- AC-02: The guild union tab still renders in `mu_client` and clears on route
  change or disconnect.
- AC-03: Decoded union state resets on logout and session loss.
- AC-04: Unit tests cover packet decoding and shell overlay against live union
  snapshots.
- AC-05: Docs mention the live union sync behavior.

## Open Questions / Assumptions

- Assume `byResult == 1` means the alliance list is valid and any other result
  should clear the visible union roster.
- Assume the union packet only updates the union roster and leaves the member
  roster slice untouched.
