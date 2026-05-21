# Trade, player shop, and mail

The Rust port splits the legacy social item flows into gameplay state objects
and UI snapshots:

- `mu_gameplay::trade` tracks the active trade partner, gold, confirmation, and
  trade wait state.
- `mu_ui::trade` renders the trade window snapshot for request, active,
  confirming, and error states.
- `mu_gameplay::player_shop` tracks the personal shop title, open state, item
  selection, and price-entry dialog.
- `mu_ui::player_shop` renders the player-shop marketplace snapshot for edit,
  open, pricing, and error states.
- `mu_gameplay::mail` tracks mail alerts, compose state, the selected letter,
  and the local read/delete actions used by the friend route.

## Trade

Use `TradeManager` to mirror the legacy request/confirm flow:

```rust
use mu_gameplay::TradeManager;

let mut trade = TradeManager::new();
trade.request_trade("Blade", 320, 4);
trade.accept_trade();
trade.set_my_trade_gold(150_000);
trade.toggle_my_confirmed();
```

The trade manager keeps the partner id normalized to the legacy username
length and restores the confirm wait timer when a confirmed gold value changes.

## Player shop

Use `PlayerShopManager` to track the personal shop editor and price dialog:

```rust
use mu_gameplay::PlayerShopManager;

let mut shop = PlayerShopManager::new();
shop.set_title("Ares Market");
shop.set_item_count(8);
shop.set_personal_shop_enabled(true);
shop.set_item_selection(Some(4), Some(0));
shop.set_price(250_000);
```

The `marketplace` UI route shows the same flow in snapshots. Title input is
clamped to the legacy shop-title length before the shop can open.

## Mail

Use `MailManager` to keep the mail alert and compose state in memory:

```rust
use mu_gameplay::MailManager;

let mut mail = MailManager::new();
mail.set_new_mail_alert(true);
mail.set_compose("Blade", "Daily note", "Meet in Lorencia.");
mail.select_letter(0x0102_0304);
mail.mark_letter_read(0x0102_0304);
mail.remove_letter(0x0102_0304);
```

Recipient, subject, and body fields are clamped to the legacy compose limits.
The mail state stays in gameplay and feeds the friend route without
duplicating the compose limits. The friend inbox actions use the selected
`letter_id` to mark a letter as read or delete it from the live inbox.
