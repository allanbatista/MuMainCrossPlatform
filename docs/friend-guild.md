# Friend and guild

The Rust port splits the legacy social windows into two `mu_ui` routes:

- `friend` covers the friend roster, letter inbox/compose surface, chat-room
  list, and the global mail/chat alert indicators. Live inbox rows come from
  `MailManager` once the letter list has been requested and decoded; until
  then, the shell keeps the sample rows as a fallback.
- `guild` covers the guild summary, member management, union list, no-guild,
  and error states.

## Runtime shells

The graphical Rust client now exposes both routes as visible Bevy shells.

- `POST /command?name=friend` opens the friend shell in `mu_client` and
  mirrors the current `MailManager` snapshot, then overlays decoded friend
  roster data from the live session when it arrives.
- `POST /command?name=friend-roster`, `friend-inbox`, `friend-compose`, and
  `friend-chat-rooms` smoke the matching friend subviews. `friend-inbox`
  also queues one `letter_list_request` per logged-in inbox activation before
  the live letter rows replace the sample inbox.
- `POST /command?name=guild` opens the guild shell in `mu_client` and mirrors
  the current guild snapshot model, then overlays decoded guild score,
  rival-name, member-role, and union data from the live session when it
  arrives.
- `POST /command?name=guild-summary`, `guild-members`, `guild-union`,
  `guild-no-guild`, and `guild-error` smoke the matching guild subviews.
- `POST /command?name=guild-create` queues the guild creation packet when a
  guild name and 32-byte emblem payload are provided as `guild_name=` and
  `guild_emblem=` hex. The name follows the legacy 4-8 character limit.
- Both shells clear when the route changes away or the session disconnects.
- When `friend` or `guild` opens while the session is logged in, the runtime
  queues the matching live list request once per activation and clears that
  latch again when the route exits or the session logs out. The decoded
  friend/guild roster data stays in the shell snapshot until logout or
  disconnect clears it. `friend-inbox` also queues the live letter list once
  per logged-in inbox activation.
- Opening `guild-union` while logged in also queues the alliance list request
  once per activation before the union shell stays visible, and the decoded
  alliance list replaces the placeholder unions when it arrives.

## Friend

Use `friend_screen` with `FriendScreenState::{Roster, Inbox, Compose,
ChatRooms, Error}`.

The letter tab consumes `mu_gameplay::MailManager` for the selected letter and
compose draft state. When `MailManager::letters_loaded()` is true, inbox rows
come from the live letter list; otherwise the roster tab keeps the friend list
and friend-button alerts aligned with the legacy client, and the live session
overlay replaces the placeholder roster list once the decoded friend packet
arrives.

```rust
use mu_gameplay::MailManager;
use mu_ui::{friend_screen, FriendScreenState};

let mut mail = MailManager::new();
mail.set_new_mail_alert(true);
mail.select_letter(0x0102_0304);

let snapshot = friend_screen(FriendScreenState::Inbox, &mail).snapshot();
```

## Guild

Use `guild_screen` with `GuildScreenState::{Summary, Members, Union, NoGuild,
Error}`.

The summary tab shows the guild score, rival guild, and notices. The members
tab exposes the master-only appoint, disband, and fire actions. The union tab
shows allied guilds and the break/banish actions used by the legacy guild
window. `guild-fire` queues the existing member-kick packet when a target
player and security code are provided, and `guild-ban-union` queues the
existing alliance-removal packet when a target guild name is provided. When
the live guild list arrives, the summary and members data are overlaid with
the decoded score, rival name, and member roles from the session. When the
live alliance list arrives, the union tab replaces the placeholder unions
with the decoded allied guilds and member counts.
`guild-create` queues the existing guild create packet when a guild name and
hex-encoded 32-byte emblem payload are provided; the name follows the legacy
4-8 character limit.

```rust
use mu_ui::{guild_screen, GuildScreenState};

let snapshot = guild_screen(GuildScreenState::Members).snapshot();
```
