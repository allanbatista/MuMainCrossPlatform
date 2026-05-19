pub mod assets;
pub mod events;
pub mod settings;

pub use assets::{AudioAssets, AudioAssetsError};
pub use events::{SkillAudioEvent, SkillAudioQueue};
pub use settings::Settings;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
