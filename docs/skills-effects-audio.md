# Skills, Effects and Audio

This slice ports the legacy skill catalog, requirement checks, delay gating, and
presentation cues used by the gameplay layer.

## Skill Rules

- `SkillDefinition::skill_energy_cost()` reproduces the legacy energy formula.
  - Knights use the lower base cost.
  - Summon Explosion and Summon Requiem use the summon-specific multiplier.
- `SkillDefinition::requirement()` keeps the legacy requirement energy formula.
- `SkillRequirement::is_fulfilled_by()` checks level, strength, dexterity,
  vitality, energy, and charisma against a `SkillStatsSnapshot`.
- `SkillDefinition::display_info()` adds the dark-horse distance bonus.
- `SkillCatalog::set_replacement()` and `resolve_base_skill()` preserve the
  replacement chain for mastery skill IDs.
- `SkillRequirementsCache::ensure_current()` refreshes the requirement cache and
  applies the Empire Guardian teleport block.
- `SkillManager::check_skill_delay()` and `tick_skill_delays()` preserve the
  per-slot delay gate and carry the remaining milliseconds in
  `SkillSlotState`.

## Presentation Cues

- `SkillDefinition::presentation()` derives a `SkillPresentation` from the
  legacy effect id plus skill-id heuristics.
- `SkillEffectQueue` and `SkillAudioQueue` convert a `SkillPresentation` into
  pure events for render/audio consumers.
- `SkillEffectCue::None` and `SkillAudioCue::None` are ignored by the queues.

## Runtime Wrapper

- `mu_audio::AudioRuntime` loads validated converted audio assets, keeps the
  persisted volume settings, and drains queued skill-audio events into a
  runtime-ready diagnostics snapshot.

## Rust Code Locations

- `port_rust/crates/mu_gameplay/src/skills.rs`
- `port_rust/crates/mu_gameplay/src/lib.rs`
- `port_rust/crates/mu_render/src/effects.rs`
- `port_rust/crates/mu_render/src/lib.rs`
- `port_rust/crates/mu_audio/src/{diagnostics.rs,events.rs,lib.rs,runtime.rs}`
