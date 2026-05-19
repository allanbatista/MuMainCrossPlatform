use bevy::prelude::{App, Plugin, Resource};
use camino::{Utf8Path, Utf8PathBuf};
use mu_assets::{validate_asset_root, AssetManifest, AssetRuntimeError};
use thiserror::Error;

use crate::{
    build_render_material_catalog, collect_render_models, collect_render_textures,
    RenderMaterialCatalog, RenderModelCatalog, RenderTextureCatalog,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderAssetsState {
    #[default]
    Inactive,
    Ready,
    AssetError,
}

impl RenderAssetsState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
            Self::AssetError => "asset-error",
        }
    }
}

#[derive(Debug, Default)]
pub struct RenderAssets {
    asset_root: Option<Utf8PathBuf>,
    manifest: Option<AssetManifest>,
    textures: RenderTextureCatalog,
    models: RenderModelCatalog,
    materials: RenderMaterialCatalog,
    last_error: Option<String>,
}

impl Resource for RenderAssets {}

#[derive(Debug, Default, Clone, Copy)]
pub struct RenderAssetsPlugin;

impl Plugin for RenderAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderAssets>();
    }
}

#[derive(Debug, Error)]
pub enum RenderAssetsError {
    #[error(transparent)]
    Runtime(#[from] AssetRuntimeError),
}

impl RenderAssets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> RenderAssetsState {
        if self.last_error.is_some() {
            return RenderAssetsState::AssetError;
        }

        if self.manifest.is_some() {
            return RenderAssetsState::Ready;
        }

        RenderAssetsState::Inactive
    }

    pub fn asset_root(&self) -> Option<&Utf8Path> {
        self.asset_root.as_deref()
    }

    pub fn manifest(&self) -> Option<&AssetManifest> {
        self.manifest.as_ref()
    }

    pub fn textures(&self) -> &RenderTextureCatalog {
        &self.textures
    }

    pub fn models(&self) -> &RenderModelCatalog {
        &self.models
    }

