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
use mu_audio::{AudioRuntime, AudioRuntimePlugin};
use mu_gameplay::presentation_for_skill_id;
use mu_gameplay::{
    DuelManager, DuelPlugin, EquipmentManager, EquipmentPlugin, EquipmentSlot, EventPlugin,
    GameShopPlugin, GensPlugin, GuildCachePlugin, InventoryPlugin, MailManager, MailPlugin,
    MovementPlugin, MuHelperRuntimePlugin, NpcPlugin, PartyPlugin, PlayerShopPlugin, QuestPlugin,
    TradePlugin, VaultManager, VaultPlugin, WorldEntitiesPlugin, WorldMonsterPlugin,
    WorldNpcPlugin, WorldPlugin,
};
use mu_render::{
    RenderAssetsPlugin, RenderEntitiesPlugin, SkillParticlePlugin, SkillParticleQueue,
    TerrainPlugin,
};
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
use crate::player_shop_shell::PlayerShopShellPlugin;
use crate::quests_shell::QuestsShellPlugin;
use crate::siege_shell::SiegeShellPlugin;
use crate::trade_shell::TradeShellPlugin;
use crate::world_hud::WorldHudPlugin;
use crate::world_motion::WorldMotionPlugin;
use crate::world_scene::WorldScenePlugin;
use crate::Config;
use crate::{AppState, Cli, ClientRuntime};
use mu_gameplay::{InventoryManager, InventorySlot};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
struct GraphicalInitialState {
    state: AppState,
}

impl GraphicalInitialState {
    fn new(state: AppState) -> Self {
        Self { state }
    }
}

