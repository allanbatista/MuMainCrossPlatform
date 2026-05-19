use bevy::prelude::{App, Plugin, Resource};
use camino::Utf8Path;
use thiserror::Error;

use crate::diagnostics::AudioRuntimeDiagnostics;
use crate::{AudioAssets, AudioAssetsError, Settings, SkillAudioEvent, SkillAudioQueue};
use mu_gameplay::skills::{SkillId, SkillPresentation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioRuntimeState {
    #[default]
    Inactive,
    Ready,
    Muted,
    AssetError,
}

impl AudioRuntimeState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
            Self::Muted => "muted",
            Self::AssetError => "asset-error",
        }
    }
}

#[derive(Debug, Default)]
pub struct AudioRuntime {
    settings: Settings,
    assets: Option<AudioAssets>,
    queued_audio: SkillAudioQueue,
    last_error: Option<String>,
}

impl Resource for AudioRuntime {}

#[derive(Debug, Default, Clone, Copy)]
pub struct AudioRuntimePlugin;

impl Plugin for AudioRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioRuntime>();
    }
}

#[derive(Debug, Error)]
pub enum AudioRuntimeError {
    #[error(transparent)]
    Assets(#[from] AudioAssetsError),
}

impl AudioRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn set_settings(&mut self, settings: Settings) {
        self.settings = settings.normalized();
    }

    pub fn assets(&self) -> Option<&AudioAssets> {
        self.assets.as_ref()
    }

