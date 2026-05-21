---
status: OPEN
---

# Friend Mail Inbox Live Data Progress

Current state: the slice is complete. Live friend inbox letters now flow
through the mail resource, request-on-open gate, bootstrap packet decode, UI
overlay, and the visible windowed e2e pass.

## Tasks

| ID | Status | Owner | Planned Files | Actual Files | Required Evidence | Produced Evidence | Blocker |
|---|---|---|---|---|---|---|---|
| F0.S1.T1 | done | local | `.features/20260520-1730-friend-mail-inbox-live-data/spec.md`, `.features/20260520-1730-friend-mail-inbox-live-data/plan.md`, `.features/20260520-1730-friend-mail-inbox-live-data/progress.md` | `.features/20260520-1730-friend-mail-inbox-live-data/spec.md`, `.features/20260520-1730-friend-mail-inbox-live-data/plan.md`, `.features/20260520-1730-friend-mail-inbox-live-data/progress.md` | feature-workflow audit output | docs drafted | none |
| F1.S1.T1 | done | local | `port_rust/crates/mu_gameplay/src/mail.rs`, `port_rust/crates/mu_gameplay/src/lib.rs` | `port_rust/crates/mu_gameplay/src/mail.rs`, `port_rust/crates/mu_gameplay/src/lib.rs` | live letter storage and mutation helpers | `MailManager` stores live letters plus loaded/alert state | none |
| F1.S1.T2 | done | local | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | letter-alert/delete packet decode and mail-state application | `0xC6`/`0xC8` decode and live mail mutations | none |
| F1.S2.T1 | done | local | `port_rust/crates/mu_gameplay/src/mail.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_gameplay/src/mail.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | unit tests for mail state and packet decode | mail model tests + bootstrap mail packet tests | none |
| F2.S1.T1 | done | local | `port_rust/crates/mu_app/src/friend_shell.rs` | `port_rust/crates/mu_app/src/friend_shell.rs` | one-shot inbox request guard | separate friend and inbox request latches | none |
| F2.S1.T2 | done | local | `port_rust/crates/mu_ui/src/friend.rs`, `port_rust/crates/mu_app/src/friend_shell.rs` | `port_rust/crates/mu_ui/src/friend.rs`, `port_rust/crates/mu_app/src/friend_shell.rs` | live inbox snapshot overlay | live mail rows render when loaded | none |
| F2.S2.T1 | done | local | `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | `port_rust/crates/mu_app/src/friend_shell.rs`, `port_rust/crates/mu_app/src/bootstrap_runtime.rs` | request-gating and overlay tests | request latch + live inbox coverage | none |
| F2.S3.T1 | done | local | `docs/player-rust-client.md`, `port_rust/README.md`, `docs/friend-guild.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/README.md`, `.codexpotter/kb/friend-mail-inbox-live-data.md` | `docs/player-rust-client.md`, `port_rust/README.md`, `docs/friend-guild.md`, `port_rust/docs/control-http.md`, `.codexpotter/kb/README.md`, `.codexpotter/kb/friend-mail-inbox-live-data.md` | usage docs and KB index updates | request-on-open inbox behavior documented | none |
| F3.S1.T1 | done | local | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.memory/TODO.md` | `.codexpotter/projects/2026/05/20/1/MAIN.md`, `.memory/TODO.md` | fmt/test/build/smoke logs and workflow sync | fmt/test/build/headless smoke passed; visible client run on `127.0.0.1:45415` showed the `MU Rust Client` window, `friend-inbox` set `ui_route=friend` and `friend_screen_state=inbox`, and `exit` closed the session | none |

## Files Touched

New:

- `.features/20260520-1730-friend-mail-inbox-live-data/spec.md`
- `.features/20260520-1730-friend-mail-inbox-live-data/plan.md`
- `.features/20260520-1730-friend-mail-inbox-live-data/progress.md`

Modified:

- `docs/player-rust-client.md`
- `docs/friend-guild.md`
- `port_rust/README.md`
- `port_rust/docs/control-http.md`
- `port_rust/crates/mu_gameplay/src/lib.rs`
- `port_rust/crates/mu_gameplay/src/mail.rs`
- `port_rust/crates/mu_app/src/bootstrap_runtime.rs`
- `port_rust/crates/mu_app/src/friend_shell.rs`
- `port_rust/crates/mu_ui/src/friend.rs`
- `.codexpotter/kb/README.md`
- `.codexpotter/kb/friend-mail-inbox-live-data.md`
- `.codexpotter/projects/2026/05/20/1/MAIN.md`
- `.memory/TODO.md`

Removed:

- none

## Done

- Wired live mail inbox data through the gameplay mail resource, bootstrap
  packet decode, friend-shell request latch, and friend UI overlay, then
  updated the docs/KB index and verified the slice with targeted crate tests,
  workspace tests, build, and headless smoke runs. Files changed:
  `port_rust/crates/mu_gameplay/src/lib.rs`,
  `port_rust/crates/mu_gameplay/src/mail.rs`,
  `port_rust/crates/mu_app/src/bootstrap_runtime.rs`,
  `port_rust/crates/mu_app/src/friend_shell.rs`,
  `port_rust/crates/mu_ui/src/friend.rs`,
  `docs/player-rust-client.md`, `docs/friend-guild.md`,
  `port_rust/README.md`, `port_rust/docs/control-http.md`,
  `.codexpotter/kb/README.md`,
  `.codexpotter/kb/friend-mail-inbox-live-data.md`.
