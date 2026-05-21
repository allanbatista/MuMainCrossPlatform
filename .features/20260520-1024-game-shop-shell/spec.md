# Game Shop Shell

Status: READY_FOR_PLAN

## Goal

Render the GameShop route in the Bevy client using the existing gameplay
state, and expose a dedicated local control-http smoke command so developers
can inspect the cash-shop flow without touching the network protocol. The
first slice should keep the current boot, world, inventory, NPC/shop, party,
gate, quests, chat, and trade behavior unchanged while adding a route-gated
GameShop shell that renders the existing `mu_ui` snapshot.

## Users And Journeys

- Player/developer opens the GameShop route and sees the cash-shop shell.
- QA/developer drives the runtime with `--control-http` and switches to
  GameShop with a dedicated command for smoke testing.
- Maintainer validates that the shell only appears on the GameShop route and
  clears cleanly on route changes or disconnect/reset.

## Requirements

- The GameShop shell is visible only when the GameShop route is active and
  the runtime is not disconnected.
- The shell uses the existing `mu_ui::game_shop_screen()` snapshot as its
  visible source model.
- The shell clears when the route changes away from GameShop or when the
  session disconnects.
- The local control-http API can switch the runtime into the GameShop route
  for smoke testing without colliding with the existing NPC shop command.
- Every code change has automated tests.
- Usage docs mention the GameShop route shell and smoke command.
- Existing headless/control-http smoke remains deterministic.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| GameShop shell | `UiRoute::GameShop` | `game-shop` | GameShop | Bevy route shell | GameShop route, session phase, game shop manager mode | local runtime GameShop snapshot only; no auth or tenant data | hidden outside GameShop route or on disconnect; empty state shows the current no-items notice; error state shows the sync error; loading keeps the previous shell until the manager snapshot is ready | players see the same shell as QA/dev; only dev/QA use the control-http smoke command |

## Acceptance Criteria

- AC-01. The graphical GameShop route shows a visible cash-shop shell.
- AC-02. The shell is hidden outside the GameShop route and after
  disconnect/reset.
- AC-03. Automated tests cover spawn, cleanup, and snapshot body formatting.
- AC-04. Player-facing docs describe the GameShop route shell and smoke path.
- AC-05. Existing headless/control-http smoke remains unchanged when the new
  route is unused.

## Scope

In scope:

- Route-gated GameShop shell rendering.
- Control-http route command for GameShop smoke.
- Cleanup when leaving the GameShop route.
- Usage docs for the new gameplay surface.

Out of scope:

- Actual cash-shop transaction flow changes.
- New network protocol or persistence changes.
- Route changes outside the GameShop shell surface.

## Open Questions

None blocking. Assumption: the local smoke command should be `game-shop` so
it stays distinct from the existing NPC shop `shop` command.