    pub fn asset_root(&self) -> Option<&Utf8Path> {
        self.assets().map(AudioAssets::asset_root)
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn state(&self) -> AudioRuntimeState {
        if self.last_error.is_some() {
            return AudioRuntimeState::AssetError;
        }

        if self.assets.is_some() {
            return if self.settings.is_muted() {
                AudioRuntimeState::Muted
            } else {
                AudioRuntimeState::Ready
            };
        }

        AudioRuntimeState::Inactive
    }

    pub fn load_assets(
        &mut self,
        asset_root: impl AsRef<Utf8Path>,
    ) -> Result<(), AudioRuntimeError> {
        match AudioAssets::load(asset_root) {
            Ok(assets) => {
                self.assets = Some(assets);
                self.last_error = None;
                Ok(())
            }
            Err(error) => {
                self.assets = None;
                self.last_error = Some(error.to_string());
                Err(error.into())
            }
        }
    }

    pub fn set_assets(&mut self, assets: AudioAssets) {
        self.assets = Some(assets);
        self.last_error = None;
    }

    pub fn clear_assets(&mut self) {
        self.assets = None;
        self.last_error = None;
    }

    pub fn queue_skill_audio(
        &mut self,
        skill_id: SkillId,
        presentation: SkillPresentation,
        looped: bool,
    ) -> bool {
        self.queued_audio
            .push_presentation(skill_id, presentation, looped)
    }

    pub fn push_audio_event(&mut self, event: SkillAudioEvent) {
        self.queued_audio.push(event);
    }

    pub fn pending_audio_events(&self) -> usize {
        self.queued_audio.len()
    }

    pub fn drain_audio_events(&mut self) -> Vec<SkillAudioEvent> {
        self.queued_audio.drain()
    }

    pub fn pop_audio_event(&mut self) -> Option<SkillAudioEvent> {
        self.queued_audio.pop()
    }

    pub fn diagnostics(&self) -> AudioRuntimeDiagnostics {
        AudioRuntimeDiagnostics::from_runtime(self)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use bevy::prelude::App;
    use camino::Utf8PathBuf;

    use super::{AudioRuntime, AudioRuntimeError, AudioRuntimePlugin, AudioRuntimeState};
    use crate::{AudioAssetsError, AudioRuntimeDiagnostics, Settings, SkillAudioEvent};
    use mu_assets::{
        sha256_hex, AssetManifest, AssetManifestEntry, AssetRuntimeError, MANIFEST_FILE_NAME,
        SUPPORTED_SCHEMA_VERSION,
    };
    use mu_gameplay::skills::{SkillAudioCue, SkillEffectCue, SkillPresentation};

    fn temp_root() -> Utf8PathBuf {
        let root = std::env::temp_dir().join(format!(
            "mu_audio_runtime_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let root = Utf8PathBuf::from_path_buf(root).unwrap();
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }

    fn write_manifest(root: &Utf8PathBuf, manifest: &AssetManifest) {
        fs::write(
            root.join(MANIFEST_FILE_NAME),
            manifest.to_json_string().unwrap(),
        )
        .unwrap();
    }

    fn write_audio_file(root: &Utf8PathBuf, relative_path: &str, contents: &[u8]) {
        let path = root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    fn manifest_entry(relative_path: &str, contents: &[u8]) -> AssetManifestEntry {
        AssetManifestEntry {
            source_path: relative_path.to_string(),
            converted_path: relative_path.to_string(),
            kind: "audio".to_string(),
            source_hash: sha256_hex(contents),
            converted_hash: sha256_hex(contents),
            dependencies: Vec::new(),
        }
    }

    fn manifest_fixture(entries: Vec<AssetManifestEntry>) -> AssetManifest {
        AssetManifest {
            schema_version: SUPPORTED_SCHEMA_VERSION,
            source_client_version: "1.0.0".to_string(),
            content_hash: String::new(),
            generated_at: "2026-05-18T00:00:00Z".to_string(),
            entries,
        }
        .with_computed_content_hash()
    }

    #[test]
    fn runtime_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(AudioRuntimePlugin);

        let runtime = app.world().resource::<AudioRuntime>();
        assert_eq!(runtime.state(), AudioRuntimeState::Inactive);
        assert_eq!(runtime.pending_audio_events(), 0);
        assert_eq!(runtime.settings(), &Settings::default());
    }

    #[test]
    fn runtime_loads_assets_and_reports_diagnostics() {
        let root = temp_root();
        let music_bytes = b"music";
        let sound_bytes = b"sound";
        write_audio_file(&root, "Data/Music/Pub.mp3", music_bytes);
        write_audio_file(&root, "Data/Sound/iButtonClick.wav", sound_bytes);

        let manifest = manifest_fixture(vec![
            manifest_entry("Data/Music/Pub.mp3", music_bytes),
            manifest_entry("Data/Sound/iButtonClick.wav", sound_bytes),
        ]);
        write_manifest(&root, &manifest);

        let mut runtime = AudioRuntime::new();
        runtime.set_settings(Settings {
            master: 7,
            music: 6,
            effects: 4,
            mute: false,
        });
        runtime.load_assets(&root).unwrap();
        assert_eq!(runtime.state(), AudioRuntimeState::Ready);

        let diagnostics: AudioRuntimeDiagnostics = runtime.diagnostics();
        assert_eq!(diagnostics.state, AudioRuntimeState::Ready);
        assert_eq!(diagnostics.asset_root.as_deref(), Some(root.as_str()));
        assert_eq!(diagnostics.music_assets, 1);
        assert_eq!(diagnostics.sound_effect_assets, 1);
        assert_eq!(diagnostics.queued_audio_events, 0);
        assert_eq!(diagnostics.master_volume, 7);
        assert_eq!(diagnostics.music_volume, 6);
        assert_eq!(diagnostics.effects_volume, 4);
        assert!(!diagnostics.muted);
        assert_eq!(diagnostics.last_error, None);
        assert!(diagnostics.to_string().contains("state=ready"));
    }

    #[test]
    fn runtime_queues_and_drains_audio_events() {
        let mut runtime = AudioRuntime::new();
        let presentation =
            SkillPresentation::new(SkillEffectCue::MagicCast, SkillAudioCue::IceArrow);

        assert!(runtime.queue_skill_audio(51, presentation, true));
        runtime.push_audio_event(SkillAudioEvent {
            skill_id: 52,
            cue: SkillAudioCue::Bow,
            looped: false,
        });
        assert_eq!(runtime.pending_audio_events(), 2);
        assert_eq!(
            runtime.pop_audio_event(),
            Some(SkillAudioEvent {
                skill_id: 51,
                cue: SkillAudioCue::IceArrow,
                looped: true,
            })
        );
        assert_eq!(runtime.drain_audio_events().len(), 1);
        assert_eq!(runtime.pending_audio_events(), 0);
    }

    #[test]
    fn runtime_reports_missing_assets_through_last_error() {
        let root = Utf8PathBuf::from_path_buf(std::env::temp_dir().join(format!(
            "mu_audio_missing_root_{}_{}",
            std::process::id(),
            unique_suffix()
        )))
        .unwrap();
        let mut runtime = AudioRuntime::new();

        let error = runtime.load_assets(&root).unwrap_err();

        assert!(matches!(
            error,
            AudioRuntimeError::Assets(AudioAssetsError::Runtime(
                AssetRuntimeError::MissingAssetRoot { .. }
            ))
        ));
        assert_eq!(runtime.state(), AudioRuntimeState::AssetError);
        assert!(runtime.last_error().is_some());
    }
}
