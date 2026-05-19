pub mod duel;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use duel::{
    DuelChannelInfo, DuelCharacterSnapshot, DuelManager, DuelPlayerInfo, DuelPlugin,
    MAX_DUEL_CHANNELS, MAX_DUEL_PLAYERS, MAX_USERNAME_SIZE,
};
