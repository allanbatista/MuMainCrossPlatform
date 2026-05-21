---
status: READY_FOR_PLAN
---

# Friend Mail Inbox Live Data

## Goal

When the logged-in friend inbox opens in the Rust client, the live session
should populate the inbox letter list and keep delete/remove updates in sync
instead of showing only the hardcoded sample letters. The visible friend and
guild shells stay in place; this slice only wires the live inbox data path and
the one-shot letter list request needed to feed it.

## Users And Journeys

- QA/dev logs in, opens the friend inbox, and sees live letters from the
  session.
- QA/dev deletes a letter and the inbox removes it from the live list.
- QA/dev leaves the friend route or logs out, then re-opens the inbox and the
  client requests the letter list again.
- If the session is disconnected, the client does not queue a live request and
  the mail state clears.

## Requirements

- The friend inbox must queue `letter_list_request` once per logged-in inbox
  activation.
- The bootstrap runtime must decode `FS_LETTER_ALERT` packets into live mail
  state with sender, subject, date, time, read flag, and letter id.
- The bootstrap runtime must decode `FS_LETTER_RESULT` delete responses and
  remove the matching letter from live state.
- The friend inbox snapshot must render live letters when present and can fall
  back to the existing sample inbox only when no live mail has arrived yet.
- Logout and disconnect must clear live mail state and reset the inbox request
  guard.
- Existing friend roster/chat-room route behavior and control-http overrides
  must stay unchanged.
- Automated tests must cover the mail data model, packet decode, request
  gating, and live inbox snapshot overlay.
- Usage docs must mention that opening the friend inbox now asks the live
  session for the letter list.

## Product Inventory

| Route/Page | Slug/ID | User-visible label | Visual / output type | Filters | States | Persona differences |
|---|---|---|---|---|---|---|
| Friend inbox | `friend-mail-inbox` | Friend | Visible Bevy friend shell with live letter rows | logged-in friend inbox activation | `roster`, `inbox`, `compose`, `chat-rooms`, `error`; inbox can be empty, live, or error | QA/dev can smoke it through the route or control-http; players see the same inbox surface |

## Interfaces / Contracts

- `MailManager` gains live inbox letter storage and mutation helpers.
- `BootstrapRuntime` gains a `LetterListRequest` command and mail packet
  signals.
- `FriendShellState` gains a one-shot inbox request guard.
- `friend_screen` uses live mail entries from `MailManager` when present.
- No new CLI flags or control-http routes are required.

## Scope

In scope:

- inbox request-on-open for the friend route
- decoding letter alert and delete-result packets
- live friend inbox snapshot overlay
- docs and KB sync

Out of scope:

- letter body cache/read flow from `FS_LETTER_TEXT`
- letter send/compose submission
- chat-room invitation flow
- new friend/guild route controls

## Acceptance Criteria

- AC-01. Opening the friend inbox while logged in queues `letter_list_request`
  once.
- AC-02. `FS_LETTER_ALERT` packets populate the live inbox letter list with
  the decoded sender, subject, date, time, and read state.
- AC-03. `FS_LETTER_RESULT` delete success removes the matching letter from the
  live inbox state.
- AC-04. Leaving the route, logging out, or disconnecting clears the mail
  state and resets the request guard.
- AC-05. Usage docs mention that the inbox now asks the live session for the
  letter list when available.

## Open Questions

Assumption: when no live mail has arrived yet, the inbox may continue to show
the existing sample rows until the first decoded alert replaces them. The
letter-body cache/read flow is explicitly deferred to the next slice.