pub fn run_graphical(
    cli: &Cli,
    client_runtime: ClientRuntime,
    initial_state: AppState,
) -> ExitCode {
    let control_http = match cli.control_http {
        Some(address) => match control_http::spawn(address, initial_state) {
            Ok(handle) => Some(handle),
            Err(error) => {
                eprintln!("control-http bind failed: {error}");
                return ExitCode::from(1);
            }
        },
        None => None,
    };

    let mut app = build_graphical_app(cli, client_runtime, initial_state);
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

pub fn build_graphical_app(
    cli: &Cli,
    client_runtime: ClientRuntime,
    initial_state: AppState,
) -> App {
    let config = GraphicalRuntimeConfig::from_cli(cli);
    let client_config = load_graphical_config(&config.config_path);
    let mut app = App::new();
    app.insert_resource(GraphicalInitialState::new(initial_state));
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
            SkillParticlePlugin,
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
            GuildCachePlugin,
            QuestPlugin,
        ))
        .add_plugins((
            InventoryPlugin,
            EquipmentPlugin,
            VaultPlugin,
            PlayerShopPlugin,
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
            SiegeShellPlugin,
            DuelShellPlugin,
            EventsShellPlugin,
            GensShellPlugin,
            GameShopShellPlugin,
            PlayerShopShellPlugin,
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
        .add_systems(Startup, setup_boot_camera_and_initial_route);
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

fn initial_ui_route(state: AppState) -> UiRoute {
    match state {
        AppState::AssetCheckFailed | AppState::Exit => UiRoute::Error,
        AppState::Boot | AppState::ReadyForLogin => UiRoute::Login,
    }
}

fn setup_boot_camera_and_initial_route(
    mut commands: Commands,
    initial_state: Res<GraphicalInitialState>,
    mut ui_shell: ResMut<UiShellState>,
) {
    commands.spawn(Camera2d);
    ui_shell.set_route(initial_ui_route(initial_state.state));
}

fn sync_control_http_snapshot_to_runtime(
    control_http: Option<ResMut<ControlHttpState>>,
    bootstrap: Option<ResMut<BootstrapRuntime>>,
    mut audio_runtime: Option<ResMut<AudioRuntime>>,
    mut skill_particles: Option<ResMut<SkillParticleQueue>>,
    mut mail: ResMut<MailManager>,
    mut inventory: ResMut<InventoryManager>,
    mut equipment: ResMut<EquipmentManager>,
    mut vault: ResMut<VaultManager>,
    mut duel_manager: Option<ResMut<DuelManager>>,
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

    let mut mail_letter_action = None;
    match snapshot.last_command {
        Some(ControlCommand::LetterRead) => {
            if let Some(letter_id) = snapshot.letter_id {
                if apply_letter_read_command(&mut mail, letter_id) {
                    mail_letter_action = Some(ControlCommand::LetterRead);
                }
            }
        }
        Some(ControlCommand::LetterDelete) => {
            if let Some(letter_id) = snapshot.letter_id {
                if apply_letter_delete_command(&mut mail, letter_id) {
                    mail_letter_action = Some(ControlCommand::LetterDelete);
                }
            }
        }
        _ => {}
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
        Some(ControlCommand::LetterRead) => {
            if mail_letter_action == Some(ControlCommand::LetterRead) {
                if let Some(letter_id) = snapshot.letter_id {
                    let _ = bootstrap.queue_letter_read_request(letter_id);
                }
            }
        }
        Some(ControlCommand::LetterDelete) => {
            if mail_letter_action == Some(ControlCommand::LetterDelete) {
                if let Some(letter_id) = snapshot.letter_id {
                    let _ = bootstrap.queue_letter_delete_request(letter_id);
                }
            }
        }
        Some(ControlCommand::GuildJoin) => {
            if let Some(guild_master_player_id) = snapshot.guild_master_player_id {
                let _ = bootstrap.queue_guild_join_request(guild_master_player_id);
            }
        }
        Some(ControlCommand::GuildCreate) => {
            if let (Some(guild_name), Some(guild_emblem)) = (
                snapshot.guild_create_name.as_deref(),
                snapshot.guild_create_emblem,
            ) {
                let _ = bootstrap.queue_guild_create_request(guild_name, guild_emblem);
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
        Some(ControlCommand::GuildFire) => {
            if let (Some(player_name), Some(security_code)) = (
                snapshot.guild_player_name.as_deref(),
                snapshot.guild_security_code.as_deref(),
            ) {
                let _ = bootstrap.queue_guild_kick_player_request(player_name, security_code);
            }
        }
        Some(ControlCommand::GuildBanUnion) => {
            if let Some(guild_name) = snapshot.guild_union_name.as_deref() {
                let _ = bootstrap.queue_guild_ban_union_request(guild_name);
            }
        }
        Some(ControlCommand::DuelStart) => {
            if let (Some(player_id), Some(player_name)) = (
                snapshot.duel_player_id,
                snapshot.duel_player_name.as_deref(),
            ) {
                let _ = bootstrap.queue_duel_start_request(player_id, player_name);
            }
        }
        Some(ControlCommand::DuelChannelJoin) => {
            if let Some(channel_id) = snapshot.duel_channel_id {
                if let Some(duel_manager) = duel_manager.as_deref_mut() {
                    duel_manager.remove_all_duel_watch_user();
                    duel_manager.set_current_channel(i32::from(channel_id));
                }
                let _ = bootstrap.queue_duel_channel_join_request(channel_id);
            }
        }
        Some(ControlCommand::DuelChannelQuit) => {
            if let Some(duel_manager) = duel_manager.as_deref_mut() {
                duel_manager.remove_all_duel_watch_user();
                duel_manager.set_current_channel(-1);
            }
            let _ = bootstrap.queue_duel_channel_quit_request();
        }
        Some(ControlCommand::SkillTargeted) => {
            if let (Some(skill_id), Some(target_id)) = (snapshot.skill_id, snapshot.skill_target_id)
            {
                queue_skill_targeted_feedback(
                    &*bootstrap,
                    audio_runtime.as_deref_mut(),
                    skill_particles.as_deref_mut(),
                    skill_id,
                    target_id,
                );
            }
        }
        Some(ControlCommand::DuelStop) => {
            let _ = bootstrap.queue_duel_stop_request();
        }
        Some(ControlCommand::InventoryMove) => {
            apply_inventory_move_command(
                &mut bootstrap,
                &mut inventory,
                snapshot.last_command,
                snapshot.inventory_move_from_slot,
                snapshot.inventory_move_to_slot,
            );
        }
        Some(ControlCommand::InventoryUse) => {
            apply_inventory_use_command(
                &mut bootstrap,
                &inventory,
                snapshot.last_command,
                snapshot.inventory_use_slot,
                snapshot.inventory_use_target,
                snapshot.inventory_use_add_points,
            );
        }
        Some(ControlCommand::InventoryEquip) => {
            apply_inventory_equip_command(
                &mut bootstrap,
                &mut inventory,
                &mut equipment,
                snapshot.last_command,
                snapshot.inventory_equip_slot,
            );
        }
        Some(ControlCommand::InventoryUnequip) => {
            apply_inventory_unequip_command(
                &mut bootstrap,
                &mut inventory,
                &mut equipment,
                snapshot.last_command,
                snapshot.inventory_unequip_slot,
            );
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

fn queue_skill_targeted_feedback(
    bootstrap: &BootstrapRuntime,
    audio_runtime: Option<&mut AudioRuntime>,
    skill_particles: Option<&mut SkillParticleQueue>,
    skill_id: u16,
    target_id: u16,
) {
    if !bootstrap.queue_skill_targeted_request(skill_id, target_id) {
        return;
    }

    let presentation = presentation_for_skill_id(skill_id);

    if let Some(skill_particles) = skill_particles {
        let _ = skill_particles.push_presentation(skill_id, presentation, Some(target_id));
    }

    if let Some(audio_runtime) = audio_runtime {
        let _ = audio_runtime.queue_skill_audio(skill_id, presentation, false);
    }
}

fn apply_letter_read_command(mail: &mut MailManager, letter_id: u32) -> bool {
    if !mail.letters().iter().any(|entry| entry.id == letter_id) {
        return false;
    }

    mail.mark_letter_read(letter_id);
    mail.select_letter(letter_id);
    true
}

fn apply_letter_delete_command(mail: &mut MailManager, letter_id: u32) -> bool {
    if !mail.letters().iter().any(|entry| entry.id == letter_id) {
        return false;
    }

    mail.remove_letter(letter_id);
    true
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

fn apply_inventory_move_command(
    bootstrap: &mut BootstrapRuntime,
    inventory: &mut InventoryManager,
    command: Option<ControlCommand>,
    from_slot: Option<u8>,
    to_slot: Option<u8>,
) {
    let Some(ControlCommand::InventoryMove) = command else {
        return;
    };

    let Some(from_slot) = from_slot else {
        return;
    };
    let Some(to_slot) = to_slot else {
        return;
    };

    let Some(from_inventory_slot) = InventorySlot::from_linear(usize::from(from_slot)) else {
        return;
    };
    let Some(to_inventory_slot) = InventorySlot::from_linear(usize::from(to_slot)) else {
        return;
    };

    if inventory
        .move_item(from_inventory_slot, to_inventory_slot)
        .is_ok()
    {
        let _ = bootstrap.queue_inventory_move_request(from_slot, to_slot);
    }
}

fn apply_inventory_use_command(
    bootstrap: &mut BootstrapRuntime,
    inventory: &InventoryManager,
    command: Option<ControlCommand>,
    slot: Option<u8>,
    target: Option<u8>,
    add_points: Option<bool>,
) {
    let Some(ControlCommand::InventoryUse) = command else {
        return;
    };

    let Some(slot) = slot else {
        return;
    };
    let Some(inventory_slot) = InventorySlot::from_linear(usize::from(slot)) else {
        return;
    };

    if inventory.item_at(inventory_slot).is_none() {
        return;
    }

    let target = target.unwrap_or(0);
    let add_points = add_points.unwrap_or(true);
    let _ = bootstrap.queue_inventory_use_request(slot, target, add_points);
}

fn apply_inventory_equip_command(
    bootstrap: &mut BootstrapRuntime,
    inventory: &mut InventoryManager,
    equipment: &mut EquipmentManager,
    command: Option<ControlCommand>,
    slot: Option<u8>,
) {
    let Some(ControlCommand::InventoryEquip) = command else {
        return;
    };

    let Some(slot) = slot else {
        return;
    };
    let Some(inventory_slot) = InventorySlot::from_linear(usize::from(slot)) else {
        return;
    };

    let Some(item_snapshot) = inventory.item_at(inventory_slot).cloned() else {
        return;
    };
    let Ok(equipment_slot) = equipment.resolve_slot(&item_snapshot) else {
        return;
    };

    let item = match inventory.take(inventory_slot) {
        Ok(item) => item,
        Err(_) => return,
    };

    let rollback_item = item.clone();
    if equipment
        .force_equip_without_requirements(equipment_slot, item)
        .is_err()
    {
        let _ = inventory.insert(inventory_slot, rollback_item);
        return;
    }

    let _ = bootstrap.queue_inventory_move_request(slot, equipment_slot.as_index() as u8);
}

fn apply_inventory_unequip_command(
    bootstrap: &mut BootstrapRuntime,
    inventory: &mut InventoryManager,
    equipment: &mut EquipmentManager,
    command: Option<ControlCommand>,
    slot: Option<u8>,
) {
    let Some(ControlCommand::InventoryUnequip) = command else {
        return;
    };

    let Some(slot) = slot else {
        return;
    };
    let Some(equipment_slot) = EquipmentSlot::from_index(usize::from(slot)) else {
        return;
    };

    let item = match equipment.take(equipment_slot) {
        Ok(item) => item,
        Err(_) => return,
    };

    let rollback_item = item.clone();
    let inventory_slot = match inventory.add(item) {
        Ok(slot) => slot,
        Err(_) => {
            let _ = equipment.force_equip_without_requirements(equipment_slot, rollback_item);
            return;
        }
    };

    let inventory_slot_index = inventory_slot
        .linear_index()
        .expect("inventory slot must have a linear index");
    let _ = bootstrap.queue_inventory_move_request(slot, inventory_slot_index as u8);
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
        setup_boot_camera_and_initial_route, sync_control_http_snapshot_to_runtime,
        GraphicalInitialState, GraphicalRuntimeConfig,
    };
    use crate::bootstrap_runtime::BootstrapRuntime;
    use crate::control_http::{ControlCommand, ControlHttpState, ControlSnapshot};
    use crate::{AppState, Cli, ClientRuntime, Config, SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_audio::{AudioRuntime, AudioRuntimePlugin};
    use mu_gameplay::{
        DuelManager, DuelPlugin, EquipmentManager, EquipmentSlot, InventoryManager, InventorySlot,
        Item, ItemPacketData, ItemRequirements, ItemSize, MailLetterEntry, MailManager, MailMode,
        VaultManager, AT_SKILL_TELEPORT,
    };
    use mu_render::{SkillParticlePlugin, SkillParticleQueue};
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
        app.insert_resource(GraphicalInitialState::new(AppState::Boot));
        app.add_systems(bevy::prelude::Startup, setup_boot_camera_and_initial_route);
        app.update();

        let ui_shell = app.world().resource::<UiShellState>();
        assert_eq!(ui_shell.current(), UiRoute::Login);
    }

    #[test]
    fn graphical_app_starts_on_the_error_route_for_asset_failure() {
        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.insert_resource(GraphicalInitialState::new(AppState::AssetCheckFailed));
        app.add_systems(bevy::prelude::Startup, setup_boot_camera_and_initial_route);
        app.update();

        let ui_shell = app.world().resource::<UiShellState>();
        assert_eq!(ui_shell.current(), UiRoute::Error);
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
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
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
        app.add_plugins(DuelPlugin);
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
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
        app.add_plugins(DuelPlugin);
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
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
    fn control_http_snapshot_queues_letter_read_and_marks_mail() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(DuelPlugin);
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut mail = app.world_mut().resource_mut::<MailManager>();
            mail.upsert_letter(MailLetterEntry {
                id: 0x1234,
                sender: "Astra".to_string(),
                subject: "Potion run".to_string(),
                date: "05/19/2026".to_string(),
                time: "10:12".to_string(),
                read: false,
            });
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.letter_id = Some(0x1234);
            snapshot.apply_command(ControlCommand::LetterRead);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("letter read command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::LetterRead(letter_id) => {
                assert_eq!(letter_id, 0x1234);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        let mail = app.world().resource::<MailManager>();
        assert_eq!(mail.mode(), MailMode::Reading);
        assert_eq!(mail.selected_letter_id(), Some(0x1234));
        assert!(mail.letters()[0].read);
        assert_eq!(
            app.world().resource::<UiShellState>().current(),
            UiRoute::Friend
        );
    }

    #[test]
    fn control_http_snapshot_queues_letter_delete_and_updates_mail() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(DuelPlugin);
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut mail = app.world_mut().resource_mut::<MailManager>();
            mail.upsert_letter(MailLetterEntry {
                id: 0x1235,
                sender: "Blade".to_string(),
                subject: "Castle prep".to_string(),
                date: "05/18/2026".to_string(),
                time: "21:40".to_string(),
                read: false,
            });
            mail.select_letter(0x1235);
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.letter_id = Some(0x1235);
            snapshot.apply_command(ControlCommand::LetterDelete);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("letter delete command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::LetterDelete(letter_id) => {
                assert_eq!(letter_id, 0x1235);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        let mail = app.world().resource::<MailManager>();
        assert_eq!(mail.mode(), MailMode::Inbox);
        assert_eq!(mail.selected_letter_id(), None);
        assert!(mail.letters().is_empty());
        assert!(!mail.new_mail_alert());
        assert_eq!(
            app.world().resource::<UiShellState>().current(),
            UiRoute::Friend
        );
    }

    #[test]
    fn control_http_snapshot_queues_duel_actions() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(DuelPlugin);
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.duel_player_id = Some(0x1234);
            snapshot.duel_player_name = Some("Astra".to_string());
            snapshot.apply_command(ControlCommand::DuelStart);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("duel start command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::DuelStart {
                player_id,
                player_name,
            } => {
                assert_eq!(player_id, 0x1234);
                assert_eq!(player_name, "Astra");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.duel_channel_id = Some(7);
            snapshot.apply_command(ControlCommand::DuelChannelJoin);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("duel channel join command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::DuelChannelJoin(channel_id) => {
                assert_eq!(channel_id, 7);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
        assert_eq!(app.world().resource::<DuelManager>().current_channel(), 7);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.apply_command(ControlCommand::DuelChannelQuit);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("duel channel quit command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::DuelChannelQuit => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
        assert_eq!(app.world().resource::<DuelManager>().current_channel(), -1);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.apply_command(ControlCommand::DuelStop);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("duel stop command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::DuelStop => {}
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
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
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
    fn control_http_snapshot_queues_inventory_moves() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut inventory = app.world_mut().resource_mut::<InventoryManager>();
            inventory
                .insert(
                    InventorySlot::main(0),
                    Item::new(ItemPacketData::new(1, 1), ItemSize::new(1, 1)),
                )
                .expect("seed inventory item");
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.inventory_move_from_slot = Some(0);
            snapshot.inventory_move_to_slot = Some(1);
            snapshot.apply_command(ControlCommand::InventoryMove);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("inventory move command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::InventoryMove { from_slot, to_slot } => {
                assert_eq!(from_slot, 0);
                assert_eq!(to_slot, 1);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        assert!(app
            .world()
            .resource::<InventoryManager>()
            .item_at(InventorySlot::main(1))
            .is_some());
        assert!(app
            .world()
            .resource::<InventoryManager>()
            .item_at(InventorySlot::main(0))
            .is_none());
        assert_eq!(
            app.world().resource::<UiShellState>().current(),
            UiRoute::Inventory
        );
        assert_eq!(
            app.world().resource::<SessionState>().phase(),
            SessionPhase::LoggedIn
        );
    }

    #[test]
    fn control_http_snapshot_queues_inventory_use() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut inventory = app.world_mut().resource_mut::<InventoryManager>();
            inventory
                .insert(
                    InventorySlot::main(0),
                    Item::stackable(ItemPacketData::new(1, 1), ItemSize::new(1, 1), 20, 5),
                )
                .expect("seed inventory item");
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.inventory_use_slot = Some(0);
            snapshot.inventory_use_target = Some(3);
            snapshot.inventory_use_add_points = Some(false);
            snapshot.apply_command(ControlCommand::InventoryUse);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("inventory use command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::InventoryUse {
                slot,
                target,
                add_points,
            } => {
                assert_eq!(slot, 0);
                assert_eq!(target, 3);
                assert!(!add_points);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        assert!(app
            .world()
            .resource::<InventoryManager>()
            .item_at(InventorySlot::main(0))
            .is_some());
        assert_eq!(
            app.world().resource::<UiShellState>().current(),
            UiRoute::Inventory
        );
        assert_eq!(
            app.world().resource::<SessionState>().phase(),
            SessionPhase::LoggedIn
        );
    }

    #[test]
    fn control_http_snapshot_equips_inventory_items() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut inventory = app.world_mut().resource_mut::<InventoryManager>();
            inventory
                .insert(
                    InventorySlot::main(0),
                    Item::equipment(
                        ItemPacketData::new(0, 10),
                        ItemSize::new(1, 2),
                        EquipmentSlot::WeaponRight,
                        ItemRequirements::default(),
                        false,
                    ),
                )
                .expect("seed inventory item");
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.inventory_equip_slot = Some(0);
            snapshot.apply_command(ControlCommand::InventoryEquip);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("inventory equip command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::InventoryMove { from_slot, to_slot } => {
                assert_eq!(from_slot, 0);
                assert_eq!(to_slot, 0);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        assert!(app
            .world()
            .resource::<InventoryManager>()
            .item_at(InventorySlot::main(0))
            .is_none());
        assert!(app
            .world()
            .resource::<EquipmentManager>()
            .slot(EquipmentSlot::WeaponRight)
            .is_some());
        assert_eq!(
            app.world().resource::<UiShellState>().current(),
            UiRoute::Inventory
        );
        assert_eq!(
            app.world().resource::<SessionState>().phase(),
            SessionPhase::LoggedIn
        );
    }

    #[test]
    fn control_http_snapshot_unequips_equipment_items() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut equipment = app.world_mut().resource_mut::<EquipmentManager>();
            equipment
                .force_equip_without_requirements(
                    EquipmentSlot::WeaponRight,
                    Item::equipment(
                        ItemPacketData::new(0, 11),
                        ItemSize::new(1, 2),
                        EquipmentSlot::WeaponRight,
                        ItemRequirements::default(),
                        false,
                    ),
                )
                .expect("seed equipment item");
        }

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.inventory_unequip_slot = Some(0);
            snapshot.apply_command(ControlCommand::InventoryUnequip);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("inventory unequip command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::InventoryMove { from_slot, to_slot } => {
                assert_eq!(from_slot, 0);
                assert_eq!(to_slot, 0);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        assert!(app
            .world()
            .resource::<EquipmentManager>()
            .slot(EquipmentSlot::WeaponRight)
            .is_none());
        assert!(app
            .world()
            .resource::<InventoryManager>()
            .item_at(InventorySlot::main(0))
            .is_some());
        assert_eq!(
            app.world().resource::<UiShellState>().current(),
            UiRoute::Inventory
        );
        assert_eq!(
            app.world().resource::<SessionState>().phase(),
            SessionPhase::LoggedIn
        );
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
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
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
    fn control_http_snapshot_queues_guild_create() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.guild_create_name = Some("Guild".to_string());
            snapshot.guild_create_emblem = Some([1u8; 32]);
            snapshot.apply_command(ControlCommand::GuildCreate);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("guild create command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::GuildCreate {
                guild_name,
                guild_emblem,
            } => {
                assert_eq!(guild_name, "Guild");
                assert_eq!(guild_emblem, [1u8; 32]);
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
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
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
    fn control_http_snapshot_queues_guild_fire() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.guild_player_name = Some("Blade".to_string());
            snapshot.guild_security_code = Some("1234".to_string());
            snapshot.apply_command(ControlCommand::GuildFire);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("guild fire command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::GuildKickPlayer {
                player_name,
                security_code,
            } => {
                assert_eq!(player_name, "Blade");
                assert_eq!(security_code, "1234");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn control_http_snapshot_queues_guild_ban_union() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.guild_union_name = Some("Alliance".to_string());
            snapshot.apply_command(ControlCommand::GuildBanUnion);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("guild ban-union command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::GuildBanUnion(guild_name) => {
                assert_eq!(guild_name, "Alliance");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn control_http_snapshot_queues_skill_targeted() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins(mu_ui::UiShellPlugin);
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.skill_id = Some(0x1234);
            snapshot.skill_target_id = Some(0x5678);
            snapshot.apply_command(ControlCommand::SkillTargeted);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("skill targeted command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::SkillTargeted {
                skill_id,
                target_id,
            } => {
                assert_eq!(skill_id, 0x1234);
                assert_eq!(target_id, 0x5678);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
        assert_eq!(
            app.world().resource::<UiShellState>().current(),
            UiRoute::World
        );
    }

    #[test]
    fn control_http_snapshot_queues_skill_feedback_when_skill_is_targeted() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));

        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        let mut app = App::new();
        app.add_plugins((
            mu_ui::UiShellPlugin,
            AudioRuntimePlugin,
            SkillParticlePlugin,
        ));
        app.init_resource::<SessionState>();
        app.insert_resource(InventoryManager::new());
        app.insert_resource(EquipmentManager::new());
        app.insert_resource(VaultManager::new());
        app.insert_resource(MailManager::new());
        app.insert_resource(bootstrap);
        app.insert_resource(ControlHttpState::new(snapshot.clone()));
        app.add_systems(
            bevy::prelude::PreUpdate,
            sync_control_http_snapshot_to_runtime,
        );
        drop(signal_sender);

        {
            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            snapshot.skill_id = Some(AT_SKILL_TELEPORT);
            snapshot.skill_target_id = Some(0x5678);
            snapshot.apply_command(ControlCommand::SkillTargeted);
        }

        app.update();

        match command_receiver
            .try_recv()
            .expect("skill targeted command missing")
        {
            crate::bootstrap_runtime::BootstrapCommand::SkillTargeted {
                skill_id,
                target_id,
            } => {
                assert_eq!(skill_id, AT_SKILL_TELEPORT);
                assert_eq!(target_id, 0x5678);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        let particles = app.world().resource::<SkillParticleQueue>();
        assert_eq!(particles.len(), 1);
        assert!(particles.snapshot().contains("teleport-burst"));

        let audio_runtime = app.world().resource::<AudioRuntime>();
        assert_eq!(audio_runtime.pending_audio_events(), 1);
        assert!(audio_runtime
            .diagnostics()
            .to_string()
            .contains("queued_audio_events=1"));
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
