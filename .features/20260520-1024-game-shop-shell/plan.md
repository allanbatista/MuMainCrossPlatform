# Game Shop Shell

Status: READY_FOR_EXEC

## Summary

Add a visible GameShop shell to `mu_app`, wire a dedicated `game-shop`
control-http command, and document the new gameplay surface without changing
headless boot behavior.

## Interfaces / Contracts

- `mu_app::game_shop_shell` owns the route-gated GameShop shell plugin.
- `mu_app::graphical_runtime` registers the `mu_gameplay::GameShopPlugin`
  resource and the GameShop shell plugin.
- `mu_app::control_http` accepts a `game-shop` command for local smoke
  testing, with an underscore alias for convenience.
- `mu_ui::game_shop_screen` remains the visible source model for the shell.
- `docs/player-rust-client.md`, `port_rust/README.md`, and
  `port_rust/docs/control-http.md` document the GameShop shell and smoke
  command.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters / state | Datasets / permissions | Renderer / test target | Retailer/industry compatibility | Compatibility expectations |
|---|---|---|---|---|---|---|---|---|
| `game-shop-shell` | `mu_app::game_shop_shell`, `mu_ui::game_shop_screen`, `mu_gameplay::GameShopManager` | visible route shell | GameShop route, session phase, game shop manager mode | GameShop route only, not disconnected | local runtime GameShop projection only | `mu_app` tests | n/a; single-client port | renders the GameShop shell and clears on route exit |
| `game-shop-control-http` | `mu_app::control_http`, `mu_app::graphical_runtime` | control-plane route | `game-shop` command | route/session snapshot only | local smoke only | control-http smoke | n/a; local QA smoke only | lets QA drive the GameShop shell without touching the network protocol |

## Parallelization

- Shell implementation and control-http command wiring can be edited together
  because they touch the same feature boundary.
- Documentation updates can follow once the runtime behavior lands.
- Validation runs after the code and docs are in place.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write and audit the feature workflow docs for the GameShop shell
  slice.

### F1. Runtime shell and smoke hook

#### F1.S1 Tasks

- F1.S1.T1: Add a route-gated GameShop shell in `mu_app`.
- F1.S1.T2: Extend the local control-http commands to drive the GameShop
  route for smoke testing.

#### F1.S2 Tasks

- F1.S2.T1: Add unit tests for shell visibility, cleanup, and snapshot body
  formatting.

### F2. Docs and tracking

#### F2.S1 Tasks

- F2.S1.T1: Update usage docs to describe the GameShop shell and control-http
  smoke command.

#### F2.S2 Tasks

- F2.S2.T1: Refresh the GameShop KB note and any pending-work tracking that is
  still relevant after the slice lands.

### F3. Validation

#### F3.S1 Tasks

- F3.S1.T1: Run fmt, tests, build, and local smoke validation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F3.S1.T1 | `mu_app` tests and graphical smoke cover the GameShop route shell. |
| AC-02 | F1.S1.T1, F1.S2.T1, F3.S1.T1 | cleanup tests cover route exit and disconnect teardown. |
| AC-03 | F1.S2.T1, F3.S1.T1 | `cargo test` and smoke logs cover the shell body formatting and lifecycle. |
| AC-04 | F2.S1.T1, F3.S1.T1 | Validation evidence: `rtk rg -n "GameShop|game-shop" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md` confirms the usage docs mention the shell and smoke command. |
| AC-05 | F1.S1.T2, F3.S1.T1 | Validation evidence: `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0` plus a `game-shop` command smoke keeps the deterministic headless/control-http path intact. |

## Validation Gates

- Gate F0: feature doc audit for `.features/20260520-1024-game-shop-shell`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk rg -n "GameShop|game-shop" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md`
- Gate F3: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`; `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`; `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`; `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`; `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`; `e2e-validator` handoff for the visible GameShop smoke when a windowed run is available

## Risks

- The new control-http command must not collide with the existing NPC shop
  `shop` command.
- The GameShop shell can become stale if route cleanup and control-http sync
  drift apart.
- The shell must stay a startup-time cost only, not a per-frame rebuild.

## Rollback

- Remove the new route-shell registration and control-http command together if
  the slice destabilizes boot or smoke tests.
- Restore the previous docs wording if the GameShop shell needs to be backed
  out.
