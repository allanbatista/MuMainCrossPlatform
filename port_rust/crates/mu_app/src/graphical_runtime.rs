use std::process::ExitCode;

use bevy::log::LogPlugin;
use bevy::prelude::{
    App, Camera2d, ClearColor, Color, Commands, DefaultPlugins, PluginGroup, Resource, Startup,
};
use bevy::window::{Window, WindowPlugin, WindowResolution};
use camino::Utf8PathBuf;
use mu_audio::AudioRuntimePlugin;
use mu_gameplay::{
    MovementPlugin, PartyPlugin, WorldEntitiesPlugin, WorldMonsterPlugin, WorldNpcPlugin,
    WorldPlugin,
};
use mu_render::{RenderAssetsPlugin, RenderEntitiesPlugin, TerrainPlugin};
use mu_ui::UiShellPlugin;

use crate::Cli;

const WINDOW_TITLE: &str = "MU Rust Client";
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const CLEAR_COLOR: Color = Color::srgb(0.02, 0.02, 0.025);

#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct GraphicalRuntimeConfig {
    pub asset_root: Option<Utf8PathBuf>,
    pub server: Option<String>,
    pub config_path: Utf8PathBuf,
    pub editor_admin: bool,
    pub offline_fixture: Option<String>,
    pub evidence_dir: Option<Utf8PathBuf>,
}

impl GraphicalRuntimeConfig {
    pub fn from_cli(cli: &Cli) -> Self {
        Self {
            asset_root: cli.asset_root.clone(),
            server: cli.server.clone(),
            config_path: cli.config_path(),
            editor_admin: cli.editor_admin,
            offline_fixture: cli.offline_fixture.clone(),
            evidence_dir: cli.evidence_dir.clone(),
        }
    }
}

pub fn run_graphical(cli: &Cli) -> ExitCode {
    let mut app = build_graphical_app(cli);
    app.run();
    ExitCode::SUCCESS
}

pub fn build_graphical_app(cli: &Cli) -> App {
    let mut app = App::new();
    app.add_plugins(default_plugins());
    configure_project_plugins(&mut app, GraphicalRuntimeConfig::from_cli(cli));
    app
}

fn configure_project_plugins(app: &mut App, config: GraphicalRuntimeConfig) {
    app.insert_resource(config)
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins((
            RenderAssetsPlugin,
            TerrainPlugin,
            RenderEntitiesPlugin,
            AudioRuntimePlugin,
            UiShellPlugin,
            WorldPlugin,
            MovementPlugin,
            PartyPlugin,
            WorldEntitiesPlugin,
            WorldNpcPlugin,
            WorldMonsterPlugin,
        ))
        .add_systems(Startup, setup_boot_camera);
}

fn default_plugins() -> impl PluginGroup {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        })
        .disable::<LogPlugin>()
}

fn setup_boot_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::{configure_project_plugins, GraphicalRuntimeConfig};
    use crate::Cli;
    use bevy::prelude::App;

    fn cli() -> Cli {
        Cli {
            asset_root: Some("port_rust/assets".into()),
            server: Some("127.0.0.1:44405".to_string()),
            config: None,
            editor_admin: false,
            offline_fixture: None,
            headless: false,
            control_http: None,
            evidence_dir: None,
        }
    }

    #[test]
    fn graphical_app_registers_runtime_config() {
        let cli = cli();
        let mut app = App::new();
        configure_project_plugins(&mut app, GraphicalRuntimeConfig::from_cli(&cli));
        let config = app.world().resource::<GraphicalRuntimeConfig>();

        assert_eq!(config.asset_root, cli.asset_root);
        assert_eq!(config.server, cli.server);
        assert_eq!(config.config_path, cli.config_path());
    }
}
