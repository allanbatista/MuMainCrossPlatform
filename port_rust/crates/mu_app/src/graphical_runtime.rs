use std::process::ExitCode;

use crate::control_http::{self, ControlHttpState};
use crate::SessionState;
use bevy::app::AppExit;
use bevy::log::LogPlugin;
use bevy::prelude::{
    App, Camera2d, ClearColor, Color, Commands, DefaultPlugins, MessageWriter, PluginGroup,
    PostUpdate, PreUpdate, Res, ResMut, Resource, Startup,
};
use bevy::window::{Window, WindowPlugin, WindowResolution};
use camino::Utf8PathBuf;
use mu_audio::AudioRuntimePlugin;
use mu_gameplay::{
    EquipmentPlugin, InventoryPlugin, MovementPlugin, NpcPlugin, PartyPlugin, QuestPlugin,
    VaultPlugin, WorldEntitiesPlugin, WorldMonsterPlugin, WorldNpcPlugin, WorldPlugin,
};
use mu_render::{RenderAssetsPlugin, RenderEntitiesPlugin, TerrainPlugin};
use mu_ui::{UiRoute, UiShellPlugin, UiShellState};

use crate::auth_shell::AuthShellPlugin;
use crate::bootstrap_runtime::BootstrapRuntimePlugin;
use crate::inventory_route::InventoryRoutePlugin;
use crate::inventory_shell::InventoryShellPlugin;
use crate::npc_shop_shell::NpcShopShellPlugin;
use crate::party_shell::PartyShellPlugin;
use crate::quests_shell::QuestsShellPlugin;
use crate::world_hud::WorldHudPlugin;
use crate::world_motion::WorldMotionPlugin;
use crate::world_scene::WorldScenePlugin;
use crate::{AppState, Cli, ClientRuntime};

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
    let control_http = match cli.control_http {
        Some(address) => match control_http::spawn(address, AppState::ReadyForLogin) {
            Ok(handle) => Some(handle),
            Err(error) => {
                eprintln!("control-http bind failed: {error}");
                return ExitCode::from(1);
            }
        },
        None => None,
    };

    let mut app = build_graphical_app(cli, client_runtime);
    if let Some(handle) = control_http.as_ref() {
        println!("control-http listening on http://{}", handle.address());
        app.insert_resource(ControlHttpState::new(handle.shared_snapshot()));
    }

    app.run();

    if let Some(handle) = control_http {
        handle.request_shutdown();
        let _ = handle.join();
    }

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
        ))
        .add_plugins((
            WorldEntitiesPlugin,
            WorldNpcPlugin,
            WorldMonsterPlugin,
            NpcPlugin,
            QuestPlugin,
            InventoryPlugin,
            EquipmentPlugin,
            VaultPlugin,
        ))
        .add_plugins((
            InventoryRoutePlugin,
            WorldMotionPlugin,
            AuthShellPlugin,
            NpcShopShellPlugin,
            QuestsShellPlugin,
            PartyShellPlugin,
            BootstrapRuntimePlugin,
            WorldScenePlugin,
            InventoryShellPlugin,
        ))
        .add_plugins(WorldHudPlugin)
        .add_systems(PreUpdate, sync_control_http_snapshot_to_runtime)
        .add_systems(
            PostUpdate,
            (
                sync_control_http_snapshot_from_runtime,
                request_app_exit_when_control_http_exit,
            ),
        )
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

fn sync_control_http_snapshot_to_runtime(
    control_http: Option<ResMut<ControlHttpState>>,
    mut session_state: ResMut<SessionState>,
    mut ui_shell: ResMut<UiShellState>,
) {
    let Some(mut control_http) = control_http else {
        return;
    };

    let snapshot = control_http.snapshot();

    if snapshot.command_count <= control_http.last_applied_command_count() {
        return;
    }

    if ui_shell.current() != snapshot.ui_route {
        ui_shell.set_route(snapshot.ui_route);
    }

    if session_state.phase() != snapshot.session_phase {
        session_state.sync_phase(snapshot.session_phase);
    }

    control_http.mark_applied(snapshot.command_count);
}

fn sync_control_http_snapshot_from_runtime(
    control_http: Option<ResMut<ControlHttpState>>,
    session_state: Res<SessionState>,
    ui_shell: Res<UiShellState>,
) {
    let Some(control_http) = control_http else {
        return;
    };

    let snapshot = control_http.snapshot();
    if snapshot.command_count != control_http.last_applied_command_count() {
        return;
    }

    control_http.sync_from_runtime(ui_shell.current(), session_state.phase());
}

fn request_app_exit_when_control_http_exit(
    control_http: Option<Res<ControlHttpState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    let Some(control_http) = control_http else {
        return;
    };

    if control_http.snapshot().state == AppState::Exit {
        app_exit.write(AppExit::Success);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        configure_project_plugins, request_app_exit_when_control_http_exit,
        setup_boot_camera_and_login_route, sync_control_http_snapshot_to_runtime,
        GraphicalRuntimeConfig,
    };
    use crate::control_http::{ControlCommand, ControlHttpState, ControlSnapshot};
    use crate::{AppState, Cli, ClientRuntime, SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_ui::{UiRoute, UiShellState};
    use std::sync::{Arc, Mutex};

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

    #[test]
    fn control_http_snapshot_drives_the_runtime_route_and_session() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));
        snapshot
            .lock()
            .expect("control snapshot mutex poisoned")
            .apply_command(ControlCommand::LoginSuccess);

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(ControlHttpState::new(snapshot));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        app.update();

        let ui_shell = app.world().resource::<UiShellState>();
        let session_state = app.world().resource::<SessionState>();

        assert_eq!(ui_shell.current(), UiRoute::CharacterSelect);
        assert_eq!(session_state.phase(), SessionPhase::LoggedIn);
    }

    #[test]
    fn control_http_exit_requests_app_shutdown() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));
        snapshot
            .lock()
            .expect("control snapshot mutex poisoned")
            .apply_command(ControlCommand::Exit);

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(ControlHttpState::new(snapshot));
        app.add_systems(
            bevy::prelude::PostUpdate,
            request_app_exit_when_control_http_exit,
        );

        app.update();

        assert!(app.should_exit().is_some());
    }
}
