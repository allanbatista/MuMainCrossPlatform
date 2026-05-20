# Fake-Server Login Bootstrap

Status: READY_FOR_EXEC

## Summary

Implement the next playable slice after graphical boot: use the existing
network session, fake-server harness, UI snapshots, and client runtime to
progress from login to server select, character select, and initial world
bootstrap while keeping failure states safe.

## Interfaces / Contracts

- `mu_app::runtime::run(Cli)` keeps headless and control-http behavior stable,
  but the normal graphical path now advances through a login bootstrap state
  machine instead of stopping after the boot shell.
- `mu_app::session_state::SessionState` remains the canonical login/logout/
  disconnect tracker.
- `mu_network::session::Session` continues to classify login, logout, and
  disconnect packets; the fake-server harness remains the local smoke driver.
- `mu_ui::UiRoute` and `mu_ui::UiShellState` remain the route contract for the
  login/server-select/character-select/loading/world surfaces.
- `mu_app::client_runtime::ClientRuntime` continues to own asset/world
  projection loading and sync.
- The initial world bootstrap continues to use the converted asset root and the
  sample `world_1` bundle until later slices wire server-selected world data.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `session-bootstrap` | `mu_app::SessionState`, `mu_network::Session` | session phase | login/logout/disconnect packet flow | login failure, logout, disconnect | local fake server only | no split | `mu_app` tests | Canonical state machine for the new slice |
| `ui-login` | `mu_ui::login::LoginScreen`, `mu_ui::UiShellState` | UI snapshot | auth form submit, login response | username/password, remember username | local session phase | no split | `mu_ui` snapshot tests | Must keep login failure on the auth shell |
| `ui-server-select` | `mu_ui::server_select::ServerSelectScreen` | UI snapshot | connect-server list response | server list, selected server | connect-server response | no split | `mu_ui` snapshot tests | Uses the existing server list response shape |
| `ui-character-select` | `mu_ui::character_select::CharacterSelectScreen` | UI snapshot | character roster response | roster, slot selection | login response + roster data | no split | `mu_ui` snapshot tests | Represents the handoff before world entry |
| `world-bootstrap` | `mu_app::ClientRuntime`, `mu_gameplay::WorldManager`, `mu_render::TerrainRenderer` | Bevy window + runtime snapshot | asset-root load, terrain bundle load | selected character, world id | valid `--asset-root` | no split | `mu_app` tests + client smoke | Loads the initial sample world bundle |
| `fake-server-smoke` | `mu_network::fake_server::FakeServerScenario`, `ConnectionScript` | local TCP script | packet sequence | scripted send/expect/delay/close flow | loopback only | no split | integration tests | Drives deterministic login/bootstrap paths |
| `headless-control` | `mu_app::control_http` | HTTP state output | command name | control request name | loopback only | no split | control-http smoke | Must stay deterministic while bootstrap evolves |

## Parallelization

- Runtime/session wiring and world bootstrap loading can be implemented in
  parallel once the shared bootstrap state is defined.
- Test coverage can start once the new state transitions are stable.
- Usage docs can be updated after the behavior contract is finalized.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for this
  slice and audit them.

### F1. Runtime bootstrap

- F1.S1.T1: Introduce the bootstrap orchestration resource or plugin in
  `mu_app` to hold session phase, UI route, and world-bootstrap state.
- F1.S1.T2: Wire the graphical runtime so the login bootstrap state machine can
  advance the UI from boot to login, server select, character select, and
  loading.
- F1.S2.T1: Load the initial world bundle after a successful bootstrap and
  keep the render/world projections synchronized.
- F1.S2.T2: Preserve the current headless and control-http behavior while the
  new bootstrap path is active.

### F2. Tests and docs

- F2.S1.T1: Add automated tests for login success, login failure, disconnect,
  route progression, and initial world bootstrap.
- F2.S2.T1: Update usage documentation for the fake-server smoke path and the
  normal graphical bootstrap path.
- F2.S2.T2: Validate the usage docs with a grep-based smoke check so the new
  commands remain discoverable.

