# World Scene Shell

Status: READY_FOR_PLAN

## Goal

Make the Rust client show a visible Bevy world shell after the existing login
and world handoff path, instead of only changing routes. This slice should
render a real 3D scene with a camera, lighting, terrain, and sampled scene
markers from the loaded world bundle, while headless and control-http behavior
stay unchanged.

## Users And Journeys

- Player/dev: starts `mu_client` normally, reaches the world route, and sees a
  visible 3D scene instead of a blank shell.
- QA/dev: runs `mu_client --headless`; existing headless state output stays
  deterministic.
- QA/dev: runs `mu_client --control-http 127.0.0.1:0`; control inspection
  continues to work for automation.
- Player/dev on invalid world data: the existing loading/error surfaces remain
  in control; the new scene is only built once the world bundle is ready.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| World scene shell | `mu_client` world route | `world-scene-shell` | Cliente MU | Bevy 3D scene window | optional `--asset-root`, current world route | local converted assets, `ClientRuntime` world projection | if world data is missing or invalid, the existing loading/error surfaces remain; the scene is only built once the world bundle is ready | player/dev sees terrain and sampled scene markers; headless/dev smoke is unchanged |

## Requirements

- Non-headless client must keep running after the world route becomes active.
- The world scene must be driven by existing runtime projections, not by hard
  coded mock data.
- The slice does not need full movement sync, chat, inventory, or network-fed
  player/NPC/monster parity yet.
- Headless and control-http behavior stay unchanged.
- Every code change must have automated tests.
- Usage docs must explain the visible world shell and its current limits.

## Acceptance Criteria

- AC-01. Normal `mu_client` boot still reaches the world route and stays alive.
- AC-02. The world route spawns a visible Bevy 3D scene with a camera,
  lighting, terrain, and sampled world markers from the loaded bundle.
- AC-03. Existing headless and control-http smoke behavior is unchanged.
- AC-04. Automated tests cover scene bootstrap and cleanup.
- AC-05. Usage docs describe the visible world shell and its limits.

## Scope

In scope:

- World-scene plugin and route-based scene lifecycle.
- Primitive 3D rendering for terrain and sampled world markers.
- Tests for scene construction and cleanup.
- Usage docs for the visible shell.

Out of scope:

- Player movement.
- Network-driven local player/NPC/monster sync.
- Chat, inventory, GameShop, MU Helper, editor/admin tooling.
- Full asset/model parity.
- Cross-platform build automation changes.

## Open Questions

None blocking. Assumption: this slice uses Bevy primitives and existing runtime
projections as the first visible shell; it is not the final asset-faithful
renderer.
