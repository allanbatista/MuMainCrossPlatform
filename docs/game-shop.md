# GameShop

The Rust port keeps the legacy in-game cash shop split into a gameplay runtime
resource and a UI snapshot layer.

## Gameplay runtime

- `mu_gameplay::GameShopManager` owns the current mode, wallet, catalog,
  storage, execution flags, and validation error.
- `GameShopMode` exposes `empty`, `catalog`, `details`, `storage`, and `error`
  states.
- `show_catalog`, `show_details`, `show_storage`, `show_empty`, `show_error`,
  `set_versions`, and `set_wallet` keep the runtime visible for the UI.

## UI snapshot

- `mu_ui::game_shop_screen` renders `GameShopManager` into a pure snapshot for
  the `game-shop` route.
- The visible states cover catalog, details, storage, empty, and error.
- The action bar changes with the visible mode so tests can assert the current
  affordances.

## Packet helpers

- `mu_protocol::cash_shop` provides the request helpers used by the legacy
  flow: point info, open/close state, storage list, item buy/gift,
  delete/consume storage item, and event item list.
- The gameplay tests use `mu_network::FakeServer` to verify the expected
  packet sequence without depending on the live server.

## Transaction safety

- `mu_gameplay::GameShopTransactionManager` keeps a single in-flight shop
  action, records the last completion outcome, and lets `GameShopManager`
  clear pending work when the shop closes.
- `mu_network::shop::CashShopClient` wraps the cash-shop request helpers with
  the same single-active-transaction guard and clears pending work on
  disconnect or reconnect.
- Gift submission logs redacted receiver and message fields so the transaction
  trace stays useful without leaking private content.

## Example

```rust
use mu_gameplay::GameShopManager;
use mu_ui::game_shop_screen;

let mut game_shop = GameShopManager::new();
game_shop.set_wallet(1200.0, 540.0, 150.0, 25.0, 90.0);
game_shop.show_catalog("Lorencia", "Featured", 1, 4, 12, 8);

let screen = game_shop_screen(&game_shop);
assert_eq!(screen.state.as_str(), "catalog");
```

The slice is covered by unit tests for the gameplay resource, the UI snapshot,
and a fake-server packet-sequence smoke test.
