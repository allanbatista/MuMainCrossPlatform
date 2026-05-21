---
status: READY_FOR_EXEC
---

# Friend Mail Inbox Live Data Plan

## Technical Inventory

| Slug/ID | Components | Output type | Queries / packets | Filters / URL state | Datasets / permissions | Renderer / test target | Retailer/industry compatibility | Compatibility expectations |
|---|---|---|---|---|---|---|---|---|
| `friend-inbox-live-mail` | `port_rust/crates/mu_gameplay/src/mail.rs`, `port_rust/crates/mu_ui/src/friend.rs`, `port_rust/crates/mu_app/src/friend_shell.rs` | friend inbox letter rows | live `MailManager` state and inbox selection | friend route, inbox state, logged-in session | live session plus existing mail snapshot | `mu_ui` snapshot tests and `mu_app` shell tests | N/A - no retailer/industry split | preserve the current friend route and control-http overrides |
| `letter-list-request` | `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_protocol/src/social.rs` | one-shot inbox request side effect | `letter_list_request` packet | logged-in inbox activation | live session only | `mu_app` request-gating tests | N/A - no retailer/industry split | request once per activation, reset on logout/disconnect or route exit |
| `letter-alert-delete` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs`, `port_rust/crates/mu_gameplay/src/mail.rs` | inbox list mutation | `FS_LETTER_ALERT`, `FS_LETTER_RESULT` / delete | packet-driven, session-scoped | live session packets | packet decode tests and mail-state tests | N/A - no retailer/industry split | keep the letter-body follow-up out of this slice |
| `friend-mail-usage-docs` | `docs/player-rust-client.md`, `port_rust/README.md`, `docs/friend-guild.md`, `port_rust/docs/control-http.md`, KB | usage notes | route-open behavior | no new URL state | player/dev usage only | docs grep and KB index | N/A - no retailer/industry split | stay aligned with the visible shell behavior |

## Interfaces / Contracts

- `MailManager` gains live inbox letter storage plus helpers to upsert and
  delete letters.
- `BootstrapSignal` gains mail alert and mail delete variants for the friend
  inbox packets.
- `BootstrapCommand` gains a `LetterListRequest` command.
- `friend_shell` tracks a one-shot inbox request guard scoped to the logged-in
  activation.
- `friend_screen` prefers live inbox letters from `MailManager` when they are
  available and keeps the sample rows as a fallback only.
- No new CLI flags or control-http routes are required.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1, F2.S1.T1, F2.S2.T1 | request-gating tests and bootstrap packet-send tests |
| AC-02 | F1.S1.T1, F1.S1.T2, F2.S1.T2 | mail-state tests and inbox snapshot tests |
| AC-03 | F1.S1.T1, F1.S1.T2, F2.S1.T2 | delete-result decode tests and snapshot overlay tests |
| AC-04 | F1.S1.T2, F2.S1.T1, F2.S2.T1 | logout/disconnect reset tests and request-latch reset coverage |
| AC-05 | F2.S3.T1, F3.S1.T1 | docs/player-rust-client.md, port_rust/README.md, docs/friend-guild.md, port_rust/docs/control-http.md, `.codexpotter/kb/README.md`, `rtk rg -n "letter_list_request|friend inbox|mail" docs/player-rust-client.md port_rust/README.md docs/friend-guild.md port_rust/docs/control-http.md` output, and final validation logs |

## Phases

### F0 - Workflow docs

#### Tasks

- F0.S1.T1 Create and audit the feature workflow docs for the friend mail
  inbox live-data slice.

#### Validation gate

- `node ~/.codex/skills/feature-workflow/scripts/audit-feature-docs.mjs
  .features/20260520-1730-friend-mail-inbox-live-data`

### F1 - Mail data and packet decode

#### Tasks

- F1.S1.T1 Extend `MailManager` with live inbox letter storage and mutation
  helpers.
- F1.S1.T2 Decode the friend inbox letter alert and delete-result packets into
  the live mail state.
- F1.S2.T1 Add unit tests for the mail data model and packet decode.

#### Validation gate

- `cargo test --manifest-path port_rust/Cargo.toml -p mu_gameplay mail`
- `cargo test --manifest-path port_rust/Cargo.toml -p mu_app bootstrap_runtime`

### F2 - Inbox request and snapshot overlay

#### Tasks

- F2.S1.T1 Add the one-shot letter list request guard to the friend inbox
  shell.
- F2.S1.T2 Render live inbox letters in `mu_ui::friend` and `mu_app::friend_shell`.
- F2.S2.T1 Add unit tests for the inbox request gating and live snapshot
  overlay.
- F2.S3.T1 Update usage docs and the KB note/index so the inbox request-on-
  open behavior is visible to players and developers.

#### Validation gate

- `cargo test --manifest-path port_rust/Cargo.toml -p mu_app friend_shell`
- `cargo test --manifest-path port_rust/Cargo.toml -p mu_app --workspace`

- AC-05 validation evidence: `rtk rg -n "letter_list_request|friend inbox|mail" docs/player-rust-client.md port_rust/README.md docs/friend-guild.md port_rust/docs/control-http.md`

### F3 - Final validation

#### Tasks

- F3.S1.T1 Update the main progress file and pending-work memory, then run the
  format, test, build, smoke, and e2e handoff gates.

#### Validation gate

- `cargo fmt --manifest-path port_rust/Cargo.toml --all`
- `cargo test --manifest-path port_rust/Cargo.toml -p mu_app`
- `cargo test --manifest-path port_rust/Cargo.toml --workspace`
- `cargo build --manifest-path port_rust/Cargo.toml -p mu_client`
- `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless`
- `cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0`
- `e2e-validator` handoff for a windowed inbox smoke run when the runtime is available
- `timeout 5s cargo run --manifest-path port_rust/Cargo.toml -p mu_client`

## Parallelization

- The mail data-model tests and the friend-shell request-gating tests can run
  in parallel once the shared packet decode helpers are in place.
- Docs/KB updates can land before the final validation evidence is captured.

## Risks

- The mail packet layout is fixed-width; an off-by-one in the date, time, or
  subject lengths will corrupt the decoded inbox rows.
- The inbox request guard must reset on route exit, logout, and disconnect or
  the next activation will not refresh.
- The sample inbox fallback should remain available until the first live
  packet arrives so headless and offline smoke still have visible content.
