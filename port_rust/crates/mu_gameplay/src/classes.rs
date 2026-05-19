#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterClass {
    Wizard = 0,
    Knight = 1,
    Elf = 2,
    MagicGladiator = 3,
    DarkLord = 4,
    Summoner = 5,
    RageFighter = 6,
    SoulMaster = 7,
    BladeKnight = 8,
    MuseElf = 9,
    BloodySummoner = 10,
    GrandMaster = 11,
    BladeMaster = 12,
    HighElf = 13,
    DuelMaster = 14,
    LordEmperor = 15,
    DimensionMaster = 16,
    TempleKnight = 17,
    Undefined = 255,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterSkinIndex {
    Wizard = 0,
    Knight = 1,
    Elf = 2,
    MagicGladiator = 3,
    DarkLord = 4,
    Summoner = 5,
    RageFighter = 6,
    SoulMaster = 7,
    BladeKnight = 8,
    MuseElf = 9,
    BloodySummoner = 12,
    GrandMaster = 14,
    BladeMaster = 15,
    HighElf = 16,
    DuelMaster = 17,
    LordEmperor = 18,
    DimensionMaster = 19,
    TempleKnight = 20,
    Undefined = 255,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerClassType {
    DarkWizard = 0,
    SoulMaster = 2,
    GrandMaster = 3,
    DarkKnight = 4,
    BladeKnight = 6,
    BladeMaster = 7,
    FairyElf = 8,
    MuseElf = 10,
    HighElf = 11,
    MagicGladiator = 12,
    DuelMaster = 13,
    DarkLord = 16,
    LordEmperor = 17,
    Summoner = 20,
    BloodySummoner = 22,
    DimensionMaster = 23,
    RageFighter = 24,
    FistMaster = 25,
}

pub const MASTER_EXPERIENCE_UNLOCK_LEVEL: u16 = 400;

impl CharacterClass {
    pub fn base_class(self) -> Self {
        match self {
            Self::SoulMaster | Self::GrandMaster => Self::Wizard,
            Self::BladeKnight | Self::BladeMaster => Self::Knight,
            Self::MuseElf | Self::HighElf => Self::Elf,
            Self::BloodySummoner | Self::DimensionMaster => Self::Summoner,
            Self::DuelMaster => Self::MagicGladiator,
            Self::LordEmperor => Self::DarkLord,
            Self::TempleKnight => Self::RageFighter,
            Self::Undefined => Self::Undefined,
            other => other,
        }
    }

    pub fn is_second_class(self) -> bool {
        matches!(
            self,
            Self::SoulMaster | Self::BladeKnight | Self::MuseElf | Self::BloodySummoner
        )
    }

    pub fn is_third_class(self) -> bool {
        matches!(
            self,
            Self::GrandMaster
                | Self::BladeMaster
                | Self::HighElf
                | Self::DuelMaster
                | Self::LordEmperor
                | Self::DimensionMaster
                | Self::TempleKnight
        )
    }

    pub fn is_master_level(self) -> bool {
        self.is_third_class()
    }

    pub fn is_master_experience_active(self, level: u16) -> bool {
        self.is_master_level() && level >= MASTER_EXPERIENCE_UNLOCK_LEVEL
    }

    pub fn step_class(self) -> u8 {
        if self.is_third_class() {
            3
        } else if self.is_second_class() {
            2
        } else {
            1
        }
    }

    pub fn is_female(self) -> bool {
        matches!(self.base_class(), Self::Elf | Self::Summoner)
    }

    pub fn skin_index(self) -> CharacterSkinIndex {
        match self {
            Self::Wizard => CharacterSkinIndex::Wizard,
            Self::Knight => CharacterSkinIndex::Knight,
            Self::Elf => CharacterSkinIndex::Elf,
            Self::MagicGladiator => CharacterSkinIndex::MagicGladiator,
            Self::DarkLord => CharacterSkinIndex::DarkLord,
            Self::Summoner => CharacterSkinIndex::Summoner,
            Self::RageFighter => CharacterSkinIndex::RageFighter,
            Self::SoulMaster => CharacterSkinIndex::SoulMaster,
            Self::BladeKnight => CharacterSkinIndex::BladeKnight,
            Self::MuseElf => CharacterSkinIndex::MuseElf,
            Self::BloodySummoner => CharacterSkinIndex::BloodySummoner,
            Self::GrandMaster => CharacterSkinIndex::GrandMaster,
            Self::BladeMaster => CharacterSkinIndex::BladeMaster,
            Self::HighElf => CharacterSkinIndex::HighElf,
            Self::DuelMaster => CharacterSkinIndex::DuelMaster,
            Self::LordEmperor => CharacterSkinIndex::LordEmperor,
            Self::DimensionMaster => CharacterSkinIndex::DimensionMaster,
            Self::TempleKnight => CharacterSkinIndex::TempleKnight,
            Self::Undefined => CharacterSkinIndex::Undefined,
        }
    }

    pub fn from_server_class(server_class: ServerClassType) -> Self {
        server_class.into()
    }
}

impl From<ServerClassType> for CharacterClass {
    fn from(server_class: ServerClassType) -> Self {
        match server_class {
            ServerClassType::DarkWizard => Self::Wizard,
            ServerClassType::SoulMaster => Self::SoulMaster,
            ServerClassType::GrandMaster => Self::GrandMaster,
            ServerClassType::DarkKnight => Self::Knight,
            ServerClassType::BladeKnight => Self::BladeKnight,
            ServerClassType::BladeMaster => Self::BladeMaster,
            ServerClassType::FairyElf => Self::Elf,
            ServerClassType::MuseElf => Self::MuseElf,
            ServerClassType::HighElf => Self::HighElf,
            ServerClassType::MagicGladiator => Self::MagicGladiator,
            ServerClassType::DuelMaster => Self::DuelMaster,
            ServerClassType::DarkLord => Self::DarkLord,
            ServerClassType::LordEmperor => Self::LordEmperor,
            ServerClassType::Summoner => Self::Summoner,
            ServerClassType::BloodySummoner => Self::BloodySummoner,
            ServerClassType::DimensionMaster => Self::DimensionMaster,
            ServerClassType::RageFighter => Self::RageFighter,
            ServerClassType::FistMaster => Self::TempleKnight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CharacterClass, CharacterSkinIndex, ServerClassType, MASTER_EXPERIENCE_UNLOCK_LEVEL,
    };

    #[test]
    fn class_hierarchy_matches_legacy_base_and_stage_rules() {
        assert_eq!(
            CharacterClass::SoulMaster.base_class(),
            CharacterClass::Wizard
        );
        assert_eq!(
            CharacterClass::BladeMaster.base_class(),
            CharacterClass::Knight
        );
        assert_eq!(CharacterClass::HighElf.base_class(), CharacterClass::Elf);
        assert_eq!(
            CharacterClass::DimensionMaster.base_class(),
            CharacterClass::Summoner
        );
        assert_eq!(
            CharacterClass::DuelMaster.base_class(),
            CharacterClass::MagicGladiator
        );
        assert_eq!(
            CharacterClass::LordEmperor.base_class(),
            CharacterClass::DarkLord
        );
        assert_eq!(
            CharacterClass::TempleKnight.base_class(),
            CharacterClass::RageFighter
        );

        assert!(CharacterClass::GrandMaster.is_master_level());
        assert!(
            CharacterClass::GrandMaster.is_master_experience_active(MASTER_EXPERIENCE_UNLOCK_LEVEL)
        );
        assert!(!CharacterClass::GrandMaster
            .is_master_experience_active(MASTER_EXPERIENCE_UNLOCK_LEVEL - 1));
        assert_eq!(CharacterClass::BladeMaster.step_class(), 3);
        assert_eq!(CharacterClass::BladeKnight.step_class(), 2);
        assert_eq!(CharacterClass::Wizard.step_class(), 1);
        assert!(CharacterClass::Summoner.is_female());
        assert!(!CharacterClass::RageFighter.is_female());
    }

    #[test]
    fn server_class_mapping_matches_legacy_client_class_mapping() {
        let cases = [
            (ServerClassType::DarkWizard, CharacterClass::Wizard),
            (ServerClassType::SoulMaster, CharacterClass::SoulMaster),
            (ServerClassType::GrandMaster, CharacterClass::GrandMaster),
            (ServerClassType::DarkKnight, CharacterClass::Knight),
            (ServerClassType::BladeKnight, CharacterClass::BladeKnight),
            (ServerClassType::BladeMaster, CharacterClass::BladeMaster),
            (ServerClassType::FairyElf, CharacterClass::Elf),
            (ServerClassType::MuseElf, CharacterClass::MuseElf),
            (ServerClassType::HighElf, CharacterClass::HighElf),
            (
                ServerClassType::MagicGladiator,
                CharacterClass::MagicGladiator,
            ),
            (ServerClassType::DuelMaster, CharacterClass::DuelMaster),
            (ServerClassType::DarkLord, CharacterClass::DarkLord),
            (ServerClassType::LordEmperor, CharacterClass::LordEmperor),
            (ServerClassType::Summoner, CharacterClass::Summoner),
            (
                ServerClassType::BloodySummoner,
                CharacterClass::BloodySummoner,
            ),
            (
                ServerClassType::DimensionMaster,
                CharacterClass::DimensionMaster,
            ),
            (ServerClassType::RageFighter, CharacterClass::RageFighter),
            (ServerClassType::FistMaster, CharacterClass::TempleKnight),
        ];

        for (server_class, client_class) in cases {
            assert_eq!(CharacterClass::from(server_class), client_class);
        }
    }

    #[test]
    fn skin_indices_match_legacy_render_values() {
        assert_eq!(
            CharacterClass::Wizard.skin_index(),
            CharacterSkinIndex::Wizard
        );
        assert_eq!(
            CharacterClass::MagicGladiator.skin_index(),
            CharacterSkinIndex::MagicGladiator
        );
        assert_eq!(
            CharacterClass::BloodySummoner.skin_index(),
            CharacterSkinIndex::BloodySummoner
        );
        assert_eq!(
            CharacterClass::TempleKnight.skin_index(),
            CharacterSkinIndex::TempleKnight
        );
        assert_eq!(
            CharacterClass::Undefined.skin_index(),
            CharacterSkinIndex::Undefined
        );
    }
}
