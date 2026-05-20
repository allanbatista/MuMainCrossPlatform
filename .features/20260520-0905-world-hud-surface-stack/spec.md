# World HUD Surface Stack

Status: READY_FOR_PLAN

## Goal

Extend the visible world HUD overlay so the Bevy runtime also surfaces the
legacy chat log, minimap, and hotkey bar snapshots while the client is in the
world route. The first slice should keep the existing HUD main frame visible
and add the remaining world surface data in the same route-gated overlay.

## Users And Journeys

- Player: enters the world and sees the HUD frame plus the visible chat,
  minimap, and hotkey surfaces.
- QA/developer: confirms the extra world surfaces are only visible when the
  world route is active and are cleared on route changes or world reset.
- QA/developer: keeps headless/control-http behavior unchanged.

## Requirements

- World load plus an active world route are required before the world HUD
  stack is spawned.
- The overlay uses the existing `mu_ui::hud_screen`, `mu_ui::chat_screen`,
  `mu_ui::minimap_screen`, and `mu_ui::hotkeys_screen` snapshots as its source
  model.
- The world HUD stack is visible in graphical mode and hidden when leaving the
  world route or when the world projection is cleared.
- Headless and control-http smoke remain deterministic.
- Every code change has automated tests.
- Usage docs mention the visible world HUD stack.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| World HUD stack | `UiRoute::World` / `mu_ui::{HudScreen, ChatScreen, MiniMapScreen, HotkeysScreen}` | `world-hud-surface-stack` | World HUD | Bevy overlay card stack | world route, world-ready state | loaded world bundle, world HUD snapshots | hidden outside world route or before world projection | players see the world surfaces; QA scripts keep smoke deterministic |

## Acceptance Criteria

- AC-01. The graphical world route shows the HUD frame plus visible chat,
  minimap, and hotkey surfaces.
- AC-02. The overlay is hidden outside the world route and after world reset.
- AC-03. Automated tests cover spawn, cleanup, and the world surface body
  formatting.
- AC-04. Player-facing docs describe the visible world HUD stack.
- AC-05. Existing headless/control-http smoke remains unchanged.

## Scope

In scope:

- World-route HUD stack rendering.
- Snapshot formatting for chat, minimap, and hotkeys in the world overlay.
- Cleanup when leaving the world route.
- Usage docs for the visible world HUD stack.

Out of scope:

- Chat input, minimap interaction, hotkey handling, or route switching.
- New protocol or storage changes.
- Gameplay changes beyond the visible overlay.

## Open Questions

None blocking. Assumption: the first slice is a visible stacked overlay that
projects the legacy world surfaces without reworking their interaction model.
