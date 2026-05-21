# World HUD Surface Stack

Status: READY_FOR_EXEC

## Summary

Extend the existing world HUD overlay in `mu_app` so it also renders the chat
log, minimap, and hotkey snapshot surfaces while the world route is active,
keeps cleanup route-gated, and documents the visible stack.

## Interfaces / Contracts

- `mu_app::world_hud` owns the world HUD stack plugin and visible overlay
  lifecycle.
- `mu_ui::hud_screen`, `mu_ui::chat_screen`, `mu_ui::minimap_screen`, and
  `mu_ui::hotkeys_screen` remain the source models for the visible world
  surfaces.
- `mu_app::graphical_runtime` keeps the existing plugin registration path.
- `docs/player-rust-client.md` and `port_rust/README.md` document the visible
  world HUD stack.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters / state | Datasets / permissions | Renderer / test target | Retailer/industry compatibility | Compatibility expectations |
|---|---|---|---|---|---|---|---|---|
| `world-hud-surface-stack` | `mu_app::world_hud`, `mu_ui::{hud_screen,chat_screen,minimap_screen,hotkeys_screen}` | Bevy overlay card stack | world route plus world-ready projection | current world route, world-ready state, HUD/chat/minimap/hotkey snapshots | local converted assets and UI snapshot data only | `mu_app` unit tests + `mu_client` graphical smoke | not applicable; no retailer/industry split | no change to headless/control-http, no network contract change |

## Phases

### F0. Workflow docs

#### F0.S1 Task

- F0.S1.T1 Create and audit the feature workflow docs for the world HUD
  surface stack slice.

### F1. Runtime overlay

#### F1.S1 Tasks

- F1.S1.T1 Extend the world HUD overlay to render the visible chat, minimap,
  and hotkey surfaces from the existing UI snapshots.
- F1.S1.T2 Keep the overlay route-gated and clear it when the world projection
  or route is no longer active.

### F2. Usage and validation

#### F2.S1 Tasks

- F2.S1.T1 Add unit tests for the world surface body formatting and overlay
  lifecycle.
- F2.S2.T1 Update usage docs to describe the visible world HUD stack and its
  current limits.

#### F2.S3 Tasks

- F2.S3.T1 Run fmt, tests, build, and graphical smoke validation, then record
  the evidence.
- F2.S3.T2 Update the pending-work memory and progress notes after validation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F2.S3.T1 | Validation evidence: graphical smoke reaches the world route and shows the world HUD stack. |
| AC-02 | F1.S1.T2, F2.S1.T1 | Validation evidence: `mu_app` tests cover overlay cleanup when the route leaves the world or the world projection is cleared. |
| AC-03 | F2.S1.T1 | Validation evidence: `mu_app` tests cover the world surface body formatting helpers and snapshot-driven content. |
| AC-04 | F2.S2.T1 | Validation evidence: player-facing docs mention the visible world HUD stack and its current limits. |
| AC-05 | F2.S3.T1 | Validation evidence: headless/control-http smoke still reports the same deterministic state behavior. |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0905-world-hud-surface-stack`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`; `e2e-validator` handoff for the visible world HUD stack when the environment allows a windowed smoke run
- Gate F3: `rtk rg -n "HUD|chat|minimap|hotkeys" docs/player-rust-client.md port_rust/README.md`

## Risks

- The world HUD stack can obscure the 3D world if the card composition is too
  dense; keep the overlay compact and anchored.
- The first slice intentionally reuses the static snapshot data instead of
  rebuilding the full interactive UI stack.
- If the graphical smoke cannot open a window in the current environment, keep
  the unit-test evidence and record the blocker explicitly.

## Parallelization

- The body-formatting helpers and cleanup tests can be built together once the
  overlay shape is stable.
- Docs and validation are sequential after the implementation lands.

## Rollback

- Remove the world HUD surface-stack registration if it destabilizes the
  runtime.
- Restore the previous docs wording if the visible world HUD stack needs to be
  backed out.
