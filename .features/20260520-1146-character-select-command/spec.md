# Character Select Command

Status: READY_FOR_PLAN

## Goal

Replace the character-select auto-advance shortcut with an explicit
`select-character` command path so QA automation can choose a roster entry by
name and the bootstrap can continue to world entry from that request.

## Users And Journeys

- QA runs the graphical client with `--control-http`, reaches character select
  after login, posts `select-character` with a character name, and the client
  continues to loading/world.
- Maintainer drives the fake-server smoke and sees the client stay on character
  select until a selection command is issued.
- Future UI work can reuse the same request path for keyboard or surface-driven
  selection without changing the bootstrap packet helper again.

## Requirements

- The bootstrap worker must stop auto-selecting the first usable roster entry.
- A manual selection request must queue `select_character` with the requested
  character name through the live session.
- The control HTTP API must accept `select-character` with a character name
  payload and reject empty requests.
- If no selection is issued, the client stays on character select instead of
  advancing blindly.
- The existing login/server-select/character-select route ladder and world
  handoff stay intact.
- Usage docs and KB notes must explain the new command and the waiting
  behavior.
- Automated tests must cover the bootstrap request path and the control HTTP
  command surface.

## Interfaces / Contracts

- `mu_app::bootstrap_runtime::BootstrapRuntime` gains a manual
  character-selection queue method.
- `mu_app::bootstrap_runtime` no longer sends `select_character` directly when
  the roster packet arrives.
- `mu_app::control_http::ControlSnapshot` tracks the last requested character
  name for the control-plane command.
- `mu_app::graphical_runtime` bridges the control-plane request into the
  bootstrap worker.
- `docs/player-rust-client.md`, `port_rust/README.md`, and
  `port_rust/docs/control-http.md` document the API usage.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Character select wait state | `UiRoute::CharacterSelect` | `character-select-wait` | Character Select | Auth shell wait state | logged-in roster, explicit selection request | local bootstrap session only | stays visible until a selection command arrives; if selection is missing, the client does not advance | QA uses the control plane first; players still see the same auth surface |
| Select character command | `POST /command` | `select-character` | Select Character | HTTP control command | character name payload | local control-http access | empty payload returns 400; failed queue keeps the client on character select | QA can drive the command through HTTP; future UI input can reuse the same bootstrap path |

## Scope

In scope:

- bootstrap worker selection queueing;
- `select-character` control HTTP command and payload parsing;
- fake-server bootstrap coverage for the new flow;
- usage docs and KB updates.

Out of scope:

- on-screen keyboard/mouse navigation for the character-select surface;
- character creation/deletion flows;
- roster browsing or character preview UI changes.

## Acceptance Criteria

- AC-01. The bootstrap worker no longer auto-selects a character when the
  roster packet arrives.
- AC-02. A `select-character` request queues the matching
  `select_character` packet through the live session.
- AC-03. Empty `select-character` requests return a bad-request response and
  do not advance the session.
- AC-04. Fake-server smoke reaches world only after the selection command is
  issued.
- AC-05. Usage docs and KB note the new command and the wait-on-select
  behavior.

## Open Questions

Assumption: the control-plane command will accept the character name in the
request body or a `character=` query parameter, whichever is present first.
