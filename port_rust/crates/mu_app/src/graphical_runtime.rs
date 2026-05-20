use std::process::ExitCode;

use bevy::log::LogPlugin;
use bevy::prelude::{
    App, Camera2d, ClearColor, Color, Commands, DefaultPlugins, PluginGroup, ResMut, Resource,
    Startup,
};
use bevy::window::{Window, WindowPlugin, WindowResolution};
use camino::Utf8PathBuf;
use mu_audio::AudioRuntimePlugin;
use mu_gameplay::{
    MovementPlugin, PartyPlugin, WorldEntitiesPlugin, WorldMonsterPlugin, WorldNpcPlugin,
    WorldPlugin,
};
use mu_render::{RenderAssetsPlugin, RenderEntitiesPlugin, TerrainPlugin};
use mu_ui::{UiRoute, UiShellPlugin, UiShellState};

use crate::bootstrap_runtime::BootstrapRuntimePlugin;
use crate::world_motion::WorldMotionPlugin;
use crate::world_scene::WorldScenePlugin;
use crate::{Cli, ClientRuntime};

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

pub fn run_graphical(cli: &Cli, client_runtime: ClientRuntime) -> ExitCode {
    let mut app = build_graphical_app(cli, client_runtime);
    app.run();
    ExitCode::SUCCESS
}

pub fn build_graphical_app(cli: &Cli, client_runtime: ClientRuntime) -> App {
    let mut app = App::new();
    app.add_plugins(default_plugins());
    configure_project_plugins(
        &mut app,
        GraphicalRuntimeConfig::from_cli(cli),
        client_runtime,
    );
    app
}

fn configure_project_plugins(
    app: &mut App,
    config: GraphicalRuntimeConfig,
    client_runtime: ClientRuntime,
) {
    app.insert_resource(config)
        .insert_resource(client_runtime)
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
            WorldMotionPlugin,
            BootstrapRuntimePlugin,
            WorldScenePlugin,
        ))
        .add_systems(Startup, setup_boot_camera_and_login_route);
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

fn setup_boot_camera_and_login_route(mut commands: Commands, mut ui_shell: ResMut<UiShellState>) {
    commands.spawn(Camera2d);
    ui_shell.set_route(UiRoute::Login);
}

#[cfg(test)]
mod tests {
    use super::{
        configure_project_plugins, setup_boot_camera_and_login_route, GraphicalRuntimeConfig,
    };
    use crate::{Cli, ClientRuntime};
    use bevy::prelude::App;
    use mu_ui::{UiRoute, UiShellState};

    fn cli() -> Cli {
        Cli {
            asset_root: Some("port_rust/assets".into()),
            server: None,
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
        configure_project_plugins(
            &mut app,
            GraphicalRuntimeConfig::from_cli(&cli),
            ClientRuntime::new(),
        );
        let config = app.world().resource::<GraphicalRuntimeConfig>();

        assert_eq!(config.asset_root, cli.asset_root);
        assert_eq!(config.server, cli.server);
        assert_eq!(config.config_path, cli.config_path());
    }

    #[test]
    fn graphical_app_starts_on_the_login_route() {
        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.add_systems(bevy::prelude::Startup, setup_boot_camera_and_login_route);
        app.update();

        let ui_shell = app.world().resource::<UiShellState>();
        assert_eq!(ui_shell.current(), UiRoute::Login);
    }
}
