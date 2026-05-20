# Friend and Guild View Switch

Status: READY_FOR_EXEC

## Summary

Add control-http-driven view overrides for the friend and guild shells so QA
can smoke-test all visible subviews without touching gameplay networking.

## Interfaces / Contracts

- `mu_app::control_http` stores the selected friend/guild shell view and
  accepts the new commands.
- `mu_app::friend_shell` reads the selected friend view when present.
- `mu_app::guild_shell` reads the selected guild view when present.
- `docs/player-rust-client.md`, `port_rust/README.md`, `port_rust/docs/control-http.md`,
  and `docs/friend-guild.md` describe the smoke commands.

## Phases

### F0. Workflow docs

#### F0.S1 Task

- F0.S1.T1 Create and audit the feature docs for the friend/guild view
  switch slice.

### F1. Runtime wiring

#### F1.S1 Tasks

- F1.S1.T1 Extend the control snapshot with friend/guild view selectors and
  command variants.
- F1.S1.T2 Wire the friend and guild shell renderers to honor the selected
  view when present.

### F2. Tests and docs

#### F2.S1 Tasks

- F2.S1.T1 Add unit tests for control-command parsing, snapshot serialization,
  and shell override behavior.

#### F2.S2 Tasks

- F2.S2.T1 Update usage docs and KB notes for the new smoke commands.

### F3. Validation

#### F3.S1 Tasks

- F3.S1.T1 Run fmt, tests, build, and local smoke validation.
- F3.S1.T2 Update the workflow progress and pending-work notes after
  validation.

