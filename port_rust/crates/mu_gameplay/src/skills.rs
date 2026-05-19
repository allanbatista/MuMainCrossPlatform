use bevy::prelude::{App, Plugin, Resource};
use std::collections::BTreeMap;

use crate::classes::CharacterClass;

pub type SkillId = u16;

pub const MAX_SKILLS: usize = 650;

pub const AT_SKILL_UNDEFINED: SkillId = 0;
pub const AT_SKILL_TELEPORT: SkillId = 6;
pub const AT_SKILL_TELEPORT_ALLY: SkillId = 15;
pub const AT_SKILL_TRIPLE_SHOT: SkillId = 24;
pub const AT_SKILL_ICE_ARROW: SkillId = 51;
pub const AT_SKILL_PENETRATION: SkillId = 52;
pub const AT_SKILL_SUMMON_EXPLOSION: SkillId = 223;
pub const AT_SKILL_SUMMON_REQUIEM: SkillId = 224;
pub const AT_SKILL_SUMMON_POLLUTION: SkillId = 225;
pub const AT_SKILL_CHAIN_DRIVE: SkillId = 262;
pub const AT_SKILL_DRAGON_ROAR: SkillId = 264;
pub const AT_SKILL_DRAGON_KICK: SkillId = 265;
pub const AT_SKILL_TRIPLE_SHOT_STR: SkillId = 414;
pub const AT_SKILL_PENETRATION_STR: SkillId = 416;
pub const AT_SKILL_TRIPLE_SHOT_MASTERY: SkillId = 418;
pub const AT_SKILL_ICE_ARROW_STR: SkillId = 424;
pub const AT_SKILL_CHAIN_DRIVE_STR: SkillId = 558;
pub const AT_SKILL_DRAGON_ROAR_STR: SkillId = 560;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillStatsSnapshot {
    pub level: u16,
    pub strength: u16,
    pub dexterity: u16,
    pub vitality: u16,
    pub energy: u16,
    pub charisma: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillRequirement {
    pub level: u16,
    pub strength: u16,
    pub dexterity: u16,
    pub vitality: u16,
    pub energy: u16,
    pub charisma: u16,
}

impl SkillRequirement {
    pub fn is_fulfilled_by(&self, hero: &SkillStatsSnapshot) -> bool {
        self.level <= hero.level
            && self.strength <= hero.strength
            && self.dexterity <= hero.dexterity
            && self.vitality <= hero.vitality
            && self.energy <= hero.energy
            && self.charisma <= hero.charisma
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkillDisplayInfo<'a> {
    pub name: &'a str,
    pub mana: u16,
    pub distance: f32,
    pub ability_gauge: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkillEffectCue {
    #[default]
    None,
    LegacyEffect(u16),
    Teleport,
    Projectile,
    Summon,
    MagicCast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkillAudioCue {
    #[default]
    None,
    Magic,
    Bow,
    IceArrow,
    Teleport,
    SummonExplosion,
    SummonRequiem,
    SummonPollution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillPresentation {
    pub effect: SkillEffectCue,
    pub audio: SkillAudioCue,
}

impl SkillPresentation {
    pub const fn new(effect: SkillEffectCue, audio: SkillAudioCue) -> Self {
        Self { effect, audio }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillDefinition {
    pub name: String,
    pub level: u16,
    pub damage: u16,
    pub mana: u16,
    pub ability_gauge: u16,
    pub distance: u16,
    pub delay_ms: i32,
    pub energy: i32,
    pub charisma: u16,
    pub mastery_type: u8,
    pub skill_use_type: u8,
    pub skill_brand: u32,
    pub kill_count: u8,
    pub require_duty_class: [u8; 3],
    pub require_class: [u8; 7],
    pub skill_rank: u8,
    pub magic_icon: u16,
    pub type_skill: u8,
    pub strength: u16,
    pub dexterity: u16,
    pub item_skill: u8,
    pub is_damage: u8,
    pub effect: u16,
}

impl SkillDefinition {
    pub fn display_info(&self, mounted_on_dark_horse: bool) -> SkillDisplayInfo<'_> {
        SkillDisplayInfo {
            name: self.name.as_str(),
            mana: self.mana,
            distance: self.distance as f32 + if mounted_on_dark_horse { 2.0 } else { 0.0 },
            ability_gauge: self.ability_gauge,
        }
    }

    pub fn damage(&self) -> u16 {
        self.damage
    }

    pub fn mastery_type(&self) -> u8 {
        self.mastery_type
    }

    pub fn charisma_requirement(&self) -> u16 {
        self.charisma
    }

    pub fn skill_energy_cost(&self, skill_id: SkillId, base_class: CharacterClass) -> i32 {
        if self.energy <= 0 {
            return 0;
        }

        if base_class.base_class() == CharacterClass::Knight {
            return 10 + (self.energy * i32::from(self.level) * 4 / 100);
        }

        if matches!(
            skill_id,
            AT_SKILL_SUMMON_EXPLOSION | AT_SKILL_SUMMON_REQUIEM
        ) {
            return 20 + (self.energy * i32::from(self.level) * 3 / 100);
        }

        20 + (self.energy * i32::from(self.level) * 4 / 100)
    }

    pub fn legacy_requirement_energy(&self) -> u16 {
        (20 + (self.energy * i32::from(self.level)) * 4 / 100) as u16
    }

    pub fn requirement(&self) -> SkillRequirement {
        SkillRequirement {
            level: self.level,
            strength: self.strength,
            dexterity: self.dexterity,
            vitality: 0,
            energy: self.legacy_requirement_energy(),
            charisma: self.charisma,
        }
    }

    pub fn presentation(&self, skill_id: SkillId) -> SkillPresentation {
        SkillPresentation {
            effect: self.effect_cue(skill_id),
            audio: self.audio_cue(skill_id),
        }
    }

    fn effect_cue(&self, skill_id: SkillId) -> SkillEffectCue {
        if self.effect != 0 {
            return SkillEffectCue::LegacyEffect(self.effect);
        }

        if is_teleport_skill(skill_id) {
            SkillEffectCue::Teleport
        } else if is_projectile_skill(skill_id) {
            SkillEffectCue::Projectile
        } else if is_summon_skill(skill_id) {
            SkillEffectCue::Summon
        } else {
            SkillEffectCue::MagicCast
        }
    }

    fn audio_cue(&self, skill_id: SkillId) -> SkillAudioCue {
        if is_teleport_skill(skill_id) {
            SkillAudioCue::Teleport
        } else if skill_id == AT_SKILL_ICE_ARROW || skill_id == AT_SKILL_ICE_ARROW_STR {
            SkillAudioCue::IceArrow
        } else if matches!(
            skill_id,
            AT_SKILL_PENETRATION
                | AT_SKILL_PENETRATION_STR
                | AT_SKILL_TRIPLE_SHOT
                | AT_SKILL_TRIPLE_SHOT_STR
                | AT_SKILL_TRIPLE_SHOT_MASTERY
        ) {
            SkillAudioCue::Bow
        } else if skill_id == AT_SKILL_SUMMON_EXPLOSION {
            SkillAudioCue::SummonExplosion
        } else if skill_id == AT_SKILL_SUMMON_REQUIEM {
            SkillAudioCue::SummonRequiem
        } else if skill_id == AT_SKILL_SUMMON_POLLUTION {
            SkillAudioCue::SummonPollution
        } else {
            SkillAudioCue::Magic
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillCatalog {
    definitions: Vec<Option<SkillDefinition>>,
    replacements: BTreeMap<SkillId, SkillId>,
}

impl Default for SkillCatalog {
    fn default() -> Self {
        Self {
            definitions: vec![None; MAX_SKILLS],
            replacements: BTreeMap::new(),
        }
    }
}

impl Resource for SkillCatalog {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillSlotState {
    pub skill_id: SkillId,
    pub level: u8,
    pub delay_remaining_ms: i32,
}

impl SkillSlotState {
    pub fn is_empty(&self) -> bool {
        self.skill_id == AT_SKILL_UNDEFINED && self.level == 0 && self.delay_remaining_ms == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillManager {
    slots: Vec<SkillSlotState>,
}

impl Resource for SkillManager {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillRequirementsCache {
    entries: Vec<bool>,
    dirty: bool,
}

impl Default for SkillRequirementsCache {
    fn default() -> Self {
        Self {
            entries: vec![false; MAX_SKILLS],
            dirty: true,
        }
    }
}

impl Resource for SkillRequirementsCache {}

#[derive(Debug, Default, Clone, Copy)]
pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkillCatalog>();
        app.init_resource::<SkillManager>();
        app.init_resource::<SkillRequirementsCache>();
    }
}

impl SkillCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.definitions.iter().flatten().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(
        &mut self,
        skill_id: SkillId,
        definition: SkillDefinition,
    ) -> Option<SkillDefinition> {
        let index = skill_index(skill_id)?;
        self.definitions[index].replace(definition)
    }

    pub fn definition(&self, skill_id: SkillId) -> Option<&SkillDefinition> {
        let index = skill_index(skill_id)?;
        self.definitions[index].as_ref()
    }

    pub fn definition_mut(&mut self, skill_id: SkillId) -> Option<&mut SkillDefinition> {
        let index = skill_index(skill_id)?;
        self.definitions[index].as_mut()
    }

    pub fn set_replacement(&mut self, skill_id: SkillId, base_skill: SkillId) {
        self.replacements.insert(skill_id, base_skill);
    }

    pub fn resolve_base_skill(&self, skill_id: SkillId) -> SkillId {
        let mut current = skill_id;

        for _ in 0..MAX_SKILLS {
            let Some(&next) = self.replacements.get(&current) else {
                break;
            };
            current = next;
        }

        current
    }

    pub fn resolved_definition(&self, skill_id: SkillId) -> Option<&SkillDefinition> {
        let resolved_skill = self.resolve_base_skill(skill_id);
        self.definition(resolved_skill)
            .or_else(|| self.definition(skill_id))
    }

    pub fn display_info(
        &self,
        skill_id: SkillId,
        mounted_on_dark_horse: bool,
    ) -> Option<SkillDisplayInfo<'_>> {
        self.resolved_definition(skill_id)
            .map(|definition| definition.display_info(mounted_on_dark_horse))
    }

    pub fn skill_energy_cost(&self, skill_id: SkillId, base_class: CharacterClass) -> Option<i32> {
        self.resolved_definition(skill_id)
            .map(|definition| definition.skill_energy_cost(skill_id, base_class))
    }

    pub fn skill_damage(&self, skill_id: SkillId) -> Option<u16> {
        self.resolved_definition(skill_id)
            .map(SkillDefinition::damage)
    }

    pub fn skill_charisma(&self, skill_id: SkillId) -> Option<u16> {
        self.resolved_definition(skill_id)
            .map(SkillDefinition::charisma_requirement)
    }

    pub fn mastery_type(&self, skill_id: SkillId) -> Option<u8> {
        self.resolved_definition(skill_id)
            .map(SkillDefinition::mastery_type)
    }

    pub fn requirement(&self, skill_id: SkillId) -> Option<SkillRequirement> {
        self.resolved_definition(skill_id)
            .map(SkillDefinition::requirement)
    }

    pub fn requirement_fulfilled(
        &self,
        skill_id: SkillId,
        hero: SkillStatsSnapshot,
        is_empire_guardian: bool,
    ) -> bool {
        let base_skill = self.resolve_base_skill(skill_id);

        if is_empire_guardian && matches!(base_skill, AT_SKILL_TELEPORT | AT_SKILL_TELEPORT_ALLY) {
            return false;
        }

        self.definition(base_skill)
            .map(|definition| definition.requirement().is_fulfilled_by(&hero))
            .unwrap_or(false)
    }

    pub fn presentation(&self, skill_id: SkillId) -> Option<SkillPresentation> {
        self.resolved_definition(skill_id)
            .map(|definition| definition.presentation(skill_id))
    }
}

impl SkillManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn slots(&self) -> &[SkillSlotState] {
        &self.slots
    }

    pub fn slot(&self, slot_index: usize) -> Option<&SkillSlotState> {
        self.slots.get(slot_index)
    }

    pub fn slot_mut(&mut self, slot_index: usize) -> Option<&mut SkillSlotState> {
        self.slots.get_mut(slot_index)
    }

    pub fn push_slot(&mut self, skill_id: SkillId, level: u8) -> usize {
        self.slots.push(SkillSlotState {
            skill_id,
            level,
            delay_remaining_ms: 0,
        });
        self.slots.len() - 1
    }

    pub fn set_slot(&mut self, slot_index: usize, skill_id: SkillId, level: u8) {
        if self.slots.len() <= slot_index {
            self.slots
                .resize_with(slot_index + 1, SkillSlotState::default);
        }

        self.slots[slot_index] = SkillSlotState {
            skill_id,
            level,
            delay_remaining_ms: 0,
        };
    }

    pub fn clear(&mut self) {
        self.slots.clear();
    }

    pub fn find_hero_skill(&self, skill_id: SkillId) -> bool {
        if skill_id == AT_SKILL_UNDEFINED {
            return false;
        }

        self.slots
            .iter()
            .any(|slot| slot.skill_id == skill_id && !slot.is_empty())
    }

    pub fn delay_remaining(&self, slot_index: usize) -> Option<i32> {
        self.slot(slot_index).map(|slot| slot.delay_remaining_ms)
    }

    pub fn set_delay_remaining(&mut self, slot_index: usize, delay_remaining_ms: i32) -> bool {
        let Some(slot) = self.slot_mut(slot_index) else {
            return false;
        };

        slot.delay_remaining_ms = delay_remaining_ms.max(0);
        true
    }

    pub fn check_skill_delay(
        &mut self,
        slot_index: usize,
        catalog: &SkillCatalog,
        can_attack: bool,
        hero_charisma: u16,
    ) -> bool {
        let Some(slot) = self.slot_mut(slot_index) else {
            return false;
        };

        let Some(definition) = catalog.resolved_definition(slot.skill_id) else {
            return false;
        };

        if !can_attack && is_attack_locked_skill(slot.skill_id) {
            return false;
        }

        if definition.delay_ms > 0 {
            if slot.delay_remaining_ms > 0 {
                return false;
            }

            if definition.charisma > hero_charisma {
                return false;
            }

            slot.delay_remaining_ms = definition.delay_ms;
        }

        true
    }

    pub fn tick_skill_delays(&mut self, elapsed_ms: i32) {
        if elapsed_ms <= 0 {
            return;
        }

        for slot in &mut self.slots {
            if slot.delay_remaining_ms <= 0 {
                continue;
            }

            slot.delay_remaining_ms -= elapsed_ms;
            if slot.delay_remaining_ms < 0 {
                slot.delay_remaining_ms = 0;
            }
        }
    }
}

impl SkillRequirementsCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn invalidate(&mut self) {
        self.dirty = true;
    }

    pub fn initialize(
        &mut self,
        catalog: &SkillCatalog,
        hero: SkillStatsSnapshot,
        is_empire_guardian: bool,
    ) {
        self.dirty = true;
        self.rebuild(catalog, hero, is_empire_guardian);
    }

    pub fn rebuild(
        &mut self,
        catalog: &SkillCatalog,
        hero: SkillStatsSnapshot,
        is_empire_guardian: bool,
    ) {
        if self.entries.len() != MAX_SKILLS {
            self.entries = vec![false; MAX_SKILLS];
        } else {
            self.entries.fill(false);
        }

        for skill_index in 0..MAX_SKILLS {
            let skill_id = skill_index as SkillId;
            self.entries[skill_index] =
                catalog.requirement_fulfilled(skill_id, hero, is_empire_guardian);
        }

        self.dirty = false;
    }

    pub fn ensure_current(
        &mut self,
        catalog: &SkillCatalog,
        hero: SkillStatsSnapshot,
        is_empire_guardian: bool,
    ) {
        if self.dirty {
            self.rebuild(catalog, hero, is_empire_guardian);
        }
    }

    pub fn is_fulfilled(&self, skill_id: SkillId) -> bool {
        let Some(index) = skill_index(skill_id) else {
            return false;
        };

        self.entries.get(index).copied().unwrap_or(false)
    }

    pub fn entries(&self) -> &[bool] {
        &self.entries
    }
}

fn skill_index(skill_id: SkillId) -> Option<usize> {
    let index = usize::from(skill_id);
    (index < MAX_SKILLS).then_some(index)
}

fn is_teleport_skill(skill_id: SkillId) -> bool {
    matches!(skill_id, AT_SKILL_TELEPORT | AT_SKILL_TELEPORT_ALLY)
}

fn is_projectile_skill(skill_id: SkillId) -> bool {
    matches!(
        skill_id,
        AT_SKILL_ICE_ARROW
            | AT_SKILL_ICE_ARROW_STR
            | AT_SKILL_PENETRATION
            | AT_SKILL_PENETRATION_STR
            | AT_SKILL_TRIPLE_SHOT
            | AT_SKILL_TRIPLE_SHOT_STR
            | AT_SKILL_TRIPLE_SHOT_MASTERY
    )
}

fn is_summon_skill(skill_id: SkillId) -> bool {
    matches!(
        skill_id,
        AT_SKILL_SUMMON_EXPLOSION | AT_SKILL_SUMMON_REQUIEM | AT_SKILL_SUMMON_POLLUTION
    )
}

fn is_attack_locked_skill(skill_id: SkillId) -> bool {
    matches!(
        skill_id,
        AT_SKILL_CHAIN_DRIVE
            | AT_SKILL_CHAIN_DRIVE_STR
            | AT_SKILL_DRAGON_ROAR
            | AT_SKILL_DRAGON_ROAR_STR
            | AT_SKILL_DRAGON_KICK
    )
}

#[cfg(test)]
mod tests {
    use super::{
        is_attack_locked_skill, SkillAudioCue, SkillCatalog, SkillDefinition, SkillEffectCue,
        SkillManager, SkillPresentation, SkillRequirementsCache, SkillSlotState, SkillStatsSnapshot,
        AT_SKILL_CHAIN_DRIVE, AT_SKILL_DRAGON_KICK, AT_SKILL_DRAGON_ROAR,
        AT_SKILL_DRAGON_ROAR_STR, AT_SKILL_ICE_ARROW_STR, AT_SKILL_PENETRATION_STR,
        AT_SKILL_SUMMON_EXPLOSION, AT_SKILL_SUMMON_POLLUTION, AT_SKILL_SUMMON_REQUIEM,
        AT_SKILL_TELEPORT, AT_SKILL_TELEPORT_ALLY, AT_SKILL_TRIPLE_SHOT, AT_SKILL_TRIPLE_SHOT_MASTERY,
        AT_SKILL_TRIPLE_SHOT_STR,
    };
    use crate::classes::CharacterClass;

    fn make_skill(
        name: &str,
        level: u16,
        energy: i32,
        charisma: u16,
        effect: u16,
    ) -> SkillDefinition {
        SkillDefinition {
            name: name.to_string(),
            level,
            damage: 30,
            mana: 20,
            ability_gauge: 4,
            distance: 8,
            delay_ms: 500,
            energy,
            charisma,
            mastery_type: 3,
            skill_use_type: 1,
            skill_brand: 0,
            kill_count: 0,
            require_duty_class: [0; 3],
            require_class: [0; 7],
            skill_rank: 2,
            magic_icon: 11,
            type_skill: 0,
            strength: 18,
            dexterity: 18,
            item_skill: 0,
            is_damage: 1,
            effect,
        }
    }

    #[test]
    fn skill_definition_uses_the_legacy_energy_rules() {
        let summon = make_skill("Summon", 20, 30, 12, 0);
        let knight = CharacterClass::BladeKnight;

        assert_eq!(
            summon.skill_energy_cost(AT_SKILL_SUMMON_EXPLOSION, CharacterClass::Wizard),
            38
        );
        assert_eq!(
            summon.skill_energy_cost(AT_SKILL_SUMMON_REQUIEM, knight),
            34
        );
        assert_eq!(
            summon.skill_energy_cost(AT_SKILL_TELEPORT, CharacterClass::Wizard),
            44
        );
        assert_eq!(
            summon.display_info(true),
            super::SkillDisplayInfo {
                name: "Summon",
                mana: 20,
                distance: 10.0,
                ability_gauge: 4,
            }
        );
    }

    #[test]
    fn skill_definition_builds_presentation_cues_from_legacy_ids() {
        let teleport = make_skill("Teleport", 10, 0, 0, 0);
        let projectile = make_skill("Projectile", 10, 10, 0, 0);
        let summon = make_skill("Summon", 10, 10, 0, 0);
        let legacy_effect = make_skill("Legacy", 10, 10, 0, 77);

        assert_eq!(
            teleport.presentation(AT_SKILL_TELEPORT),
            SkillPresentation::new(SkillEffectCue::Teleport, SkillAudioCue::Teleport)
        );
        assert_eq!(
            projectile.presentation(AT_SKILL_ICE_ARROW_STR),
            SkillPresentation::new(SkillEffectCue::Projectile, SkillAudioCue::IceArrow)
        );
        assert_eq!(
            summon.presentation(AT_SKILL_SUMMON_POLLUTION),
            SkillPresentation::new(SkillEffectCue::Summon, SkillAudioCue::SummonPollution)
        );
        assert_eq!(
            legacy_effect.presentation(AT_SKILL_TRIPLE_SHOT),
            SkillPresentation::new(SkillEffectCue::LegacyEffect(77), SkillAudioCue::Bow)
        );
    }

    #[test]
    fn catalog_resolves_replacement_chains_and_requirement_cache() {
        let mut catalog = SkillCatalog::new();
        catalog.insert(AT_SKILL_TELEPORT, make_skill("Teleport", 10, 10, 0, 0));
        catalog.insert(
            AT_SKILL_TRIPLE_SHOT,
            make_skill("Triple Shot", 20, 10, 0, 0),
        );
        catalog.insert(
            AT_SKILL_SUMMON_EXPLOSION,
            make_skill("Explosion", 20, 30, 0, 0),
        );
        catalog.set_replacement(AT_SKILL_TRIPLE_SHOT_STR, AT_SKILL_TRIPLE_SHOT);
        catalog.set_replacement(AT_SKILL_TRIPLE_SHOT_MASTERY, AT_SKILL_TRIPLE_SHOT_STR);

        assert_eq!(
            catalog.resolve_base_skill(AT_SKILL_TRIPLE_SHOT_MASTERY),
            AT_SKILL_TRIPLE_SHOT
        );
        assert_eq!(catalog.skill_damage(AT_SKILL_TRIPLE_SHOT), Some(30));
        assert_eq!(catalog.mastery_type(AT_SKILL_TRIPLE_SHOT), Some(3));
        assert_eq!(
            catalog.presentation(AT_SKILL_SUMMON_EXPLOSION),
            Some(SkillPresentation::new(
                SkillEffectCue::Summon,
                SkillAudioCue::SummonExplosion
            ))
        );

        let hero = SkillStatsSnapshot {
            level: 20,
            strength: 18,
            dexterity: 18,
            vitality: 15,
            energy: 30,
            charisma: 10,
        };
        let mut cache = SkillRequirementsCache::new();
        cache.rebuild(&catalog, hero, false);

        assert!(cache.is_fulfilled(AT_SKILL_TRIPLE_SHOT));
        assert!(!cache.is_fulfilled(AT_SKILL_TELEPORT_ALLY));

        cache.invalidate();
        cache.ensure_current(&catalog, hero, true);
        assert!(!cache.is_fulfilled(AT_SKILL_TELEPORT));
    }

    #[test]
    fn skill_manager_tracks_known_skills_and_delay_rules() {
        let mut catalog = SkillCatalog::new();
        catalog.insert(
            AT_SKILL_CHAIN_DRIVE,
            make_skill("Chain Drive", 10, 20, 0, 0),
        );
        catalog.insert(
            AT_SKILL_DRAGON_ROAR,
            make_skill("Dragon Roar", 10, 20, 0, 0),
        );
        catalog.insert(
            AT_SKILL_DRAGON_KICK,
            make_skill("Dragon Kick", 10, 20, 0, 0),
        );

        let mut manager = SkillManager::new();
        let slot = manager.push_slot(AT_SKILL_CHAIN_DRIVE, 1);
        manager.push_slot(AT_SKILL_DRAGON_ROAR_STR, 1);

        assert!(manager.find_hero_skill(AT_SKILL_CHAIN_DRIVE));
        assert!(!manager.find_hero_skill(AT_SKILL_SUMMON_POLLUTION));
        assert!(is_attack_locked_skill(AT_SKILL_DRAGON_KICK));

        assert!(!manager.check_skill_delay(slot, &catalog, false, 10));
        assert!(manager.check_skill_delay(slot, &catalog, true, 10));
        assert_eq!(manager.delay_remaining(slot), Some(500));

        manager.tick_skill_delays(250);
        assert_eq!(manager.delay_remaining(slot), Some(250));
        manager.tick_skill_delays(500);
        assert_eq!(manager.delay_remaining(slot), Some(0));
    }

    #[test]
    fn skill_manager_can_resize_slots_and_store_levels() {
        let mut manager = SkillManager::new();
        manager.set_slot(2, AT_SKILL_PENETRATION_STR, 3);

        assert_eq!(manager.slots().len(), 3);
        assert_eq!(
            manager.slot(2),
            Some(&SkillSlotState {
                skill_id: AT_SKILL_PENETRATION_STR,
                level: 3,
                delay_remaining_ms: 0,
            })
        );
        assert!(manager.slot(0).unwrap().is_empty());
        assert!(manager.find_hero_skill(AT_SKILL_PENETRATION_STR));
    }
}
