use crate::classes::CharacterClass;
pub use crate::experience::next_experience_for_level;
use crate::stats::{base_class_attributes, ClassAttributes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterSheet {
    pub class_: CharacterClass,
    pub level: u16,
    pub level_up_points: u16,
    pub strength: u16,
    pub dexterity: u16,
    pub vitality: u16,
    pub energy: u16,
    pub charisma: u16,
    pub life: u32,
    pub life_max: u32,
    pub mana: u32,
    pub mana_max: u32,
    pub shield: u32,
    pub shield_max: u32,
    pub add_strength: u16,
    pub add_dexterity: u16,
    pub add_vitality: u16,
    pub add_energy: u16,
    pub add_charisma: u16,
    pub add_life_max: u16,
    pub add_mana_max: u16,
    pub experience: u64,
    pub next_experience: u64,
}

impl CharacterSheet {
    pub fn new(class_: CharacterClass) -> Self {
        let stats = base_class_attributes(class_);

        Self {
            class_,
            level: 1,
            level_up_points: 0,
            strength: stats.strength,
            dexterity: stats.dexterity,
            vitality: stats.vitality,
            energy: stats.energy,
            charisma: 0,
            life: u32::from(stats.life),
            life_max: u32::from(stats.life),
            mana: u32::from(stats.mana),
            mana_max: u32::from(stats.mana),
            shield: u32::from(stats.shield),
            shield_max: u32::from(stats.shield),
            add_strength: 0,
            add_dexterity: 0,
            add_vitality: 0,
            add_energy: 0,
            add_charisma: 0,
            add_life_max: 0,
            add_mana_max: 0,
            experience: 0,
            next_experience: next_experience_for_level(1),
        }
    }

    pub fn reset(&mut self, class_: CharacterClass) {
        *self = Self::new(class_);
    }

    pub fn base_class(&self) -> CharacterClass {
        self.class_.base_class()
    }

    pub fn base_stats(&self) -> ClassAttributes {
        base_class_attributes(self.class_)
    }

    pub fn is_master_level(&self) -> bool {
        self.class_.is_master_level()
    }

    pub fn is_master_experience_active(&self) -> bool {
        self.class_.is_master_experience_active(self.level)
    }
}

#[cfg(test)]
mod tests {
    use super::{next_experience_for_level, CharacterSheet};
    use crate::classes::CharacterClass;

    #[test]
    fn new_character_uses_the_legacy_base_sheet() {
        let sheet = CharacterSheet::new(CharacterClass::Wizard);

        assert_eq!(sheet.class_, CharacterClass::Wizard);
        assert_eq!(sheet.level, 1);
        assert_eq!(sheet.level_up_points, 0);
        assert_eq!(sheet.strength, 18);
        assert_eq!(sheet.dexterity, 18);
        assert_eq!(sheet.vitality, 15);
        assert_eq!(sheet.energy, 30);
        assert_eq!(sheet.life, 80);
        assert_eq!(sheet.life_max, 80);
        assert_eq!(sheet.mana, 60);
        assert_eq!(sheet.mana_max, 60);
        assert_eq!(sheet.shield, 0);
        assert_eq!(sheet.shield_max, 0);
        assert_eq!(sheet.experience, 0);
        assert_eq!(sheet.next_experience, 100);
        assert!(!sheet.is_master_level());
    }

    #[test]
    fn advanced_classes_reuse_base_stats_and_master_rules() {
        let sheet = CharacterSheet::new(CharacterClass::GrandMaster);

        assert_eq!(sheet.base_class(), CharacterClass::Wizard);
        assert_eq!(sheet.base_stats().strength, 18);
        assert!(sheet.is_master_level());
        assert!(!sheet.is_master_experience_active());
    }

    #[test]
    fn normal_experience_progression_matches_the_legacy_curve() {
        assert_eq!(next_experience_for_level(1), 100);
        assert_eq!(next_experience_for_level(255), 171_666_000);
        assert_eq!(next_experience_for_level(256), 173_680_400);
    }
}
