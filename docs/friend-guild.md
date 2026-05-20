# Friend and guild

The Rust port splits the legacy social windows into two `mu_ui` routes:

- `friend` covers the friend roster, letter inbox/compose surface, chat-room
  list, and the global mail/chat alert indicators.
- `guild` covers the guild summary, member management, union list, no-guild,
  and error states.

## Runtime shells

The graphical Rust client now exposes both routes as visible Bevy shells.

- `POST /command?name=friend` opens the friend shell in `mu_client` and
  mirrors the current `MailManager` snapshot.
- `POST /command?name=guild` opens the guild shell in `mu_client` and mirrors
  the current guild snapshot model.
- Both shells clear when the route changes away or the session disconnects.

## Friend

Use `friend_screen` with `FriendScreenState::{Roster, Inbox, Compose,
ChatRooms, Error}`.

The letter tab consumes `mu_gameplay::MailManager` for the selected letter and
compose draft state. The roster tab keeps the friend list and friend-button
alerts aligned with the legacy client.

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
window.

```rust
use mu_ui::{guild_screen, GuildScreenState};

let snapshot = guild_screen(GuildScreenState::Members).snapshot();
```
