use std::process::ExitCode;

use crate::bootstrap_runtime::BootstrapRuntime;
use crate::control_http::{self, ControlCommand, ControlHttpState};
use crate::SessionState;
use bevy::app::AppExit;
use bevy::asset::AssetPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::{
    App, Camera2d, ClearColor, Color, Commands, DefaultPlugins, IntoScheduleConfigs, MessageReader,
    MessageWriter, PluginGroup, PostUpdate, PreUpdate, Res, ResMut, Resource, Startup,
};
use bevy::window::{Window, WindowPlugin, WindowResolution};
use camino::{Utf8Path, Utf8PathBuf};
use mu_audio::AudioRuntimePlugin;
use mu_gameplay::{
    DuelPlugin, EquipmentPlugin, EventPlugin, GameShopPlugin, GensPlugin, InventoryPlugin,
    MailPlugin, MovementPlugin, MuHelperRuntimePlugin, NpcPlugin, PartyPlugin, QuestPlugin,
    TradePlugin, VaultManager, VaultPlugin, WorldEntitiesPlugin, WorldMonsterPlugin,
    WorldNpcPlugin, WorldPlugin,
};
use mu_render::{RenderAssetsPlugin, RenderEntitiesPlugin, TerrainPlugin};
use mu_ui::{CharacterCreateScreenState, UiRoute, UiShellPlugin, UiShellState};

use crate::auth_shell::AuthShellPlugin;
use crate::bootstrap_runtime::BootstrapRuntimePlugin;
use crate::chat_composer::ChatComposerPlugin;
use crate::chat_shell::ChatShellPlugin;
use crate::duel_shell::DuelShellPlugin;
use crate::events_shell::EventsShellPlugin;
use crate::friend_shell::FriendShellPlugin;
use crate::game_shop_shell::GameShopShellPlugin;
use crate::gate_shell::GateShellPlugin;
use crate::gens_shell::GensShellPlugin;
use crate::guild_shell::GuildShellPlugin;
use crate::inventory_route::InventoryRoutePlugin;
use crate::inventory_shell::InventoryShellPlugin;
use crate::mu_helper_shell::MuHelperShellPlugin;
use crate::npc_shop_shell::NpcShopShellPlugin;
use crate::party_shell::PartyShellPlugin;
use crate::quests_shell::QuestsShellPlugin;
use crate::trade_shell::TradeShellPlugin;
use crate::world_hud::WorldHudPlugin;
use crate::world_motion::WorldMotionPlugin;
use crate::world_scene::WorldScenePlugin;
use crate::Config;
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
    let config = GraphicalRuntimeConfig::from_cli(cli);
    let client_config = load_graphical_config(&config.config_path);
    let mut app = App::new();
    app.add_plugins(default_plugins(&config));
    configure_project_plugins(&mut app, config, client_config, client_runtime);
    app
}

fn configure_project_plugins(
    app: &mut App,
    config: GraphicalRuntimeConfig,
    client_config: Config,
    client_runtime: ClientRuntime,
) {
    app.insert_resource(config)
        .insert_resource(client_config)
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
            EventPlugin,
            GensPlugin,
            QuestPlugin,
            InventoryPlugin,
            EquipmentPlugin,
            VaultPlugin,
            GameShopPlugin,
            MailPlugin,
            DuelPlugin,
            TradePlugin,
            MuHelperRuntimePlugin,
        ))
        .add_plugins((
            InventoryRoutePlugin,
            WorldMotionPlugin,
            AuthShellPlugin,
            ChatComposerPlugin,
            ChatShellPlugin,
            NpcShopShellPlugin,
            QuestsShellPlugin,
            PartyShellPlugin,
        ))
        .add_plugins((
            GateShellPlugin,
            FriendShellPlugin,
            GuildShellPlugin,
            DuelShellPlugin,
            EventsShellPlugin,
            GensShellPlugin,
            GameShopShellPlugin,
            TradeShellPlugin,
            MuHelperShellPlugin,
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
                save_graphical_config_on_exit_system.after(request_app_exit_when_control_http_exit),
            ),
        )
        .add_systems(Startup, setup_boot_camera_and_login_route);
}

