# World Terrain Lightmap Progress

Status: OPEN

Current state: the lightmap slice is implemented, documented, and validated.
The remaining backlog is broader parity work outside this slice.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1218-world-terrain-lightmap/spec.md`, `.features/20260520-1218-world-terrain-lightmap/plan.md`, `.features/20260520-1218-world-terrain-lightmap/progress.md` | `.features/20260520-1218-world-terrain-lightmap/spec.md`, `.features/20260520-1218-world-terrain-lightmap/plan.md`, `.features/20260520-1218-world-terrain-lightmap/progress.md` | feature-workflow audit output | `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-1218-world-terrain-lightmap` passed | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/world_scene.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | lightmap path resolution helper and asset-root wiring | `world_terrain_lightmap_path()` resolves the sample-world lightmap against the configured asset root and falls back to `TerrainLight.png` when the lowercase config path is missing | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | lightmap-backed terrain overlay in the world scene | `spawn_world_terrain()` now adds a third alpha-blended terrain surface for the lightmap when the path resolves, while preserving the current base/blend/fallback path | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_app/src/world_scene.rs` | `port_rust/crates/mu_app/src/world_scene.rs` | unit tests for path resolution, overlay spawning, and cleanup | `world_terrain_lightmap_path_resolves_the_checked_in_asset`; `world_scene_spawns_visible_shell_when_world_route_is_ready`; `world_scene_clears_when_route_leaves_world`; `world_scene_waits_for_world_projection` | none |
| F2.S1.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | docs now mention the terrain lightmap overlay and its fallback behavior | none |
| F2.S2.T1 | done | local | `.codexpotter/kb/world-terrain-lightmap.md`, `.codexpotter/kb/README.md`, `.memory/TODO.md` | `.codexpotter/kb/world-terrain-lightmap.md`, `.codexpotter/kb/README.md`, `.memory/TODO.md` | KB and pending-work updates | new KB note added; KB index updated; stale manual character-select debt removed | none |
| F3.S1.T1 | done | local | feature docs, `port_rust/crates/mu_app/src/world_scene.rs`, docs | `.features/20260520-1218-world-terrain-lightmap/progress.md`, `.codexpotter/projects/2026/05/20/1/MAIN.md`, `port_rust/crates/mu_app/src/world_scene.rs`, `docs/player-rust-client.md`, `port_rust/README.md`, `.codexpotter/kb/world-terrain-lightmap.md`, `.codexpotter/kb/README.md`, `.memory/TODO.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`; `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`; `cargo test --manifest-path port_rust/Cargo.toml --workspace`; `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0` with `/state` and `exit`; `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client` started the graphical client and reached the timeout guard after boot | none |

## Files Touched

New:

- `.features/20260520-1218-world-terrain-lightmap/spec.md`
- `.features/20260520-1218-world-terrain-lightmap/plan.md`
- `.features/20260520-1218-world-terrain-lightmap/progress.md`
- `.codexpotter/kb/world-terrain-lightmap.md`

Modified:

- `.features/20260520-1218-world-terrain-lightmap/spec.md`
- `.features/20260520-1218-world-terrain-lightmap/plan.md`
- `.features/20260520-1218-world-terrain-lightmap/progress.md`
- `port_rust/crates/mu_app/src/world_scene.rs`
- `docs/player-rust-client.md`
- `port_rust/README.md`
- `.codexpotter/kb/README.md`
- `.memory/TODO.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`

Removed:

- none
