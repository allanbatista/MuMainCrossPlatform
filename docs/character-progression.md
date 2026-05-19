# Character Progression

This port slice mirrors the legacy class hierarchy, base stat table, normal
experience curve, and master-level experience curve.

## Class Rules

- `CharacterClass::base_class()` folds second/third classes back to their
  playable base class.
- `is_second_class()`, `is_third_class()`, `is_master_level()`, and
  `is_master_experience_active(level)` match the legacy `CharacterManager`
  rules.
- `CharacterClass::skin_index()` preserves the legacy render skin mapping.
- Master experience unlocks at level `400` for third classes.

## Base Stat Table

The Rust gameplay crate stores the legacy creation stats in
`port_rust/crates/mu_gameplay/src/stats.rs`.

| Base class | Str | Dex | Vit | Ene | Life | Mana | LvLife | LvMana | Vit->Life | Ene->Mana |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Wizard | 18 | 18 | 15 | 30 | 80 | 60 | 1 | 2 | 1 | 2 |
| Knight | 28 | 20 | 25 | 10 | 110 | 20 | 2 | 1 | 2 | 1 |
| Elf | 50 | 50 | 50 | 30 | 110 | 30 | 110 | 30 | 6 | 3 |
| Magic Gladiator | 30 | 30 | 30 | 30 | 120 | 80 | 1 | 1 | 2 | 2 |
| Dark Lord | 30 | 30 | 30 | 30 | 120 | 80 | 1 | 1 | 2 | 2 |
| Summoner | 50 | 50 | 50 | 30 | 110 | 30 | 110 | 30 | 6 | 3 |
| Rage Fighter | 32 | 27 | 25 | 20 | 100 | 40 | 1 | 3 | 1 | 1 |

Advanced classes inherit the table of their base class.

## Experience Curves

Normal character next-experience uses the legacy curve:

```text
next = (9 + level) * level^2 * 10
if level > 255:
  next += (9 + (level - 255)) * (level - 255)^2 * 1000
```

Master-level next-experience uses the same shape with:

```text
total_level = character_level + master_level + 1
over_level = total_level - 255
next = ((9 + total_level) * total_level^2 * 10
        + (9 + over_level) * over_level^2 * 1000
        - 3_892_250_000) / 2
```

## Rust Code Locations

- `port_rust/crates/mu_gameplay/src/classes.rs`
- `port_rust/crates/mu_gameplay/src/stats.rs`
- `port_rust/crates/mu_gameplay/src/characters.rs`
- `port_rust/crates/mu_gameplay/src/master_level.rs`
