# Inventory Route Shell

Status: READY_FOR_EXEC

## Summary

Wire the existing Tab inventory binding into the Bevy runtime, make the
inventory route render a visible shell, and document the toggle.

## Interfaces / Contracts

- `mu_app::inventory_route::InventoryRoutePlugin` captures the Tab toggle and
  switches between `UiRoute::World` and `UiRoute::Inventory`.
- `mu_app::inventory_shell::InventoryShellPlugin` renders a visible inventory
  shell from `mu_ui::inventory_screen()`.
- `mu_app::graphical_runtime` registers the inventory route and shell plugins.
- `mu_app::ClientRuntime` and `mu_ui::UiShellState` remain the route/session
  sources of truth.
- `docs/player-rust-client.md` and `port_rust/README.md` document the Tab
  toggle.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|
| `inventory-toggle` | `mu_app::inventory_route`, `bevy::input::ButtonInput<KeyCode>`, `mu_ui::UiShellState` | route changes | Tab key, current route | world route only | world-ready runtime only | `mu_app` tests | Switches between world and inventory |
| `inventory-shell` | `mu_app::inventory_shell`, `mu_ui::inventory_screen` | visible shell | inventory route | world route ready | local runtime projection only | `mu_app` tests | Renders the visible inventory shell and clears on route exit |
| `inventory-docs` | `docs/player-rust-client.md`, `port_rust/README.md` | usage docs | Tab toggle | local inventory route only | player-facing documentation only | docs grep | Documents the new toggle |

## Parallelization

- The route toggle and shell renderer can be implemented together because
  both depend on the same route state.
- Tests can be added once the route and shell behavior is stable.
- Docs can land after the control flow is fixed.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for the
  inventory route shell slice.

### F1. Runtime inventory shell

- F1.S1.T1: Add a Tab-driven route toggle between world and inventory.
- F1.S1.T2: Render the inventory shell when the inventory route is active and
  clear it on route exit.

### F2. Tests and docs

- F2.S1.T1: Add automated tests for route toggling and shell visibility.
- F2.S2.T1: Update usage documentation with the inventory toggle.

### F3. Validation

- F3.S1.T1: Run the relevant Cargo tests, build the client, and smoke the
  graphical/headless boot paths.
- F3.S1.T2: Track any remaining inventory or route-toggle debt in the
  progress and memory files.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Required evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260520-0740-inventory-route-shell/spec.md`, `.features/20260520-0740-inventory-route-shell/plan.md`, `.features/20260520-0740-inventory-route-shell/progress.md` | none | docs exist and the slice is fully scoped | feature doc review |
| F1.S1.T1 Inventory route toggle | local | `port_rust/crates/mu_app/src/inventory_route.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | F0.S1.T1 | Tab toggles world <-> inventory on the live runtime | route toggle tests |
| F1.S1.T2 Inventory shell renderer | local | `port_rust/crates/mu_app/src/inventory_shell.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs` | F1.S1.T1 | inventory route renders a visible shell and clears on exit | shell visibility tests |
| F2.S1.T1 Automated tests | local | `port_rust/crates/mu_app/src/*` | F1 | tests cover route toggling and shell visibility | `cargo test` outputs |
| F2.S2.T1 Usage docs | local | `docs/player-rust-client.md`, `port_rust/README.md` | F1 | docs explain the Tab inventory toggle and route shell | docs diff |
| F3.S1.T1 Validation | local | progress docs | F1-F2 | fmt/test/build/smoke evidence is captured | command output / logs |
| F3.S1.T2 Follow-up tracking | local | `.memory/TODO.md`, progress | F3.S1.T1 | remaining inventory or route-toggle debt is recorded | memory diff |

## Validation Gates

- Validation Gate F0: feature doc review for the new slice folder
- Validation Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Validation Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Validation Gate F3: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Validation Gate F4: graphical and headless local smoke for `mu_client`
- Validation Gate F5: `rtk rg -n "Tab|inventory" docs/player-rust-client.md port_rust/README.md`

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F2.S1.T1 | route toggle tests |
| AC-02 | F1.S1.T1, F2.S1.T1 | route toggle tests |
| AC-03 | F1.S1.T1, F2.S1.T1 | route guard tests |
| AC-04 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | `cargo test` output |
| AC-05 | F2.S2.T1, F3.S1.T1 | Validation Gate F5 |
| AC-06 | F3.S1.T1 | Validation Gate F4 smoke evidence |

## Risks

- The inventory route can blank the world scene if the toggle and shell cleanup
  drift apart; keep the route and shell systems close together.
- The Tab binding must not interfere with the existing movement loop or
  change the deterministic smoke path.
- Disconnect handling must clear the inventory shell instead of leaving a
  stale route visible.

## Rollback

- Revert the inventory toggle and shell as one slice if the route becomes
  unstable.
- Keep the world bootstrap and control-http flow isolated so boot smoke stays
  intact if the inventory route needs to be backed out.
