# Combat, Buffs and Experience

This slice ports the deterministic combat, buff, and experience helpers used by
the gameplay layer.

## Combat Damage Rules

- `magic_skill_damage()` mirrors the legacy magic-skill path:
  - summon explosion/requiem skills keep the base magic damage range and skip
    skill/item bonuses;
  - other magic skills add the skill damage to the character range;
  - the result is scaled by the magic-power percentage bonus; then flat mastery
    and skill-attack bonuses are added.
- `curse_skill_damage()` only produces a value for Summoner base classes.
  - summon curse skills add their skill damage to the curse range;
  - non-summon curse skills keep the base curse range.
- `skill_damage()` mirrors the generic skill path:
  - start from `damage .. damage + damage / 2`;
  - add the flat mastery and skill-attack bonuses.

The Rust helpers live in `port_rust/crates/mu_gameplay/src/combat.rs`.

## Buff Rules

- `BuffRegistry` stores active buff states and keeps their ordering stable.
- Token groups are collapsed before a new buff is registered, matching the
  legacy "token" behaviour for mutually exclusive families:
  - Castle Regiment Defense/Attack 1/2/3
  - Crywolf altar/NPC states
  - PC Room Seal and New Wealth Seal
  - Seal family plus HP/MP recovery and master seals
  - Elite Scroll family plus battle/strengthen scrolls
  - Secret Potion family
- `BuffDefinition::time_type()` maps a buff effect to the legacy timer id
  `1005 + effect_type`.
- `BuffTimerRegistry` tracks remaining milliseconds and exposes helper splits
  for day/hour/minute/second display without coupling the gameplay layer to UI
  text.

The Rust helpers live in `port_rust/crates/mu_gameplay/src/buffs.rs`.

## Experience Helpers

Normal character experience keeps the legacy curve:

```text
next = (9 + level) * level^2 * 10
if level > 255:
  next += (9 + (level - 255)) * (level - 255)^2 * 1000
```

Master-level experience uses the same shape with the legacy offset:

```text
total_level = character_level + master_level + 1
over_level = total_level - 255
next = ((9 + total_level) * total_level^2 * 10
        + (9 + over_level) * over_level^2 * 1000
        - 3_892_250_000) / 2
```

- `ExperienceBand::for_level(level)` returns the lower and upper thresholds for
  the current normal level.
- `ExperienceBand::for_master_level(character_level, master_level)` returns the
  equivalent master-level band.
- `previous_experience_for_level(level)` and
  `previous_master_level_experience(character_level, master_level)` expose the
  lower threshold directly for gauge rendering.

The shared helpers live in `port_rust/crates/mu_gameplay/src/experience.rs`.

## Rust Code Locations

- `port_rust/crates/mu_gameplay/src/combat.rs`
- `port_rust/crates/mu_gameplay/src/buffs.rs`
- `port_rust/crates/mu_gameplay/src/experience.rs`
