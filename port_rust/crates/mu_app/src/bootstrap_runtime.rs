use std::net::SocketAddr;
#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, IntoScheduleConfigs, Res, ResMut, Resource, Startup, Update};
use camino::Utf8Path;
use mu_gameplay::{
    mail::{MAX_MAIL_DATE_LENGTH, MAX_MAIL_TIME_LENGTH},
    CharacterClass, MailLetterEntry, MailManager, MovementCommand, PartyMemberInfo,
    MAX_MAIL_RECIPIENT_LENGTH, MAX_MAIL_SUBJECT_LENGTH,
};
use mu_network::{Session, SessionEvent};
use mu_protocol::chat::public_chat_message;
use mu_protocol::events::gens_ranking_request;
use mu_protocol::events::{
    decode_gens_ranking_info, duel_start_request, duel_stop_request, GensRankingInfo,
};
use mu_protocol::guild::{
    guild_join_request, guild_kick_player_request, guild_list_request, guild_role_assign_request,
    remove_alliance_guild_request, request_alliance_list,
};
use mu_protocol::items::{consume_item_request, item_move_request_extended, ItemStorageKind};
use mu_protocol::login::{create_character, request_character_list, select_character};
use mu_protocol::movement::{decode_movement_update, walk_request, MovementUpdate};
use mu_protocol::social::{
    friend_add_request, friend_delete, friend_list_request, letter_list_request, party_list_request,
};
use mu_protocol::vault::{vault_move_money_request, VaultMoneyMoveDirection};
use mu_protocol::{decode_packet, PacketFrame};
use mu_ui::{
    character_select_screen, CharacterCreateScreenState, CharacterSelectCharacter,
    CharacterSelectScreenState, FriendEntry, FriendPresence, GuildMemberEntry, GuildMemberRole,
    UiRoute, UiShellState,
};
use tokio::sync::mpsc as tokio_mpsc;

use crate::{ClientRuntime, Config, GraphicalRuntimeConfig, SessionState};

const SESSION_CONNECT_TIMEOUT: Duration = Duration::from_millis(250);
const SESSION_READ_TIMEOUT: Duration = Duration::from_millis(250);
const DEFAULT_CHARACTER_CREATE_CLASS: u8 = CharacterClass::Knight as u8;
const CHARACTER_CREATE_FAILURE_MESSAGE: &str = "character creation failed";
const INVENTORY_STORAGE_KIND: ItemStorageKind = 0;
const FRIEND_LIST_ENTRY_LEN: usize = 11;
const GUILD_LIST_ENTRY_LEN: usize = 13;
const PARTY_LIST_ENTRY_LEN: usize = 24;
const PARTY_INFO_ENTRY_LEN: usize = 1;
const LETTER_ALERT_RESERVED_LEN: usize = 30 - MAX_MAIL_DATE_LENGTH - MAX_MAIL_TIME_LENGTH - 1;
const LETTER_ALERT_PAYLOAD_LEN: usize = 2
    + MAX_MAIL_RECIPIENT_LENGTH
    + MAX_MAIL_DATE_LENGTH
    + 1
    + MAX_MAIL_TIME_LENGTH
    + LETTER_ALERT_RESERVED_LEN
    + MAX_MAIL_SUBJECT_LENGTH
    + 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FriendRosterSnapshot {
    pub(crate) memo_count: u8,
    pub(crate) max_memo: u8,
    pub(crate) count: u8,
    pub(crate) friends: Vec<FriendEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GuildRosterSnapshot {
    pub(crate) result: u8,
    pub(crate) count: u8,
    pub(crate) total_score: u32,
    pub(crate) score: u8,
    pub(crate) rival_guild_name: Option<String>,
    pub(crate) members: Vec<GuildMemberEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PartyListSnapshot {
    pub(crate) count: u8,
    pub(crate) members: Vec<PartyMemberInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PartyInfoSnapshot {
    pub(crate) count: u8,
    pub(crate) step_hp_values: Vec<u8>,
}

#[derive(Debug)]
pub(crate) enum BootstrapSignal {
    Session(SessionEvent),
    ServerList,
    CharacterList,
    CharacterCreateSuccess,
    CharacterCreateFailure,
    Movement(MovementUpdate),
    FriendRoster(FriendRosterSnapshot),
    GuildRoster(GuildRosterSnapshot),
    MailLetter(MailLetterEntry),
    MailLetterDelete(u32),
    PartyList(PartyListSnapshot),
    PartyInfo(PartyInfoSnapshot),
    PartyLeave,
    GensRanking(GensRankingInfo),
    Logout(u8),
    JoinMap(u8),
    Error(String),
}

#[derive(Debug)]
pub(crate) enum BootstrapCommand {
    Walk(MovementCommand),
    Chat {
        sender: String,
        message: String,
    },
    SelectCharacter(String),
    CreateCharacter(String),
    FriendListRequest,
    LetterListRequest,
    GensRankingRequest,
    PartyListRequest,
    FriendAdd(String),
    FriendDelete(String),
    GuildListRequest,
    GuildAllianceListRequest,
    GuildJoin(u16),
    GuildRoleAssign {
        player_name: String,
        role: u8,
        assignment_type: u8,
    },
    GuildKickPlayer {
        player_name: String,
        security_code: String,
    },
    GuildBanUnion(String),
    InventoryUse {
        slot: u8,
        target: u8,
        add_points: bool,
    },
    InventoryMove {
        from_slot: u8,
        to_slot: u8,
    },
    VaultMoneyTransfer {
        direction: VaultMoneyMoveDirection,
        amount: u32,
    },
    DuelStart {
        player_id: u16,
        player_name: String,
    },
    DuelStop,
}

#[derive(Debug, Resource)]
pub struct BootstrapRuntime {
    inbox: Mutex<Receiver<BootstrapSignal>>,
    command_sender: Option<tokio_mpsc::UnboundedSender<BootstrapCommand>>,
    pending_world_map: Option<u8>,
    character_list_ready: bool,
    character_select_index: Option<usize>,
    selected_character_name: Option<String>,
    character_create_state: CharacterCreateScreenState,
    last_error: Option<String>,
    friend_roster: Option<FriendRosterSnapshot>,
    guild_roster: Option<GuildRosterSnapshot>,
    gens_ranking_snapshot: Option<GensRankingInfo>,
    #[cfg(test)]
    friend_list_request_count: AtomicUsize,
    #[cfg(test)]
    letter_list_request_count: AtomicUsize,
    #[cfg(test)]
    gens_ranking_request_count: AtomicUsize,
    #[cfg(test)]
    party_list_request_count: AtomicUsize,
    #[cfg(test)]
    guild_list_request_count: AtomicUsize,
    #[cfg(test)]
    guild_alliance_list_request_count: AtomicUsize,
    #[cfg(test)]
    test_request_queues: bool,
}

impl BootstrapRuntime {
    pub(crate) fn new(
        receiver: Receiver<BootstrapSignal>,
        command_sender: Option<tokio_mpsc::UnboundedSender<BootstrapCommand>>,
    ) -> Self {
        Self {
            inbox: Mutex::new(receiver),
            command_sender,
            pending_world_map: None,
            character_list_ready: false,
            character_select_index: None,
            selected_character_name: None,
            character_create_state: CharacterCreateScreenState::Ready,
            last_error: None,
            friend_roster: None,
            guild_roster: None,
            gens_ranking_snapshot: None,
            #[cfg(test)]
            friend_list_request_count: AtomicUsize::new(0),
            #[cfg(test)]
            letter_list_request_count: AtomicUsize::new(0),
            #[cfg(test)]
            gens_ranking_request_count: AtomicUsize::new(0),
            #[cfg(test)]
            party_list_request_count: AtomicUsize::new(0),
            #[cfg(test)]
            guild_list_request_count: AtomicUsize::new(0),
            #[cfg(test)]
            guild_alliance_list_request_count: AtomicUsize::new(0),
            #[cfg(test)]
            test_request_queues: false,
        }
    }

    pub(crate) fn idle() -> Self {
        let (_sender, receiver) = mpsc::channel();
        Self::new(receiver, None)
    }

    #[cfg(test)]
    pub(crate) fn test_stub() -> Self {
        let mut runtime = Self::idle();
        runtime.test_request_queues = true;
        runtime
    }

    pub(crate) fn with_error(message: impl Into<String>) -> Self {
        let mut runtime = Self::idle();
        runtime.last_error = Some(message.into());
        runtime
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub(crate) fn friend_roster_snapshot(&self) -> Option<FriendRosterSnapshot> {
        self.friend_roster.clone()
    }

    pub(crate) fn guild_roster_snapshot(&self) -> Option<GuildRosterSnapshot> {
        self.guild_roster.clone()
    }

    pub(crate) fn clear_social_rosters(&mut self) {
        self.friend_roster = None;
        self.guild_roster = None;
    }

    pub(crate) fn gens_ranking_snapshot(&self) -> Option<GensRankingInfo> {
        self.gens_ranking_snapshot
    }

    pub(crate) fn set_gens_ranking_snapshot(&mut self, snapshot: Option<GensRankingInfo>) {
        self.gens_ranking_snapshot = snapshot;
    }

    pub(crate) fn clear_gens_ranking_snapshot(&mut self) {
        self.gens_ranking_snapshot = None;
    }

    pub(crate) fn set_friend_roster_snapshot(&mut self, roster: Option<FriendRosterSnapshot>) {
        self.friend_roster = roster;
    }

    pub(crate) fn set_guild_roster_snapshot(&mut self, roster: Option<GuildRosterSnapshot>) {
        self.guild_roster = roster;
    }

    pub(crate) fn drain_signals(&self) -> Vec<BootstrapSignal> {
        let inbox = self.inbox.lock().expect("bootstrap inbox mutex poisoned");
        let mut signals = Vec::new();

        while let Ok(signal) = inbox.try_recv() {
            signals.push(signal);
        }

        signals
    }

    pub(crate) fn queue_movement_request(&self, movement: MovementCommand) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::Walk(movement))
            .is_ok()
    }

    pub(crate) fn queue_chat_message_request(
        &self,
        sender: impl Into<String>,
        message: impl Into<String>,
    ) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::Chat {
                sender: sender.into(),
                message: message.into(),
            })
            .is_ok()
    }

    pub(crate) fn queue_character_select_request(
        &mut self,
        character_name: impl Into<String>,
    ) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        let character_name = character_name.into();
        let sent = command_sender
            .send(BootstrapCommand::SelectCharacter(character_name.clone()))
            .is_ok();

        if sent {
            self.selected_character_name = Some(character_name);
        }

        sent
    }

    pub(crate) fn queue_character_create_request(
        &mut self,
        character_name: impl Into<String>,
    ) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        let sent = command_sender
            .send(BootstrapCommand::CreateCharacter(character_name.into()))
            .is_ok();

        if sent {
            self.character_create_state = CharacterCreateScreenState::Submitting;
        }

        sent
    }

    pub(crate) fn queue_friend_list_request(&self) -> bool {
        #[cfg(test)]
        if self.test_request_queues {
            self.friend_list_request_count
                .fetch_add(1, Ordering::SeqCst);
            return true;
        }

        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::FriendListRequest)
            .is_ok()
    }

    pub(crate) fn queue_letter_list_request(&self) -> bool {
        #[cfg(test)]
        if self.test_request_queues {
            self.letter_list_request_count
                .fetch_add(1, Ordering::SeqCst);
            return true;
        }

        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::LetterListRequest)
            .is_ok()
    }

    pub(crate) fn queue_gens_ranking_request(&self) -> bool {
        #[cfg(test)]
        if self.test_request_queues {
            self.gens_ranking_request_count
                .fetch_add(1, Ordering::SeqCst);
            return true;
        }

        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::GensRankingRequest)
            .is_ok()
    }

    pub(crate) fn queue_party_list_request(&self) -> bool {
        #[cfg(test)]
        if self.test_request_queues {
            self.party_list_request_count.fetch_add(1, Ordering::SeqCst);
            return true;
        }

        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::PartyListRequest)
            .is_ok()
    }

    pub(crate) fn queue_friend_add_request(&self, friend_name: impl Into<String>) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::FriendAdd(friend_name.into()))
            .is_ok()
    }

    pub(crate) fn queue_friend_delete_request(&self, friend_name: impl Into<String>) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::FriendDelete(friend_name.into()))
            .is_ok()
    }

    pub(crate) fn queue_guild_list_request(&self) -> bool {
        #[cfg(test)]
        if self.test_request_queues {
            self.guild_list_request_count.fetch_add(1, Ordering::SeqCst);
            return true;
        }

        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::GuildListRequest)
            .is_ok()
    }

    pub(crate) fn queue_guild_alliance_list_request(&self) -> bool {
        #[cfg(test)]
        if self.test_request_queues {
            self.guild_alliance_list_request_count
                .fetch_add(1, Ordering::SeqCst);
            return true;
        }

        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::GuildAllianceListRequest)
            .is_ok()
    }

    #[cfg(test)]
    pub(crate) fn party_list_request_count(&self) -> usize {
        self.party_list_request_count.load(Ordering::SeqCst)
    }

    pub(crate) fn queue_guild_join_request(&self, guild_master_player_id: u16) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::GuildJoin(guild_master_player_id))
            .is_ok()
    }

    pub(crate) fn queue_guild_role_assign_request(
        &self,
        player_name: impl Into<String>,
        role: u8,
        assignment_type: u8,
    ) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::GuildRoleAssign {
                player_name: player_name.into(),
                role,
                assignment_type,
            })
            .is_ok()
    }

    pub(crate) fn queue_guild_kick_player_request(
        &self,
        player_name: impl Into<String>,
        security_code: impl Into<String>,
    ) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::GuildKickPlayer {
                player_name: player_name.into(),
                security_code: security_code.into(),
            })
            .is_ok()
    }

    pub(crate) fn queue_guild_ban_union_request(&self, guild_name: impl Into<String>) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::GuildBanUnion(guild_name.into()))
            .is_ok()
    }

    pub(crate) fn queue_inventory_move_request(&self, from_slot: u8, to_slot: u8) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::InventoryMove { from_slot, to_slot })
            .is_ok()
    }

    pub(crate) fn queue_inventory_use_request(
        &self,
        slot: u8,
        target: u8,
        add_points: bool,
    ) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::InventoryUse {
                slot,
                target,
                add_points,
            })
            .is_ok()
    }

    pub(crate) fn queue_vault_money_transfer_request(
        &self,
        direction: VaultMoneyMoveDirection,
        amount: u32,
    ) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::VaultMoneyTransfer { direction, amount })
            .is_ok()
    }

    pub(crate) fn queue_duel_start_request(
        &self,
        player_id: u16,
        player_name: impl Into<String>,
    ) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::DuelStart {
                player_id,
                player_name: player_name.into(),
            })
            .is_ok()
    }

    pub(crate) fn queue_duel_stop_request(&self) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender.send(BootstrapCommand::DuelStop).is_ok()
    }

    #[cfg(test)]
    pub(crate) fn friend_list_request_count(&self) -> usize {
        self.friend_list_request_count.load(Ordering::SeqCst)
    }

    #[cfg(test)]
    pub(crate) fn letter_list_request_count(&self) -> usize {
        self.letter_list_request_count.load(Ordering::SeqCst)
    }

    #[cfg(test)]
    pub(crate) fn gens_ranking_request_count(&self) -> usize {
        self.gens_ranking_request_count.load(Ordering::SeqCst)
    }

    #[cfg(test)]
    pub(crate) fn guild_list_request_count(&self) -> usize {
        self.guild_list_request_count.load(Ordering::SeqCst)
    }

    #[cfg(test)]
    pub(crate) fn guild_alliance_list_request_count(&self) -> usize {
        self.guild_alliance_list_request_count
            .load(Ordering::SeqCst)
    }

    pub(crate) fn character_list_ready(&self) -> bool {
        self.character_list_ready
    }

    pub(crate) fn set_character_list_ready(&mut self, ready: bool) {
        self.character_list_ready = ready;
    }

    pub(crate) fn clear_character_select_selection(&mut self) {
        self.character_select_index = None;
        self.selected_character_name = None;
    }

    pub(crate) fn character_select_index(&self) -> Option<usize> {
        self.character_select_index
    }

    pub(crate) fn set_character_select_index(&mut self, index: Option<usize>) {
        self.character_select_index = index;
    }

    pub(crate) fn character_create_state(&self) -> CharacterCreateScreenState {
        self.character_create_state
    }

    pub(crate) fn set_character_create_state(&mut self, state: CharacterCreateScreenState) {
        self.character_create_state = state;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BootstrapRuntimePlugin;

impl Plugin for BootstrapRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BootstrapRuntime::idle())
            .init_resource::<SessionState>()
            .add_systems(Startup, bootstrap_startup)
            .add_systems(
                Update,
                (poll_bootstrap_signals_system, finish_world_bootstrap_system).chain(),
            );
        app.add_systems(
            Update,
            character_select_input_system.after(finish_world_bootstrap_system),
        );
    }
}

