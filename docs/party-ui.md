# Party UI

The legacy party flow has two UI surfaces:

- `src/source/UI/NewUI/Party/NewUIPartyInfoWindow.cpp`
- `src/source/UI/NewUI/Party/NewUIPartyListWindow.cpp`

The shared gameplay state lives in `mu_gameplay::party::PartyManager`. The Rust
UI snapshot in `mu_ui::party` reads that resource and exposes four states:

- `info`: the large party info window with member cards, HP bars, map id,
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
- The Rust snapshot keeps the numeric party map id from `PartyMemberInfo`; the
  legacy client resolves the display name through the map manager.
