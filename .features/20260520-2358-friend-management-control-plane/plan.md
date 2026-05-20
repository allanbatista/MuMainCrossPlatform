# Friend Management Control Plane

Status: READY_FOR_EXEC

## Summary

Expose friend-add and friend-delete over the local control HTTP API and bridge
those commands into the live session so QA can drive the existing friend
packet helpers without touching the gameplay protocol.

## Interfaces / Contracts

- `mu_app::bootstrap_runtime` owns the queue methods that send friend add and
  friend delete packets.
- `mu_app::control_http` parses the new friend action commands and stores the
  requested name in the snapshot.
- `mu_app::graphical_runtime` forwards the snapshot command into
  `BootstrapRuntime`.
- `docs/player-rust-client.md`, `port_rust/README.md`, and
  `port_rust/docs/control-http.md` document the new commands.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters / state | Datasets / permissions | Retailer/industry compatibility | Renderer / test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `friend-add-command` | `mu_app::control_http`, `mu_app::graphical_runtime`, `mu_app::bootstrap_runtime`, `mu_protocol::social` | local HTTP command + friend add packet | `friend-add` | friend name payload | local control-http only | not applicable; no retailer/industry split | `mu_app` tests | queues `friend_add_request` once per command |
| `friend-delete-command` | `mu_app::control_http`, `mu_app::graphical_runtime`, `mu_app::bootstrap_runtime`, `mu_protocol::social` | local HTTP command + friend delete packet | `friend-delete` | friend name payload | local control-http only | not applicable; no retailer/industry split | `mu_app` tests | queues `friend_delete` once per command |
| `friend-management-docs` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, KB | usage docs | local friend commands | docs only | no split | not applicable; no retailer/industry split | docs grep | explains the control-plane request shape |

## Parallelization

- The bootstrap queue methods and the control-http bridge can be implemented
  together because they share the same command shape.
- Docs can land once the request shape is fixed.
- Validation runs after the code and docs are in place.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for the
  friend-management control-plane slice and audit them.

### F1. Friend action plumbing

- F1.S1.T1: Add friend add/delete queue methods and packet send handling in
  `bootstrap_runtime.rs`.
- F1.S1.T2: Extend `control_http.rs` and `graphical_runtime.rs` so
  `friend-add` and `friend-delete` accept a friend name payload and forward
  the command into the live session bridge.

### F2. Tests and docs

- F2.S1.T1: Add automated tests for request validation, control-plane parsing,
  and packet queueing.
- F2.S1.T2: Update usage docs and KB notes to describe the new friend action
  commands.

### F3. Validation

- F3.S1.T1: Run the relevant Cargo tests, build the client, and execute the
  local control-http and smoke checks.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | queue and control-http tests show `friend-add` sends `friend_add_request` |
| AC-02 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | queue and control-http tests show `friend-delete` sends `friend_delete` |
| AC-03 | F1.S1.T2, F2.S1.T1 | empty request validation tests return bad-request |
| AC-04 | F2.S1.T1, F3.S1.T1 | `cargo test` output |
| AC-05 | F2.S1.T2, F3.S1.T1 | docs grep validation for the new friend actions |

## Validation Gates

- Gate F0: feature-doc audit for `.features/20260520-2358-friend-management-control-plane`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime control_http graphical_runtime`
- Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Gate F3: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Gate F4: local control-http smoke that posts `friend-add` and `friend-delete`
- Gate F5: `rtk rg -n "friend-add|friend-delete|friend management" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md`
- Gate F6: `e2e-validator` handoff is not required for this local control-plane slice because validation is fully covered by package tests, build, docs grep, and control-http smoke.

## Risks

- The command payload must stay explicit so control-http never guesses a
  friend name.
- The new commands should not disturb the existing friend route shell or the
  headless smoke path.

## Rollback

- Remove the new friend action command branches and queue methods together if
  the slice destabilizes control-http or bootstrap tests.
