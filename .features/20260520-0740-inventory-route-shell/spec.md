# Inventory Route Shell

Status: READY_FOR_PLAN

## Goal

Make the inventory route visible from the world route with the existing Tab
binding. Pressing Tab in the world should open a Bevy inventory shell, and
pressing Tab again should return to the world route. The headless smoke path
must stay deterministic when inventory is unused.

## Users And Journeys

- Player/developer: loads a world, presses Tab, sees an inventory shell, then
  presses Tab again to return to the world.
- QA/developer: confirms inventory does not open during loading or after a
  disconnect/reset and that the shell clears cleanly when the route changes.
- QA/developer: runs the headless/control-http smoke path and sees the same
  deterministic boot behavior when inventory is not used.

## Requirements

- Tab opens inventory only from the active world route while the world is
  ready.
- Tab closes inventory back to the world route.
- The inventory shell renders a visible Bevy surface driven by
  `mu_ui::inventory_screen()`.
- Inventory is ignored outside the world route and after reset/disconnect.
- Every code change has automated tests.
- Usage docs mention the inventory toggle and the inventory shell.
- Existing headless/control-http smoke remains unchanged.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Inventory route shell | `UiRoute::Inventory` | `inventory` | Inventory | Bevy window + inventory shell | Tab toggle, world route, session state | Loaded world bundle and runtime route state | Inventory is ignored outside the world route and clears on disconnect/reset | Players can open and close the inventory; QA can verify route transitions; developers keep boot smoke stable |

## Acceptance Criteria

- AC-01. Pressing Tab in the world opens the inventory shell.
- AC-02. Pressing Tab again returns to the world route.
- AC-03. Inventory input is ignored outside the world route and after
  disconnect/reset.
- AC-04. Automated tests cover the route toggle and shell visibility.
- AC-05. Player-facing docs describe the Tab inventory toggle.
- AC-06. Existing headless/control-http smoke remains unchanged.

## Scope

In scope:

- World-route inventory toggle via the existing Tab binding.
- Inventory shell rendering for the visible route.
- Cleanup when leaving the inventory route.
- Usage docs for the first inventory toggle.

Out of scope:

- Actual inventory item binding, movement, split, use, drop, store, or vault
  persistence.
- Equipment and vault interactions beyond the visible shell.
- Network protocol expansion for inventory transactions.

## Open Questions

None blocking. Assumption: for this slice, inventory is a full-screen route
shell rather than a modal overlay on top of the world.