pub(crate) fn bootstrap_startup(
    mut commands: Commands,
    config: Res<GraphicalRuntimeConfig>,
    mut ui_shell: ResMut<UiShellState>,
) {
    ui_shell.set_route(UiRoute::Login);

    let Some(server) = config.server.as_ref() else {
        return;
    };

    let Ok(address) = server.parse::<SocketAddr>() else {
        commands.insert_resource(BootstrapRuntime::with_error(format!(
            "invalid server address: {server}"
        )));
        ui_shell.set_route(UiRoute::Error);
        return;
    };

    let character_list_language = character_list_language_byte(&config.config_path);
    let (sender, receiver) = mpsc::channel();
    let command_sender = spawn_bootstrap_worker(address, sender, character_list_language);
    commands.insert_resource(BootstrapRuntime::new(receiver, Some(command_sender)));
}

pub(crate) fn poll_bootstrap_signals(
    bootstrap: &mut BootstrapRuntime,
    session_state: &mut SessionState,
    ui_shell: &mut UiShellState,
    client_runtime: &mut ClientRuntime,
    mail: &mut MailManager,
) {
    let signals = bootstrap.drain_signals();

    for signal in signals {
        apply_bootstrap_signal_with_mail(
            signal,
            bootstrap,
            session_state,
            ui_shell,
            client_runtime,
            mail,
        );
    }
}

pub(crate) fn poll_bootstrap_signals_system(
    mut bootstrap: ResMut<BootstrapRuntime>,
    mut session_state: ResMut<SessionState>,
    mut ui_shell: ResMut<UiShellState>,
    mut client_runtime: ResMut<ClientRuntime>,
    mut mail: ResMut<MailManager>,
) {
    poll_bootstrap_signals(
        &mut bootstrap,
        &mut session_state,
        &mut ui_shell,
        &mut client_runtime,
        &mut mail,
    );
}

pub(crate) fn finish_world_bootstrap(
    bootstrap: &mut BootstrapRuntime,
    client_runtime: &mut ClientRuntime,
    config: &GraphicalRuntimeConfig,
    ui_shell: &mut UiShellState,
) {
    let Some(world_map) = bootstrap.pending_world_map.take() else {
        return;
    };

    let Some(asset_root) = config.asset_root.as_ref() else {
        bootstrap.last_error = Some("missing asset root".to_string());
        ui_shell.set_route(UiRoute::Error);
        return;
    };

    match client_runtime.load_world_from_assets_with_local_player_label(
        asset_root,
        u32::from(world_map),
        bootstrap.selected_character_name.as_deref(),
    ) {
        Ok(()) => {
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::World);
        }
        Err(error) => {
            bootstrap.last_error = Some(error.to_string());
            ui_shell.set_route(UiRoute::Error);
        }
    }
}

pub(crate) fn finish_world_bootstrap_system(
    mut bootstrap: ResMut<BootstrapRuntime>,
    mut client_runtime: ResMut<ClientRuntime>,
    config: Res<GraphicalRuntimeConfig>,
    mut ui_shell: ResMut<UiShellState>,
) {
    finish_world_bootstrap(&mut bootstrap, &mut client_runtime, &config, &mut ui_shell);
}

#[cfg(test)]
fn apply_bootstrap_signal(
    signal: BootstrapSignal,
    bootstrap: &mut BootstrapRuntime,
    session_state: &mut SessionState,
    ui_shell: &mut UiShellState,
    client_runtime: &mut ClientRuntime,
) {
    let mut mail = MailManager::new();
    apply_bootstrap_signal_with_mail(
        signal,
        bootstrap,
        session_state,
        ui_shell,
        client_runtime,
        &mut mail,
    );
}

fn apply_bootstrap_signal_with_mail(
    signal: BootstrapSignal,
    bootstrap: &mut BootstrapRuntime,
    session_state: &mut SessionState,
    ui_shell: &mut UiShellState,
    client_runtime: &mut ClientRuntime,
    mail: &mut MailManager,
) {
    match signal {
        BootstrapSignal::Session(event) => apply_session_event(
            event,
            bootstrap,
            session_state,
            ui_shell,
            client_runtime,
            mail,
        ),
        BootstrapSignal::ServerList => {
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::ServerSelect);
        }
        BootstrapSignal::CharacterList => {
            bootstrap.set_character_list_ready(true);
            bootstrap.clear_character_select_selection();
            bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::CharacterSelect);
        }
        BootstrapSignal::CharacterCreateSuccess => {
            bootstrap.set_character_list_ready(true);
            bootstrap.clear_character_select_selection();
            bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::CharacterSelect);
        }
        BootstrapSignal::CharacterCreateFailure => {
            bootstrap.set_character_create_state(CharacterCreateScreenState::Error);
            bootstrap.last_error = Some(CHARACTER_CREATE_FAILURE_MESSAGE.to_string());
            ui_shell.set_route(UiRoute::CharacterCreate);
        }
        BootstrapSignal::Movement(update) => apply_movement_update(update, client_runtime),
        BootstrapSignal::FriendRoster(roster) => {
            bootstrap.set_friend_roster_snapshot(Some(roster));
            bootstrap.last_error = None;
        }
        BootstrapSignal::GuildRoster(roster) => {
            bootstrap.set_guild_roster_snapshot(Some(roster));
            bootstrap.last_error = None;
        }
        BootstrapSignal::MailLetter(letter) => {
            mail.upsert_letter(letter);
            bootstrap.last_error = None;
        }
        BootstrapSignal::MailLetterDelete(letter_id) => {
            mail.remove_letter(letter_id);
            bootstrap.last_error = None;
        }
        BootstrapSignal::PartyList(roster) => {
            apply_party_list_snapshot(roster, client_runtime);
            bootstrap.last_error = None;
        }
        BootstrapSignal::PartyInfo(info) => {
            apply_party_info_snapshot(info, client_runtime);
            bootstrap.last_error = None;
        }
        BootstrapSignal::PartyLeave => {
            clear_party_state(client_runtime);
            bootstrap.last_error = None;
        }
        BootstrapSignal::GensRanking(snapshot) => {
            bootstrap.set_gens_ranking_snapshot(Some(snapshot));
            bootstrap.last_error = None;
        }
        BootstrapSignal::Logout(kind) => {
            apply_logout(kind, bootstrap, ui_shell, client_runtime, mail)
        }
        BootstrapSignal::JoinMap(map) => {
            bootstrap.pending_world_map = Some(map);
            bootstrap.set_character_list_ready(false);
            bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::Loading);
        }
        BootstrapSignal::Error(message) => {
            bootstrap.pending_world_map = None;
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = Some(message);
            bootstrap.clear_social_rosters();
            mail.reset();
            clear_party_state(client_runtime);
            bootstrap.clear_gens_ranking_snapshot();

            if bootstrap.character_create_state() == CharacterCreateScreenState::Submitting {
                bootstrap.set_character_create_state(CharacterCreateScreenState::Error);
                ui_shell.set_route(UiRoute::CharacterCreate);
            } else {
                ui_shell.set_route(UiRoute::Error);
            }
        }
    }
}

