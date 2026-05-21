---
status: READY_FOR_PLAN
---

# World Camera Zoom

## Goal

Make the world-route camera respect the persisted `[Camera] zoom` value and
mouse wheel input so the player can frame the scene closer or farther while
keeping the existing world shell, headless mode, and control-http behavior
stable.

## Users And Journeys

- Player/dev launches `mu_client`, reaches the world route, and the camera
  starts at the saved zoom distance instead of a fixed follow offset.
- Player/dev scrolls the mouse wheel on the world route and the camera zooms
  in or out within the legacy clamp range.
- Player/dev exits the graphical client and the updated zoom persists back to
  `config/client.toml`.
- QA/dev still gets the same headless and control-http smoke behavior.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| World route camera zoom | `mu_client` world route | `world-camera-zoom` | Cliente MU | Bevy 3D scene camera + config persistence | current UI route, mouse wheel, saved config | local `config/client.toml` and world projection | camera zoom is ignored outside the world route; missing config falls back to defaults | player/dev can adjust zoom in the world; headless/dev smoke stays unchanged |

## Requirements

- The world camera must scale from the persisted zoom value instead of a fixed
  distance.
- Mouse wheel input on the world route must change the active zoom.
- Zoom changes must persist back to `config/client.toml`.
- Zoom values must stay clamped to the legacy `600..3000` range.
- Headless, control-http, and route cleanup behavior must remain unchanged.
- Tests must cover the zoom math and the config round-trip path.
- Usage docs must mention the camera zoom control and persistence.

## Acceptance Criteria

- AC-01. The world route camera starts at the saved zoom distance when the
  world scene loads.
- AC-02. Mouse wheel input in the world route changes the zoom and the camera
  follows the new distance.
- AC-03. Exiting the graphical client persists the new zoom to
  `config/client.toml`.
- AC-04. `mu_client --headless` still prints `ready-for-login`.
- AC-05. `mu_client --headless --control-http 127.0.0.1:0` still serves the
  same state/control endpoints.
- AC-06. Unit tests cover the zoom scale, clamping, and config round-trip.
- AC-07. Player-facing docs mention the zoom control and its persistence.

## Scope

In scope:

- World-scene camera zoom scaling.
- Graphical config load/save for the camera setting.
- World-route mouse wheel zoom handling.
- Usage docs and KB updates.

Out of scope:

- Full orbital camera rotation / pitch controls.
- F10 zoom lock and F11 reset behavior.
- Network protocol changes.
- Any camera preset overhaul beyond the world-route zoom path.

## Open Questions

None blocking. Assumption: the follow camera should preserve its current
angle and scale its distance proportionally from the saved zoom value, which
keeps the visible framing simple while avoiding a full orbital-camera pass.
