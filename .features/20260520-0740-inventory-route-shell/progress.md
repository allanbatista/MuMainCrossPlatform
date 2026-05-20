# Inventory Route Shell Progress

Status: DONE_FOR_INVENTORY_ROUTE_SHELL

Current state: the inventory route shell slice is implemented, documented,
and validated. Tab now toggles the world and inventory routes, and the
inventory shell rerenders when inventory/equipment/vault summaries change.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-0740-inventory-route-shell/spec.md`, `.features/20260520-0740-inventory-route-shell/plan.md`, `.features/20260520-0740-inventory-route-shell/progress.md` | `.features/20260520-0740-inventory-route-shell/spec.md`, `.features/20260520-0740-inventory-route-shell/plan.md`, `.features/20260520-0740-inventory-route-shell/progress.md` | feature docs exist and the slice is scoped | docs created | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_app/src/inventory_route.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/inventory_route.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | route toggle tests | `tab_toggles_inventory_from_the_world_route`, `tab_returns_from_inventory_to_the_world_route`, `tab_ignores_non_world_routes_and_disconnected_sessions` | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/inventory_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | `port_rust/crates/mu_app/src/inventory_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | shell visibility tests | `inventory_shell_view_renders_ready_and_error_surfaces`, `plugin_spawns_and_clears_the_visible_shell`, summary rerender regression | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/*` | `port_rust/crates/mu_app/src/inventory_route.rs`, `port_rust/crates/mu_app/src/inventory_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | cargo test outputs | `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace` | none |
| F2.S2.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md` | `docs/player-rust-client.md`, `port_rust/README.md` | docs diff | Tab inventory toggle and inventory shell usage text added | none |
| F3.S1.T1 | done | local | progress docs | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.features/20260520-0740-inventory-route-shell/progress.md` | fmt/test/build/smoke logs | `cargo fmt --manifest-path port_rust/Cargo.toml --all`, `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`, `cargo test --manifest-path port_rust/Cargo.toml --workspace`, `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`, `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`, `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`, `rg -n \"Tab|inventory\" docs/player-rust-client.md port_rust/README.md` | none |
| F3.S1.T2 | done | local | `.memory/TODO.md`, progress | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.features/20260520-0740-inventory-route-shell/progress.md`, `.codexpotter/kb/inventory-route-shell.md`, `.codexpotter/kb/README.md` | memory diff | no follow-up inventory debt remained, so no `.memory/TODO.md` entry was needed | none |

## Done

- Created the feature spec, plan, and progress documents for the inventory
  route shell slice. This locked the slice scope before implementation.
- Implemented the inventory route toggle and visible shell in `mu_app`,
  including live summary rerenders when inventory/equipment/vault state
  changes. Files changed:
  `port_rust/crates/mu_app/src/inventory_route.rs`,
  `port_rust/crates/mu_app/src/inventory_shell.rs`,
  `port_rust/crates/mu_app/src/graphical_runtime.rs`,
  `port_rust/crates/mu_app/src/lib.rs`.
- Updated docs, KB tracking, and validation evidence for the inventory slice.
  Files changed:
  `docs/player-rust-client.md`, `port_rust/README.md`,
  `.codexpotter/kb/inventory-route-shell.md`, `.codexpotter/kb/README.md`,
  `.codexpotter/projects/2026/05/20/1/MAIN.md`.
