# World Terrain Ground Follow Progress

Status: OPEN

Current state: the terrain ground-follow slice is implemented and validated.
The local and remote player markers now rest on the visible terrain surface
in `mu_app::world_scene`.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1511-world-terrain-ground-follow/spec.md`, `.features/20260520-1511-world-terrain-ground-follow/plan.md`, `.features/20260520-1511-world-terrain-ground-follow/progress.md` | `.features/20260520-1511-world-terrain-ground-follow/spec.md`, `.features/20260520-1511-world-terrain-ground-follow/plan.md`, `.features/20260520-1511-world-terrain-ground-follow/progress.md` | feature-workflow audit output | `rtk node /home/allanbatista/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-1511-world-terrain-ground-follow` passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | terrain surface sampler and grounding helper | `terrain_surface_height_at_position()` now samples the visible heightfield surface for marker grounding | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | local/remote marker transform anchoring | `grounded_world_transform()` now anchors local and remote marker meshes to the sampled terrain in spawn and sync paths | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | unit tests for sampler and transform grounding | `terrain_surface_height_at_position_matches_the_visible_bundle_sample`, `grounded_marker_translation_lifts_local_and_remote_players_above_the_surface`, and `world_scene_reconciles_the_local_player_marker_after_runtime_motion` | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/world-terrain-ground-follow.md`, `.codexpotter/kb/README.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/world-terrain-ground-follow.md`, `.codexpotter/kb/README.md` | docs diff | user-facing docs and KB now mention grounded local/remote markers and the visual-only scope | none |
| F3.S1.T1 | done | local | progress docs | `.features/20260520-1511-world-terrain-ground-follow/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | fmt/test/build/smoke logs | `cargo fmt`, `cargo test -p mu_app`, `cargo test --workspace`, `cargo build -p mu_client`, headless control-http smoke, and graphical timeout smoke all passed | none |
| F3.S1.T2 | done | local | `.memory/TODO.md`, progress | `.features/20260520-1511-world-terrain-ground-follow/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md` | memory/progress diff | no new pending debt was introduced for this slice; progress now records the finished state | none |

## Files Touched

New:

- `.features/20260520-1511-world-terrain-ground-follow/spec.md`
- `.features/20260520-1511-world-terrain-ground-follow/plan.md`
- `.features/20260520-1511-world-terrain-ground-follow/progress.md`

Modified:

- `docs/player-rust-client.md`
- `port_rust/README.md`
- `.codexpotter/kb/README.md`
- `.codexpotter/kb/world-terrain-ground-follow.md`
- `port_rust/crates/mu_app/src/world_scene.rs`
- `.features/20260520-1511-world-terrain-ground-follow/progress.md`

Removed:

- none
