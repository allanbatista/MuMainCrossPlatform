# Login Character List Request

Status: READY_FOR_EXEC

## Summary

Teach the bootstrap worker to request the character list immediately after a
successful login, using the legacy locale byte mapping, then update the fake
server smoke and docs to match.

## Interfaces / Contracts

- `mu_app::bootstrap_runtime` owns the login-success follow-up request.
- `mu_app::graphical_runtime` continues to feed the bootstrap worker.
- `mu_app::config` provides the locale normalization used by the request
  mapping.
- `mu_protocol::login::request_character_list` remains the packet helper.
- `docs/player-rust-client.md` and `port_rust/README.md` document the
  automatic roster request.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `login-roster-request` | `mu_app::bootstrap_runtime`, `mu_protocol::login`, `mu_app::config` | network bootstrap packet | login success | locale byte, login phase | local fake server only | no split | `mu_app` tests | Sends the roster request automatically after login success |
| `login-roster-docs` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, KB | usage docs | automatic roster request | login success only | docs only | no split | grep/doc checks | Documents the new request chain |

## Phases

### F0. Workflow docs

- F0.S1.T1: Write and audit the feature spec, plan, and progress documents for
  the login character-list request slice.

### F1. Bootstrap request chain

- F1.S1.T1: Add the login-success follow-up request in the bootstrap worker,
  including the legacy locale byte mapping.
- F1.S1.T2: Update the fake-server login/bootstrap coverage so it expects the
  new character-list request.

### F2. Tests and docs

- F2.S1.T1: Update the usage docs and KB note to mention the automatic roster
  request after login success.

### F3. Validation

- F3.S1.T1: Run fmt, tests, build, and local smoke validation.
- F3.S1.T2: Update the pending-work memory and progress notes after validation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2 | fake-server test coverage and packet expectation |
| AC-02 | F1.S1.T1 | locale mapping unit coverage |
| AC-03 | F1.S1.T2, F3.S1.T1 | `cargo test` outputs |
| AC-04 | F2.S1.T1 | docs diff |
| AC-05 | F3.S1.T1 | headless/control-http smoke logs |

## Validation Gates

- Gate F0: feature-workflow audit for `.features/20260520-1116-login-character-list-request`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Gate F3: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Gate F4: `rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
- Gate F5: `rtk rg -n "character list|login success|login bootstrap" docs/player-rust-client.md port_rust/README.md`

## Risks

- The locale byte mapping can drift from the legacy client if the config locale
  aliases are not normalized consistently.
- The fake-server script must expect the new packet order or the bootstrap
  tests will become flaky.
- The request must stay inside the local bootstrap path and not leak into the
  control-http smoke surface.

## Rollback

- Remove the automatic roster request and restore the prior fake-server packet
  order if the follow-up destabilizes bootstrap tests.
