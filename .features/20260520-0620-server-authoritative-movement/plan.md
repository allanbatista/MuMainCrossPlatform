# Server Authoritative Movement

Status: READY_FOR_EXEC

## Summary

Add the first network-backed movement slice after local motion: encode the
world movement request, accept server move updates, keep the rendered local
avatar aligned with the authoritative runtime pose, and document the flow.

## Interfaces / Contracts

- `mu_protocol::movement` owns the movement request helpers and the
  authoritative move-update decoders/encoders.
- `mu_app::bootstrap_runtime` owns the live session bridge and applies
  authoritative move updates to `ClientRuntime`.
- `mu_app::world_motion` captures world-route movement input and routes it to
  the live session when available.
- `mu_app::ClientRuntime` and `mu_gameplay::WorldEntitiesManager` own the local
  avatar pose that the authoritative replies correct.
- `mu_app::WorldScenePlugin` stays a pure projection of runtime pose data.

## Technical Inventory

| Slug | Components | Output type | Queries | Filters | Dataset/permission gating | Retailer/industry compatibility | Renderer/test target | Notes |
|---|---|---|---|---|---|---|---|---|
| `movement-request-bridge` | `mu_app::world_motion`, `mu_app::bootstrap_runtime`, `mu_protocol::movement` | queued walk request | world-route input, live session present | world route active only | live session / offline fallback split | no split | `mu_app` tests | Sends movement requests when the session bridge exists |
| `authoritative-move-sync` | `mu_app::bootstrap_runtime`, `mu_app::client_runtime`, `mu_gameplay::entities`, `mu_app::world_scene` | pose updates | move-character / move-position packets | non-world route / reset / disconnect | live session authoritative replies only | no split | `mu_app` tests | Applies committed server pose to the local avatar and scene marker |
| `movement-protocol-coverage` | `mu_protocol::movement` | packet helpers | request / reply packets | malformed or short packets | protocol layer only | no split | `mu_protocol` tests | Keeps the packet contract testable without the app runtime |
| `movement-docs` | `docs/player-rust-client.md`, `port_rust/README.md` | usage docs | movement controls | authoritative sync note | player-facing documentation only | no split | docs grep | Documents the server-backed movement flow |

## Parallelization

- The protocol helpers and runtime pose reconciliation can be built together.
- The session bridge can be wired once the packet contract is fixed.
- Docs can land after the movement flow is stable.

## Phases

### F0. Feature workflow setup

- F0.S1.T1: Write and audit the feature spec, plan, and progress documents for
  the server-authoritative movement slice.

### F1. Runtime authority

- F1.S1.T1: Add movement packet helpers for authoritative move updates and
  runtime pose reconciliation helpers.
- F1.S1.T2: Add the live session bridge so world-route movement requests can be
  sent and server replies can update `ClientRuntime`.
- F1.S2.T1: Keep movement ignored outside the world route and preserve the
  offline fallback when no live session bridge exists.

### F2. Tests and docs

- F2.S1.T1: Add automated tests for movement request encoding, authoritative
  packet decode, runtime pose sync, and fake-server round trips.
- F2.S2.T1: Update usage documentation with the server-authoritative movement
  flow and the offline fallback.

### F3. Validation

- F3.S1.T1: Run the relevant Cargo tests, build the client, and smoke the
  graphical/headless boot paths.
- F3.S1.T2: Track any remaining movement or network-sync debt in the progress
  and memory files.

## Tasks

| ID | Owner | Planned Files | Dependencies | Done When | Required evidence |
|---|---|---|---|---|---|
| F0.S1.T1 Feature docs | local | `.features/20260520-0620-server-authoritative-movement/spec.md`, `.features/20260520-0620-server-authoritative-movement/plan.md`, `.features/20260520-0620-server-authoritative-movement/progress.md` | none | docs exist and audit passes | feature-workflow audit output |
| F1.S1.T1 Movement protocol helpers | local | `port_rust/crates/mu_protocol/src/movement.rs` | F0.S1.T1 | move-update helpers exist and are unit tested | protocol helper tests |
| F1.S1.T2 Session bridge | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/client_runtime.rs` | F1.S1.T1 | authoritative move replies update runtime pose | runtime / bridge tests |
| F1.S2.T1 Route guards and fallback | local | `port_rust/crates/mu_app/src/world_motion.rs`, `port_rust/crates/mu_app/src/world_scene.rs` | F1.S1.T2 | movement stays ignored outside world route and fallback remains deterministic | route / fallback tests |
| F2.S1.T1 Automated tests | local | `port_rust/crates/mu_app/src/*`, `port_rust/crates/mu_protocol/src/*` | F1 | tests cover protocol, runtime, and fake-server movement round trips | `cargo test` outputs |
| F2.S2.T1 Usage docs | local | `docs/player-rust-client.md`, `port_rust/README.md` | F1 | docs explain the authoritative movement flow and offline fallback | docs diff |
| F3.S1.T1 Validation | local | progress docs | F1-F2 | fmt/test/build/smoke evidence is captured | command output / logs |
| F3.S1.T2 Follow-up tracking | local | `.memory/TODO.md`, progress | F3.S1.T1 | remaining movement/network debt is recorded | memory diff |

## Validation Gates

- Validation Gate F0: feature-workflow audit for the new docs folder
- Validation Gate F1: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_protocol`
- Validation Gate F2: `rtk cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- Validation Gate F3: `rtk cargo test --manifest-path port_rust/Cargo.toml --workspace`
- Validation Gate F4: `rtk cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- Validation Gate F5: graphical and headless local smoke for `mu_client`
- Validation Gate F6: `e2e-validator` handoff for the visible movement path once the live bridge is wired
- Validation Gate F7: `rtk rg -n "movement|avatar|WASD" docs/player-rust-client.md port_rust/README.md`

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T2, F2.S1.T1 | live session bridge and fake-server round trip tests |
| AC-02 | F1.S1.T1, F1.S1.T2, F2.S1.T1 | authoritative pose sync tests |
| AC-03 | F1.S2.T1, F2.S1.T1 | route / reset / disconnect guard tests |
| AC-04 | F1.S1.T1, F2.S1.T1, F3.S1.T1 | `cargo test` output |
| AC-05 | F2.S2.T1, F3.S1.T1 | validation evidence: Validation Gate F7 |
| AC-06 | F3.S1.T1 | Validation Gate F5 headless/control smoke evidence |

## Risks

- Movement can drift if the local prediction path and authoritative updates use
  different pose conversions; keep the conversion helpers in one place.
- The live session bridge must not make the offline/headless smoke path flaky.
- A reset or disconnect must clear the local avatar out of the world scene.

## Rollback

- Revert the movement bridge and authoritative packet handling as one slice if
  the world route becomes unstable.
- Keep the offline fallback path intact so the graphical boot smoke can remain
  deterministic while the networked movement slice is debugged.
