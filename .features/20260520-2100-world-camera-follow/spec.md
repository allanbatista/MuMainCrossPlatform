---
status: READY_FOR_PLAN
---

# World Camera Follow

## Goal

Keep the visible world route centered on the local avatar with a follow camera
so movement stays readable and playable after the scene loads, while leaving
headless, control-http, and network handoff behavior unchanged.

## Users And Journeys

- Player/dev starts `mu_client` normally, reaches the world route, and the
  camera keeps the local avatar framed while moving.
- QA/dev runs `mu_client --headless`; ready-for-login output stays
  deterministic.
- QA/dev runs `mu_client --headless --control-http 127.0.0.1:0`; state
  inspection and command handling stay stable.
- Player/dev on invalid world data still sees the existing loading/error
  behavior; the camera change only applies once the world shell is active.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| World route camera | `mu_client` world route | `world-camera-follow` | Cliente MU | Bevy 3D scene camera | current UI route, local player pose | local converted assets and `ClientRuntime` world projection | the camera stays dormant until the world route and local avatar exist; route cleanup still despawns the scene | player/dev see the camera stay framed on the avatar; headless/dev smoke is unchanged |

## Requirements

- The world camera must follow the local avatar on the world route.
- The world scene must still spawn and clear with route changes.
- Headless and control-http behavior must not change.
- Camera tracking must be deterministic and covered by automated tests.
- Usage docs must explain the visible follow-camera behavior and its current
  limits.

## Acceptance Criteria

- AC-01. The graphical world route keeps the local avatar framed while the
  player moves.
- AC-02. The camera continues to spawn and clear with the world scene on
  route changes.
- AC-03. `mu_client --headless` still prints `ready-for-login`.
- AC-04. `mu_client --headless --control-http 127.0.0.1:0` still serves the
  same state/control endpoints.
- AC-05. Tests cover the camera follow path and the world-scene lifecycle.
- AC-06. Player-facing docs mention the follow camera and its current limits.

## Scope

In scope:

- World-scene camera tracking in `mu_app`.
- Wiring the camera to the existing runtime projection for the local avatar.
- Usage docs and validation updates.

Out of scope:

- Orbital mouse controls.
- Persisted camera zoom / F10 / F11 camera workflow.
- Full camera preset parity beyond the visible follow behavior.
- Network protocol changes.

## Open Questions

None blocking. Assumption: a fixed follow offset anchored to the seeded local
avatar is acceptable for this slice and keeps the runtime stable without
waiting for orbital camera input work.
