# Siege Shell
Status: READY_FOR_EXEC

## Summary

Expose the siege route in `mu_app`, wire the control HTTP smoke path to the
route, and document the visible siege shell.

## Interfaces / Contracts

- `mu_app::siege_shell` owns the route-gated siege view and maps the active
  control command to the visible siege snapshot.
- `mu_app::graphical_runtime` registers `GuildCachePlugin` plus the siege
  shell plugin so the route can project guild mark indices.
- `mu_app::control_http` accepts siege route commands and keeps the runtime
  snapshot in sync with the selected siege mode.
- `docs/player-rust-client.md`, `port_rust/README.md`, and
  `port_rust/docs/control-http.md` describe the siege route and smoke
  commands.

## Technical Inventory

| Slug/ID | Components | Output type | Queries | Filters / URL state | Datasets / permissions | Renderer / test target | Retailer / industry compatibility | Compatibility expectations |
|---|---|---|---|---|---|---|---|---|
| `siege-shell` | `mu_app::siege_shell`, `mu_ui::siege_screen`, `mu_gameplay::GuildCache` | visible route shell | `UiRoute::Siege`, control command state | session connected, siege mode command | existing guild cache only | `mu_app` unit tests and graphical smoke | not applicable | render the siege inactive/observer/soldier/commander snapshot and clear on exit |
| `siege-control-http` | `mu_app::control_http` | control-plane route | siege command names | HTTP command string | local smoke only | HTTP command tests and local smoke | not applicable | route the runtime into the siege shell without game networking |

## Phases

### F0. Workflow docs

#### F0.S1 Task

- F0.S1.T1 Create and audit the feature workflow docs for the siege shell
  slice.

### F1. Runtime shell and smoke hook

#### F1.S1 Tasks

- F1.S1.T1 Add a route-gated siege shell in `mu_app` and register the guild
  cache gameplay plugin.
- F1.S1.T2 Extend the local control HTTP commands to drive the siege route
  for smoke testing.

### F2. Tests and docs

#### F2.S1 Tasks

- F2.S1.T1 Add unit tests for siege shell visibility and mode mapping.
- F2.S1.T2 Extend control-command parsing coverage for the siege route.

#### F2.S2 Tasks

- F2.S2.T1 Update usage docs to describe the siege shell and control HTTP
  smoke commands.

### F3. Validation

#### F3.S1 Tasks

- F3.S1.T1 Run fmt, tests, build, and local smoke validation.
- F3.S1.T2 Update the KB and workflow progress after validation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F2.S1.T1 | Validation evidence: `mu_app` tests and graphical smoke cover the siege route shell. |
| AC-02 | F1.S1.T1, F2.S1.T1 | Validation evidence: cleanup tests cover route exit and disconnect teardown for the siege shell. |
| AC-03 | F1.S1.T2, F2.S1.T2 | Validation evidence: control-command parser tests and HTTP smoke cover siege commands. |
| AC-04 | F2.S1.T1, F2.S1.T2, F3.S1.T1 | Validation evidence: automated unit tests and smoke logs cover the route/mode wiring. |
| AC-05 | F2.S2.T1 | Validation evidence: player-facing docs mention the siege route and the smoke commands. |

## Validation Gates

- Gate F0: feature doc audit for `.features/20260520-2134-siege-shell`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check`;
  `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`;
  `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`;
  `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`;
  `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client`;
  `e2e-validator` handoff for a windowed siege smoke run when the
  environment allows it
- Gate F3: `rtk rg -n "siege" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md docs/siege-warfare.md`

## Risks

- The siege shell can drift if `GuildCachePlugin` is not registered in the
  graphical runtime.
- The smoke path must not disturb the existing friend/guild, world, or chat
  route behavior.
- If the control HTTP command names drift, the visible shell must still remain
  testable through direct unit tests.

## Rollback

- Remove the new route-shell registration and control HTTP commands together
  if the slice destabilizes boot or smoke tests.
- Restore the previous docs wording if the siege shell needs to be backed out.