fn load_graphical_config(path: impl AsRef<Utf8Path>) -> Config {
    match Config::load(path) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("failed to load config: {error}");
            Config::default()
        }
    }
}

fn save_graphical_config(config: &Config, path: impl AsRef<Utf8Path>) {
    if let Err(error) = config.save(path) {
        eprintln!("failed to save config: {error}");
    }
}

fn save_graphical_config_on_exit_system(
    config: Res<Config>,
    runtime_config: Res<GraphicalRuntimeConfig>,
    mut app_exit: Option<MessageReader<AppExit>>,
) {
    let Some(mut app_exit) = app_exit.take() else {
        return;
    };

    if app_exit.read().next().is_none() {
        return;
    }

    save_graphical_config(&config, &runtime_config.config_path);
}

fn default_plugins(config: &GraphicalRuntimeConfig) -> impl PluginGroup {
    let file_path = config
        .asset_root
        .as_ref()
        .map(|path| path.to_string())
        .unwrap_or_else(|| "assets".to_string());

    DefaultPlugins
        .set(AssetPlugin {
            file_path,
            ..Default::default()
        })
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
    bootstrap: Option<ResMut<BootstrapRuntime>>,
    mut vault: ResMut<VaultManager>,
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

    let Some(mut bootstrap) = bootstrap else {
        control_http.mark_applied(snapshot.command_count);
        return;
    };

    match snapshot.last_command {
        Some(ControlCommand::CharacterCreate) => {
            bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
        }
        Some(ControlCommand::SelectCharacter) => {
            if let Some(character_name) = snapshot.selected_character_name.as_deref() {
                let _ = bootstrap.queue_character_select_request(character_name);
            }
        }
        Some(ControlCommand::CreateCharacter) => {
            if let Some(character_name) = snapshot.selected_character_name.as_deref() {
                if !bootstrap.queue_character_create_request(character_name) {
                    bootstrap.set_character_create_state(CharacterCreateScreenState::Error);
                }
            }
        }
        Some(ControlCommand::FriendAdd) => {
            if let Some(friend_name) = snapshot.friend_name.as_deref() {
                let _ = bootstrap.queue_friend_add_request(friend_name);
            }
        }
        Some(ControlCommand::FriendDelete) => {
            if let Some(friend_name) = snapshot.friend_name.as_deref() {
                let _ = bootstrap.queue_friend_delete_request(friend_name);
            }
        }
        Some(ControlCommand::GuildJoin) => {
            if let Some(guild_master_player_id) = snapshot.guild_master_player_id {
                let _ = bootstrap.queue_guild_join_request(guild_master_player_id);
            }
        }
        Some(ControlCommand::GuildRoleAssign) => {
            if let (Some(player_name), Some(role), Some(assignment_type)) = (
                snapshot.guild_player_name.as_deref(),
                snapshot.guild_role,
                snapshot.guild_assignment_type,
            ) {
                let _ =
                    bootstrap.queue_guild_role_assign_request(player_name, role, assignment_type);
            }
        }
        Some(ControlCommand::VaultDeposit) | Some(ControlCommand::VaultWithdraw) => {
            apply_vault_money_transfer_command(
                &mut bootstrap,
                &mut vault,
                snapshot.last_command,
                snapshot.vault_money_amount,
            );
        }
        _ => {}
    }

    control_http.mark_applied(snapshot.command_count);
}

