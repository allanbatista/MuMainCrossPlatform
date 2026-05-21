# Chat Shell

Status: READY_FOR_PLAN

## Goal

Render the `Chat` route as a visible Bevy shell in `mu_app` and expose a
deterministic `chat` control-http smoke command, using `mu_ui::chat_screen()`
as the source of truth for the visible snapshot.

## Users And Journeys

- Player/developer opens the chat route and sees a visible shell instead of an
  inert route entry.
- QA/developer drives the chat route with `--control-http` during local smoke
  tests.
- Maintainer validates the change with the existing Rust build/test flow.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Chat shell | `UiRoute::Chat` | `chat-shell` | Chat | Bevy window/card | route state | local UI snapshot only | clears when route/session is not visible | player/dev sees the route shell; QA can smoke it locally |

## Requirements

- The shell must render when `UiRoute::Chat` is active and the session is not
  disconnected.
- The shell body must stay snapshot-driven from `mu_ui::chat_screen()`.
- `--control-http` must accept `chat` and switch the runtime to the chat route
  for smoke testing.
- Existing login, world, inventory, and other gameplay shells must keep their
  current behavior.
- Documentation must mention the new route smoke path.

## Acceptance Criteria

- The chat shell spawns and clears correctly on route changes.
- `POST /command?name=chat` updates the control snapshot and the graphical
  runtime to the chat route.
- Usage docs mention the chat smoke path.
- Tests cover shell spawn/cleanup and the new control command.

## Scope

In scope:

- `mu_app` chat route shell.
- `control-http` `chat` command.
- Player/admin usage docs.

Out of scope:

- Interactive chat text entry.
- Chat packet send/receive integration.
- World HUD redesign.

## Open Questions

None blocking.
