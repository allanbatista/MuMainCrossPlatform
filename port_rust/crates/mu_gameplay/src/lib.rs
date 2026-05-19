pub mod buffs;
pub mod characters;
pub mod classes;
pub mod combat;
pub mod duel;
pub mod experience;
pub mod master_level;
pub mod party;
pub mod stats;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use buffs::{
    split_duration, split_minutes_duration, BuffCatalog, BuffClass, BuffDefinition, BuffPlugin,
    BuffRegistry, BuffState, BuffTimeType, BuffTimerRegistry, BuffValueLoadType, DurationParts,
};
pub use characters::{next_experience_for_level, CharacterSheet};
pub use classes::{
    CharacterClass, CharacterSkinIndex, ServerClassType, MASTER_EXPERIENCE_UNLOCK_LEVEL,
};
pub use combat::{
    curse_skill_damage, magic_skill_damage, skill_damage, CombatBonuses, CombatProfile,
    DamageRange, SkillDamageProfile,
};
pub use duel::{
    DuelChannelInfo, DuelCharacterSnapshot, DuelManager, DuelPlayerInfo, DuelPlugin,
    MAX_DUEL_CHANNELS, MAX_DUEL_PLAYERS, MAX_USERNAME_SIZE,
};
pub use experience::{
    next_experience_for_level as experience_next_experience_for_level,
    next_master_level_experience as experience_next_master_level_experience,
    previous_experience_for_level, previous_master_level_experience, ExperienceBand,
    MASTER_LEVEL_EXPERIENCE_OFFSET, MASTER_LEVEL_EXPONENT_BONUS, MASTER_LEVEL_EXPONENT_SCALE,
    MASTER_LEVEL_OVERFLOW_SCALE, NORMAL_EXPERIENCE_LEVEL_BONUS, NORMAL_EXPERIENCE_SCALE,
    OVERLEVEL_EXPERIENCE_SCALE, OVERLEVEL_EXPERIENCE_THRESHOLD,
};
pub use master_level::{next_master_level_experience, MasterLevelState};
pub use party::{
    PartyCharacterSnapshot, PartyManager, PartyMemberInfo, PartyPlugin, MAX_PARTY_ACTIVE_MEMBERS,
    MAX_PARTY_MEMBERS, PARTY_INDEX_HERO, PARTY_INDEX_NOT_FOUND, PARTY_INDEX_UNSEARCHED,
};
pub use stats::{base_class_attributes, ClassAttributes, BASE_CLASS_COUNT, CLASS_ATTRIBUTES};