fn apply_session_event(
    event: SessionEvent,
    bootstrap: &mut BootstrapRuntime,
    session_state: &mut SessionState,
    ui_shell: &mut UiShellState,
    client_runtime: &mut ClientRuntime,
    mail: &mut MailManager,
) {
    match event {
        SessionEvent::LoginSuccess => {
            session_state.apply_event(event);
            bootstrap.pending_world_map = None;
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
            bootstrap.last_error = None;
            bootstrap.clear_social_rosters();
            bootstrap.clear_gens_ranking_snapshot();
            mail.reset();
            ui_shell.set_route(UiRoute::CharacterSelect);
        }
        SessionEvent::LoginFailure => {
            session_state.apply_event(event);
            bootstrap.pending_world_map = None;
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
            bootstrap.last_error = Some("login failed".to_string());
            bootstrap.clear_social_rosters();
            mail.reset();
            clear_party_state(client_runtime);
            bootstrap.clear_gens_ranking_snapshot();
            ui_shell.set_route(UiRoute::Login);
        }
        SessionEvent::Logout => {
            session_state.apply_event(event);
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
            bootstrap.clear_social_rosters();
            mail.reset();
            clear_party_state(client_runtime);
            bootstrap.clear_gens_ranking_snapshot();
            ui_shell.set_route(UiRoute::Login);
        }
        SessionEvent::Disconnect => {
            let pending_world_map = bootstrap.pending_world_map.is_some();
            let world_ready = client_runtime.world_ready();
            let create_pending =
                bootstrap.character_create_state() == CharacterCreateScreenState::Submitting;
            session_state.apply_event(event);
            bootstrap.clear_social_rosters();
            mail.reset();
            clear_party_state(client_runtime);
            bootstrap.clear_gens_ranking_snapshot();

            if pending_world_map {
                return;
            }

            bootstrap.pending_world_map = None;
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = Some("connection lost".to_string());

            if create_pending {
                bootstrap.set_character_create_state(CharacterCreateScreenState::Error);
                ui_shell.set_route(UiRoute::CharacterCreate);
                return;
            }

            if !world_ready {
                ui_shell.set_route(UiRoute::Error);
            }
        }
    }
}

fn apply_movement_update(update: MovementUpdate, client_runtime: &mut ClientRuntime) {
    let Some(local_key) = client_runtime.world_entities().local_player_key() else {
        return;
    };

    match update {
        MovementUpdate::Character(update) if local_key == u32::from(update.key) => {
            client_runtime.set_local_player_tile_position(update.target_x, update.target_y);
        }
        MovementUpdate::Position(update) if local_key == u32::from(update.key) => {
            client_runtime.set_local_player_tile_position(update.position_x, update.position_y);
        }
        _ => {}
    }
}

fn apply_logout(
    kind: u8,
    bootstrap: &mut BootstrapRuntime,
    ui_shell: &mut UiShellState,
    client_runtime: &mut ClientRuntime,
    mail: &mut MailManager,
) {
    bootstrap.pending_world_map = None;
    bootstrap.set_character_list_ready(false);
    bootstrap.clear_character_select_selection();
    bootstrap.set_character_create_state(CharacterCreateScreenState::Ready);
    bootstrap.last_error = None;
    bootstrap.clear_social_rosters();
    mail.reset();
    clear_party_state(client_runtime);
    bootstrap.clear_gens_ranking_snapshot();

    match kind {
        1 => {
            ui_shell.set_route(UiRoute::CharacterSelect);
        }
        2 => {
            ui_shell.set_route(UiRoute::Login);
        }
        _ => {
            bootstrap.last_error = Some(format!("unexpected logout kind: {kind}"));
            ui_shell.set_route(UiRoute::Login);
        }
    }
}

fn spawn_bootstrap_worker(
    address: SocketAddr,
    sender: Sender<BootstrapSignal>,
    character_list_language: u8,
) -> tokio_mpsc::UnboundedSender<BootstrapCommand> {
    let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();

    thread::spawn(move || {
        let runtime = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime,
            Err(error) => {
                let _ = sender.send(BootstrapSignal::Error(error.to_string()));
                return;
            }
        };

        runtime.block_on(async move {
            let mut session = match Session::connect(
                address,
                SESSION_CONNECT_TIMEOUT,
                SESSION_READ_TIMEOUT,
            )
            .await
            {
                Ok(session) => session,
                Err(error) => {
                    let _ = sender.send(BootstrapSignal::Error(error.to_string()));
                    return;
                }
            };

            loop {
                let previous_event = session.last_event();
                tokio::select! {
                    maybe_command = command_receiver.recv() => {
                        let Some(command) = maybe_command else {
                            break;
                        };

                        if let Err(error) = send_bootstrap_command(&mut session, command).await {
                            let _ = sender.send(BootstrapSignal::Error(error));
                            break;
                        }
                    }
                    packet = session.receive() => {
                        let packet = match packet {
                            Ok(Some(packet)) => packet,
                            Ok(None) => {
                                let _ = sender.send(BootstrapSignal::Session(SessionEvent::Disconnect));
                                break;
                            }
                            Err(error) => {
                                let _ = sender.send(BootstrapSignal::Error(error.to_string()));
                                break;
                            }
                        };

                        if session.last_event() != previous_event {
                            if let Some(event) = session.last_event() {
                                if !matches!(event, SessionEvent::Logout) {
                                    if matches!(event, SessionEvent::LoginSuccess) {
                                        if let Err(error) = send_character_list_request(
                                            &mut session,
                                            character_list_language,
                                        )
                                        .await
                                        {
                                            let _ = sender.send(BootstrapSignal::Error(error));
                                            break;
                                        }
                                    }

                                    let _ = sender.send(BootstrapSignal::Session(event));
                                }
                            }
                        }

                        let Ok(frame) = decode_packet(&packet) else {
                            let _ = sender.send(BootstrapSignal::Error(
                                "failed to decode packet".to_string(),
                            ));
                            break;
                        };

                        if let Some(signal) = classify_bootstrap_packet(&frame) {
                            let should_stop = matches!(signal, BootstrapSignal::Error(_));
                            let _ = sender.send(signal);

                            if should_stop {
                                break;
                            }
                        }
                    }
                }
            }
        });
    });

    command_sender
}

fn character_list_language_byte(config_path: impl AsRef<Utf8Path>) -> u8 {
    let config = Config::load(config_path).unwrap_or_default();
    legacy_language_byte(&config.locale.language)
}

fn legacy_language_byte(locale: &str) -> u8 {
    match locale.trim().to_ascii_lowercase().as_str() {
        "pt" | "por" => 1,
        "es" | "spn" => 2,
        _ => 0,
    }
}

async fn send_character_list_request(
    session: &mut Session,
    character_list_language: u8,
) -> Result<(), String> {
    let packet =
        request_character_list(character_list_language).map_err(|error| error.to_string())?;

    session
        .send(packet)
        .await
        .map_err(|error| error.to_string())
}

async fn send_character_select_request(
    session: &mut Session,
    character_name: impl AsRef<[u8]>,
) -> Result<(), String> {
    let packet = select_character(character_name).map_err(|error| error.to_string())?;

    session
        .send(packet)
        .await
        .map_err(|error| error.to_string())
}

fn classify_bootstrap_packet(frame: &PacketFrame<'_>) -> Option<BootstrapSignal> {
    if let Some(update) = decode_movement_update(frame) {
        return Some(BootstrapSignal::Movement(update));
    }

    match (frame.headcode, frame.subcode) {
        (0xC0, _) => Some(match decode_friend_roster(frame) {
            Ok(roster) => BootstrapSignal::FriendRoster(roster),
            Err(error) => BootstrapSignal::Error(error),
        }),
        (0xC6, _) => Some(match decode_letter_alert(frame) {
            Ok(letter) => BootstrapSignal::MailLetter(letter),
            Err(error) => BootstrapSignal::Error(error),
        }),
        (0xC8, _) => match decode_letter_delete_result(frame) {
            Ok(Some(letter_id)) => Some(BootstrapSignal::MailLetterDelete(letter_id)),
            Ok(None) => None,
            Err(error) => Some(BootstrapSignal::Error(error)),
        },
        (0x03, 0x42) => Some(match decode_party_list(frame) {
            Ok(roster) => BootstrapSignal::PartyList(roster),
            Err(error) => BootstrapSignal::Error(error),
        }),
        (0x03, 0x43) => Some(BootstrapSignal::PartyLeave),
        (0x03, 0x44) => Some(match decode_party_info(frame) {
            Ok(info) => BootstrapSignal::PartyInfo(info),
            Err(error) => BootstrapSignal::Error(error),
        }),
        (0xF4, 0x06) => Some(BootstrapSignal::ServerList),
        (0xF4, 0x05) => Some(BootstrapSignal::Error("server is busy".to_string())),
        (0xF3, 0x00) => Some(BootstrapSignal::CharacterList),
        (0xF3, 0x01) => match frame.payload.first().copied() {
            Some(1) => Some(BootstrapSignal::CharacterCreateSuccess),
            Some(_) => Some(BootstrapSignal::CharacterCreateFailure),
            _ => None,
        },
        (0xF3, 0x03) => frame.payload.get(2).copied().map(BootstrapSignal::JoinMap),
        (0xF1, 0x02) => frame.payload.first().copied().map(BootstrapSignal::Logout),
        (0xF8, 0x07) => Some(match decode_gens_ranking_info(frame) {
            Ok(snapshot) => BootstrapSignal::GensRanking(snapshot),
            Err(error) => BootstrapSignal::Error(error),
        }),
        (0xD1, 0x52) => Some(match decode_guild_roster(frame) {
            Ok(roster) => BootstrapSignal::GuildRoster(roster),
            Err(error) => BootstrapSignal::Error(error),
        }),
        _ => None,
    }
}

fn decode_friend_roster(frame: &PacketFrame<'_>) -> Result<FriendRosterSnapshot, String> {
    let payload = frame.payload;
    if payload.len() < 2 {
        return Err("friend list packet missing roster header".to_string());
    }

    let max_memo = payload[0];
    let count = payload[1];
    let entries = &payload[2..];
    let expected_len = usize::from(count) * FRIEND_LIST_ENTRY_LEN;
    if entries.len() < expected_len {
        return Err(format!(
            "friend list packet truncated: expected {expected_len} roster bytes, found {}",
            entries.len()
        ));
    }

    let friends = entries
        .chunks_exact(FRIEND_LIST_ENTRY_LEN)
        .take(usize::from(count))
        .map(|entry| FriendEntry {
            name: decode_legacy_name(&entry[..10]),
            server: friend_entry_server(entry[10]),
            presence: friend_presence_from_server(entry[10]),
            selected: false,
        })
        .collect();

    Ok(FriendRosterSnapshot {
        memo_count: frame.subcode,
        max_memo,
        count,
        friends,
    })
}

fn decode_guild_roster(frame: &PacketFrame<'_>) -> Result<GuildRosterSnapshot, String> {
    let payload = frame.payload;
    if payload.len() < 14 {
        return Err("guild list packet missing roster header".to_string());
    }

    let result = frame.subcode;
    let count = payload[0];
    let total_score = u32::from_le_bytes([payload[1], payload[2], payload[3], payload[4]]);
    let score = payload[5];
    let rival_guild_name = decode_legacy_name(&payload[6..14]);
    let entries = &payload[14..];
    let expected_len = usize::from(count) * GUILD_LIST_ENTRY_LEN;
    if entries.len() < expected_len {
        return Err(format!(
            "guild list packet truncated: expected {expected_len} roster bytes, found {}",
            entries.len()
        ));
    }

    let members = entries
        .chunks_exact(GUILD_LIST_ENTRY_LEN)
        .take(usize::from(count))
        .map(|entry| GuildMemberEntry {
            name: decode_legacy_name(&entry[..10]),
            number: entry[10],
            server: guild_member_server(entry[11]),
            role: guild_member_role(entry[12]),
            selected: false,
            is_self: false,
        })
        .collect();

    Ok(GuildRosterSnapshot {
        result,
        count,
        total_score,
        score,
        rival_guild_name: (!rival_guild_name.is_empty()).then_some(rival_guild_name),
        members,
    })
}

