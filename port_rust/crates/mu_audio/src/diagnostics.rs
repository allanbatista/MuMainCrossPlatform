use std::fmt;

use crate::{AudioRuntime, AudioRuntimeState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioRuntimeDiagnostics {
    pub state: AudioRuntimeState,
    pub asset_root: Option<String>,
    pub music_assets: usize,
    pub sound_effect_assets: usize,
    pub queued_audio_events: usize,
    pub master_volume: u8,
    pub music_volume: u8,
    pub effects_volume: u8,
    pub muted: bool,
    pub last_error: Option<String>,
}

impl AudioRuntimeDiagnostics {
    pub fn from_runtime(runtime: &AudioRuntime) -> Self {
        let (asset_root, music_assets, sound_effect_assets) = match runtime.assets() {
            Some(assets) => (
                Some(assets.asset_root().to_string()),
                assets.music().len(),
                assets.sound_effects().len(),
            ),
            None => (None, 0, 0),
        };

        let settings = runtime.settings();
        Self {
            state: runtime.state(),
            asset_root,
            music_assets,
            sound_effect_assets,
            queued_audio_events: runtime.pending_audio_events(),
            master_volume: settings.master,
            music_volume: settings.music,
            effects_volume: settings.effects,
            muted: settings.is_muted(),
            last_error: runtime.last_error().map(str::to_owned),
        }
    }
}

impl fmt::Display for AudioRuntimeDiagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "state={}|asset_root={:?}|music_assets={}|sound_effect_assets={}|queued_audio_events={}|master_volume={}|music_volume={}|effects_volume={}|muted={}|last_error={:?}",
            self.state.as_str(),
            self.asset_root,
            self.music_assets,
            self.sound_effect_assets,
            self.queued_audio_events,
            self.master_volume,
            self.music_volume,
            self.effects_volume,
            self.muted,
            self.last_error
        )
    }
}
