pub mod assets;
pub mod diagnostics;
pub mod events;
pub mod runtime;
pub mod settings;

pub use assets::{AudioAssets, AudioAssetsError};
pub use diagnostics::AudioRuntimeDiagnostics;
pub use events::{SkillAudioEvent, SkillAudioQueue};
pub use runtime::{AudioRuntime, AudioRuntimeError, AudioRuntimePlugin, AudioRuntimeState};
pub use settings::Settings;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
