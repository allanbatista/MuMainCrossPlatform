# Character Select Command

Status: READY_FOR_EXEC

## Summary

Replace the bootstrap's character-select auto-advance with an explicit manual
selection request path, then expose that request through the local control HTTP
API and update the usage docs.

## Technical Inventory

- `mu_app::bootstrap_runtime` owns the session worker and the command queue
  used to send `select_character`.
- `mu_app::control_http` owns the local HTTP control surface and its snapshot
  serialization.
- `mu_app::graphical_runtime` bridges control HTTP state into the runtime
  resources.
- `mu_protocol::login::select_character` remains the packet helper.
- `docs/player-rust-client.md`, `port_rust/README.md`, and
  `port_rust/docs/control-http.md` are the usage docs that must describe the
  new behavior.

## Interfaces / Contracts

- `BootstrapRuntime::queue_character_select_request(name)` queues a manual
  selection packet for the worker thread.
- `ControlCommand` gains `select-character` as a payload-bearing command.
- `ControlSnapshot` stores the requested character name so the runtime bridge
  can forward it once per command.
- `POST /command?name=select-character` accepts the requested character name in
  the body or `character=` query parameter.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `character-select-wait` | `mu_app::bootstrap_runtime`, `mu_app::graphical_runtime`, `mu_ui::routes` | auth shell wait state | character list received, selection command pending | logged-in session, roster loaded | local session/bootstrap only | no split | `mu_app` tests | Client stays on character select until a selection command arrives |
| `select-character-command` | `mu_app::control_http`, `mu_app::graphical_runtime`, `mu_app::bootstrap_runtime`, `mu_protocol::login` | HTTP control command + network packet | `select-character` | character name payload | local control-http access | no split | control-http and bootstrap tests | Bridges the HTTP request into `select_character` once per command |
| `select-character-docs` | `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`, KB | usage docs | explicit selection command | manual selection only | docs only | no split | docs/grep checks | Explains the new waiting behavior and request shape |

## Parallelization

- The bootstrap worker change and the control HTTP command parsing can be
  implemented independently once the queue method contract is fixed.
- Docs and KB updates can happen after the command flow and tests are settled.

## Phases

### F0. Workflow docs

- F0.S1.T1: Write the feature spec, plan, and progress documents and audit the
  new slice boundary.

### F1. Bootstrap selection path

- F1.S1.T1: Add a manual character-selection queue method to
  `bootstrap_runtime.rs` and stop the worker from auto-selecting the first
  roster entry.
- F1.S1.T2: Update the fake-server bootstrap tests so they wait for character
  select and then queue the manual selection request before world entry.

### F2. Control HTTP bridge

- F2.S1.T1: Extend `control_http.rs` so `select-character` accepts a character
  name payload, stores it in the snapshot, and rejects empty requests.
- F2.S1.T2: Bridge the new snapshot command from `graphical_runtime.rs` into
  `BootstrapRuntime` so local automation can drive the live session.

### F3. Tests and docs

- F3.S1.T1: Update the usage docs and KB notes to explain the explicit
  selection command and the waiting behavior.

### F4. Validation

- F4.S1.T1: Run the relevant Cargo tests, build the client, and execute the
  local control-http and fake-server smoke checks.
- F4.S1.T2: Record any blockers or follow-up work in the progress and memory
  files.

### F5. Final review

- F5.S1.T1: Hand the graphical client and control-http flow to the
  e2e-validator if the API smoke exposes a UI/routing concern that needs
  browser-level confirmation.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2 | Validation evidence: bootstrap tests and packet-order assertions show the worker no longer auto-selects and waits for a manual request. |
| AC-02 | F1.S1.T1, F2.S1.T2 | Validation evidence: the queued manual selection path sends `select_character` through the live session. |
| AC-03 | F2.S1.T1 | Validation evidence: control-http request/response tests reject empty selection payloads and serialize the requested character name. |
| AC-04 | F1.S1.T2, F2.S1.T2, F4.S1.T1 | Validation evidence: local control-http and fake-server smoke logs show the client stays on character select until `select-character` is issued. |
| AC-05 | F3.S1.T1, F4.S1.T1 | Validation evidence: player docs, README, control-http docs, and KB notes describe the new manual selection command and waiting behavior. |

## Validation Gates

- Gate F0: `rtk node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs .features/20260520-1146-character-select-command`
- Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime`
- Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app control_http graphical_runtime`
- Gate F3: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Gate F4: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Gate F5: local control-http smoke that posts `select-character` and reaches world
- Gate F6: fake-server smoke that waits on character select, queues the manual
  selection request, and verifies the world handoff
- Gate F7: e2e-validator handoff for the character-select command flow if the
  API smoke exposes a visible routing or focus issue

## Risks

- Removing the auto-select shortcut will stall existing smokes until they issue
  the new command, so every validation path must be updated together.
- The control-plane payload must stay explicit; if the request body is empty,
  the client should fail fast instead of guessing a roster entry.
- The runtime bridge must only forward the selection once per request or the
  worker will resend the packet on every frame.

## Rollback

- Restore the auto-select shortcut in `bootstrap_runtime.rs` if the new manual
  command path blocks bootstrap validation.
- Keep the control HTTP payload handling isolated so the command can be rolled
  back without touching the world handoff or other auth commands.
