# Guild Union Live Decoding Plan

Status: READY_FOR_EXEC

## Technical Inventory

| Slug/ID | Components | Output type | Queries | Filters / URL state | Datasets / permissions | Renderer / test target | Retailer / industry compatibility | Compatibility expectations |
|---|---|---|---|---|---|---|---|---|
| `guild-union-live` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_app/src/guild_shell.rs` | live union roster overlay | alliance-list packets, guild union tab render | no extra filters; session-scoped only | active session union packet; logged-in player session required | `mu_app` unit tests and shell snapshot coverage | not applicable | overlay live union data without changing the shell route contract |
| `legacy-guild-union-layout` | `src/source/Network/Server/WSclient.cpp`, `src/source/Network/Server/WSclient.h` | packet layout and member-count mapping | `0xE9` alliance-list response | n/a | legacy server response layout | decode fixtures against legacy layout | not applicable | source of truth for decode shape and roster metadata |

## Interfaces / Contracts

- `BootstrapSignal` gains a decoded guild-union roster variant.
- `mu_app` gains runtime union roster storage for live alliance data.
- Guild shell rerender keys include live union snapshots so packet updates
  repaint the union view.
- No public CLI or HTTP contract changes.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F1.S1.T2 | packet decode tests and guild shell snapshot coverage |
| AC-02 | F2.S1.T1 | shell rerender tests and build/smoke logs |
| AC-03 | F1.S1.T3 | reset tests and logout/disconnect validation logs |
| AC-04 | F3.S1.T1 | unit test output and shell snapshot evidence |
| AC-05 | F3.S1.T1, F3.S1.T2 | validation evidence: docs diff and final docs grep/logs |

## Phases

### F0 Docs

Audit the legacy packet layout, then record the code and packet inventory in
KB.

### F1 Decode and Store

Add the runtime union roster resource and packet decode path for alliance
lists.

### F2 Shell Overlay

Overlay the decoded union data into the guild shell and ensure the view
rerenders when packet state changes.

### F3 Validate and Document

Update usage docs, then run the format, test, build, and smoke gates.

## Validation Gates

- `cargo fmt --manifest-path port_rust/Cargo.toml --all`
- `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
- `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`
- `e2e-validator`: validate live union roster rendering in `mu_client`,
  confirm logout/disconnect reset behavior, and capture screenshots/logs.
- `rtk rg -n "live union|alliance list|guild-union" docs/friend-guild.md docs/player-rust-client.md port_rust/README.md port_rust/docs/control-http.md`

## Risks

- Packet layout drift versus the legacy structs.
- The shell must include union state in its rerender keys or the UI will stay
  stale after packet updates.
- Logout and disconnect must reset the union state or the next session will
  inherit stale data.
