use bevy::prelude::Resource;
use camino::Utf8Path;
use mu_assets::{load_terrain_world_bundle, TerrainWorldBundle, TerrainWorldError};
use mu_gameplay::{
    MovementManager, PartyManager, WorldEntitiesManager, WorldEntityPose, WorldManager,
    WorldMonsterManager, WorldNpcManager, WorldPlayerRole, WorldPlayerSpawn,
};
use mu_render::{
    RenderAssets, RenderAssetsError, RenderEntities, RenderEntitiesState, TerrainRenderer,
};
use thiserror::Error;

const DEFAULT_LOCAL_PLAYER_KEY: u32 = 0;
const DEFAULT_LOCAL_PLAYER_LABEL: &str = "Player";
const DEFAULT_LOCAL_PLAYER_MODEL: &str = "local-player";
const DEFAULT_LOCAL_PLAYER_POSITION: [f64; 3] = [0.0, 1.0, 0.0];
const DEFAULT_LOCAL_PLAYER_ROTATION: [f64; 3] = [0.0, 0.0, 0.0];
const DEFAULT_LOCAL_PLAYER_SCALE: [f64; 3] = [1.0, 1.0, 1.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClientRuntimeState {
    #[default]
    Inactive,
    AssetsReady,
    WorldReady,
    Ready,
    AssetError,
}

impl ClientRuntimeState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::AssetsReady => "assets-ready",
            Self::WorldReady => "world-ready",
            Self::Ready => "ready",
            Self::AssetError => "asset-error",
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientRuntimeError {
    #[error(transparent)]
    RenderAssets(#[from] RenderAssetsError),
    #[error(transparent)]
    World(#[from] TerrainWorldError),
}

#[derive(Debug, Default, Resource)]
pub struct ClientRuntime {
    render_assets: RenderAssets,
    world: WorldManager,
    terrain: TerrainRenderer,
    movement: MovementManager,
    party: PartyManager,
    world_entities: WorldEntitiesManager,
    world_npcs: WorldNpcManager,
    world_monsters: WorldMonsterManager,
    render_entities: RenderEntities,
    world_error: Option<String>,
}

impl ClientRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_assets(&self) -> &RenderAssets {
        &self.render_assets
    }

    pub fn world(&self) -> &WorldManager {
        &self.world
    }

    pub fn terrain(&self) -> &TerrainRenderer {
        &self.terrain
    }

    pub fn movement(&self) -> &MovementManager {
        &self.movement
    }

    pub fn party(&self) -> &PartyManager {
        &self.party
    }

    pub fn party_mut(&mut self) -> &mut PartyManager {
        &mut self.party
    }

    pub fn world_entities(&self) -> &WorldEntitiesManager {
        &self.world_entities
    }

    pub fn world_npcs(&self) -> &WorldNpcManager {
        &self.world_npcs
    }

    pub fn world_monsters(&self) -> &WorldMonsterManager {
        &self.world_monsters
    }

    pub fn render_entities(&self) -> &RenderEntities {
        &self.render_entities
    }

    pub fn last_error(&self) -> Option<&str> {
        self.world_error
            .as_deref()
            .or_else(|| self.render_assets.last_error())
    }

    pub fn state(&self) -> ClientRuntimeState {
        if self.render_assets.last_error().is_some() || self.world_error.is_some() {
            return ClientRuntimeState::AssetError;
        }

        let assets_ready = matches!(
            self.render_assets.state(),
            mu_render::RenderAssetsState::Ready
        );
        let world_ready = matches!(self.world.state(), mu_gameplay::WorldState::Ready);

        match (assets_ready, world_ready) {
            (true, true) => ClientRuntimeState::Ready,
            (true, false) => ClientRuntimeState::AssetsReady,
            (false, true) => ClientRuntimeState::WorldReady,
            (false, false) => ClientRuntimeState::Inactive,
        }
    }

    pub fn load_render_assets(
        &mut self,
        asset_root: impl AsRef<Utf8Path>,
    ) -> Result<(), ClientRuntimeError> {
        self.render_assets.load_assets(asset_root)?;
        Ok(())
    }

    pub fn load_world_from_assets(
        &mut self,
        asset_root: impl AsRef<Utf8Path>,
        world: u32,
    ) -> Result<(), ClientRuntimeError> {
        match load_terrain_world_bundle(asset_root.as_ref(), world) {
            Ok(bundle) => {
                self.load_world_bundle(bundle);
                Ok(())
            }
            Err(error) => {
                self.clear_world_projection();
                self.world_error = Some(error.to_string());
                Err(error.into())
            }
        }
    }

    pub fn load_world_bundle(&mut self, bundle: TerrainWorldBundle) {
        self.clear_world_projection();
        self.world.set_bundle(bundle);
        self.seed_local_player();
        self.sync_world_projection();
        self.world_error = None;
    }

    pub fn translate_local_player(&mut self, delta: [f64; 3]) {
        self.world_entities.translate_local_player(delta);
        self.sync_world_projection();
    }

    pub fn sync_world_projection(&mut self) {
        self.world_entities.sync_world(&self.world);
        self.terrain.sync_world(&self.world);
        self.movement.sync_world(&self.world);
        self.render_entities.sync_world(
            &self.world_entities,
            &self.world_npcs,
            &self.world_monsters,
        );
    }

    pub fn clear_world_projection(&mut self) {
        self.world.clear();
        self.world_entities.reset();
        self.world_npcs.reset();
        self.world_monsters.reset();
        self.terrain.clear();
        self.movement.reset();
        self.party.reset();
        self.render_entities.reset();
    }

    pub fn snapshot(&self) -> String {
        format!(
            "state={}|render_assets={}|world={}|world_entities={}|terrain={}|movement={}|party_number={}|render_entities={}|world_error={:?}",
            self.state().as_str(),
            self.render_assets.state().as_str(),
            self.world.state().as_str(),
            self.world_entities.state().as_str(),
            self.terrain.state().as_str(),
            self.movement.state().as_str(),
            self.party.party_number(),
            self.render_entities.state().as_str(),
            self.world_error,
        )
    }

    fn seed_local_player(&mut self) {
        if self.world_entities.local_player().is_some() {
            return;
        }

        self.world_entities
            .set_local_player(default_local_player_spawn());
    }
}

impl ClientRuntime {
    pub fn render_assets_ready(&self) -> bool {
        matches!(
            self.render_assets.state(),
            mu_render::RenderAssetsState::Ready
        )
    }

    pub fn world_ready(&self) -> bool {
        matches!(self.world.state(), mu_gameplay::WorldState::Ready)
    }

    pub fn terrain_ready(&self) -> bool {
        matches!(self.terrain.state(), mu_render::TerrainState::Ready)
    }

    pub fn movement_ready(&self) -> bool {
        matches!(self.movement.state(), mu_gameplay::MovementState::Ready)
    }

    pub fn render_entities_ready(&self) -> bool {
        matches!(self.render_entities.state(), RenderEntitiesState::Ready)
    }
}

fn default_local_player_spawn() -> WorldPlayerSpawn {
    WorldPlayerSpawn::new(
        WorldPlayerRole::Local,
        DEFAULT_LOCAL_PLAYER_LABEL,
        DEFAULT_LOCAL_PLAYER_KEY,
        DEFAULT_LOCAL_PLAYER_MODEL,
        WorldEntityPose::new(
            DEFAULT_LOCAL_PLAYER_POSITION,
            DEFAULT_LOCAL_PLAYER_ROTATION,
            DEFAULT_LOCAL_PLAYER_SCALE,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ClientRuntime, ClientRuntimeState, DEFAULT_LOCAL_PLAYER_LABEL,
        DEFAULT_LOCAL_PLAYER_POSITION,
    };
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use camino::Utf8PathBuf;
    use mu_assets::{
        load_terrain_world_bundle, sha256_hex, AssetManifest, AssetManifestEntry,
        MANIFEST_FILE_NAME, SUPPORTED_SCHEMA_VERSION,
    };
    use mu_gameplay::PartyMemberInfo;

    fn repo_world_root() -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../port_rust/assets")
                .canonicalize()
                .unwrap(),
        )
        .unwrap()
    }

    fn temp_render_assets_root() -> Utf8PathBuf {
        let root = std::env::temp_dir().join(format!(
            "mu_client_runtime_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let root = Utf8PathBuf::from_path_buf(root).unwrap();
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn unique_suffix() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
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
    fn runtime_starts_inactive() {
        let runtime = ClientRuntime::new();

        assert_eq!(runtime.state(), ClientRuntimeState::Inactive);
        assert!(runtime.last_error().is_none());
        assert_eq!(runtime.party().party_number(), 0);
        assert!(!runtime.render_assets_ready());
        assert!(!runtime.world_ready());
        assert!(!runtime.terrain_ready());
        assert!(!runtime.movement_ready());
        assert!(!runtime.render_entities_ready());
    }

    #[test]
    fn runtime_loads_world_bundle_and_render_assets() {
        let render_root = temp_render_assets_root();
        let world_root = repo_world_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();
        let render_bytes = b"render-texture";
        let model_bytes = b"render-model";
        write_file(&render_root, "data/world_1/TileGround01.png", render_bytes);
        write_file(&render_root, "data/world_1/terrain.glb", model_bytes);
        write_manifest(
            &render_root,
            &manifest_fixture(vec![
                manifest_entry(
                    "World1/TileGround01.bmp",
                    "data/world_1/TileGround01.png",
                    "texture",
                    render_bytes,
                ),
                manifest_entry(
                    "World1/Terrain.bmd",
                    "data/world_1/terrain.glb",
                    "model",
                    model_bytes,
                ),
            ]),
        );

        let mut runtime = ClientRuntime::new();

        runtime.load_world_bundle(bundle);

        assert_eq!(runtime.state(), ClientRuntimeState::WorldReady);
        assert!(!runtime.render_assets_ready());
        assert!(runtime.world_ready());
        assert!(runtime.terrain_ready());
        assert!(runtime.movement_ready());
        assert!(runtime.render_entities_ready());
        assert_eq!(
            runtime.world_entities().local_player().unwrap().label,
            DEFAULT_LOCAL_PLAYER_LABEL
        );
        assert_eq!(
            runtime
                .world_entities()
                .local_player()
                .unwrap()
                .pose
                .position,
            DEFAULT_LOCAL_PLAYER_POSITION
        );
        assert_eq!(runtime.party().party_number(), 0);
        assert!(runtime.snapshot().contains("render_assets=inactive"));
        assert!(runtime.snapshot().contains("world=ready"));
        assert!(runtime.snapshot().contains("party_number=0"));

        runtime.translate_local_player([4.0, 0.0, -2.0]);

        assert_eq!(
            runtime
                .world_entities()
                .local_player()
                .unwrap()
                .pose
                .position,
            [4.0, 1.0, -2.0]
        );
        assert_eq!(
            runtime
                .render_entities()
                .catalog()
                .local_player
                .as_ref()
                .unwrap()
                .pose
                .position,
            [4.0, 1.0, -2.0]
        );

        runtime.load_render_assets(&render_root).unwrap();

        assert_eq!(runtime.state(), ClientRuntimeState::Ready);
        assert!(runtime.render_assets_ready());
        assert!(runtime.world_ready());
        assert!(runtime.terrain_ready());
        assert!(runtime.movement_ready());
        assert!(runtime.render_entities_ready());
        assert_eq!(runtime.party().party_number(), 0);
        assert!(runtime.snapshot().contains("render_assets=ready"));
        assert!(runtime.snapshot().contains("world=ready"));
        assert!(runtime.snapshot().contains("party_number=0"));
        assert!(runtime.snapshot().contains("render_entities=ready"));
    }

    #[test]
    fn runtime_reports_asset_errors() {
        let mut runtime = ClientRuntime::new();
        let missing_root = Utf8PathBuf::from("__missing_mu_asset_root__");

        assert!(runtime.load_render_assets(&missing_root).is_err());
        assert_eq!(runtime.state(), ClientRuntimeState::AssetError);
        assert!(runtime.last_error().is_some());
        assert!(runtime.snapshot().contains("render_assets=asset-error"));
    }

    #[test]
    fn runtime_clears_party_state_with_projection_reset() {
        let mut runtime = ClientRuntime::new();
        runtime.party_mut().set_member(
            0,
            PartyMemberInfo {
                name: "Astra".into(),
                ..PartyMemberInfo::default()
            },
        );
        runtime.party_mut().set_party_number(1);

        runtime.clear_world_projection();

        assert_eq!(runtime.party().party_number(), 0);
        assert!(runtime
            .party()
            .members()
            .iter()
            .all(|member| member.name.is_empty()));
        assert_eq!(
            runtime.party().members()[0].index,
            mu_gameplay::PARTY_INDEX_UNSEARCHED
        );
    }
}
