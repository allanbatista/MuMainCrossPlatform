# World Terrain Ground Follow

Status: READY_FOR_PLAN

## Goal

Use the loaded world terrain heightfield to visually ground the local and
remote player markers in the world route so they rest on the visible surface
instead of hovering at a fixed height. This is a visible parity improvement to
the existing world shell, not full collision or physics.

## Users And Journeys

- Player/developer: starts `mu_client` normally, reaches the world route, and
  sees the player markers rest on the terrain surface instead of floating.
- QA/developer: runs `mu_client --headless`; existing headless state output
  remains deterministic.
- QA/developer: runs `mu_client --headless --control-http 127.0.0.1:0`; state
  inspection and exit command handling remain unchanged.
- Player/developer on invalid or missing world data: the current loading/error
  surfaces still gate entry to the world shell.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| World route in `mu_client` | `UiRoute::World` | `world-terrain-ground-follow` | Cliente MU | Bevy 3D world scene with grounded player markers | current UI route, loaded world bundle, local/remote marker transforms | converted world bundle heightfield data only | missing or invalid world data still stays on the existing loading/error path; marker grounding only happens after the world is ready | players see grounded markers; QA/dev smoke stays unchanged |
| Headless / control smoke | `mu_client --headless`, `--control-http` | `headless-control` | ready-for-login / control-http | stdout + local HTTP state | command name | app/session state | unchanged deterministic behavior | QA/dev automation only |

## Requirements

- Ground only the local and remote player markers that currently use fixed
  Bevy marker meshes.
- Derive marker height from the loaded terrain bundle, using the same visible
  world surface that drives the terrain mesh.
- Preserve object, NPC, and monster rendering and world cleanup behavior.
- Keep headless and control-http smoke behavior unchanged.
- Every code change must have automated tests.
- Usage docs must explain the grounded-marker behavior and its limit: this
  slice does not add full collision or movement physics.

## Acceptance Criteria

- AC-01. In normal `mu_client`, the world route still reaches the visible
  shell and the local/remote player markers rest on the terrain surface.
- AC-02. Grounding uses the loaded terrain bundle and does not change the
  object, NPC, or monster scene rendering paths.
- AC-03. `mu_client --headless` still prints `ready-for-login`.
- AC-04. `mu_client --headless --control-http 127.0.0.1:0` still serves the
  same endpoints.
- AC-05. Automated tests cover terrain sample lookup and grounded marker
  transforms.
- AC-06. Usage docs mention the grounded world markers and the fact that full
  collision remains out of scope.

## Scope

In scope:

- Terrain sample lookup from the loaded world bundle.
- Grounding local and remote player marker transforms in the world scene.
- Unit tests for the terrain sampler and marker anchoring.
- Usage docs and KB notes for the grounded-marker behavior.

Out of scope:

- Full collision or physics.
- Player model replacement.
- Movement or network protocol changes.
- Object/NPC/monster visual changes.
- New assets or bundle formats.

## Open Questions

None blocking. Assumption: this slice uses the existing visible terrain
surface as the ground reference and only adjusts marker placement in the
world scene.