    pub fn materials(&self) -> &RenderMaterialCatalog {
        &self.materials
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn load_assets(
        &mut self,
        asset_root: impl AsRef<Utf8Path>,
    ) -> Result<(), RenderAssetsError> {
        self.clear_assets();

        match validate_asset_root(asset_root.as_ref()) {
            Ok(manifest) => {
                let textures = collect_render_textures(&manifest);
                let models = collect_render_models(&manifest);
                let materials = build_render_material_catalog(&textures);

                self.asset_root = Some(asset_root.as_ref().to_path_buf());
                self.manifest = Some(manifest);
                self.textures = textures;
                self.models = models;
                self.materials = materials;
                self.last_error = None;
                Ok(())
            }
            Err(error) => {
                self.last_error = Some(error.to_string());
                Err(error.into())
            }
        }
    }

    pub fn clear_assets(&mut self) {
        self.asset_root = None;
        self.manifest = None;
        self.textures = RenderTextureCatalog::default();
        self.models = RenderModelCatalog::default();
        self.materials = RenderMaterialCatalog::default();
        self.last_error = None;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use bevy::prelude::App;
    use camino::Utf8PathBuf;

    use super::{RenderAssets, RenderAssetsError, RenderAssetsPlugin, RenderAssetsState};
    use mu_assets::{
        sha256_hex, AssetManifest, AssetManifestEntry, AssetRuntimeError, MANIFEST_FILE_NAME,
        SUPPORTED_SCHEMA_VERSION,
    };

    fn temp_root() -> Utf8PathBuf {
        let root = std::env::temp_dir().join(format!(
            "mu_render_assets_{}_{}",
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

    fn write_file(root: &Utf8PathBuf, relative_path: &str, contents: &[u8]) {
        let path = root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    fn manifest_entry(
        source_path: &str,
        converted_path: &str,
        kind: &str,
        contents: &[u8],
    ) -> AssetManifestEntry {
        AssetManifestEntry {
            source_path: source_path.to_string(),
            converted_path: converted_path.to_string(),
            kind: kind.to_string(),
            source_hash: sha256_hex(source_path.as_bytes()),
            converted_hash: sha256_hex(contents),
            dependencies: Vec::new(),
        }
    }

    fn manifest_fixture(entries: Vec<AssetManifestEntry>) -> AssetManifest {
        AssetManifest {
            schema_version: SUPPORTED_SCHEMA_VERSION,
            source_client_version: "1.0.0".to_string(),
            content_hash: String::new(),
            generated_at: "2026-05-19T00:00:00Z".to_string(),
            entries,
        }
        .with_computed_content_hash()
    }

    #[test]
    fn render_assets_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(RenderAssetsPlugin);

        let assets = app.world().resource::<RenderAssets>();
        assert_eq!(assets.state(), RenderAssetsState::Inactive);
        assert!(assets.textures().is_empty());
        assert!(assets.models().is_empty());
        assert!(assets.materials().is_empty());
    }

    #[test]
    fn render_assets_loads_catalogs_from_valid_root() {
        let root = temp_root();
        let interface_bytes = b"interface";
        let world_bytes = b"world";
        let object_bytes = b"object";
        let scene_bytes = b"scene";

        write_file(&root, "Interface/icon.bmp", interface_bytes);
        write_file(&root, "World1/TerrainLight.bmp", world_bytes);
        write_file(&root, "Object1/tree.bmp", object_bytes);
        write_file(&root, "World1/scene.obj", scene_bytes);
        write_file(&root, "Interface/icon.png", interface_bytes);
        write_file(&root, "data/world_1/TerrainLight.png", world_bytes);
        write_file(&root, "data/object_1/tree.png", object_bytes);
        write_file(&root, "data/world_1/terrain.glb", b"terrain");
        write_file(&root, "data/object_1/tree.glb", b"tree");
        write_file(&root, "data/world_1/scene_objects.json", scene_bytes);

        let manifest = manifest_fixture(vec![
            manifest_entry(
                "Interface/icon.bmp",
                "Interface/icon.png",
                "texture",
                interface_bytes,
            ),
            manifest_entry(
                "World1/TerrainLight.bmp",
                "data/world_1/TerrainLight.png",
                "texture",
                world_bytes,
            ),
            manifest_entry(
                "Object1/tree.bmp",
                "data/object_1/tree.png",
                "texture",
                object_bytes,
            ),
            manifest_entry(
                "World1/Terrain.bmd",
                "data/world_1/terrain.glb",
                "model",
                b"terrain",
            ),
            manifest_entry(
                "Object1/tree.bmd",
                "data/object_1/tree.glb",
                "model",
                b"tree",
            ),
            manifest_entry(
                "World1/scene.obj",
                "data/world_1/scene_objects.json",
                "model",
                scene_bytes,
            ),
        ]);
        write_manifest(&root, &manifest);

        let mut assets = RenderAssets::new();
        assets.load_assets(&root).unwrap();

        assert_eq!(assets.state(), RenderAssetsState::Ready);
        assert_eq!(assets.asset_root(), Some(root.as_path()));
        assert_eq!(assets.manifest().unwrap().entries.len(), 6);
        assert_eq!(assets.textures().total_textures(), 3);
        assert_eq!(assets.models().total_models(), 3);
        assert_eq!(assets.materials().total_materials(), 3);
        assert_eq!(assets.last_error(), None);
    }

    #[test]
    fn missing_asset_root_is_reported() {
        let root = Utf8PathBuf::from("/tmp/mu_render_assets_missing_root");
        let mut assets = RenderAssets::new();

        let error = assets.load_assets(&root).unwrap_err();

        assert!(matches!(
            error,
            RenderAssetsError::Runtime(AssetRuntimeError::MissingAssetRoot { .. })
        ));
        assert_eq!(assets.state(), RenderAssetsState::AssetError);
        assert!(assets.last_error().is_some());
    }
}
