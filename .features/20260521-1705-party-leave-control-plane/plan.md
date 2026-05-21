# Party Leave Control Plane Plan

Status: READY_FOR_IMPLEMENTATION

## Approach

Mirror the other control-plane slices: parse the party member number from
HTTP, store it in the control snapshot, forward the snapshot command through
the graphical runtime, and let the bootstrap worker encode the existing
social packet helper.

## Technical Inventory

- `port_rust/crates/mu_app/src/control_http.rs`
  - `ControlCommand::PartyLeave`, `ControlSnapshot.party_member_number`, and
    `party_leave_from_request`.
- `port_rust/crates/mu_app/src/graphical_runtime.rs`
  - `sync_control_http_snapshot_to_runtime` branch that queues the party leave
    request.
- `port_rust/crates/mu_app/src/bootstrap_runtime.rs`
  - `BootstrapCommand::PartyLeave`, queue helper, and packet send path.
- `port_rust/crates/mu_protocol/src/social.rs`
  - `party_player_kick_request(player_index: u8)`.
- Usage docs
  - `docs/party-ui.md`, `docs/player-rust-client.md`,
    `port_rust/docs/control-http.md`, `port_rust/README.md`.

## Interfaces / Contracts

- New control command: `party-leave`.
- Request payload: `member_number` query/body value, parsed as `u8`.
- Control snapshot field: `party_member_number: Option<u8>`.
- Bootstrap command: `BootstrapCommand::PartyLeave(u8)`.
- Packet helper: `mu_protocol::social::party_player_kick_request`.

## Phases

### Phase 1: Control Surface

- Add the `party-leave` command name and payload parser to control-http.
- Store the selected member number in the control snapshot.
- Keep the route/session behavior aligned with the other party commands.

### Phase 2: Runtime Bridge

- Add the bootstrap queue method and command variant.
- Forward the snapshot payload from the graphical runtime to the bootstrap
  worker.
- Encode the legacy party leave packet with the selected member number.

### Phase 3: Verification And Docs

- Add tests for request parsing, runtime queueing, and packet bytes.
- Update usage docs and KB notes.
- Run fmt, package tests, workspace tests, client build, and a focused smoke.

## AC Traceability

- AC-01 -> F1.S1.T1, F1.S1.T2
- AC-02 -> F2.S1.T1
- AC-03 -> F1.S1.T2
- AC-04 -> F2.S1.T2
- AC-05 -> F3.S1.T1

## Risks

- The command targets the member number from the roster, so the docs must be
  explicit about the required payload.
- The party leave packet is also the kick packet, so the wording must avoid
  implying a separate new protocol.

## Rollback

- Remove the `party-leave` command branch, runtime queue method, and packet
  send path.
- Drop the docs and KB note if the slice is reverted.
