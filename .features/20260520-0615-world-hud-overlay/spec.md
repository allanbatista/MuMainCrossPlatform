# World HUD Overlay

Status: READY_FOR_PLAN

## Goal

Render the legacy world HUD main frame in the Bevy runtime while the client is
in the world route. The first slice should expose a visible overlay driven by
the existing `mu_ui::hud_screen` snapshot data and keep it hidden outside the
world scene.

## Users And Journeys

- Player: enters the world and sees the HUD main frame with gauges and buttons
  on top of the 3D scene.
- QA/developer: verifies the HUD appears only after the world route is active
  and clears on route changes or world reset.
- QA/developer: keeps headless/control-http behavior unchanged.

## Requirements

- World load plus an active world route are required before the overlay is
  spawned.
- The HUD overlay uses `mu_ui::hud_screen` as its source model and preserves
  the legacy gauges, buttons, layout, and widget-group data.
- The overlay is visible in graphical mode and hidden when leaving the world
  route or when the world projection is cleared.
- Headless and control-http smoke remain deterministic.
- Every code change has automated tests.
- Usage docs mention the visible HUD overlay.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| World HUD main frame | `UiRoute::World` / `mu_ui::HudScreen` | `world-hud-overlay` | HUD | Bevy overlay card | world route, world-ready state | loaded world bundle, HUD snapshot | hidden outside world route or before world projection | players see the overlay; QA scripts keep smoke deterministic |

## Acceptance Criteria

- AC-01. The graphical world route shows a visible HUD overlay.
- AC-02. The overlay is hidden outside the world route and after world reset.
- AC-03. Automated tests cover spawn, cleanup, and HUD body formatting.
- AC-04. Player-facing docs describe the HUD overlay.
- AC-05. Existing headless/control-http smoke remains unchanged.

## Scope

In scope:

- World-route HUD overlay rendering.
- HUD snapshot formatting for the world main frame.
- Cleanup when leaving the world route.
- Usage docs for the visible HUD state.

Out of scope:

- Full chat, minimap, and hotkey bar parity.
- HUD toggle/keybinding work.
- New protocol or storage changes.
- Gameplay changes beyond the visible overlay.

## Open Questions

None blocking. Assumption: the first HUD slice is a compact overlay that
surfaces the legacy main-frame state without reworking the rest of the world UI
stack.