fn decode_letter_alert(frame: &PacketFrame<'_>) -> Result<MailLetterEntry, String> {
    let payload = frame.payload;
    if payload.len() < LETTER_ALERT_PAYLOAD_LEN {
        return Err(format!(
            "letter alert packet truncated: expected {LETTER_ALERT_PAYLOAD_LEN} bytes, found {}",
            payload.len()
        ));
    }

    let index = u16::from_le_bytes([payload[0], payload[1]]) as u32;
    let name_start = 2;
    let date_start = name_start + MAX_MAIL_RECIPIENT_LENGTH;
    let time_start = date_start + MAX_MAIL_DATE_LENGTH + 1;
    let subject_start = time_start + MAX_MAIL_TIME_LENGTH + LETTER_ALERT_RESERVED_LEN;
    let read_index = subject_start + MAX_MAIL_SUBJECT_LENGTH;

    Ok(MailLetterEntry {
        id: index,
        sender: decode_legacy_name(&payload[name_start..date_start]),
        subject: decode_legacy_name(&payload[subject_start..read_index]),
        date: decode_legacy_name(&payload[date_start..date_start + MAX_MAIL_DATE_LENGTH]),
        time: decode_legacy_name(&payload[time_start..time_start + MAX_MAIL_TIME_LENGTH]),
        read: payload[read_index] == 0x01,
    })
}

fn decode_letter_delete_result(frame: &PacketFrame<'_>) -> Result<Option<u32>, String> {
    let payload = frame.payload;
    if payload.len() < 3 {
        return Err("letter delete result packet missing result header".to_string());
    }

    let result = payload[0];
    let index = u16::from_le_bytes([payload[1], payload[2]]) as u32;

    Ok((result == 0x01).then_some(index))
}

fn decode_party_list(frame: &PacketFrame<'_>) -> Result<PartyListSnapshot, String> {
    let payload = frame.payload;
    if payload.len() < 2 {
        return Err("party list packet missing roster header".to_string());
    }

    let count = payload[1];
    let entries = &payload[2..];
    let expected_len = usize::from(count) * PARTY_LIST_ENTRY_LEN;
    if entries.len() < expected_len {
        return Err(format!(
            "party list packet truncated: expected {expected_len} roster bytes, found {}",
            entries.len()
        ));
    }

    let members = entries
        .chunks_exact(PARTY_LIST_ENTRY_LEN)
        .take(usize::from(count))
        .map(|entry| PartyMemberInfo {
            name: decode_legacy_name(&entry[..10]),
            number: entry[10],
            map: entry[11],
            x: entry[12],
            y: entry[13],
            curr_hp: i32::from_le_bytes([entry[16], entry[17], entry[18], entry[19]]),
            max_hp: i32::from_le_bytes([entry[20], entry[21], entry[22], entry[23]]),
            ..PartyMemberInfo::default()
        })
        .collect();

    Ok(PartyListSnapshot { count, members })
}

fn decode_party_info(frame: &PacketFrame<'_>) -> Result<PartyInfoSnapshot, String> {
    let payload = frame.payload;
    if payload.is_empty() {
        return Err("party info packet missing info header".to_string());
    }

    let count = payload[0];
    let entries = &payload[1..];
    let expected_len = usize::from(count) * PARTY_INFO_ENTRY_LEN;
    if entries.len() < expected_len {
        return Err(format!(
            "party info packet truncated: expected {expected_len} info bytes, found {}",
            entries.len()
        ));
    }

    let step_hp_values = entries
        .iter()
        .copied()
        .take(usize::from(count))
        .map(|value| value & 0x0f)
        .collect();

    Ok(PartyInfoSnapshot {
        count,
        step_hp_values,
    })
}

fn apply_party_list_snapshot(snapshot: PartyListSnapshot, client_runtime: &mut ClientRuntime) {
    let party = client_runtime.party_mut();
    party.set_party_number(usize::from(snapshot.count));

    for (index, member) in snapshot
        .members
        .into_iter()
        .enumerate()
        .take(usize::from(snapshot.count))
    {
        party.set_member(index, member);
    }
}

fn apply_party_info_snapshot(snapshot: PartyInfoSnapshot, client_runtime: &mut ClientRuntime) {
    let party = client_runtime.party_mut();
    let limit = usize::from(snapshot.count).min(party.party_number());

    for (index, step_hp) in snapshot.step_hp_values.into_iter().enumerate().take(limit) {
        party.member_mut(index).step_hp = step_hp.min(10);
    }
}

fn clear_party_state(client_runtime: &mut ClientRuntime) {
    client_runtime.party_mut().reset();
}

