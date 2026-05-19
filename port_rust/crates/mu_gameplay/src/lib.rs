pub mod duel;
pub mod party;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use duel::{
    DuelChannelInfo, DuelCharacterSnapshot, DuelManager, DuelPlayerInfo, DuelPlugin,
    MAX_DUEL_CHANNELS, MAX_DUEL_PLAYERS, MAX_USERNAME_SIZE,
};
pub use party::{
    PartyCharacterSnapshot, PartyManager, PartyMemberInfo, PartyPlugin, MAX_PARTY_ACTIVE_MEMBERS,
    MAX_PARTY_MEMBERS, PARTY_INDEX_HERO, PARTY_INDEX_NOT_FOUND, PARTY_INDEX_UNSEARCHED,
};