### F3. Validation

- F3.S1.T1: Run the relevant Cargo tests, build the client, and execute the
  local fake-server smoke checks.
- F3.S1.T2: Capture any remaining blockers or follow-up work in the progress
  and memory files.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Required evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260520-0400-fake-server-login-world-bootstrap/spec.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/plan.md`, `.features/20260520-0400-fake-server-login-world-bootstrap/progress.md` | none | docs exist and audit passes | feature-workflow audit output |
| F1.S1.T1 Bootstrap resource | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/graphical_runtime.rs`, `port_rust/crates/mu_app/src/lib.rs`, maybe `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | F0.S1.T1 | runtime has an explicit bootstrap state/resource | unit tests for the bootstrap resource |
| F1.S1.T2 UI route wiring | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/session_state.rs`, `port_rust/crates/mu_app/src/client_runtime.rs` | F1.S1.T1 | session events move the UI through login/server-select/character-select/loading | route/state transition tests |
| F1.S2.T1 World bootstrap load | local | `port_rust/crates/mu_app/src/client_runtime.rs`, `port_rust/crates/mu_app/src/runtime.rs` | F1.S1.T1 | successful bootstrap loads the initial world bundle and keeps projections in sync | world bootstrap test |
| F1.S2.T2 Headless parity | local | `port_rust/crates/mu_app/src/runtime.rs`, `port_rust/crates/mu_app/src/state.rs` | F1.S1.T1 | headless and control-http state reporting remain stable | headless/control-http smoke |
| F2.S1.T1 Automated tests | local | `port_rust/crates/mu_app/src/*`, `port_rust/crates/mu_network/src/*` | F1 | tests cover success, failure, disconnect, and world bootstrap | `cargo test` outputs |
| F2.S2.T1 Usage docs | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | F1 | docs explain how to run the fake-server smoke path | docs diff |
| F2.S2.T2 Docs validation | local | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | F2.S2.T1 | docs grep gate passes | grep output |
| F3.S1.T1 Validation | local | progress docs | F1-F2 | fmt/test/build/smoke evidence is captured | command output / logs |
| F3.S1.T2 Follow-up tracking | local | `.memory/TODO.md`, progress | F3.S1.T1 | any remaining bootstrap debt is recorded | memory diff |

## Validation Gates

- Validation Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-0400-fake-server-login-world-bootstrap`
- Validation Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Validation Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_network`
- Validation Gate F3: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Validation Gate F4: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Validation Gate F5: local fake-server smoke against `mu_client` and `mu_fake_server`; add an e2e-validator handoff once the bootstrap path is wired.
- Validation Gate F6: `rtk rg -n "fake-server|login|character-select" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md`

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F1.S2.T1, F2.S1.T1, F3.S1.T1 | fake-server login/bootstrap tests and local smoke output |
| AC-02 | F1.S1.T2, F1.S2.T2, F2.S1.T1 | login failure/disconnect tests and control-http smoke |
| AC-03 | F1.S2.T1, F2.S1.T1, F3.S1.T1 | world bootstrap tests and client build/smoke logs |
| AC-04 | F1.S2.T2, F3.S1.T1 | headless/control-http validation evidence |
| AC-05 | F2.S1.T1, F3.S1.T1 | automated test output |
| AC-06 | F2.S2.T1, F2.S2.T2, F3.S1.T1 | Validation Gate F6 evidence: `rtk rg -n "fake-server|login|character-select" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md` |

## Risks

- Session and UI route state can drift if the bootstrap orchestration is split
  across too many files; keep the state machine small and centralized.
- World bootstrap can regress if initial world loading is coupled too tightly to
  login success; keep the data load path isolated and testable.
- Failure handling must remain explicit so the client never falls back to an
  unintended `Exit` path.

## Rollback

- Revert the bootstrap orchestration changes as a single commit if the new
  state machine disrupts headless smoke or graphical boot stability.
- Keep the initial world load behind the bootstrap boundary so the login path
  can be rolled back without touching the lower-level asset validators.
