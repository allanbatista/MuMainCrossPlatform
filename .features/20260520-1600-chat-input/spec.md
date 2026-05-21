# Chat Input

Status: READY_FOR_PLAN

## Goal

Let the Chat route accept typed text and send a public chat packet through the
live session when the player presses Enter, so chat becomes an actual gameplay
surface instead of a read-only shell.

## Users And Journeys

- Player opens the Chat route, types a message, sees the draft update on
  screen, presses Enter, and the message is sent to the live session.
- QA opens the Chat route through `--control-http` and validates the draft/send
  flow in the graphical client.
- Maintainer validates the slice with tests and a fake-server smoke.

## Requirements

- The Chat route stays snapshot-driven for the visible shell.
- The composer must accept printable text, Backspace, Escape, and Enter.
- Enter sends the current draft as a public chat packet through the live
  session and clears the draft on success.
- Draft length must stay within the legacy chat box limit.
- The composer must not send while disconnected or while no bootstrap session
  exists.
- Existing route shells and gameplay surfaces keep their current behavior.
- Usage docs must mention how to type and send chat.

## Acceptance Criteria

- AC1: Typing updates the draft shown in the Chat shell.
- AC2: Backspace removes the last character and Escape clears the draft.
- AC3: Pressing Enter queues a public chat packet through a fake-server-backed
  test and clears the draft on success.
- AC4: Docs mention the chat input behavior.
- AC5: Automated tests cover draft editing and send wiring.

## Scope

In scope:

- `mu_app` chat composer input handling.
- `BootstrapRuntime` chat send command.
- Chat shell draft/status rendering.
- Usage docs.

Out of scope:

- Whisper/private chat modes.
- Party, guild, and gens prefixes.
- Chat history navigation.
- World HUD chat overlay integration.
- Chat server packet receive handling.

## Open Questions

None blocking.