fn decode_legacy_name(bytes: &[u8]) -> String {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

fn friend_entry_server(server: u8) -> Option<u8> {
    match server {
        0xFC | 0xFD | 0xFE | 0xFF => None,
        value => Some(value.saturating_add(1)),
    }
}

fn friend_presence_from_server(server: u8) -> FriendPresence {
    match server {
        0xFD => FriendPresence::Busy,
        0xFC | 0xFE | 0xFF => FriendPresence::Offline,
        _ => FriendPresence::Online,
    }
}

fn guild_member_server(current_server: u8) -> Option<u8> {
    if current_server & 0x80 == 0 {
        return None;
    }

    Some(current_server & 0x7F)
}

fn guild_member_role(status: u8) -> GuildMemberRole {
    match status {
        128 => GuildMemberRole::Master,
        64 => GuildMemberRole::SubMaster,
        32 => GuildMemberRole::BattleMaster,
        _ => GuildMemberRole::Member,
    }
}

async fn send_bootstrap_command(
    session: &mut Session,
    command: BootstrapCommand,
) -> Result<(), String> {
    match command {
        BootstrapCommand::Walk(movement) => {
            let packet = walk_request(
                movement.source_x,
                movement.source_y,
                movement.step_count,
                movement.target_rotation,
                &movement.directions,
            )
            .map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::Chat { sender, message } => {
            let packet = public_chat_message(sender, message).map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::SelectCharacter(character_name) => {
            send_character_select_request(session, character_name).await
        }
        BootstrapCommand::CreateCharacter(character_name) => {
            let packet = create_character(character_name, DEFAULT_CHARACTER_CREATE_CLASS)
                .map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::FriendListRequest => {
            let packet = friend_list_request().map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::LetterListRequest => {
            let packet = letter_list_request().map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::PartyListRequest => {
            let packet = party_list_request().map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::FriendAdd(friend_name) => {
            let packet = friend_add_request(friend_name).map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::FriendDelete(friend_name) => {
            let packet = friend_delete(friend_name).map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::GensRankingRequest => {
            let packet = gens_ranking_request().map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::GuildListRequest => {
            let packet = guild_list_request().map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::GuildAllianceListRequest => {
            let packet = request_alliance_list().map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::GuildJoin(guild_master_player_id) => {
            let packet =
                guild_join_request(guild_master_player_id).map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::GuildRoleAssign {
            player_name,
            role,
            assignment_type,
        } => {
            let packet = guild_role_assign_request(role, player_name, assignment_type)
                .map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::GuildKickPlayer {
            player_name,
            security_code,
        } => {
            let packet = guild_kick_player_request(player_name, security_code)
                .map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::GuildBanUnion(guild_name) => {
            let packet =
                remove_alliance_guild_request(guild_name).map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::DuelStart {
            player_id,
            player_name,
        } => {
            let packet =
                duel_start_request(player_id, player_name).map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::DuelStop => {
            let packet = duel_stop_request().map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::InventoryUse {
            slot,
            target,
            add_points,
        } => {
            let packet = consume_item_request(slot, target, u8::from(add_points))
                .map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::InventoryMove { from_slot, to_slot } => {
            let packet = item_move_request_extended(
                INVENTORY_STORAGE_KIND,
                from_slot,
                INVENTORY_STORAGE_KIND,
                to_slot,
            )
            .map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
        BootstrapCommand::VaultMoneyTransfer { direction, amount } => {
            let packet =
                vault_move_money_request(direction, amount).map_err(|error| error.to_string())?;

            session
                .send(packet)
                .await
                .map_err(|error| error.to_string())
        }
    }
}

pub(crate) fn character_select_input_system(
    mut bootstrap: ResMut<BootstrapRuntime>,
    ui_shell: Res<UiShellState>,
    session_state: Res<SessionState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    apply_character_select_input(
        &mut bootstrap,
        ui_shell.current(),
        session_state.phase(),
        &keys,
    );
}

pub(crate) fn apply_character_select_input(
    bootstrap: &mut BootstrapRuntime,
    route: UiRoute,
    phase: mu_network::SessionPhase,
    keys: &ButtonInput<KeyCode>,
) {
    if route != UiRoute::CharacterSelect || phase != mu_network::SessionPhase::LoggedIn {
        bootstrap.clear_character_select_selection();
        return;
    }

    if !bootstrap.character_list_ready() {
        bootstrap.clear_character_select_selection();
        return;
    }

    let screen = character_select_screen(CharacterSelectScreenState::Ready);
    let characters = screen.characters;

    if characters.is_empty() {
        bootstrap.clear_character_select_selection();
        return;
    }

    let default_index = characters
        .iter()
        .position(character_is_selected)
        .unwrap_or(0);
    let current_index = bootstrap
        .character_select_index()
        .filter(|index| *index < characters.len())
        .unwrap_or(default_index);

    bootstrap.set_character_select_index(Some(current_index));

    let direction = character_select_navigation_delta(keys);
    if direction != 0 {
        let next_index = cycle_character_select_index(current_index, characters.len(), direction);
        bootstrap.set_character_select_index(Some(next_index));
    }

    if keys.just_pressed(KeyCode::Enter) {
        let selected_index = bootstrap
            .character_select_index()
            .filter(|index| *index < characters.len())
            .unwrap_or(default_index);

        if let Some(character) = characters.get(selected_index) {
            let _ = bootstrap.queue_character_select_request(character.name);
        }
    }
}

fn character_select_navigation_delta(keys: &ButtonInput<KeyCode>) -> isize {
    let previous = keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowLeft);
    let next = keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::ArrowRight);

    match (previous, next) {
        (true, false) => -1,
        (false, true) => 1,
        _ => 0,
    }
}

fn cycle_character_select_index(current: usize, len: usize, delta: isize) -> usize {
    ((current as isize + delta).rem_euclid(len as isize)) as usize
}

fn character_is_selected(character: &CharacterSelectCharacter) -> bool {
    character.selected
}

#[cfg(test)]
mod tests {
    use super::{
        apply_bootstrap_signal, apply_bootstrap_signal_with_mail, apply_character_select_input,
        classify_bootstrap_packet, finish_world_bootstrap, legacy_language_byte, BootstrapCommand,
        BootstrapRuntime, BootstrapSignal, DEFAULT_CHARACTER_CREATE_CLASS,
    };
    use crate::bootstrap_runtime::spawn_bootstrap_worker;
    use crate::{ClientRuntime, GraphicalRuntimeConfig, SessionState};
    use bevy::input::keyboard::KeyCode;
    use bevy::input::ButtonInput;
    use camino::Utf8PathBuf;
    use mu_gameplay::{MailLetterEntry, MailManager, MailMode, MovementCommand};
    use mu_network::Session;
    use mu_network::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::chat::public_chat_message;
    use mu_protocol::decode_packet;
    use mu_protocol::encode_packet;
    use mu_protocol::events::{
        duel_start_request, duel_stop_request, gens_ranking_request, GensRankingInfo,
    };
    use mu_protocol::guild::guild_join_request;
    use mu_protocol::guild::guild_kick_player_request;
    use mu_protocol::guild::guild_list_request;
    use mu_protocol::guild::guild_role_assign_request;
    use mu_protocol::guild::remove_alliance_guild_request;
    use mu_protocol::guild::request_alliance_list;
    use mu_protocol::login::{create_character, request_character_list, select_character};
    use mu_protocol::movement::{encode_move_position_update, walk_request};
    use mu_protocol::session::{
        character_creation_failed, character_creation_successful, character_list_extended,
        game_server_entered, CharacterListEntry,
    };
    use mu_protocol::social::{friend_add_request, friend_delete, friend_list_request};
    use mu_protocol::vault::vault_move_money_request;
    use mu_ui::{
        CharacterCreateScreenState, FriendPresence, GuildMemberRole, UiRoute, UiShellState,
    };
    use std::time::Duration;
    use tokio::io::AsyncReadExt;
    use tokio::net::TcpListener;
    use tokio::sync::mpsc as tokio_mpsc;

    fn world_root() -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../port_rust/assets")
                .canonicalize()
                .unwrap(),
        )
        .unwrap()
    }

    fn config(server: Option<String>) -> GraphicalRuntimeConfig {
        GraphicalRuntimeConfig {
            asset_root: Some(world_root()),
            server,
            config_path: Utf8PathBuf::from("config/client.toml"),
            editor_admin: false,
            offline_fixture: None,
            evidence_dir: None,
        }
    }

    fn join_map_packet(map: u8) -> Vec<u8> {
        encode_packet(0xC1, 0xF3, 0x03, &[0, 0, map, 0]).unwrap()
    }

    fn fixed_name<const N: usize>(value: &[u8]) -> [u8; N] {
        let mut bytes = [0u8; N];
        let len = value.len().min(N);
        bytes[..len].copy_from_slice(&value[..len]);
        bytes
    }

    fn friend_roster_packet() -> Vec<u8> {
        let mut payload = Vec::new();
        payload.push(8);
        payload.push(2);
        payload.extend_from_slice(&fixed_name::<10>(b"Astra"));
        payload.push(2);
        payload.extend_from_slice(&fixed_name::<10>(b"Blade"));
        payload.push(0xFD);

        encode_packet(0xC2, 0xC0, 0x02, &payload).unwrap()
    }

    fn mail_letter_alert_packet(
        index: u16,
        sender: &[u8],
        date: &[u8],
        time: &[u8],
        subject: &[u8],
        read: u8,
    ) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&index.to_le_bytes());
        payload.extend_from_slice(&fixed_name::<10>(sender));
        payload.extend_from_slice(&fixed_name::<10>(date));
        payload.push(0);
        payload.extend_from_slice(&fixed_name::<8>(time));
        payload.extend_from_slice(&[0; super::LETTER_ALERT_RESERVED_LEN]);
        payload.extend_from_slice(&fixed_name::<60>(subject));
        payload.push(read);

        encode_packet(0xC2, 0xC6, 0x00, &payload).unwrap()
    }

    fn mail_letter_delete_result_packet(index: u16, result: u8) -> Vec<u8> {
        let payload = [result, index as u8, (index >> 8) as u8];
        encode_packet(0xC2, 0xC8, 0x00, &payload).unwrap()
    }

    fn guild_roster_packet() -> Vec<u8> {
        let mut payload = Vec::new();
        payload.push(2);
        payload.extend_from_slice(&12_500u32.to_le_bytes());
        payload.push(7);
        payload.extend_from_slice(&fixed_name::<8>(b"Rivals"));
        payload.extend_from_slice(&fixed_name::<10>(b"Astra"));
        payload.push(11);
        payload.push(0x83);
        payload.push(128);
        payload.extend_from_slice(&fixed_name::<10>(b"Blade"));
        payload.push(22);
        payload.push(0x00);
        payload.push(64);

        encode_packet(0xC2, 0xD1, 0x52, &payload).unwrap()
    }

    fn party_member_bytes(
        name: &[u8],
        number: u8,
        map: u8,
        x: u8,
        y: u8,
        curr_hp: i32,
        max_hp: i32,
    ) -> Vec<u8> {
        let mut entry = Vec::with_capacity(24);
        entry.extend_from_slice(&fixed_name::<10>(name));
        entry.push(number);
        entry.push(map);
        entry.push(x);
        entry.push(y);
        entry.extend_from_slice(&[0, 0]);
        entry.extend_from_slice(&curr_hp.to_le_bytes());
        entry.extend_from_slice(&max_hp.to_le_bytes());
        entry
    }

    fn party_roster_packet() -> Vec<u8> {
        let mut payload = Vec::new();
        payload.push(0);
        payload.push(2);
        payload.extend_from_slice(&party_member_bytes(b"Astra", 11, 3, 12, 21, 480, 600));
        payload.extend_from_slice(&party_member_bytes(b"Blade", 22, 6, 44, 15, 220, 450));

        encode_packet(0xC1, 0x03, 0x42, &payload).unwrap()
    }

    fn party_info_packet() -> Vec<u8> {
        let payload = [2, 10, 7];

        encode_packet(0xC1, 0x03, 0x44, &payload).unwrap()
    }

    fn party_leave_packet() -> Vec<u8> {
        encode_packet(0xC1, 0x03, 0x43, &[]).unwrap()
    }

    fn gens_ranking_packet() -> Vec<u8> {
        let payload = [
            1, // Duprian
            0x09, 0x00, 0x00, 0x00, // ranking
            0x02, 0x00, 0x00, 0x00, // gens class
            0x64, 0x00, 0x00, 0x00, // contribution
            0xC8, 0x00, 0x00, 0x00, // next contribution
        ];

        encode_packet(0xC1, 0xF8, 0x07, &payload).unwrap()
    }

    async fn drive_bootstrap_until_world(
        bootstrap: &mut BootstrapRuntime,
        config: &GraphicalRuntimeConfig,
        session_state: &mut SessionState,
        ui_shell: &mut UiShellState,
        client_runtime: &mut ClientRuntime,
        character_name: Option<&str>,
    ) {
        let mut selection_requested = false;

        for _ in 0..100 {
            let signals = bootstrap.drain_signals();
            for signal in signals {
                apply_bootstrap_signal(signal, bootstrap, session_state, ui_shell, client_runtime);
            }

            if !selection_requested && ui_shell.current() == UiRoute::CharacterSelect {
                if let Some(character_name) = character_name {
                    assert!(bootstrap.queue_character_select_request(character_name));
                }
                selection_requested = true;
            }

            finish_world_bootstrap(bootstrap, client_runtime, config, ui_shell);

            if ui_shell.current() == UiRoute::World {
                break;
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn wait_for_session_disconnect(
        bootstrap: &mut BootstrapRuntime,
        session_state: &mut SessionState,
        ui_shell: &mut UiShellState,
        client_runtime: &mut ClientRuntime,
    ) {
        for _ in 0..50 {
            let signals = bootstrap.drain_signals();
            for signal in signals {
                apply_bootstrap_signal(signal, bootstrap, session_state, ui_shell, client_runtime);
            }

            if session_state.is_disconnected() {
                break;
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    #[test]
    fn legacy_language_byte_maps_expected_aliases() {
        assert_eq!(legacy_language_byte("en"), 0);
        assert_eq!(legacy_language_byte("ENG"), 0);
        assert_eq!(legacy_language_byte("pt"), 1);
        assert_eq!(legacy_language_byte("por"), 1);
        assert_eq!(legacy_language_byte("es"), 2);
        assert_eq!(legacy_language_byte("spn"), 2);
    }

    #[test]
    fn routes_progress_from_login_to_world() {
        let mut bootstrap = BootstrapRuntime::idle();
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::ServerList,
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        assert_eq!(ui_shell.current(), UiRoute::ServerSelect);

        apply_bootstrap_signal(
            BootstrapSignal::Session(mu_network::SessionEvent::LoginSuccess),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        assert_eq!(ui_shell.current(), UiRoute::CharacterSelect);
        assert!(!bootstrap.character_list_ready());

        apply_bootstrap_signal(
            BootstrapSignal::CharacterList,
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        assert_eq!(ui_shell.current(), UiRoute::CharacterSelect);
        assert!(bootstrap.character_list_ready());

        apply_bootstrap_signal(
            BootstrapSignal::JoinMap(1),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        assert_eq!(ui_shell.current(), UiRoute::Loading);

        let config = config(None);
        finish_world_bootstrap(&mut bootstrap, &mut client_runtime, &config, &mut ui_shell);

        assert_eq!(ui_shell.current(), UiRoute::World);
        assert!(client_runtime.world_ready());
        assert!(
            bootstrap.last_error().is_none(),
            "unexpected bootstrap error: {:?}",
            bootstrap.last_error()
        );
    }

    #[test]
    fn login_failure_stays_on_the_login_surface() {
        let mut bootstrap = BootstrapRuntime::idle();
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::Session(mu_network::SessionEvent::LoginFailure),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert_eq!(ui_shell.current(), UiRoute::Login);
        assert_eq!(
            session_state.phase(),
            mu_network::SessionPhase::ReadyForLogin
        );
        assert!(bootstrap.last_error().is_some());
    }

    #[test]
    fn character_select_input_waits_for_the_character_list() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let mut bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Enter);

        apply_character_select_input(
            &mut bootstrap,
            UiRoute::CharacterSelect,
            mu_network::SessionPhase::LoggedIn,
            &keys,
        );

        assert!(!bootstrap.character_list_ready());
        assert!(bootstrap.character_select_index().is_none());
        assert!(command_receiver.try_recv().is_err());
    }

    #[test]
    fn friend_and_guild_list_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_friend_list_request());
        assert!(bootstrap.queue_gens_ranking_request());
        assert!(bootstrap.queue_guild_list_request());
        assert!(bootstrap.queue_guild_alliance_list_request());

        match command_receiver
            .try_recv()
            .expect("friend list command missing")
        {
            BootstrapCommand::FriendListRequest => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        match command_receiver
            .try_recv()
            .expect("gens ranking command missing")
        {
            BootstrapCommand::GensRankingRequest => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        match command_receiver
            .try_recv()
            .expect("guild list command missing")
        {
            BootstrapCommand::GuildListRequest => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        match command_receiver
            .try_recv()
            .expect("guild alliance list command missing")
        {
            BootstrapCommand::GuildAllianceListRequest => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn vault_money_transfer_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_vault_money_transfer_request(0, 125_000));

        match command_receiver
            .try_recv()
            .expect("vault money transfer command missing")
        {
            BootstrapCommand::VaultMoneyTransfer { direction, amount } => {
                assert_eq!(direction, 0);
                assert_eq!(amount, 125_000);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn gens_ranking_requests_are_counted_in_test_mode() {
        let bootstrap = BootstrapRuntime::test_stub();

        assert_eq!(bootstrap.gens_ranking_request_count(), 0);
        assert!(bootstrap.queue_gens_ranking_request());
        assert_eq!(bootstrap.gens_ranking_request_count(), 1);
    }

    #[tokio::test]
    async fn gens_ranking_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(&mut session, BootstrapCommand::GensRankingRequest)
            .await
            .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(received, gens_ranking_request().unwrap());
    }

    #[test]
    fn guild_join_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_guild_join_request(0x1234));

        match command_receiver
            .try_recv()
            .expect("guild join command missing")
        {
            BootstrapCommand::GuildJoin(guild_master_player_id) => {
                assert_eq!(guild_master_player_id, 0x1234);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn guild_role_assign_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_guild_role_assign_request("Astra", 64, 2));

        match command_receiver
            .try_recv()
            .expect("guild role-assign command missing")
        {
            BootstrapCommand::GuildRoleAssign {
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
    fn inventory_move_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_inventory_move_request(0x12, 0x34));

        match command_receiver
            .try_recv()
            .expect("inventory move command missing")
        {
            BootstrapCommand::InventoryMove { from_slot, to_slot } => {
                assert_eq!(from_slot, 0x12);
                assert_eq!(to_slot, 0x34);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn inventory_use_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_inventory_use_request(0x12, 0x34, true));

        match command_receiver
            .try_recv()
            .expect("inventory use command missing")
        {
            BootstrapCommand::InventoryUse {
                slot,
                target,
                add_points,
            } => {
                assert_eq!(slot, 0x12);
                assert_eq!(target, 0x34);
                assert!(add_points);
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[tokio::test]
    async fn guild_alliance_list_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(&mut session, BootstrapCommand::GuildAllianceListRequest)
            .await
            .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(received, request_alliance_list().unwrap());
    }

    #[tokio::test]
    async fn inventory_move_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(
            &mut session,
            BootstrapCommand::InventoryMove {
                from_slot: 0x12,
                to_slot: 0x34,
            },
        )
        .await
        .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            received,
            super::item_move_request_extended(
                super::INVENTORY_STORAGE_KIND,
                0x12,
                super::INVENTORY_STORAGE_KIND,
                0x34,
            )
            .unwrap()
        );
    }

    #[tokio::test]
    async fn inventory_use_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(
            &mut session,
            BootstrapCommand::InventoryUse {
                slot: 0x12,
                target: 0x34,
                add_points: false,
            },
        )
        .await
        .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            received,
            super::consume_item_request(0x12, 0x34, 0).unwrap()
        );
    }

    #[tokio::test]
    async fn vault_money_transfer_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(
            &mut session,
            BootstrapCommand::VaultMoneyTransfer {
                direction: 1,
                amount: 0x0102_0304,
            },
        )
        .await
        .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(received, vault_move_money_request(1, 0x0102_0304).unwrap());
    }

    #[tokio::test]
    async fn guild_join_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(&mut session, BootstrapCommand::GuildJoin(0x1234))
            .await
            .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(received, guild_join_request(0x1234).unwrap());
    }

    #[tokio::test]
    async fn guild_role_assign_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(
            &mut session,
            BootstrapCommand::GuildRoleAssign {
                player_name: "Astra".to_string(),
                role: 64,
                assignment_type: 2,
            },
        )
        .await
        .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            received,
            guild_role_assign_request(64, b"Astra", 2).unwrap()
        );
    }

    #[tokio::test]
    async fn guild_fire_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(
            &mut session,
            BootstrapCommand::GuildKickPlayer {
                player_name: "Blade".to_string(),
                security_code: "1234".to_string(),
            },
        )
        .await
        .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            received,
            guild_kick_player_request(b"Blade", b"1234").unwrap()
        );
    }

    #[tokio::test]
    async fn guild_ban_union_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(
            &mut session,
            BootstrapCommand::GuildBanUnion("Alliance".to_string()),
        )
        .await
        .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            received,
            remove_alliance_guild_request(b"Alliance").unwrap()
        );
    }

    #[tokio::test]
    async fn duel_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(
            &mut session,
            BootstrapCommand::DuelStart {
                player_id: 0x1234,
                player_name: "Astra".to_string(),
            },
        )
        .await
        .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(received, duel_start_request(0x1234, b"Astra").unwrap());
    }

    #[tokio::test]
    async fn duel_stop_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(&mut session, BootstrapCommand::DuelStop)
            .await
            .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(received, duel_stop_request().unwrap());
    }

    #[test]
    fn gens_ranking_packets_classify_and_store_runtime_state() {
        let packet = gens_ranking_packet();
        let frame = decode_packet(&packet).expect("gens ranking frame");
        let snapshot = match classify_bootstrap_packet(&frame) {
            Some(BootstrapSignal::GensRanking(snapshot)) => snapshot,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };

        assert_eq!(
            snapshot,
            GensRankingInfo {
                influence: 1,
                ranking: 9,
                gens_class: 2,
                contribution_point: 100,
                next_contribution_point: 200,
            }
        );

        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let mut bootstrap = BootstrapRuntime::new(signal_receiver, None);
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::GensRanking(snapshot),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert_eq!(bootstrap.gens_ranking_snapshot(), Some(snapshot));
    }

    #[test]
    fn gens_ranking_snapshot_clears_on_session_reset() {
        let snapshot = GensRankingInfo {
            influence: 2,
            ranking: 4,
            gens_class: 1,
            contribution_point: 300,
            next_contribution_point: 400,
        };

        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let mut bootstrap = BootstrapRuntime::new(signal_receiver, None);
        bootstrap.set_gens_ranking_snapshot(Some(snapshot));
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::Session(mu_network::SessionEvent::LoginSuccess),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        assert!(bootstrap.gens_ranking_snapshot().is_none());

        bootstrap.set_gens_ranking_snapshot(Some(snapshot));
        apply_bootstrap_signal(
            BootstrapSignal::Session(mu_network::SessionEvent::Logout),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        assert!(bootstrap.gens_ranking_snapshot().is_none());

        bootstrap.set_gens_ranking_snapshot(Some(snapshot));
        apply_bootstrap_signal(
            BootstrapSignal::Session(mu_network::SessionEvent::Disconnect),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        assert!(bootstrap.gens_ranking_snapshot().is_none());
    }

    #[test]
    fn friend_management_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_friend_add_request("Astra"));
        assert!(bootstrap.queue_friend_delete_request("Astra"));

        match command_receiver
            .try_recv()
            .expect("friend add command missing")
        {
            BootstrapCommand::FriendAdd(friend_name) => {
                assert_eq!(friend_name, "Astra");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        match command_receiver
            .try_recv()
            .expect("friend delete command missing")
        {
            BootstrapCommand::FriendDelete(friend_name) => {
                assert_eq!(friend_name, "Astra");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn letter_list_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_letter_list_request());

        match command_receiver
            .try_recv()
            .expect("letter list command missing")
        {
            BootstrapCommand::LetterListRequest => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn letter_packets_update_and_reset_mail_state() {
        let letter_index = 0x0304;
        let letter_packet = mail_letter_alert_packet(
            letter_index,
            b"Astra",
            b"05/19/2026",
            b"10:12",
            b"Potion run",
            0x02,
        );
        let frame = decode_packet(&letter_packet).expect("letter alert frame");
        let letter = match classify_bootstrap_packet(&frame) {
            Some(BootstrapSignal::MailLetter(letter)) => letter,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };

        assert_eq!(
            letter,
            MailLetterEntry {
                id: u32::from(letter_index),
                sender: "Astra".to_string(),
                subject: "Potion run".to_string(),
                date: "05/19/2026".to_string(),
                time: "10:12".to_string(),
                read: false,
            }
        );

        let delete_packet = mail_letter_delete_result_packet(letter_index, 0x01);
        let delete_frame = decode_packet(&delete_packet).expect("letter delete frame");
        match classify_bootstrap_packet(&delete_frame) {
            Some(BootstrapSignal::MailLetterDelete(letter_id)) => {
                assert_eq!(letter_id, u32::from(letter_index));
            }
            other => panic!("unexpected bootstrap signal: {other:?}"),
        }

        let mut bootstrap = BootstrapRuntime::idle();
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();
        let mut mail = MailManager::new();

        apply_bootstrap_signal_with_mail(
            BootstrapSignal::MailLetter(letter.clone()),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            &mut mail,
        );

        assert!(mail.letters_loaded());
        assert_eq!(mail.letters().len(), 1);
        assert!(mail.new_mail_alert());

        mail.select_letter(letter.id);
        assert_eq!(mail.mode(), MailMode::Reading);

        apply_bootstrap_signal_with_mail(
            BootstrapSignal::MailLetterDelete(letter.id),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            &mut mail,
        );

        assert!(mail.letters_loaded());
        assert!(mail.letters().is_empty());
        assert_eq!(mail.selected_letter_id(), None);
        assert_eq!(mail.mode(), MailMode::Inbox);
        assert!(!mail.new_mail_alert());

        mail.upsert_letter(letter);
        mail.select_letter(u32::from(letter_index));

        apply_bootstrap_signal_with_mail(
            BootstrapSignal::Session(mu_network::SessionEvent::LoginSuccess),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            &mut mail,
        );

        assert!(mail.letters().is_empty());
        assert!(!mail.letters_loaded());
        assert_eq!(mail.selected_letter_id(), None);
        assert_eq!(mail.mode(), MailMode::Inbox);
        assert!(!mail.new_mail_alert());

        mail.upsert_letter(MailLetterEntry {
            id: 0x0102_0305,
            sender: "Selene".to_string(),
            subject: "Castle prep".to_string(),
            date: "05/18/2026".to_string(),
            time: "21:40".to_string(),
            read: true,
        });
        mail.select_letter(0x0102_0305);

        apply_bootstrap_signal_with_mail(
            BootstrapSignal::Logout(1),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            &mut mail,
        );

        assert!(mail.letters().is_empty());
        assert!(!mail.letters_loaded());
        assert_eq!(mail.selected_letter_id(), None);
        assert_eq!(mail.mode(), MailMode::Inbox);
        assert!(!mail.new_mail_alert());

        mail.upsert_letter(MailLetterEntry {
            id: 0x0102_0306,
            sender: "Marlon".to_string(),
            subject: "Guild meeting".to_string(),
            date: "05/17/2026".to_string(),
            time: "18:05".to_string(),
            read: true,
        });
        mail.select_letter(0x0102_0306);

        apply_bootstrap_signal_with_mail(
            BootstrapSignal::Session(mu_network::SessionEvent::Disconnect),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            &mut mail,
        );

        assert!(mail.letters().is_empty());
        assert!(!mail.letters_loaded());
        assert_eq!(mail.selected_letter_id(), None);
        assert_eq!(mail.mode(), MailMode::Inbox);
        assert!(!mail.new_mail_alert());
    }

    #[test]
    fn party_list_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_party_list_request());

        match command_receiver
            .try_recv()
            .expect("party list command missing")
        {
            BootstrapCommand::PartyListRequest => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn guild_fire_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_guild_kick_player_request("Blade", "1234"));

        match command_receiver
            .try_recv()
            .expect("guild fire command missing")
        {
            BootstrapCommand::GuildKickPlayer {
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
    fn guild_ban_union_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_guild_ban_union_request("Alliance"));

        match command_receiver
            .try_recv()
            .expect("guild ban-union command missing")
        {
            BootstrapCommand::GuildBanUnion(guild_name) => {
                assert_eq!(guild_name, "Alliance");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[tokio::test]
    async fn party_list_packets_send_the_expected_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer).await.unwrap();
            buffer
        });

        let mut session = Session::connect(
            address,
            super::SESSION_CONNECT_TIMEOUT,
            super::SESSION_READ_TIMEOUT,
        )
        .await
        .unwrap();

        super::send_bootstrap_command(&mut session, BootstrapCommand::PartyListRequest)
            .await
            .unwrap();

        drop(session);
        let received = tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(received, super::party_list_request().unwrap());
    }

    #[test]
    fn duel_requests_queue_commands() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_duel_start_request(0x1234, "Astra"));
        assert!(bootstrap.queue_duel_stop_request());

        match command_receiver
            .try_recv()
            .expect("duel start command missing")
        {
            BootstrapCommand::DuelStart {
                player_id,
                player_name,
            } => {
                assert_eq!(player_id, 0x1234);
                assert_eq!(player_name, "Astra");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        match command_receiver
            .try_recv()
            .expect("duel stop command missing")
        {
            BootstrapCommand::DuelStop => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn friend_roster_packets_classify_and_update_runtime_state() {
        let packet = friend_roster_packet();
        let frame = decode_packet(&packet).expect("friend roster frame");
        let roster = match classify_bootstrap_packet(&frame) {
            Some(BootstrapSignal::FriendRoster(roster)) => roster,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };

        assert_eq!(roster.memo_count, 0x02);
        assert_eq!(roster.max_memo, 8);
        assert_eq!(roster.count, 2);
        assert_eq!(roster.friends.len(), 2);
        assert_eq!(roster.friends[0].name, "Astra");
        assert_eq!(roster.friends[0].server, Some(3));
        assert_eq!(roster.friends[0].presence, FriendPresence::Online);
        assert_eq!(roster.friends[1].name, "Blade");
        assert_eq!(roster.friends[1].server, None);
        assert_eq!(roster.friends[1].presence, FriendPresence::Busy);

        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let mut bootstrap = BootstrapRuntime::new(signal_receiver, None);
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::FriendRoster(roster),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        let live_roster = bootstrap
            .friend_roster_snapshot()
            .expect("friend roster snapshot");
        assert_eq!(live_roster.friends[0].name, "Astra");
    }

    #[test]
    fn guild_roster_packets_classify_and_update_runtime_state() {
        let packet = guild_roster_packet();
        let frame = decode_packet(&packet).expect("guild roster frame");
        let roster = match classify_bootstrap_packet(&frame) {
            Some(BootstrapSignal::GuildRoster(roster)) => roster,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };

        assert_eq!(roster.result, 0x52);
        assert_eq!(roster.count, 2);
        assert_eq!(roster.total_score, 12_500);
        assert_eq!(roster.score, 7);
        assert_eq!(roster.rival_guild_name.as_deref(), Some("Rivals"));
        assert_eq!(roster.members.len(), 2);
        assert_eq!(roster.members[0].name, "Astra");
        assert_eq!(roster.members[0].number, 11);
        assert_eq!(roster.members[0].server, Some(3));
        assert_eq!(roster.members[0].role, GuildMemberRole::Master);
        assert_eq!(roster.members[1].name, "Blade");
        assert_eq!(roster.members[1].server, None);
        assert_eq!(roster.members[1].role, GuildMemberRole::SubMaster);

        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let mut bootstrap = BootstrapRuntime::new(signal_receiver, None);
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::GuildRoster(roster),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        let live_roster = bootstrap
            .guild_roster_snapshot()
            .expect("guild roster snapshot");
        assert_eq!(live_roster.members[0].name, "Astra");
    }

    #[test]
    fn party_roster_packets_classify_and_update_runtime_state() {
        let packet = party_roster_packet();
        let frame = decode_packet(&packet).expect("party roster frame");
        let roster = match classify_bootstrap_packet(&frame) {
            Some(BootstrapSignal::PartyList(roster)) => roster,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };

        assert_eq!(roster.count, 2);
        assert_eq!(roster.members.len(), 2);
        assert_eq!(roster.members[0].name, "Astra");
        assert_eq!(roster.members[0].number, 11);
        assert_eq!(roster.members[0].map, 3);
        assert_eq!(roster.members[0].x, 12);
        assert_eq!(roster.members[0].y, 21);
        assert_eq!(roster.members[0].curr_hp, 480);
        assert_eq!(roster.members[0].max_hp, 600);
        assert_eq!(roster.members[1].name, "Blade");
        assert_eq!(roster.members[1].number, 22);
        assert_eq!(roster.members[1].map, 6);
        assert_eq!(roster.members[1].x, 44);
        assert_eq!(roster.members[1].y, 15);
        assert_eq!(roster.members[1].curr_hp, 220);
        assert_eq!(roster.members[1].max_hp, 450);

        let info_packet = party_info_packet();
        let info_frame = decode_packet(&info_packet).expect("party info frame");
        let info = match classify_bootstrap_packet(&info_frame) {
            Some(BootstrapSignal::PartyInfo(info)) => info,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };

        assert_eq!(info.count, 2);
        assert_eq!(info.step_hp_values, vec![10, 7]);

        let leave_packet = party_leave_packet();
        let leave_frame = decode_packet(&leave_packet).expect("party leave frame");
        assert!(matches!(
            classify_bootstrap_packet(&leave_frame),
            Some(BootstrapSignal::PartyLeave)
        ));

        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let mut bootstrap = BootstrapRuntime::new(signal_receiver, None);
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::PartyList(roster),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        apply_bootstrap_signal(
            BootstrapSignal::PartyInfo(info),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert_eq!(client_runtime.party().party_number(), 2);
        assert_eq!(client_runtime.party().member(0).name, "Astra");
        assert_eq!(client_runtime.party().member(0).step_hp, 10);
        assert_eq!(client_runtime.party().member(1).name, "Blade");
        assert_eq!(client_runtime.party().member(1).step_hp, 7);

        apply_bootstrap_signal(
            BootstrapSignal::PartyLeave,
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert_eq!(client_runtime.party().party_number(), 0);
    }

    #[test]
    fn social_rosters_clear_on_logout_and_disconnect() {
        let friend_packet = friend_roster_packet();
        let friend_roster = match classify_bootstrap_packet(
            &decode_packet(&friend_packet).expect("friend roster frame"),
        ) {
            Some(BootstrapSignal::FriendRoster(roster)) => roster,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };
        let guild_packet = guild_roster_packet();
        let guild_roster = match classify_bootstrap_packet(
            &decode_packet(&guild_packet).expect("guild roster frame"),
        ) {
            Some(BootstrapSignal::GuildRoster(roster)) => roster,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };
        let party_packet = party_roster_packet();
        let party_roster = match classify_bootstrap_packet(
            &decode_packet(&party_packet).expect("party roster frame"),
        ) {
            Some(BootstrapSignal::PartyList(roster)) => roster,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };
        let party_info_packet = party_info_packet();
        let party_info = match classify_bootstrap_packet(
            &decode_packet(&party_info_packet).expect("party info frame"),
        ) {
            Some(BootstrapSignal::PartyInfo(info)) => info,
            other => panic!("unexpected bootstrap signal: {other:?}"),
        };

        let mut bootstrap = BootstrapRuntime::idle();
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::FriendRoster(friend_roster.clone()),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        apply_bootstrap_signal(
            BootstrapSignal::GuildRoster(guild_roster.clone()),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        apply_bootstrap_signal(
            BootstrapSignal::PartyList(party_roster.clone()),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        apply_bootstrap_signal(
            BootstrapSignal::PartyInfo(party_info.clone()),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert!(bootstrap.friend_roster_snapshot().is_some());
        assert!(bootstrap.guild_roster_snapshot().is_some());
        assert_eq!(client_runtime.party().party_number(), 2);

        apply_bootstrap_signal(
            BootstrapSignal::Session(mu_network::SessionEvent::Logout),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert!(bootstrap.friend_roster_snapshot().is_none());
        assert!(bootstrap.guild_roster_snapshot().is_none());
        assert_eq!(client_runtime.party().party_number(), 0);

        apply_bootstrap_signal(
            BootstrapSignal::FriendRoster(friend_roster),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        apply_bootstrap_signal(
            BootstrapSignal::GuildRoster(guild_roster),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        apply_bootstrap_signal(
            BootstrapSignal::PartyList(party_roster),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        apply_bootstrap_signal(
            BootstrapSignal::PartyInfo(party_info),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        apply_bootstrap_signal(
            BootstrapSignal::Session(mu_network::SessionEvent::Disconnect),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert!(bootstrap.friend_roster_snapshot().is_none());
        assert!(bootstrap.guild_roster_snapshot().is_none());
        assert_eq!(client_runtime.party().party_number(), 0);
    }

    #[test]
    fn character_create_request_marks_the_state_submitting_and_queues_the_name() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let mut bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_character_create_request("Astra"));
        assert_eq!(
            bootstrap.character_create_state(),
            CharacterCreateScreenState::Submitting
        );

        match command_receiver
            .try_recv()
            .expect("create character command missing")
        {
            BootstrapCommand::CreateCharacter(character_name) => {
                assert_eq!(character_name, "Astra");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }

    #[test]
    fn character_create_packets_classify_success_and_failure() {
        let success = character_creation_successful(b"Astra", 0, 255, 1, 32, b"preview")
            .expect("success packet");
        let success_frame = decode_packet(&success).expect("success packet frame");
        assert!(matches!(
            classify_bootstrap_packet(&success_frame),
            Some(BootstrapSignal::CharacterCreateSuccess)
        ));

        let failure = character_creation_failed().expect("failure packet");
        let failure_frame = decode_packet(&failure).expect("failure packet frame");
        assert!(matches!(
            classify_bootstrap_packet(&failure_frame),
            Some(BootstrapSignal::CharacterCreateFailure)
        ));

        let failure_two = encode_packet(0xC1, 0xF3, 0x01, &[2]).expect("failure2 packet");
        let failure_two_frame = decode_packet(&failure_two).expect("failure2 packet frame");
        assert!(matches!(
            classify_bootstrap_packet(&failure_two_frame),
            Some(BootstrapSignal::CharacterCreateFailure)
        ));
    }

    #[tokio::test]
    async fn fake_server_packets_drive_the_character_create_request_worker() {
        let server_list = mu_protocol::connect::encode_server_list_response(&[
            mu_protocol::connect::ServerEntry::new(7, 42),
        ])
        .unwrap();
        let login_success = game_server_entered(true, 7, b"1.0.0").unwrap();
        let character_list = character_list_extended(
            1,
            2,
            true,
            &[CharacterListEntry {
                slot_index: 0,
                name: b"Astra",
                level: 255,
                status: 32,
                is_item_block_active: false,
                appearance: b"appearance-data",
                guild_position: 0,
            }],
        )
        .unwrap();
        let create_character_request =
            create_character(b"Astra", DEFAULT_CHARACTER_CREATE_CLASS).unwrap();
        let create_character_success =
            character_creation_successful(b"Astra", 0, 255, 1, 32, b"preview").unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .send_packet(server_list.clone())
                    .send_packet(login_success.clone())
                    .expect_packet(request_character_list(0).unwrap())
                    .send_packet(character_list.clone())
                    .expect_packet(create_character_request.clone())
                    .send_packet(create_character_success.clone())
                    .delay(Duration::from_millis(50))
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut bootstrap = {
            let (sender, receiver) = std::sync::mpsc::channel();
            let command_sender = spawn_bootstrap_worker(server.address(), sender, 0);
            BootstrapRuntime::new(receiver, Some(command_sender))
        };
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        for _ in 0..100 {
            let signals = bootstrap.drain_signals();
            for signal in signals {
                apply_bootstrap_signal(
                    signal,
                    &mut bootstrap,
                    &mut session_state,
                    &mut ui_shell,
                    &mut client_runtime,
                );
            }

            if ui_shell.current() == UiRoute::CharacterSelect && bootstrap.character_list_ready() {
                break;
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        ui_shell.set_route(UiRoute::CharacterCreate);
        assert!(bootstrap.queue_character_create_request("Astra"));
        assert_eq!(
            bootstrap.character_create_state(),
            CharacterCreateScreenState::Submitting
        );

        for _ in 0..100 {
            let signals = bootstrap.drain_signals();
            for signal in signals {
                apply_bootstrap_signal(
                    signal,
                    &mut bootstrap,
                    &mut session_state,
                    &mut ui_shell,
                    &mut client_runtime,
                );
            }

            if ui_shell.current() == UiRoute::CharacterSelect
                && bootstrap.character_create_state() == CharacterCreateScreenState::Ready
            {
                break;
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        server.finish().await.unwrap();

        assert_eq!(ui_shell.current(), UiRoute::CharacterSelect);
        assert_eq!(
            bootstrap.character_create_state(),
            CharacterCreateScreenState::Ready
        );
        assert!(bootstrap.last_error().is_none());
    }

    #[test]
    fn character_create_signal_updates_the_visible_route_state() {
        let mut bootstrap = BootstrapRuntime::idle();
        bootstrap.set_character_create_state(CharacterCreateScreenState::Submitting);
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::CharacterCreateSuccess,
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert_eq!(ui_shell.current(), UiRoute::CharacterSelect);
        assert_eq!(
            bootstrap.character_create_state(),
            CharacterCreateScreenState::Ready
        );
        assert!(bootstrap.character_list_ready());
    }

    #[test]
    fn character_create_failure_keeps_the_create_route_in_error() {
        let mut bootstrap = BootstrapRuntime::idle();
        bootstrap.set_character_create_state(CharacterCreateScreenState::Submitting);
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::CharacterCreate);
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::CharacterCreateFailure,
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert_eq!(ui_shell.current(), UiRoute::CharacterCreate);
        assert_eq!(
            bootstrap.character_create_state(),
            CharacterCreateScreenState::Error
        );
    }

    #[test]
    fn character_create_disconnect_keeps_the_create_route_in_error() {
        let mut bootstrap = BootstrapRuntime::idle();
        bootstrap.set_character_create_state(CharacterCreateScreenState::Submitting);
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::CharacterCreate);
        let mut client_runtime = ClientRuntime::new();

        apply_bootstrap_signal(
            BootstrapSignal::Session(mu_network::SessionEvent::Disconnect),
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );

        assert_eq!(
            session_state.phase(),
            mu_network::SessionPhase::Disconnected
        );
        assert_eq!(ui_shell.current(), UiRoute::CharacterCreate);
        assert_eq!(
            bootstrap.character_create_state(),
            CharacterCreateScreenState::Error
        );
    }

    #[test]
    fn character_select_input_wraps_navigation_and_queues_the_selected_character() {
        let (_signal_sender, signal_receiver) = std::sync::mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let mut bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));
        bootstrap.set_character_list_ready(true);

        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::ArrowDown);

        apply_character_select_input(
            &mut bootstrap,
            UiRoute::CharacterSelect,
            mu_network::SessionPhase::LoggedIn,
            &keys,
        );

        assert_eq!(bootstrap.character_select_index(), Some(1));

        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Enter);

        apply_character_select_input(
            &mut bootstrap,
            UiRoute::CharacterSelect,
            mu_network::SessionPhase::LoggedIn,
            &keys,
        );

        match command_receiver
            .try_recv()
            .expect("select_character command missing")
        {
            BootstrapCommand::SelectCharacter(character_name) => {
                assert_eq!(character_name, "Selene");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        assert_eq!(bootstrap.selected_character_name.as_deref(), Some("Selene"));

        bootstrap.clear_character_select_selection();
        assert!(bootstrap.selected_character_name.is_none());
    }

    #[tokio::test]
    async fn fake_server_packets_drive_the_bootstrap_worker() {
        let server_list = mu_protocol::connect::encode_server_list_response(&[
            mu_protocol::connect::ServerEntry::new(7, 42),
        ])
        .unwrap();
        let login_success = game_server_entered(true, 7, b"1.0.0").unwrap();
        let character_list = character_list_extended(
            1,
            2,
            true,
            &[CharacterListEntry {
                slot_index: 0,
                name: b"Astra",
                level: 255,
                status: 32,
                is_item_block_active: false,
                appearance: b"appearance-data",
                guild_position: 0,
            }],
        )
        .unwrap();
        let join_map = join_map_packet(1);
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .send_packet(server_list.clone())
                    .send_packet(login_success.clone())
                    .expect_packet(request_character_list(0).unwrap())
                    .send_packet(character_list.clone())
                    .expect_packet(select_character(b"Astra").unwrap())
                    .send_packet(join_map.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut bootstrap = {
            let (sender, receiver) = std::sync::mpsc::channel();
            let command_sender = spawn_bootstrap_worker(server.address(), sender, 0);
            BootstrapRuntime::new(receiver, Some(command_sender))
        };
        let config = config(Some(server.address().to_string()));
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        drive_bootstrap_until_world(
            &mut bootstrap,
            &config,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            Some("Astra"),
        )
        .await;

        assert_eq!(client_runtime.local_player_label(), Some("Astra"));
        assert!(client_runtime
            .world_entities()
            .snapshot()
            .contains("label=Astra"));

        wait_for_session_disconnect(
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        )
        .await;

        assert_eq!(ui_shell.current(), UiRoute::World);
        assert!(client_runtime.world_ready());
        assert_eq!(
            session_state.phase(),
            mu_network::SessionPhase::Disconnected
        );
        assert!(bootstrap.last_error().is_none());

        server.finish().await.unwrap();
    }

    #[tokio::test]
    async fn fake_server_packets_drive_the_social_request_worker() {
        let friend_request = friend_list_request().unwrap();
        let guild_request = guild_list_request().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 3];

            stream.read_exact(&mut buffer).await.unwrap();
            assert_eq!(buffer.as_slice(), friend_request.as_slice());

            stream.read_exact(&mut buffer).await.unwrap();
            assert_eq!(buffer.as_slice(), guild_request.as_slice());
        });

        let bootstrap = {
            let (sender, receiver) = std::sync::mpsc::channel();
            let command_sender = spawn_bootstrap_worker(address, sender, 0);
            BootstrapRuntime::new(receiver, Some(command_sender))
        };

        assert!(bootstrap.queue_friend_list_request());
        assert!(bootstrap.queue_guild_list_request());

        server.await.unwrap();
    }

    #[tokio::test]
    async fn fake_server_packets_drive_the_friend_management_request_worker() {
        let server_list = mu_protocol::connect::encode_server_list_response(&[
            mu_protocol::connect::ServerEntry::new(7, 42),
        ])
        .unwrap();
        let login_success = game_server_entered(true, 7, b"1.0.0").unwrap();
        let character_list = character_list_extended(
            1,
            2,
            true,
            &[CharacterListEntry {
                slot_index: 0,
                name: b"Astra",
                level: 255,
                status: 32,
                is_item_block_active: false,
                appearance: b"appearance-data",
                guild_position: 0,
            }],
        )
        .unwrap();
        let join_map = join_map_packet(1);
        let friend_add_request = friend_add_request(b"Astra").unwrap();
        let friend_delete_request = friend_delete(b"Astra").unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .send_packet(server_list.clone())
                    .send_packet(login_success.clone())
                    .expect_packet(request_character_list(0).unwrap())
                    .send_packet(character_list.clone())
                    .expect_packet(select_character(b"Astra").unwrap())
                    .send_packet(join_map.clone())
                    .expect_packet(friend_add_request.clone())
                    .expect_packet(friend_delete_request.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut bootstrap = {
            let (sender, receiver) = std::sync::mpsc::channel();
            let command_sender = spawn_bootstrap_worker(server.address(), sender, 0);
            BootstrapRuntime::new(receiver, Some(command_sender))
        };
        let config = config(Some(server.address().to_string()));
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        drive_bootstrap_until_world(
            &mut bootstrap,
            &config,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            Some("Astra"),
        )
        .await;

        assert!(bootstrap.queue_friend_add_request("Astra"));
        assert!(bootstrap.queue_friend_delete_request("Astra"));

        for _ in 0..100 {
            let signals = bootstrap.drain_signals();
            for signal in signals {
                apply_bootstrap_signal(
                    signal,
                    &mut bootstrap,
                    &mut session_state,
                    &mut ui_shell,
                    &mut client_runtime,
                );
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        server.finish().await.unwrap();
    }

    #[tokio::test]
    async fn fake_server_applies_authoritative_movement_updates() {
        let server_list = mu_protocol::connect::encode_server_list_response(&[
            mu_protocol::connect::ServerEntry::new(7, 42),
        ])
        .unwrap();
        let login_success = game_server_entered(true, 7, b"1.0.0").unwrap();
        let character_list = character_list_extended(
            1,
            2,
            true,
            &[CharacterListEntry {
                slot_index: 0,
                name: b"Astra",
                level: 255,
                status: 32,
                is_item_block_active: false,
                appearance: b"appearance-data",
                guild_position: 0,
            }],
        )
        .unwrap();
        let join_map = join_map_packet(1);
        let movement_request = walk_request(0, 0, 0, 0, []).unwrap();
        let movement_commit = encode_move_position_update(0, 3, 4).unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .send_packet(server_list.clone())
                    .send_packet(login_success.clone())
                    .expect_packet(request_character_list(0).unwrap())
                    .send_packet(character_list.clone())
                    .expect_packet(select_character(b"Astra").unwrap())
                    .send_packet(join_map.clone())
                    .expect_packet(movement_request.clone())
                    .send_packet(movement_commit.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut bootstrap = {
            let (sender, receiver) = std::sync::mpsc::channel();
            let command_sender = spawn_bootstrap_worker(server.address(), sender, 0);
            BootstrapRuntime::new(receiver, Some(command_sender))
        };
        let config = config(Some(server.address().to_string()));
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();
        let expected_position = mu_gameplay::world_position_from_tile(3, 4);

        drive_bootstrap_until_world(
            &mut bootstrap,
            &config,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            Some("Astra"),
        )
        .await;

        assert_eq!(ui_shell.current(), UiRoute::World);
        assert!(bootstrap.queue_movement_request(MovementCommand::new(0, 0, 0, 0, [])));

        for _ in 0..100 {
            let signals = bootstrap.drain_signals();
            for signal in signals {
                apply_bootstrap_signal(
                    signal,
                    &mut bootstrap,
                    &mut session_state,
                    &mut ui_shell,
                    &mut client_runtime,
                );
            }

            if client_runtime
                .world_entities()
                .local_player()
                .map(|player| player.pose.position == expected_position)
                .unwrap_or(false)
            {
                break;
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        assert_eq!(
            client_runtime
                .world_entities()
                .local_player()
                .unwrap()
                .pose
                .position,
            expected_position
        );
        assert_eq!(
            client_runtime
                .render_entities()
                .catalog()
                .local_player
                .as_ref()
                .unwrap()
                .pose
                .position,
            expected_position
        );

        server.finish().await.unwrap();
    }

    #[tokio::test]
    async fn fake_server_sends_public_chat_messages() {
        let server_list = mu_protocol::connect::encode_server_list_response(&[
            mu_protocol::connect::ServerEntry::new(7, 42),
        ])
        .unwrap();
        let login_success = game_server_entered(true, 7, b"1.0.0").unwrap();
        let character_list = character_list_extended(
            1,
            2,
            true,
            &[CharacterListEntry {
                slot_index: 0,
                name: b"Astra",
                level: 255,
                status: 32,
                is_item_block_active: false,
                appearance: b"appearance-data",
                guild_position: 0,
            }],
        )
        .unwrap();
        let join_map = join_map_packet(1);
        let chat_packet = public_chat_message(b"Hero", b"hello").unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .send_packet(server_list.clone())
                    .send_packet(login_success.clone())
                    .expect_packet(request_character_list(0).unwrap())
                    .send_packet(character_list.clone())
                    .expect_packet(select_character(b"Astra").unwrap())
                    .send_packet(join_map.clone())
                    .expect_packet(chat_packet.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut bootstrap = {
            let (sender, receiver) = std::sync::mpsc::channel();
            let command_sender = spawn_bootstrap_worker(server.address(), sender, 0);
            BootstrapRuntime::new(receiver, Some(command_sender))
        };
        let config = config(Some(server.address().to_string()));
        let mut session_state = SessionState::new();
        let mut ui_shell = UiShellState::default();
        let mut client_runtime = ClientRuntime::new();

        drive_bootstrap_until_world(
            &mut bootstrap,
            &config,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
            Some("Astra"),
        )
        .await;

        assert_eq!(ui_shell.current(), UiRoute::World);
        assert!(bootstrap.queue_chat_message_request("Hero", "hello"));

        for _ in 0..100 {
            let signals = bootstrap.drain_signals();
            for signal in signals {
                apply_bootstrap_signal(
                    signal,
                    &mut bootstrap,
                    &mut session_state,
                    &mut ui_shell,
                    &mut client_runtime,
                );
            }

            if session_state.is_disconnected() {
                break;
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        server.finish().await.unwrap();
    }
}
