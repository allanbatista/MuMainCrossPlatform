use std::collections::BTreeMap;
use std::fs;
use std::io;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerrainMapHeader {
    pub version: u8,
    pub map_number: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainLayerStats {
    pub min: u8,
    pub max: u8,
    pub mean: f64,
    pub unique_values: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainLayerStatsSet {
    pub layer1: TerrainLayerStats,
    pub layer2: TerrainLayerStats,
    pub alpha: TerrainLayerStats,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainMapJson {
    pub header: TerrainMapHeader,
    pub terrain_size: u32,
    pub layer_stats: TerrainLayerStatsSet,
    pub layer1: Vec<Vec<u8>>,
    pub layer2: Vec<Vec<u8>>,
    pub alpha: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainWorldSize {
    pub width: u32,
    pub depth: u32,
    pub scale: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainTextureLayer {
    pub id: String,
    pub path: String,
    pub scale: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerrainWorldMetadata {
    #[serde(default)]
    pub generated_placeholder: bool,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub world: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainWorldConfig {
    pub size: TerrainWorldSize,
    pub height_multiplier: f64,
    pub legacy_terrain_scale: f64,
    pub texture_layers: Vec<TerrainTextureLayer>,
    pub alpha_map: String,
    pub lightmap: String,
    #[serde(default)]
    pub metadata: TerrainWorldMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerrainTextureSlotMetadata {
    #[serde(default)]
    pub generated: bool,
    #[serde(default)]
    pub generator: String,
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerrainTextureSlotCatalog {
    pub world: u32,
    pub slots: BTreeMap<u8, String>,
    #[serde(default)]
    pub metadata: TerrainTextureSlotMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainCameraTourMetadata {
    #[serde(default)]
    pub generated_placeholder: bool,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub world: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainCameraWaypoint {
    pub index: u32,
    pub position: [f64; 3],
    pub look_at: [f64; 3],
    pub move_acceleration: f64,
    pub distance_level: f64,
    pub delay: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainCameraTour {
    pub waypoints: Vec<TerrainCameraWaypoint>,
    #[serde(rename = "loop")]
    pub r#loop: bool,
    pub blend_distance: f64,
    pub interpolation: String,
    #[serde(default)]
    pub metadata: TerrainCameraTourMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainSceneObject {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: u32,
    pub model: String,
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub scale: [f64; 3],
    #[serde(default)]
    pub properties: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainSceneObjectCollection {
    pub objects: Vec<TerrainSceneObject>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerrainWorldBundle {
    pub world: u32,
    pub world_directory: Utf8PathBuf,
    pub config: TerrainWorldConfig,
    pub map: TerrainMapJson,
    pub texture_slots: TerrainTextureSlotCatalog,
    pub camera_tour: TerrainCameraTour,
    pub scene_objects: TerrainSceneObjectCollection,
    pub height_map_path: Utf8PathBuf,
}

impl TerrainWorldBundle {
    pub fn summary(&self) -> TerrainWorldSummary {
        TerrainWorldSummary {
            world: self.world,
            world_directory: self.world_directory.to_string(),
            map_number: self.map.header.map_number,
            terrain_size: self.map.terrain_size,
            texture_layers: self
                .config
                .texture_layers
                .iter()
                .map(|layer| layer.id.clone())
                .collect(),
            texture_slots: self.texture_slots.slots.len(),
            scene_objects: self.scene_objects.objects.len(),
            camera_waypoints: self.camera_tour.waypoints.len(),
            layer_stats: self.map.layer_stats.clone(),
            height_map_path: self.height_map_path.to_string(),
            alpha_map_path: self.config.alpha_map.clone(),
            lightmap_path: self.config.lightmap.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerrainWorldSummary {
    pub world: u32,
    pub world_directory: String,
    pub map_number: u8,
    pub terrain_size: u32,
    pub texture_layers: Vec<String>,
    pub texture_slots: usize,
    pub scene_objects: usize,
    pub camera_waypoints: usize,
    pub layer_stats: TerrainLayerStatsSet,
    pub height_map_path: String,
    pub alpha_map_path: String,
    pub lightmap_path: String,
}

impl TerrainWorldSummary {
    pub fn snapshot(&self) -> String {
        format!(
            "world={}|world_directory={}|map_number={}|terrain_size={}|texture_layers=[{}]|layer_stats=layer1[min={}|max={}|mean={:.5}|unique={}];layer2[min={}|max={}|mean={:.5}|unique={}];alpha[min={}|max={}|mean={:.5}|unique={}]|texture_slots={}|scene_objects={}|camera_waypoints={}|height_map_path={}|alpha_map_path={}|lightmap_path={}",
            self.world,
            self.world_directory,
            self.map_number,
            self.terrain_size,
            self.texture_layers.join(","),
            self.layer_stats.layer1.min,
            self.layer_stats.layer1.max,
            self.layer_stats.layer1.mean,
            self.layer_stats.layer1.unique_values,
            self.layer_stats.layer2.min,
            self.layer_stats.layer2.max,
            self.layer_stats.layer2.mean,
            self.layer_stats.layer2.unique_values,
            self.layer_stats.alpha.min,
            self.layer_stats.alpha.max,
            self.layer_stats.alpha.mean,
            self.layer_stats.alpha.unique_values,
            self.texture_slots,
            self.scene_objects,
            self.camera_waypoints,
            self.height_map_path,
            self.alpha_map_path,
            self.lightmap_path,
        )
    }
}

#[derive(Debug, Error)]
pub enum TerrainWorldError {
    #[error("terrain world root missing: {path}")]
    MissingWorldRoot { path: Utf8PathBuf },
    #[error("terrain world directory missing: {path}")]
    MissingWorldDirectory { path: Utf8PathBuf },
    #[error("failed to read terrain world file {path}: {source}")]
    ReadWorldFile {
        path: Utf8PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse terrain world file {path}: {source}")]
    ParseWorldFile {
        path: Utf8PathBuf,
        #[source]
        source: serde_json::Error,
    },
}

pub fn terrain_world_relative_directory(world: u32) -> Utf8PathBuf {
    Utf8PathBuf::from(format!("data/world_{world}"))
}

pub fn terrain_world_directory(asset_root: &Utf8Path, world: u32) -> Utf8PathBuf {
    asset_root.join(terrain_world_relative_directory(world))
}

pub fn load_terrain_world_bundle(
    asset_root: &Utf8Path,
    world: u32,
) -> Result<TerrainWorldBundle, TerrainWorldError> {
    if !asset_root.exists() || !asset_root.is_dir() {
        return Err(TerrainWorldError::MissingWorldRoot {
            path: asset_root.to_path_buf(),
        });
    }

    let world_directory = terrain_world_directory(asset_root, world);
    if !world_directory.exists() || !world_directory.is_dir() {
        return Err(TerrainWorldError::MissingWorldDirectory {
            path: world_directory,
        });
    }

    let config: TerrainWorldConfig = load_json_file(&world_directory.join("terrain_config.json"))?;
    let map: TerrainMapJson = load_json_file(&world_directory.join("terrain_map.json"))?;
    let texture_slots: TerrainTextureSlotCatalog =
        load_json_file(&world_directory.join("terrain_texture_slots.json"))?;
    let camera_tour: TerrainCameraTour = load_json_file(&world_directory.join("camera_tour.json"))?;
    let scene_objects: TerrainSceneObjectCollection =
        load_json_file(&world_directory.join("scene_objects.json"))?;

    Ok(TerrainWorldBundle {
        world,
        world_directory: terrain_world_relative_directory(world),
        config,
        map,
        texture_slots,
        camera_tour,
        scene_objects,
        height_map_path: terrain_world_relative_directory(world).join("terrain_height.json"),
    })
}

fn load_json_file<T>(path: &Utf8Path) -> Result<T, TerrainWorldError>
where
    T: DeserializeOwned,
{
    let contents = fs::read_to_string(path).map_err(|source| TerrainWorldError::ReadWorldFile {
        path: path.to_path_buf(),
        source,
    })?;

    serde_json::from_str(&contents).map_err(|source| TerrainWorldError::ParseWorldFile {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use super::{load_terrain_world_bundle, terrain_world_relative_directory, TerrainWorldSummary};

    fn repo_root() -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../")
                .canonicalize()
                .unwrap(),
        )
        .unwrap()
    }

    fn fixture_asset_root() -> Utf8PathBuf {
        repo_root().join("port_rust/crates/mu_assets/tests/fixtures/terrain_world_root")
    }

    #[test]
    fn terrain_world_loader_reads_runtime_bundle() {
        let asset_root = fixture_asset_root();
        let bundle = load_terrain_world_bundle(&asset_root, 1).unwrap();

        assert_eq!(bundle.world, 1);
        assert_eq!(bundle.world_directory, terrain_world_relative_directory(1));
        assert_eq!(
            bundle.height_map_path,
            terrain_world_relative_directory(1).join("terrain_height.json")
        );
        assert_eq!(bundle.config.size.width, 256);
        assert_eq!(bundle.config.texture_layers.len(), 3);
        assert_eq!(bundle.texture_slots.slots.len(), 17);
        assert_eq!(bundle.camera_tour.waypoints.len(), 8);
        assert!(!bundle.scene_objects.objects.is_empty());
        assert_eq!(bundle.scene_objects.objects[0].id, "obj_00000");
        assert_eq!(
            bundle.scene_objects.objects[0].model,
            "data/object_1/stone_01.glb"
        );
        assert_eq!(bundle.map.header.map_number, 1);
        assert_eq!(bundle.map.layer_stats.layer1.unique_values, 9);
        assert_eq!(bundle.map.layer_stats.layer2.unique_values, 8);
        assert_eq!(bundle.map.layer_stats.alpha.unique_values, 17);
    }

    #[test]
    fn terrain_world_summary_matches_expected_snapshot() {
        let asset_root = fixture_asset_root();
        let bundle = load_terrain_world_bundle(&asset_root, 1).unwrap();
        let summary: TerrainWorldSummary = bundle.summary();

        assert_eq!(
            summary.snapshot(),
            "world=1|world_directory=data/world_1|map_number=1|terrain_size=256|texture_layers=[grass01,ground01,rock01]|layer_stats=layer1[min=0|max=8|mean=0.76103|unique=9];layer2[min=0|max=255|mean=171.43759|unique=8];alpha[min=0|max=255|mean=38.53535|unique=17]|texture_slots=17|scene_objects=2870|camera_waypoints=8|height_map_path=data/world_1/terrain_height.json|alpha_map_path=data/world_1/alpha_tile01.png|lightmap_path=data/world_1/terrain_light.png"
        );
    }
}
