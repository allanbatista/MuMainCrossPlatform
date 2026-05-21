use bevy::prelude::{App, Plugin, Resource};
pub use mu_assets::{
    load_terrain_world_bundle, terrain_world_directory, terrain_world_relative_directory,
    TerrainWorldBundle, TerrainWorldError, TerrainWorldSummary,
};

const MAP_NAME_UNKNOWN: &str = "Unknown";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorldState {
    #[default]
    Inactive,
    Ready,
}

impl WorldState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
        }
    }
}

#[derive(Debug, Default)]
pub struct WorldManager {
    terrain_world: Option<TerrainWorldBundle>,
}

impl Resource for WorldManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldManager>();
    }
}

impl WorldManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> WorldState {
        if self.terrain_world.is_some() {
            WorldState::Ready
        } else {
            WorldState::Inactive
        }
    }

    pub fn bundle(&self) -> Option<&TerrainWorldBundle> {
        self.terrain_world.as_ref()
    }

    pub fn summary(&self) -> Option<TerrainWorldSummary> {
        self.terrain_world.as_ref().map(TerrainWorldBundle::summary)
    }

    pub fn set_bundle(&mut self, bundle: TerrainWorldBundle) {
        self.terrain_world = Some(bundle);
    }

    pub fn clear(&mut self) {
        self.terrain_world = None;
    }

    pub fn snapshot(&self) -> String {
        match self.summary() {
            Some(summary) => format!("state={}|{}", self.state().as_str(), summary.snapshot()),
            None => "state=inactive".to_string(),
        }
    }
}

pub const fn map_name(map_id: i32) -> &'static str {
    match map_id {
        0 => "Lorencia",
        1 => "Dungeon",
        2 => "Devias",
        3 => "Noria",
        4 => "Lost Tower",
        5 => MAP_NAME_UNKNOWN,
        6 => "Stadium",
        7 => "Atlans",
        8 => "Tarkan",
        9 => "Devil Square",
        10 => "Icarus",
        11..=17 => "Blood Castle",
        18..=23 => "Chaos Castle",
        24..=29 => "Hellas",
        30 => "Battle Castle",
        31 => "Hunting Ground",
        32 => "Devil Square",
        33 => "Aida",
        34 | 35 => "Crywolf",
        36 => "Hidden Hellas",
        37 => "Kanturu 1st",
        38 => "Kanturu 2nd",
        39 => "Kanturu 3rd",
        40 => "GM Area",
        41 => "Balgas Barrack",
        42 => "Balgas Refuge",
        45..=50 => "Cursed Temple",
        51 => "6th Character Home",
        52 => "Blood Castle",
        53 => "Chaos Castle",
        54 => "Character Scene",
        55 => "Login Scene",
        56 => "Swamp of Quiet",
        57 => "Ice City",
        58 => "Ice City Boss",
        62 => "Santa Town",
        63 => "PK Field",
        64 => "Duel Arena",
        65..=68 => "Doppelganger",
        69..=72 => "Empire Guardian",
        73 | 77 => "New Login Scene",
        74 | 78 => "New Character Scene",
        79 => "United Marketplace",
        80..=81 => "Karutan",
        _ => MAP_NAME_UNKNOWN,
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use super::{map_name, WorldManager, WorldPlugin, WorldState};
    use mu_assets::load_terrain_world_bundle;

    fn repo_assets_root() -> camino::Utf8PathBuf {
        camino::Utf8PathBuf::from_path_buf(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../port_rust/assets")
                .canonicalize()
                .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn world_plugin_registers_manager_resource() {
        let mut app = App::new();
        app.add_plugins(WorldPlugin);

        let world = app.world().resource::<WorldManager>();
        assert_eq!(world.state(), WorldState::Inactive);
        assert!(world.bundle().is_none());
    }

    #[test]
    fn world_manager_tracks_loaded_bundle_summary() {
        let asset_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&asset_root, 1).unwrap();
        let mut world = WorldManager::new();

        world.set_bundle(bundle);

        let summary = world.summary().unwrap();
        assert_eq!(world.state(), WorldState::Ready);
        assert_eq!(summary.world, 1);
        assert_eq!(
            summary.texture_layers,
            vec![
                "grass01".to_string(),
                "ground01".to_string(),
                "rock01".to_string()
            ]
        );
        assert_eq!(summary.texture_slots, 17);
        assert_eq!(summary.camera_waypoints, 8);
        assert_eq!(summary.scene_objects, 2_870);
        assert_eq!(
            world.snapshot(),
            "state=ready|world=1|world_directory=data/world_1|map_number=1|terrain_size=256|texture_layers=[grass01,ground01,rock01]|layer_stats=layer1[min=0|max=8|mean=0.76103|unique=9];layer2[min=0|max=255|mean=171.43759|unique=8];alpha[min=0|max=255|mean=38.53535|unique=17]|texture_slots=17|scene_objects=2870|camera_waypoints=8|height_map_path=data/world_1/terrain_height.json|alpha_map_path=data/world_1/alpha_tile01.png|lightmap_path=data/world_1/terrain_light.png"
        );
    }

    #[test]
    fn map_name_matches_legacy_map_manager_labels() {
        assert_eq!(map_name(0), "Lorencia");
        assert_eq!(map_name(3), "Noria");
        assert_eq!(map_name(6), "Stadium");
        assert_eq!(map_name(9), "Devil Square");
        assert_eq!(map_name(10), "Icarus");
        assert_eq!(map_name(33), "Aida");
        assert_eq!(map_name(34), "Crywolf");
        assert_eq!(map_name(36), "Hidden Hellas");
        assert_eq!(map_name(63), "PK Field");
        assert_eq!(map_name(79), "United Marketplace");
        assert_eq!(map_name(81), "Karutan");
        assert_eq!(map_name(999), "Unknown");
    }
}
