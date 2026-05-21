# World Terrain Lightmap

Status: READY_FOR_PLAN

## Goal

Use the bundled terrain lightmap in the graphical world scene so the visible
terrain picks up the baked legacy lighting data instead of staying on the
current flat-looking surface, while keeping the existing route-driven world
startup and cleanup behavior unchanged.

## Users And Journeys

- Player/developer runs the graphical client and sees the world terrain with
  the baked lightmap applied.
- QA/developer runs `mu_client --headless`; boot and control-http behavior stay
  unchanged.
- Maintainer validates the slice locally with Cargo tests and the existing
  graphical smoke.

## Product Inventory

| Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|
| World route in `mu_client` | `world-terrain-lightmap` | `World Terrain Lightmap` | Bevy 3D terrain surface with baked lighting overlay | current UI route, loaded world bundle, lightmap asset availability | local converted terrain bundle only; no auth or tenant data | missing lightmap falls back to the current terrain surface without blocking the world route; leaving the route still clears the scene | player/dev/QA all see the same world surface; dev/QA use headless/control-http smoke |

## Requirements

- Use the bundle lightmap when the world scene is spawned.
- Resolve the actual asset path safely enough for the checked-in sample data.
- Keep the current route gating and scene cleanup behavior unchanged.
- Preserve the solid fallback when the lightmap asset cannot be resolved.
- Document the visible behavior and the current limit of this slice.

## Acceptance Criteria

- AC-01. The graphical world route renders the terrain with the shipped
  lightmap applied on top of the existing terrain surface.
- AC-02. Leaving the world route still clears the world scene and re-entering
  the route rebuilds it cleanly.
- AC-03. `mu_client --headless` still reports `ready-for-login`.
- AC-04. `mu_client --headless --control-http 127.0.0.1:0` still serves the
  same state/control endpoints.
- AC-05. Tests cover lightmap path resolution and the world-scene lifecycle.
- AC-06. Player-facing docs mention the lightmap-backed terrain and its
  fallback behavior.

## Scope

In scope:

- Lightmap resolution and terrain-surface wiring in `mu_app`.
- Usage docs and KB updates for the visible terrain change.

Out of scope:

- Dynamic terrain light updates.
- New shader work or a full terrain-renderer rewrite.
- Network protocol changes.
- Terrain physics, streaming, or LOD changes.

## Open Questions

None blocking. Assumption: using the converted `TerrainLight.png` asset as a
static world-terrain lighting overlay is an acceptable approximation for this
slice.
