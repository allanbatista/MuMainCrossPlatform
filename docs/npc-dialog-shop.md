# NPC dialog and shop

The Rust port splits the legacy NPC interaction UI into two player-facing
surfaces:

- `mu_ui::npc` covers the dialogue/quest/confirmation window.
- `mu_ui::shop` covers the NPC buy/sell/repair window.
- `mu_gameplay::npc` keeps the in-memory interaction state that the UI can read
  and mutate during a session.

## NPC dialog

The dialogue surface is used for normal NPC talk, quest lists, confirmation
prompts, and visible error states.

Typical usage:

```rust
use mu_gameplay::NpcManager;

let mut npc = NpcManager::new();
npc.open_dialogue(236, "Marlon", 18);
npc.set_quest_list_mode(true);
npc.set_dialogue_page(1, 3);
```

## NPC shop

The shop surface mirrors the legacy NPC shop window:

- buy and sell mode is the default state;
- repair mode is available only for NPCs that support it;
- the sale flow can expose a confirm/cancel state;
- failure states stay visible instead of silently closing.

Typical usage:

```rust
use mu_gameplay::NpcManager;

let mut npc = NpcManager::new();
npc.open_shop(236, "Potion Merchant", 5, true);
npc.toggle_shop_mode();
npc.set_shop_selling_item(true);
```

The tests for this slice cover the gameplay state resource plus the `npc` and
`shop` UI snapshots.
