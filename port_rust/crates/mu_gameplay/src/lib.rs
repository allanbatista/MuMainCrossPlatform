pub mod characters;
pub mod classes;
pub mod duel;
pub mod master_level;
pub mod party;
pub mod stats;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use characters::{next_experience_for_level, CharacterSheet};
pub use classes::{
    CharacterClass, CharacterSkinIndex, ServerClassType, MASTER_EXPERIENCE_UNLOCK_LEVEL,
};
pub use duel::{
    DuelChannelInfo, DuelCharacterSnapshot, DuelManager, DuelPlayerInfo, DuelPlugin,
    MAX_DUEL_CHANNELS, MAX_DUEL_PLAYERS, MAX_USERNAME_SIZE,
};
pub use master_level::{next_master_level_experience, MasterLevelState};
pub use party::{
    PartyCharacterSnapshot, PartyManager, PartyMemberInfo, PartyPlugin, MAX_PARTY_ACTIVE_MEMBERS,
    MAX_PARTY_MEMBERS, PARTY_INDEX_HERO, PARTY_INDEX_NOT_FOUND, PARTY_INDEX_UNSEARCHED,
};
pub use stats::{base_class_attributes, ClassAttributes, BASE_CLASS_COUNT, CLASS_ATTRIBUTES};
