# Pets, Summons and Mounts

This slice ports the legacy pet stat formulas, summon pose/tier state, and
mount camera offset rules used by the gameplay layer.

## Pet Rules

- `calculate_pet_info()` reproduces the legacy Dark Spirit and Dark Horse
  formulas.
  - experience-next uses `(10 + level + 1) * (level + 1)^3 * 100`.
  - Dark Spirit damage scales from charisma.
  - Dark Horse damage scales from strength, charisma, and level.
- `pet_item_value()` keeps the split sell value used by the legacy client.
- `dark_horse_level_requirement()` and
  `dark_spirit_charisma_requirement()` preserve the UI thresholds shown in the
  old pet window.
- `PetManager::apply_pet_info_response()` stores the hovered packet copy and
  routes equipped helper/weapon-left pet info into the mounted pet slots.

## Summon Rules

- `PlayerSummonPose::from_mount()` keeps the legacy mapping:
  - Uniria, Dinorant, and Fenrir switch the player pose.
  - Dark Horse and no mount stay on the standard pose.
- `summon_weapon_level_tier()` reproduces the weapon-tier split used for summon
  casting:
  - 0 below level 7;
  - 1 for levels 7-10;
  - 2 from level 11 onward.
- `SummonManager::cast()` records the most recent summon cast for downstream
  consumers.

## Mount Rules

- `mount_camera_offset()` returns the 30.0 camera offset for Uniria, Dark
  Horse, and Fenrir when the world is active and the player is not in a safe
  zone.
- Safe zones and inactive worlds force the offset back to 0.0.
- `MountManager::advance_camera_offset()` uses a 500 ms transition and snaps
  once the remaining delta is below 0.5.

## Rust Code Locations

- `port_rust/crates/mu_gameplay/src/pets.rs`
- `port_rust/crates/mu_gameplay/src/summons.rs`
- `port_rust/crates/mu_gameplay/src/mounts.rs`
- `port_rust/crates/mu_gameplay/src/lib.rs`
- `port_rust/crates/mu_protocol/src/pets.rs`
