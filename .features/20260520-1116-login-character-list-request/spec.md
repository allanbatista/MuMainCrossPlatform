# Login Character List Request

Status: READY_FOR_PLAN

## Goal

After login success, the Rust bootstrap worker should immediately request the
character list from the game server, using the legacy language byte derived
from the client locale. This keeps the login -> character select handoff
faithful to the C++ flow and makes the fake-server smoke path more realistic
without changing the visible route ladder.

## Users And Journeys

- QA/developer runs the fake-server bootstrap smoke and sees the client ask
  for the character roster after login success.
- Player/developer keeps the current login/server-select/character-select
  surfaces, but the network bootstrap now matches the legacy request chain
  more closely.
- QA/developer can still drive the same control-http states without changing
  the headless smoke behavior.

## Requirements

- A login-success event must trigger `request_character_list` on the active
  session worker.
- The language byte must come from the configured locale using the legacy
  mapping.
- The fake-server bootstrap tests must expect the new request.
- Existing headless and control-http smoke behavior must remain deterministic
  when the flow is unused.
- Every code change must have automated tests.
- Usage docs must mention the automatic roster request after login success.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Character roster request | `UiRoute::Login` -> `UiRoute::CharacterSelect` | `login-roster-request` | Character list request | Network bootstrap packet | locale byte, login success | session bootstrap only | if login fails, stay on login; if the request cannot be sent, land on a safe error surface | QA observes the request through fake-server smoke |

## Acceptance Criteria

- AC-01. Login success sends a character-list request before the bootstrap
  advances to character select.
- AC-02. The request uses the legacy language byte for the active locale.
- AC-03. Automated tests cover the request send path and the fake-server
  expectation.
- AC-04. Player-facing docs explain the automatic roster request.
- AC-05. Existing headless/control-http smoke stays deterministic when the
  login flow is unused.

## Scope

In scope:

- Automatic character-list request after login success.
- Legacy locale byte mapping for the request.
- Fake-server coverage for the new packet order.
- Usage docs for the bootstrap handoff.

Out of scope:

- Character selection UI input.
- Login credential persistence or new auth storage.
- New network protocol packets.
- World rendering changes.

## Open Questions

None blocking. Assumption: the locale byte follows the legacy mapping
`en/eng -> 0`, `pt/por -> 1`, `es/spn -> 2`, defaulting to `0`.
