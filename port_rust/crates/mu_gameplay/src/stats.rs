use crate::classes::CharacterClass;

pub const BASE_CLASS_COUNT: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClassAttributes {
    pub strength: u16,
    pub dexterity: u16,
    pub vitality: u16,
    pub energy: u16,
    pub life: u16,
    pub mana: u16,
    pub shield: u16,
    pub level_life: u8,
    pub level_mana: u8,
    pub vitality_to_life: u8,
    pub energy_to_mana: u8,
}

pub const CLASS_ATTRIBUTES: [ClassAttributes; BASE_CLASS_COUNT] = [
    ClassAttributes {
        strength: 18,
        dexterity: 18,
        vitality: 15,
        energy: 30,
        life: 80,
        mana: 60,
        shield: 0,
        level_life: 1,
        level_mana: 2,
        vitality_to_life: 1,
        energy_to_mana: 2,
    },
    ClassAttributes {
        strength: 28,
        dexterity: 20,
        vitality: 25,
        energy: 10,
        life: 110,
        mana: 20,
        shield: 0,
        level_life: 2,
        level_mana: 1,
        vitality_to_life: 2,
        energy_to_mana: 1,
    },
    ClassAttributes {
        strength: 50,
        dexterity: 50,
        vitality: 50,
        energy: 30,
        life: 110,
        mana: 30,
        shield: 0,
        level_life: 110,
        level_mana: 30,
        vitality_to_life: 6,
        energy_to_mana: 3,
    },
    ClassAttributes {
        strength: 30,
        dexterity: 30,
        vitality: 30,
        energy: 30,
        life: 120,
        mana: 80,
        shield: 0,
        level_life: 1,
        level_mana: 1,
        vitality_to_life: 2,
        energy_to_mana: 2,
    },
    ClassAttributes {
        strength: 30,
        dexterity: 30,
        vitality: 30,
        energy: 30,
        life: 120,
        mana: 80,
        shield: 0,
        level_life: 1,
        level_mana: 1,
        vitality_to_life: 2,
        energy_to_mana: 2,
    },
    ClassAttributes {
        strength: 50,
        dexterity: 50,
        vitality: 50,
        energy: 30,
        life: 110,
        mana: 30,
        shield: 0,
        level_life: 110,
        level_mana: 30,
        vitality_to_life: 6,
        energy_to_mana: 3,
    },
    ClassAttributes {
        strength: 32,
        dexterity: 27,
        vitality: 25,
        energy: 20,
        life: 100,
        mana: 40,
        shield: 0,
        level_life: 1,
        level_mana: 3,
        vitality_to_life: 1,
        energy_to_mana: 1,
    },
];

pub fn base_class_attributes(class_: CharacterClass) -> ClassAttributes {
    match class_.base_class() {
        CharacterClass::Wizard => CLASS_ATTRIBUTES[0],
        CharacterClass::Knight => CLASS_ATTRIBUTES[1],
        CharacterClass::Elf => CLASS_ATTRIBUTES[2],
        CharacterClass::MagicGladiator => CLASS_ATTRIBUTES[3],
        CharacterClass::DarkLord => CLASS_ATTRIBUTES[4],
        CharacterClass::Summoner => CLASS_ATTRIBUTES[5],
        CharacterClass::RageFighter => CLASS_ATTRIBUTES[6],
        CharacterClass::Undefined => panic!("undefined class has no base attributes"),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{base_class_attributes, ClassAttributes};
    use crate::classes::CharacterClass;

    #[test]
    fn legacy_base_class_attributes_match_create_class_attributes() {
        let cases = [
            (
                CharacterClass::Wizard,
                ClassAttributes {
                    strength: 18,
                    dexterity: 18,
                    vitality: 15,
                    energy: 30,
                    life: 80,
                    mana: 60,
                    shield: 0,
                    level_life: 1,
                    level_mana: 2,
                    vitality_to_life: 1,
                    energy_to_mana: 2,
                },
            ),
            (
                CharacterClass::Knight,
                ClassAttributes {
                    strength: 28,
                    dexterity: 20,
                    vitality: 25,
                    energy: 10,
                    life: 110,
                    mana: 20,
                    shield: 0,
                    level_life: 2,
                    level_mana: 1,
                    vitality_to_life: 2,
                    energy_to_mana: 1,
                },
            ),
            (
                CharacterClass::Elf,
                ClassAttributes {
                    strength: 50,
                    dexterity: 50,
                    vitality: 50,
                    energy: 30,
                    life: 110,
                    mana: 30,
                    shield: 0,
                    level_life: 110,
                    level_mana: 30,
                    vitality_to_life: 6,
                    energy_to_mana: 3,
                },
            ),
            (
                CharacterClass::MagicGladiator,
                ClassAttributes {
                    strength: 30,
                    dexterity: 30,
                    vitality: 30,
                    energy: 30,
                    life: 120,
                    mana: 80,
                    shield: 0,
                    level_life: 1,
                    level_mana: 1,
                    vitality_to_life: 2,
                    energy_to_mana: 2,
                },
            ),
            (
                CharacterClass::DarkLord,
                ClassAttributes {
                    strength: 30,
                    dexterity: 30,
                    vitality: 30,
                    energy: 30,
                    life: 120,
                    mana: 80,
                    shield: 0,
                    level_life: 1,
                    level_mana: 1,
                    vitality_to_life: 2,
                    energy_to_mana: 2,
                },
            ),
            (
                CharacterClass::Summoner,
                ClassAttributes {
                    strength: 50,
                    dexterity: 50,
                    vitality: 50,
                    energy: 30,
                    life: 110,
                    mana: 30,
                    shield: 0,
                    level_life: 110,
                    level_mana: 30,
                    vitality_to_life: 6,
                    energy_to_mana: 3,
                },
            ),
            (
                CharacterClass::RageFighter,
                ClassAttributes {
                    strength: 32,
                    dexterity: 27,
                    vitality: 25,
                    energy: 20,
                    life: 100,
                    mana: 40,
                    shield: 0,
                    level_life: 1,
                    level_mana: 3,
                    vitality_to_life: 1,
                    energy_to_mana: 1,
                },
            ),
        ];

        for (class_, expected) in cases {
            assert_eq!(base_class_attributes(class_), expected);
        }
    }

    #[test]
    fn advanced_classes_inherit_their_base_class_attributes() {
        assert_eq!(
            base_class_attributes(CharacterClass::GrandMaster),
            base_class_attributes(CharacterClass::Wizard)
        );
        assert_eq!(
            base_class_attributes(CharacterClass::BladeMaster),
            base_class_attributes(CharacterClass::Knight)
        );
        assert_eq!(
            base_class_attributes(CharacterClass::DimensionMaster),
            base_class_attributes(CharacterClass::Summoner)
        );
        assert_eq!(
            base_class_attributes(CharacterClass::TempleKnight),
            base_class_attributes(CharacterClass::RageFighter)
        );
    }
}
