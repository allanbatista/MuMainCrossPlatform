# MU Helper

The Rust port keeps the legacy MU Helper configuration as a local model that
can be saved to the server payload.

## What the helper stores

- Hunting range: `0..=6`
- Loot range: `1..=8`
- Return timeout: `0..=15`
- Potion/heal/party-heal thresholds: `0..=100` in steps of `10`
- Three buff skills and one basic skill plus two conditional skill slots
- Up to `12` extra item filters, each up to `15` ASCII characters

## Behavior

- `buff duration` and `buff duration party` follow the legacy timer flags.
- `dark raven` uses the legacy `Cease` / `Auto` / `Together` modes.
- `self defense`, `auto accept friend`, and `auto accept guild` stay local to
  the client config and are not part of the saved helper payload.
- The saved server payload preserves the legacy byte layout used by the C++
  client.

## Notes

- Empty extra-item entries are ignored.
- The helper save payload is still the legacy fixed-size packet body; the Rust
  model validates the user-facing limits before serializing it.
