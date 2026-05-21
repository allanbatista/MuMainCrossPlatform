# Party UI

The legacy party flow has two UI surfaces:

- `src/source/UI/NewUI/Party/NewUIPartyInfoWindow.cpp`
- `src/source/UI/NewUI/Party/NewUIPartyListWindow.cpp`

The shared gameplay state lives in `mu_gameplay::party::PartyManager`. The Rust
UI snapshot in `mu_ui::party` reads that resource and exposes four states:

- `info`: the large party info window with member cards, HP bars, map names,
  coordinates, and leave controls.
- `list`: the mini party list with row colors derived from the search index
  sentinels and the current selection.
- `empty`: the no-party placeholder.
- `error`: the sync-failed fallback.

Behavior notes:

- The legacy manager still uses the sentinels `-3` hero, `-2` unsearched, and
  `-1` not found.
- The list window colors rows green for found members, red for missing members,
  and keeps hero/unsearched rows in the default background.
- The info window renders the HP bar width with the legacy `currHP / maxHP`
  ratio capped to the bar width.
- The Rust snapshot resolves party map names through the shared legacy map
  name helper so the visible labels match the map manager output.
- The Rust party shell requests the live party list once per logged-in
  activation, decodes the legacy `0x42` list, `0x43` leave, and `0x44` info
  packets into `PartyManager`, and clears party state on logout, disconnect,
  and hard error paths.
- The empty-state `InviteMember` action maps to `POST /command?name=party-invite`
  and queues the legacy invite packet when you pass a target player id.
