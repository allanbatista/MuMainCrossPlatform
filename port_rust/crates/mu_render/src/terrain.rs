use bevy::prelude::{App, Plugin, Resource};
use mu_gameplay::{TerrainWorldSummary, WorldManager};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TerrainState {
    #[default]
    Inactive,
    Ready,
}

impl TerrainState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
        }
    }
}

#[derive(Debug, Default)]
pub struct TerrainRenderer {
    summary: Option<TerrainWorldSummary>,
}

impl Resource for TerrainRenderer {}

#[derive(Debug, Default, Clone, Copy)]
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainRenderer>();
    }
}

impl TerrainRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> TerrainState {
        if self.summary.is_some() {
            TerrainState::Ready
        } else {
            TerrainState::Inactive
        }
    }

    pub fn summary(&self) -> Option<&TerrainWorldSummary> {
        self.summary.as_ref()
    }

    pub fn sync_world(&mut self, world: &WorldManager) {
        self.summary = world.summary();
    }

    pub fn clear(&mut self) {
        self.summary = None;
    }

    pub fn snapshot(&self) -> String {
        match &self.summary {
            Some(summary) => format!("state={}|{}", self.state().as_str(), summary.snapshot()),
            None => "state=inactive".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use super::{TerrainPlugin, TerrainRenderer, TerrainState};
    use mu_assets::load_terrain_world_bundle;
    use mu_gameplay::WorldManager;

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
    fn terrain_plugin_registers_renderer_resource() {
        let mut app = App::new();
        app.add_plugins(TerrainPlugin);

        let terrain = app.world().resource::<TerrainRenderer>();
        assert_eq!(terrain.state(), TerrainState::Inactive);
        assert!(terrain.summary().is_none());
    }

    #[test]
    fn terrain_renderer_syncs_world_summary() {
        let asset_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&asset_root, 1).unwrap();
        let mut world = WorldManager::new();
        world.set_bundle(bundle);

        let mut terrain = TerrainRenderer::new();
        terrain.sync_world(&world);

        assert_eq!(terrain.state(), TerrainState::Ready);
        assert_eq!(terrain.snapshot(), world.snapshot());
    }
}