fn apply_vault_money_transfer_command(
    bootstrap: &mut BootstrapRuntime,
    vault: &mut VaultManager,
    command: Option<ControlCommand>,
    amount: Option<u32>,
) {
    let Some(amount) = amount else {
        return;
    };

    match command {
        Some(ControlCommand::VaultDeposit) => {
            if vault.deposit_money(amount).is_ok() {
                let _ = bootstrap.queue_vault_money_transfer_request(0, amount);
            }
        }
        Some(ControlCommand::VaultWithdraw) => {
            if vault.withdraw_money(amount).is_ok() {
                let _ = bootstrap.queue_vault_money_transfer_request(1, amount);
            }
        }
        _ => {}
    }
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
    use crate::bootstrap_runtime::BootstrapRuntime;
    use crate::control_http::{ControlCommand, ControlHttpState, ControlSnapshot};
    use crate::{AppState, Cli, ClientRuntime, Config, SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::VaultManager;
    use mu_ui::{CharacterCreateScreenState, UiRoute, UiShellState};
    use std::sync::{Arc, Mutex};
    use tokio::sync::mpsc as tokio_mpsc;

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
            Config::default(),
            ClientRuntime::new(),
        );
        let config = app.world().resource::<GraphicalRuntimeConfig>();
        let client_config = app.world().resource::<Config>();

        assert_eq!(config.asset_root, cli.asset_root);
        assert_eq!(config.server, cli.server);
        assert_eq!(config.config_path, cli.config_path());
        assert_eq!(client_config.camera.zoom, 1735);
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
        app.insert_resource(VaultManager::new());
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
    fn control_http_snapshot_queues_character_create_submit() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));
        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.apply_command(ControlCommand::CharacterCreate);
            snapshot.selected_character_name = Some("Astra".to_string());
            snapshot.apply_command(ControlCommand::CreateCharacter);
        }

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(VaultManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);
        app.update();

        match command_receiver
            .try_recv()
            .expect("create character command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::CreateCharacter(character_name) => {
                assert_eq!(character_name, "Astra");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        let bootstrap = app.world().resource::<BootstrapRuntime>();
        assert_eq!(
            bootstrap.character_create_state(),
            CharacterCreateScreenState::Submitting
        );
    }

    #[test]
    fn control_http_snapshot_queues_friend_actions() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(VaultManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.friend_name = Some("Astra".to_string());
            snapshot.apply_command(ControlCommand::FriendAdd);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("friend add command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::FriendAdd(friend_name) => {
                assert_eq!(friend_name, "Astra");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.friend_name = Some("Astra".to_string());
            snapshot.apply_command(ControlCommand::FriendDelete);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("friend delete command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::FriendDelete(friend_name) => {
                assert_eq!(friend_name, "Astra");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn control_http_snapshot_queues_vault_money_transfers() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(VaultManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut vault = app.world_mut().resource_mut::<VaultManager>();
            vault.deposit_money(500).expect("seed vault money");
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.vault_money_amount = Some(250);
            snapshot.apply_command(ControlCommand::VaultDeposit);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("vault deposit command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::VaultMoneyTransfer {
                direction,
                amount,
            } => {
                assert_eq!(direction, 0);
                assert_eq!(amount, 250);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        assert_eq!(app.world().resource::<VaultManager>().money(), 750);
        assert_eq!(
            app.world().resource::<UiShellState>().current(),
            UiRoute::Inventory
        );
        assert_eq!(
            app.world().resource::<SessionState>().phase(),
            SessionPhase::LoggedIn
        );

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.vault_money_amount = Some(125);
            snapshot.apply_command(ControlCommand::VaultWithdraw);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("vault withdraw command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::VaultMoneyTransfer {
                direction,
                amount,
            } => {
                assert_eq!(direction, 1);
                assert_eq!(amount, 125);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        assert_eq!(app.world().resource::<VaultManager>().money(), 625);
    }

    #[test]
    fn control_http_snapshot_queues_guild_join() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(VaultManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.guild_master_player_id = Some(0x1234);
            snapshot.apply_command(ControlCommand::GuildJoin);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("guild join command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::GuildJoin(guild_master_player_id) => {
                assert_eq!(guild_master_player_id, 0x1234);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn control_http_snapshot_queues_guild_role_assign() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(VaultManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.guild_player_name = Some("Astra".to_string());
            snapshot.guild_role = Some(64);
            snapshot.guild_assignment_type = Some(2);
            snapshot.apply_command(ControlCommand::GuildRoleAssign);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("guild role-assign command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::GuildRoleAssign {
                player_name,
                role,
                assignment_type,
            } => {
                assert_eq!(player_name, "Astra");
                assert_eq!(role, 64);
                assert_eq!(assignment_type, 2);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
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
