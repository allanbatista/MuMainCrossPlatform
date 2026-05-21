# Siege Shell
Status: READY_FOR_PLAN

## Goal

Expose the existing Castle Siege snapshot as a visible Bevy route shell in
the Rust client, and make the siege route reachable through local control
HTTP smoke commands. The slice should stay data-only: reuse the existing
`mu_ui` siege snapshot, register the gameplay cache it needs, and surface the
legacy inactive/observer/soldier/commander views without inventing new siege
gameplay logic.

## Users And Journeys

- QA or a developer opens the siege route and sees the Castle Siege window
  instead of an empty screen.
- QA drives the siege shell from `--control-http` without touching game
  networking.
- Route changes and disconnects tear the siege shell down cleanly.

## Requirements

- The siege shell is visible only on `UiRoute::Siege` while the session is
  connected.
- The siege shell uses the existing `mu_ui::siege_screen()` snapshot.
- The shell reflects one of the known siege modes: inactive, observer,
  soldier, or commander.
- The local control HTTP API can switch the runtime into the siege route for
  smoke testing.
- `GuildCachePlugin` is registered in the graphical runtime so the siege shell
  can project guild mark indices.
- Every code change has automated tests.
- Usage docs mention the siege route and the smoke commands.
- Existing headless/control-http smoke remains deterministic.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Visual/output type | Filters | Datasets/permissions | Empty/loading/error/unavailable behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Siege shell | `UiRoute::Siege` | `siege` | Siege | Visible Bevy route shell | session connected, siege mode command | local runtime state, guild cache | hidden outside the siege route or on disconnect; inactive when no siege command is active; observer/soldier/commander modes mirror the snapshot preset | Players and QA can open the siege window |
| Siege smoke control | local control HTTP | `siege`, `siege-inactive`, `siege-soldier`, `siege-commander` | Siege smoke | HTTP route command | command name, session route | local smoke only | unavailable when control HTTP is disabled; commands update the runtime route snapshot | QA and developers can drive the siege shell without game networking |

## Acceptance Criteria

- AC-01. The siege route shows a visible shell when the runtime is active.
- AC-02. The siege shell reflects the siege mode command and clears on route
  exit or disconnect.
- AC-03. Control HTTP accepts the siege smoke commands and routes the runtime
  accordingly.
- AC-04. Automated tests cover shell visibility, mode mapping, and command
  parsing.
- AC-05. Player-facing docs describe the siege route and the smoke commands.

## Scope

In scope:

- Route-gated siege shell rendering.
- `GuildCachePlugin` registration for the runtime resource.
- Siege control HTTP route commands for smoke testing.
- Usage docs for the siege route shell.

Out of scope:

- Siege networking, battle logic, or command execution.
- New persistence or protocol changes for siege state.
- Other siege interaction flows beyond shell exposure.

## Open Questions

None blocking. Assumption: the first slice surfaces the existing siege snapshot
model as a visible route shell, not as an interactive siege workflow.
