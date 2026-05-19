# Duel System

The Rust port now mirrors the legacy duel state in `mu_gameplay::duel`.
It is still an in-memory gameplay resource, not a full playable game loop.

## What it tracks

- Duel enabled / disabled state.
- Pet duel enablement.
- Two duel player slots with player index, ID, score, HP rate, and SD rate.
- Up to four duel channels with enabled/joinable flags and the two player IDs.
- A watch list for spectators.
- The fighter regenerated flag used by the legacy duel flow.

## Legacy matching rule

The duel manager matches a character to a duel slot in two ways:

- direct match: character key and ID match the duel player slot;
- summon match: the character is a summon, its owner ID matches the duel player
  ID, and summon handling is enabled for that check.

The legacy client limits usernames to `MAX_USERNAME_SIZE = 10`, and the Rust
state keeps the same limit when copying duel IDs.

## Usage

- Enable duel state before filling the player slots.
- Use the channel entries to represent joinable duel rooms.
- Add or remove spectator IDs as players watch or leave the duel.
- Clear the state with a reset when duel mode ends.

For the implementation details, see `port_rust/crates/mu_gameplay/src/duel.rs`.
