# Friend and Guild View Switch

Status: READY_FOR_PLAN

## Goal

Add local control-http commands that switch the visible friend and guild
shell subviews in the Rust client. The friend shell should be able to show
roster, inbox, compose, and chat-room views; the guild shell should be able
to show summary, members, union, no-guild, and error views. This slice keeps
the existing snapshot models intact and only adds a smoke-friendly view
selector on top.

## Users And Journeys

- QA opens the friend route and smoke-tests the roster, inbox, compose, and
  chat-room layouts from `--control-http`.
- QA opens the guild route and smoke-tests the summary, members, union, and
  no-guild layouts from `--control-http`.
- The shells still clear on route exit and disconnect.
- The visible route state remains deterministic and local; no gameplay
  network changes are required for this slice.

## Requirements

- Control HTTP accepts dedicated commands for the friend and guild subviews.
- The control snapshot stores the selected friend and guild view state.
- The friend shell prefers the selected view state when present and falls
  back to the current mail-driven behavior otherwise.
- The guild shell prefers the selected view state when present and falls back
  to the current session-phase-driven behavior otherwise.
- The user-facing docs describe the new commands as smoke-test selectors.
- Automated tests cover parsing, snapshot serialization, and shell override
  behavior.

## Scope

In scope:

- Control-plane view selectors for the visible friend and guild shells.
- Snapshot-backed shell overrides for the social route smoke path.
- Usage docs for the new commands.

Out of scope:

- Live friend, letter, or guild gameplay packet wiring.
- New social persistence or backend state.
- Changes to the existing friend/guild snapshot models beyond the override
  hook.

