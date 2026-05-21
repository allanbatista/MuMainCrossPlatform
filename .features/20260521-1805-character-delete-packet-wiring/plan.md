# Character Delete Packet Wiring

Status: READY_FOR_EXEC

## Summary

Add the missing character-delete route shell and wire the legacy
delete-character packet through the bootstrap worker so local smoke can drive
the delete flow end to end.

## Interfaces / Contracts

- `mu_ui::character_delete` owns the visible delete confirmation shell.
- `mu_app::auth_shell` renders `UiRoute::CharacterDelete`.
- `mu_app::control_http` accepts the delete route open and delete submit
  commands.
- `mu_app::bootstrap_runtime` queues and classifies the delete packet flow.
- `docs/player-rust-client.md`, `port_rust/README.md`, and the control-http
  docs explain the new local smoke command.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `character-delete-shell` | `mu_ui::character_delete`, `mu_app::auth_shell` | auth shell card | selected character | selected roster entry, route state | local session/bootstrap only | no split | `mu_app` / `mu_ui` tests | shows the delete confirmation route |
| `character-delete-submit` | `mu_app::control_http`, `mu_app::bootstrap_runtime` | control command + packet send | delete command | selected character, security code | local control HTTP only | no split | `mu_app` tests | sends the legacy delete packet and handles the response |
| `character-delete-docs` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md` | usage docs | delete flow smoke | none | docs only | no split | grep/doc checks | documents the new local smoke path |

## Parallelization

- The UI shell and bootstrap packet wiring are tightly coupled and should be
  implemented together.
- Tests and docs can follow once the route and packet handling are stable.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write the feature spec, plan, and progress documents for the
  character-delete packet wiring slice and audit them.

### F1. Runtime wiring

- F1.S1.T1: Add the `mu_ui::character_delete` screen module and expose it
  through the auth shell.
- F1.S1.T2: Add the delete submit command path in `control_http` and
  `bootstrap_runtime`, including delete-response classification.

### F2. Tests and docs

- F2.S1.T1: Add unit coverage for the delete route shell, packet send path,
  and response handling.
- F2.S1.T2: Update usage docs and KB notes to describe the delete flow and
  local smoke command.

### F3. Validation

- F3.S1.T1: Run fmt, tests, build, and local smoke validation.
- F3.S1.T2: Sync pending-work memory and the workflow progress after
  validation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F2.S1.T1 | auth shell and `mu_ui` tests cover the route shell |
| AC-02 | F1.S1.T2, F2.S1.T1 | bootstrap packet-send tests and fake-server expectation |
| AC-03 | F1.S1.T2, F2.S1.T1 | success response handling tests |
| AC-04 | F1.S1.T2, F2.S1.T1 | failure response handling tests |
| AC-05 | F2.S1.T1, F3.S1.T1 | `cargo test` outputs |
| AC-06 | F2.S1.T2, F3.S1.T1 | docs diff and grep evidence |

## Validation Gates

- Gate F0: feature-workflow audit for the new docs folder
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_ui`
- Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F3: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Gate F4: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Gate F5: `rtk timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`
- Gate F6: `rtk rg -n "character-delete|delete-character" docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md`

## Risks

- The delete flow can drift from the selected roster entry if the runtime does
  not keep the selected name in sync.
- The route shell must stay safe when the security code is absent or invalid.
- Delete success should not regress the existing login/bootstrap route ladder.

## Rollback

- Remove the delete route shell and the delete submit command if the slice
  destabilizes auth boot or smoke tests.
- Restore the previous docs wording if the delete flow needs to be backed
  out.
