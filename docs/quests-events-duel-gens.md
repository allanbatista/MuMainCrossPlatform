# Quests, Events, Duel and Gens

The Rust port models the legacy quest, event, duel and Gens surfaces as
separate gameplay resources plus UI snapshot modules.

## Quests

- `mu_gameplay::quests::QuestManager` keeps the current NPC, active quest
  lists, selected quest, dialogue state and reward state.
- `mu_ui::quests::quests_screen()` mirrors the legacy quest journal, dialogue,
  reward and etc-quest views as snapshots.

## Events

- `mu_gameplay::events::EventManager` tracks the active event kind, entry
  parameters, countdown, reward text and result text.
- `mu_ui::events::events_screen()` covers the entry, countdown, reward,
  result and error views used by the event windows.

## Duel

- `mu_gameplay::duel::DuelManager` keeps the duel enable flags, two player
  slots, channel list, spectator list and regenerated flag.
- `mu_ui::duel::duel_screen()` mirrors the challenge, channel list, watching
  and error views.
- The Rust client also exposes the duel route as a visible Bevy shell and
  the local control HTTP smoke path can open it with `duel`.

## Gens

- `mu_gameplay::gens::GensManager` keeps the Gens type, contribution,
  ranking, team name, title name and reward availability.
- `mu_ui::gens::gens_ranking_screen()` exposes the HUD overlay used for the
  ranking and join/reward states.

The packet helpers for these flows live in `mu_protocol::quests`,
`mu_protocol::events`, and `mu_protocol::social`.
