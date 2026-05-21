use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::{AppState, SessionPhase};
use bevy::prelude::Resource;
use mu_gameplay::MAX_GUILD_NAME_LENGTH;
use mu_ui::{
    FriendScreenState, GuildScreenState, SiegeScreenState, UiRoute,
    CHARACTER_CREATE_NAME_MIN_LENGTH,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlCommand {
    Boot,
    AssetCheckFailed,
    ReadyForLogin,
    ServerSelect,
    Options,
    CharacterSelect,
    CharacterCreate,
    CreateCharacter,
    Loading,
    World,
    Chat,
    Npc,
    Shop,
    GameShop,
    Trade,
    Marketplace,
    Party,
    Gate,
    Siege,
    SiegeInactive,
    SiegeSoldier,
    SiegeCommander,
    Events,
    Gens,
    Friend,
    Guild,
    FriendAdd,
    FriendDelete,
    GuildJoin,
    GuildCreate,
    FriendRoster,
    FriendInbox,
    FriendCompose,
    FriendChatRooms,
    GuildSummary,
    GuildMembers,
    GuildUnion,
    GuildNoGuild,
    GuildError,
    GuildRoleAssign,
    GuildFire,
    GuildBanUnion,
    InventoryUse,
    InventoryEquip,
    InventoryUnequip,
    InventoryMove,
    VaultDeposit,
    VaultWithdraw,
    Duel,
    DuelStart,
    DuelStop,
    DuelChannelJoin,
    DuelChannelQuit,
    SkillTargeted,
    Quests,
    MuHelper,
    LoginSuccess,
    LoginFailure,
    SelectCharacter,
    LogoutLogin,
    LogoutCharacter,
    Disconnect,
    Exit,
    Ping,
}

impl ControlCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Boot => "boot",
            Self::AssetCheckFailed => "asset-check-failed",
            Self::ReadyForLogin => "ready-for-login",
            Self::ServerSelect => "server-select",
            Self::Options => "options",
            Self::CharacterSelect => "character-select",
            Self::CharacterCreate => "character-create",
            Self::CreateCharacter => "create-character",
            Self::Loading => "loading",
            Self::World => "world",
            Self::Chat => "chat",
            Self::Npc => "npc",
            Self::Shop => "shop",
            Self::GameShop => "game-shop",
            Self::Trade => "trade",
            Self::Marketplace => "marketplace",
            Self::Party => "party",
            Self::Gate => "gate",
            Self::Siege => "siege",
            Self::SiegeInactive => "siege-inactive",
            Self::SiegeSoldier => "siege-soldier",
            Self::SiegeCommander => "siege-commander",
            Self::Events => "events",
            Self::Gens => "gens",
            Self::Friend => "friend",
            Self::Guild => "guild",
            Self::FriendAdd => "friend-add",
            Self::FriendDelete => "friend-delete",
            Self::GuildJoin => "guild-join",
            Self::GuildCreate => "guild-create",
            Self::FriendRoster => "friend-roster",
            Self::FriendInbox => "friend-inbox",
            Self::FriendCompose => "friend-compose",
            Self::FriendChatRooms => "friend-chat-rooms",
            Self::GuildSummary => "guild-summary",
            Self::GuildMembers => "guild-members",
            Self::GuildUnion => "guild-union",
            Self::GuildNoGuild => "guild-no-guild",
            Self::GuildError => "guild-error",
            Self::GuildRoleAssign => "guild-role-assign",
            Self::GuildFire => "guild-fire",
            Self::GuildBanUnion => "guild-ban-union",
            Self::InventoryUse => "inventory-use",
            Self::InventoryEquip => "inventory-equip",
            Self::InventoryUnequip => "inventory-unequip",
            Self::InventoryMove => "inventory-move",
            Self::VaultDeposit => "vault-deposit",
            Self::VaultWithdraw => "vault-withdraw",
            Self::Duel => "duel",
            Self::DuelStart => "duel-start",
            Self::DuelStop => "duel-stop",
            Self::DuelChannelJoin => "duel-channel-join",
            Self::DuelChannelQuit => "duel-channel-quit",
            Self::SkillTargeted => "skill-targeted",
            Self::Quests => "quests",
            Self::MuHelper => "mu-helper",
            Self::LoginSuccess => "login-success",
            Self::LoginFailure => "login-failure",
            Self::SelectCharacter => "select-character",
            Self::LogoutLogin => "logout-login",
            Self::LogoutCharacter => "logout-character",
            Self::Disconnect => "disconnect",
            Self::Exit => "exit",
            Self::Ping => "ping",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "boot" => Some(Self::Boot),
            "asset-check-failed" | "asset_check_failed" => Some(Self::AssetCheckFailed),
            "ready-for-login" | "ready_for_login" => Some(Self::ReadyForLogin),
            "server-select" | "server_select" | "server-list" | "server_list" => {
                Some(Self::ServerSelect)
            }
            "options" => Some(Self::Options),
            "character-select" | "character_select" | "character-list" | "character_list" => {
                Some(Self::CharacterSelect)
            }
            "character-create" | "character_create" => Some(Self::CharacterCreate),
            "create-character" | "create_character" => Some(Self::CreateCharacter),
            "loading" => Some(Self::Loading),
            "world" => Some(Self::World),
            "chat" => Some(Self::Chat),
            "npc" => Some(Self::Npc),
            "shop" => Some(Self::Shop),
            "game-shop" | "game_shop" => Some(Self::GameShop),
            "trade" => Some(Self::Trade),
            "marketplace" | "player-shop" | "player_shop" => Some(Self::Marketplace),
            "party" => Some(Self::Party),
            "gate" => Some(Self::Gate),
            "siege" => Some(Self::Siege),
            "siege-inactive" | "siege_inactive" => Some(Self::SiegeInactive),
            "siege-soldier" | "siege_soldier" => Some(Self::SiegeSoldier),
            "siege-commander" | "siege_commander" => Some(Self::SiegeCommander),
            "events" => Some(Self::Events),
            "gens" => Some(Self::Gens),
            "friend" => Some(Self::Friend),
            "guild" => Some(Self::Guild),
            "friend-add" | "friend_add" => Some(Self::FriendAdd),
            "friend-delete" | "friend_delete" => Some(Self::FriendDelete),
            "guild-join" | "guild_join" => Some(Self::GuildJoin),
            "guild-create" | "guild_create" => Some(Self::GuildCreate),
            "friend-roster" | "friend_roster" => Some(Self::FriendRoster),
            "friend-inbox" | "friend_inbox" => Some(Self::FriendInbox),
            "friend-compose" | "friend_compose" => Some(Self::FriendCompose),
            "friend-chat-rooms" | "friend_chat_rooms" | "friend-chat_rooms" => {
                Some(Self::FriendChatRooms)
            }
            "guild-summary" | "guild_summary" => Some(Self::GuildSummary),
            "guild-members" | "guild_members" => Some(Self::GuildMembers),
            "guild-union" | "guild_union" => Some(Self::GuildUnion),
            "guild-no-guild" | "guild_no_guild" => Some(Self::GuildNoGuild),
            "guild-error" | "guild_error" => Some(Self::GuildError),
            "guild-role-assign" | "guild_role_assign" => Some(Self::GuildRoleAssign),
            "guild-fire" | "guild_fire" | "guild-kick-player" | "guild_kick_player" => {
                Some(Self::GuildFire)
            }
            "guild-ban-union"
            | "guild_ban_union"
            | "guild-remove-alliance"
            | "guild_remove_alliance" => Some(Self::GuildBanUnion),
            "inventory-use" | "inventory_use" | "item-use" | "item_use" => Some(Self::InventoryUse),
            "inventory-equip" | "inventory_equip" | "item-equip" | "item_equip" => {
                Some(Self::InventoryEquip)
            }
            "inventory-unequip" | "inventory_unequip" | "item-unequip" | "item_unequip" => {
                Some(Self::InventoryUnequip)
            }
            "inventory-move" | "inventory_move" | "item-move" | "item_move" => {
                Some(Self::InventoryMove)
            }
            "vault-deposit" | "vault_deposit" => Some(Self::VaultDeposit),
            "vault-withdraw" | "vault_withdraw" => Some(Self::VaultWithdraw),
            "duel" => Some(Self::Duel),
            "duel-start" | "duel_start" => Some(Self::DuelStart),
            "duel-stop" | "duel_stop" => Some(Self::DuelStop),
            "duel-channel-join" | "duel_channel_join" | "duel-join-channel" => {
                Some(Self::DuelChannelJoin)
            }
            "duel-channel-quit" | "duel_channel_quit" | "duel-quit-channel" => {
                Some(Self::DuelChannelQuit)
            }
            "skill-targeted" | "skill_targeted" | "skill-target" | "skill_target" => {
                Some(Self::SkillTargeted)
            }
            "quests" => Some(Self::Quests),
            "mu-helper" | "mu_helper" => Some(Self::MuHelper),
            "login-success" | "login_success" => Some(Self::LoginSuccess),
            "login-failure" | "login_failure" => Some(Self::LoginFailure),
            "select-character" | "select_character" => Some(Self::SelectCharacter),
            "logout-login" | "logout_login" => Some(Self::LogoutLogin),
            "logout-character" | "logout_character" => Some(Self::LogoutCharacter),
            "disconnect" => Some(Self::Disconnect),
            "exit" => Some(Self::Exit),
            "ping" => Some(Self::Ping),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlSnapshot {
    pub state: AppState,
    pub ui_route: UiRoute,
    pub session_phase: SessionPhase,
    pub last_command: Option<ControlCommand>,
    pub selected_character_name: Option<String>,
    pub friend_name: Option<String>,
    pub guild_master_player_id: Option<u16>,
    pub guild_player_name: Option<String>,
    pub guild_create_name: Option<String>,
    pub guild_create_emblem: Option<[u8; 32]>,
    pub duel_player_id: Option<u16>,
    pub duel_player_name: Option<String>,
    pub duel_channel_id: Option<u8>,
    pub skill_id: Option<u16>,
    pub skill_target_id: Option<u16>,
    pub guild_role: Option<u8>,
    pub guild_assignment_type: Option<u8>,
    pub guild_security_code: Option<String>,
    pub guild_union_name: Option<String>,
    pub siege_screen_state: Option<SiegeScreenState>,
    pub inventory_use_slot: Option<u8>,
    pub inventory_use_target: Option<u8>,
    pub inventory_use_add_points: Option<bool>,
    pub inventory_equip_slot: Option<u8>,
    pub inventory_unequip_slot: Option<u8>,
    pub inventory_move_from_slot: Option<u8>,
    pub inventory_move_to_slot: Option<u8>,
    pub friend_screen_state: Option<FriendScreenState>,
    pub guild_screen_state: Option<GuildScreenState>,
    pub vault_money_amount: Option<u32>,
    pub command_count: u64,
}

impl ControlSnapshot {
    pub fn new(state: AppState) -> Self {
        Self {
            state,
            ui_route: initial_ui_route(state),
            session_phase: initial_session_phase(state),
            last_command: None,
            selected_character_name: None,
            friend_name: None,
            guild_master_player_id: None,
            guild_player_name: None,
            guild_create_name: None,
            guild_create_emblem: None,
            duel_player_id: None,
            duel_player_name: None,
            duel_channel_id: None,
            skill_id: None,
            skill_target_id: None,
            guild_role: None,
            guild_assignment_type: None,
            guild_security_code: None,
            guild_union_name: None,
            siege_screen_state: None,
            inventory_use_slot: None,
            inventory_use_target: None,
            inventory_use_add_points: None,
            inventory_equip_slot: None,
            inventory_unequip_slot: None,
            inventory_move_from_slot: None,
            inventory_move_to_slot: None,
            friend_screen_state: None,
            guild_screen_state: None,
            vault_money_amount: None,
            command_count: 0,
        }
    }

    pub fn apply_command(&mut self, command: ControlCommand) -> bool {
        self.last_command = Some(command);
        self.command_count = self.command_count.saturating_add(1);

        if !matches!(
            command,
            ControlCommand::Siege
                | ControlCommand::SiegeInactive
                | ControlCommand::SiegeSoldier
                | ControlCommand::SiegeCommander
        ) {
            self.siege_screen_state = None;
        }

        match command {
            ControlCommand::Boot => {
                self.state = AppState::Boot;
                self.ui_route = UiRoute::Boot;
                self.session_phase = SessionPhase::ReadyForLogin;
                false
            }
            ControlCommand::AssetCheckFailed => {
                self.state = AppState::AssetCheckFailed;
                self.ui_route = UiRoute::Error;
                self.session_phase = SessionPhase::ReadyForLogin;
                false
            }
            ControlCommand::ReadyForLogin => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Login;
                self.session_phase = SessionPhase::ReadyForLogin;
                false
            }
            ControlCommand::ServerSelect => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::ServerSelect;
                false
            }
            ControlCommand::Options => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Options;
                false
            }
            ControlCommand::CharacterSelect => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::CharacterSelect;
                false
            }
            ControlCommand::CharacterCreate => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::CharacterCreate;
                self.session_phase = SessionPhase::LoggedIn;
                self.selected_character_name = None;
                false
            }
            ControlCommand::CreateCharacter => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::CharacterCreate;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Loading => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Loading;
                false
            }
            ControlCommand::World => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::World;
                false
            }
            ControlCommand::Chat => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Chat;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Npc => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Npc;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Shop => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Shop;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::GameShop => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::GameShop;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Trade => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Trade;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Marketplace => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Marketplace;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Party => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Party;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Gate => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Gate;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Siege => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Siege;
                self.session_phase = SessionPhase::LoggedIn;
                self.siege_screen_state = Some(SiegeScreenState::Observer);
                false
            }
            ControlCommand::SiegeInactive => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Siege;
                self.session_phase = SessionPhase::LoggedIn;
                self.siege_screen_state = Some(SiegeScreenState::Inactive);
                false
            }
            ControlCommand::SiegeSoldier => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Siege;
                self.session_phase = SessionPhase::LoggedIn;
                self.siege_screen_state = Some(SiegeScreenState::Soldier);
                false
            }
            ControlCommand::SiegeCommander => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Siege;
                self.session_phase = SessionPhase::LoggedIn;
                self.siege_screen_state = Some(SiegeScreenState::Commander);
                false
            }
            ControlCommand::Events => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Events;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Gens => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Hud;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Friend => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Friend;
                self.session_phase = SessionPhase::LoggedIn;
                self.friend_screen_state = None;
                false
            }
            ControlCommand::FriendAdd | ControlCommand::FriendDelete => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Friend;
                self.session_phase = SessionPhase::LoggedIn;
                self.friend_screen_state = None;
                false
            }
            ControlCommand::GuildJoin => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = None;
                false
            }
            ControlCommand::GuildCreate => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = None;
                false
            }
            ControlCommand::Guild => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = None;
                false
            }
            ControlCommand::FriendRoster => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Friend;
                self.session_phase = SessionPhase::LoggedIn;
                self.friend_screen_state = Some(FriendScreenState::Roster);
                false
            }
            ControlCommand::FriendInbox => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Friend;
                self.session_phase = SessionPhase::LoggedIn;
                self.friend_screen_state = Some(FriendScreenState::Inbox);
                false
            }
            ControlCommand::FriendCompose => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Friend;
                self.session_phase = SessionPhase::LoggedIn;
                self.friend_screen_state = Some(FriendScreenState::Compose);
                false
            }
            ControlCommand::FriendChatRooms => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Friend;
                self.session_phase = SessionPhase::LoggedIn;
                self.friend_screen_state = Some(FriendScreenState::ChatRooms);
                false
            }
            ControlCommand::GuildSummary => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = Some(GuildScreenState::Summary);
                false
            }
            ControlCommand::GuildMembers => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = Some(GuildScreenState::Members);
                false
            }
            ControlCommand::GuildUnion => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = Some(GuildScreenState::Union);
                false
            }
            ControlCommand::GuildNoGuild => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = Some(GuildScreenState::NoGuild);
                false
            }
            ControlCommand::GuildError => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = Some(GuildScreenState::Error);
                false
            }
            ControlCommand::GuildRoleAssign => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = Some(GuildScreenState::Members);
                false
            }
            ControlCommand::GuildFire => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = Some(GuildScreenState::Members);
                false
            }
            ControlCommand::GuildBanUnion => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Guild;
                self.session_phase = SessionPhase::LoggedIn;
                self.guild_screen_state = Some(GuildScreenState::Union);
                false
            }
            ControlCommand::InventoryUse
            | ControlCommand::InventoryEquip
            | ControlCommand::InventoryUnequip
            | ControlCommand::InventoryMove => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Inventory;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::VaultDeposit | ControlCommand::VaultWithdraw => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Inventory;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::Duel => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Duel;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::DuelStart
            | ControlCommand::DuelStop
            | ControlCommand::DuelChannelJoin
            | ControlCommand::DuelChannelQuit
            | ControlCommand::SkillTargeted => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = if matches!(command, ControlCommand::SkillTargeted) {
                    UiRoute::World
                } else {
                    UiRoute::Duel
                };
                self.session_phase = SessionPhase::LoggedIn;
                if matches!(command, ControlCommand::DuelChannelQuit) {
                    self.duel_channel_id = None;
                }
                false
            }
            ControlCommand::Quests => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Quests;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::MuHelper => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::MuHelper;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::LoginSuccess => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::CharacterSelect;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::LoginFailure => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Login;
                self.session_phase = SessionPhase::ReadyForLogin;
                false
            }
            ControlCommand::SelectCharacter => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::CharacterSelect;
                self.session_phase = SessionPhase::LoggedIn;
                false
            }
            ControlCommand::LogoutLogin => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Login;
                self.session_phase = SessionPhase::ReadyForLogin;
                false
            }
            ControlCommand::LogoutCharacter => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::CharacterSelect;
                self.session_phase = SessionPhase::ReadyForLogin;
                false
            }
            ControlCommand::Disconnect => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Login;
                self.session_phase = SessionPhase::Disconnected;
                false
            }
            ControlCommand::Exit => {
                self.state = AppState::Exit;
                self.ui_route = UiRoute::Error;
                self.session_phase = SessionPhase::Disconnected;
                true
            }
            ControlCommand::Ping => false,
        }
    }

    pub fn to_json(&self) -> String {
        let last_command = self
            .last_command
            .map(|command| format!("\"{}\"", command.as_str()))
            .unwrap_or_else(|| "null".to_string());
        let selected_character_name = self
            .selected_character_name
            .as_ref()
            .map(|name| format!("\"{}\"", name))
            .unwrap_or_else(|| "null".to_string());
        let friend_name = self
            .friend_name
            .as_ref()
            .map(|name| format!("\"{}\"", name))
            .unwrap_or_else(|| "null".to_string());
        let guild_master_player_id = self
            .guild_master_player_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let guild_player_name = self
            .guild_player_name
            .as_ref()
            .map(|name| format!("\"{}\"", name))
            .unwrap_or_else(|| "null".to_string());
        let guild_create_name = self
            .guild_create_name
            .as_ref()
            .map(|name| format!("\"{}\"", name))
            .unwrap_or_else(|| "null".to_string());
        let guild_create_emblem = self
            .guild_create_emblem
            .as_ref()
            .map(|value| format!("\"{}\"", guild_emblem_to_hex(value)))
            .unwrap_or_else(|| "null".to_string());
        let duel_player_id = self
            .duel_player_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let duel_player_name = self
            .duel_player_name
            .as_ref()
            .map(|name| format!("\"{}\"", name))
            .unwrap_or_else(|| "null".to_string());
        let duel_channel_id = self
            .duel_channel_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let skill_id = self
            .skill_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let skill_target_id = self
            .skill_target_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let guild_role = self
            .guild_role
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let guild_assignment_type = self
            .guild_assignment_type
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let guild_security_code = self
            .guild_security_code
            .as_ref()
            .map(|name| format!("\"{}\"", name))
            .unwrap_or_else(|| "null".to_string());
        let guild_union_name = self
            .guild_union_name
            .as_ref()
            .map(|name| format!("\"{}\"", name))
            .unwrap_or_else(|| "null".to_string());
        let inventory_use_slot = self
            .inventory_use_slot
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let inventory_use_target = self
            .inventory_use_target
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let inventory_use_add_points = self
            .inventory_use_add_points
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let inventory_equip_slot = self
            .inventory_equip_slot
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let inventory_unequip_slot = self
            .inventory_unequip_slot
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let friend_screen_state = self
            .friend_screen_state
            .map(|state| format!("\"{}\"", state.as_str()))
            .unwrap_or_else(|| "null".to_string());
        let guild_screen_state = self
            .guild_screen_state
            .map(|state| format!("\"{}\"", state.as_str()))
            .unwrap_or_else(|| "null".to_string());
        let siege_screen_state = self
            .siege_screen_state
            .map(|state| format!("\"{}\"", state.as_str()))
            .unwrap_or_else(|| "null".to_string());
        let vault_money_amount = self
            .vault_money_amount
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let inventory_move_from_slot = self
            .inventory_move_from_slot
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let inventory_move_to_slot = self
            .inventory_move_to_slot
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());

        format!(
            "{{\"state\":\"{}\",\"ui_route\":\"{}\",\"session_phase\":\"{}\",\"last_command\":{},\"selected_character_name\":{},\"friend_name\":{},\"guild_master_player_id\":{},\"guild_player_name\":{},\"guild_create_name\":{},\"guild_create_emblem\":{},\"duel_player_id\":{},\"duel_player_name\":{},\"duel_channel_id\":{},\"skill_id\":{},\"skill_target_id\":{},\"guild_role\":{},\"guild_assignment_type\":{},\"guild_security_code\":{},\"guild_union_name\":{},\"inventory_use_slot\":{},\"inventory_use_target\":{},\"inventory_use_add_points\":{},\"inventory_equip_slot\":{},\"inventory_unequip_slot\":{},\"friend_screen_state\":{},\"guild_screen_state\":{},\"siege_screen_state\":{},\"vault_money_amount\":{},\"inventory_move_from_slot\":{},\"inventory_move_to_slot\":{},\"command_count\":{}}}",
            self.state.as_str(),
            self.ui_route.slug(),
            self.session_phase.as_str(),
            last_command,
            selected_character_name,
            friend_name,
            guild_master_player_id,
            guild_player_name,
            guild_create_name,
            guild_create_emblem,
            duel_player_id,
            duel_player_name,
            duel_channel_id,
            skill_id,
            skill_target_id,
            guild_role,
            guild_assignment_type,
            guild_security_code,
            guild_union_name,
            inventory_use_slot,
            inventory_use_target,
            inventory_use_add_points,
            inventory_equip_slot,
            inventory_unequip_slot,
            friend_screen_state,
            guild_screen_state,
            siege_screen_state,
            vault_money_amount,
            inventory_move_from_slot,
            inventory_move_to_slot,
            self.command_count
        )
    }
}

#[derive(Debug, Clone, Resource)]
pub struct ControlHttpState {
    snapshot: Arc<Mutex<ControlSnapshot>>,
    last_applied_command_count: u64,
}

impl ControlHttpState {
    pub fn new(snapshot: Arc<Mutex<ControlSnapshot>>) -> Self {
        Self {
            snapshot,
            last_applied_command_count: 0,
        }
    }

    pub fn snapshot(&self) -> ControlSnapshot {
        self.snapshot
            .lock()
            .expect("control snapshot mutex poisoned")
            .clone()
    }

    pub fn last_applied_command_count(&self) -> u64 {
        self.last_applied_command_count
    }

    pub fn mark_applied(&mut self, command_count: u64) {
        self.last_applied_command_count = command_count;
    }

    pub fn sync_from_runtime(&self, ui_route: UiRoute, session_phase: SessionPhase) {
        let mut snapshot = self
            .snapshot
            .lock()
            .expect("control snapshot mutex poisoned");
        snapshot.ui_route = ui_route;
        snapshot.session_phase = session_phase;
    }

    pub fn shared_snapshot(&self) -> Arc<Mutex<ControlSnapshot>> {
        Arc::clone(&self.snapshot)
    }
}

#[derive(Debug)]
pub struct ControlServerHandle {
    address: SocketAddr,
    snapshot: Arc<Mutex<ControlSnapshot>>,
    shutdown: Arc<AtomicBool>,
    join_handle: Option<JoinHandle<io::Result<()>>>,
}

impl ControlServerHandle {
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn shared_snapshot(&self) -> Arc<Mutex<ControlSnapshot>> {
        Arc::clone(&self.snapshot)
    }

    pub fn snapshot(&self) -> ControlSnapshot {
        self.snapshot
            .lock()
            .expect("control snapshot mutex poisoned")
            .clone()
    }

    pub fn request_shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
        let _ = poke_shutdown(self.address);
    }

    pub fn join(mut self) -> io::Result<ControlSnapshot> {
        let join_handle = self
            .join_handle
            .take()
            .expect("control server handle joined more than once");

        match join_handle.join() {
            Ok(result) => result?,
            Err(_) => {
                return Err(io::Error::other("control server thread panicked"));
            }
        }

        Ok(self.snapshot())
    }
}

impl Drop for ControlServerHandle {
    fn drop(&mut self) {
        if self.join_handle.is_some() {
            self.shutdown.store(true, Ordering::Release);
            let _ = poke_shutdown(self.address);
        }
    }
}

pub fn spawn(bind_addr: SocketAddr, initial_state: AppState) -> io::Result<ControlServerHandle> {
    let listener = TcpListener::bind(bind_addr)?;
    let address = listener.local_addr()?;
    let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(initial_state)));
    let shutdown = Arc::new(AtomicBool::new(false));

    let thread_snapshot = Arc::clone(&snapshot);
    let thread_shutdown = Arc::clone(&shutdown);
    let join_handle = thread::spawn(move || serve_loop(listener, thread_snapshot, thread_shutdown));

    Ok(ControlServerHandle {
        address,
        snapshot,
        shutdown,
        join_handle: Some(join_handle),
    })
}

pub fn serve(bind_addr: SocketAddr, initial_state: AppState) -> io::Result<()> {
    let listener = TcpListener::bind(bind_addr)?;
    let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(initial_state)));
    let shutdown = Arc::new(AtomicBool::new(false));

    serve_loop(listener, snapshot, shutdown)
}

fn initial_ui_route(state: AppState) -> UiRoute {
    match state {
        AppState::Boot => UiRoute::Boot,
        AppState::AssetCheckFailed => UiRoute::Error,
        AppState::ReadyForLogin => UiRoute::Login,
        AppState::Exit => UiRoute::Error,
    }
}

fn initial_session_phase(state: AppState) -> SessionPhase {
    match state {
        AppState::Exit => SessionPhase::Disconnected,
        _ => SessionPhase::ReadyForLogin,
    }
}

fn serve_loop(
    listener: TcpListener,
    snapshot: Arc<Mutex<ControlSnapshot>>,
    shutdown: Arc<AtomicBool>,
) -> io::Result<()> {
    loop {
        if shutdown.load(Ordering::Acquire) {
            return Ok(());
        }

        let (stream, _) = listener.accept()?;
        handle_connection(stream, &snapshot, &shutdown)?;

        if shutdown.load(Ordering::Acquire) {
            return Ok(());
        }
    }
}

fn handle_connection(
    stream: TcpStream,
    snapshot: &Arc<Mutex<ControlSnapshot>>,
    shutdown: &Arc<AtomicBool>,
) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let request = match read_request(&mut reader) {
        Ok(request) => request,
        Err(_) => {
            let mut stream = reader.into_inner();
            let _ = write_response(
                &mut stream,
                400,
                "Bad Request",
                "application/json",
                r#"{"error":"bad request"}"#,
            );
            return Ok(());
        }
    };

    let mut stream = reader.into_inner();
    let response = route_request(request, snapshot, shutdown);
    let _ = write_response(
        &mut stream,
        response.status,
        response.reason,
        response.content_type,
        &response.body,
    );
    Ok(())
}

fn route_request(
    request: HttpRequest,
    snapshot: &Arc<Mutex<ControlSnapshot>>,
    shutdown: &Arc<AtomicBool>,
) -> HttpResponse {
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") | ("GET", "/state") => {
            let snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            HttpResponse::json(200, "OK", snapshot.to_json())
        }
        ("POST", "/command") => {
            let command_name = request.command_name();

            let Some(command_name) = command_name else {
                return HttpResponse::json(
                    400,
                    "Bad Request",
                    r#"{"error":"missing command"}"#.to_string(),
                );
            };

            let Some(command) = ControlCommand::parse(command_name) else {
                return HttpResponse::json(
                    400,
                    "Bad Request",
                    r#"{"error":"unknown command"}"#.to_string(),
                );
            };

            let mut snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
            let should_shutdown = match command {
                ControlCommand::SelectCharacter => {
                    let Some(character_name) = character_name_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing character name"}"#.to_string(),
                        );
                    };

                    snapshot.selected_character_name = Some(character_name);
                    snapshot.apply_command(command)
                }
                ControlCommand::FriendAdd | ControlCommand::FriendDelete => {
                    let Some(friend_name) = friend_name_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing friend name"}"#.to_string(),
                        );
                    };

                    if friend_name.trim().is_empty() {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing friend name"}"#.to_string(),
                        );
                    }

                    snapshot.friend_name = Some(friend_name);
                    snapshot.apply_command(command)
                }
                ControlCommand::GuildJoin => {
                    let Some(guild_master_player_id) = guild_join_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing guild join payload"}"#.to_string(),
                        );
                    };

                    snapshot.guild_master_player_id = Some(guild_master_player_id);
                    snapshot.apply_command(command)
                }
                ControlCommand::GuildCreate => {
                    let Some((guild_name, guild_emblem)) = guild_create_from_request(&request)
                    else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing guild create payload"}"#.to_string(),
                        );
                    };

                    snapshot.guild_create_name = Some(guild_name);
                    snapshot.guild_create_emblem = Some(guild_emblem);
                    snapshot.apply_command(command)
                }
                ControlCommand::GuildRoleAssign => {
                    let Some((player_name, role, assignment_type)) =
                        guild_role_assign_from_request(&request)
                    else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing guild role assignment payload"}"#.to_string(),
                        );
                    };

                    snapshot.guild_player_name = Some(player_name);
                    snapshot.guild_role = Some(role);
                    snapshot.guild_assignment_type = Some(assignment_type);
                    snapshot.apply_command(command)
                }
                ControlCommand::GuildFire => {
                    let Some((player_name, security_code)) = guild_fire_from_request(&request)
                    else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing guild fire payload"}"#.to_string(),
                        );
                    };

                    snapshot.guild_player_name = Some(player_name);
                    snapshot.guild_security_code = Some(security_code);
                    snapshot.apply_command(command)
                }
                ControlCommand::GuildBanUnion => {
                    let Some(guild_name) = guild_ban_union_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing guild ban union payload"}"#.to_string(),
                        );
                    };

                    snapshot.guild_union_name = Some(guild_name);
                    snapshot.apply_command(command)
                }
                ControlCommand::DuelStart => {
                    let Some((player_id, player_name)) = duel_start_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing duel start payload"}"#.to_string(),
                        );
                    };

                    snapshot.duel_player_id = Some(player_id);
                    snapshot.duel_player_name = Some(player_name);
                    snapshot.apply_command(command)
                }
                ControlCommand::DuelChannelJoin => {
                    let Some(channel_id) = duel_channel_join_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing duel channel payload"}"#.to_string(),
                        );
                    };

                    snapshot.duel_channel_id = Some(channel_id);
                    snapshot.apply_command(command)
                }
                ControlCommand::DuelChannelQuit => {
                    snapshot.duel_channel_id = None;
                    snapshot.apply_command(command)
                }
                ControlCommand::SkillTargeted => {
                    let Some((skill_id, target_id)) = skill_targeted_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing skill targeted payload"}"#.to_string(),
                        );
                    };

                    snapshot.skill_id = Some(skill_id);
                    snapshot.skill_target_id = Some(target_id);
                    snapshot.apply_command(command)
                }
                ControlCommand::DuelStop => snapshot.apply_command(command),
                ControlCommand::InventoryUse => {
                    let Some((slot, target, add_points)) = inventory_use_from_request(&request)
                    else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing inventory use payload"}"#.to_string(),
                        );
                    };

                    snapshot.inventory_use_slot = Some(slot);
                    snapshot.inventory_use_target = Some(target);
                    snapshot.inventory_use_add_points = Some(add_points);
                    snapshot.apply_command(command)
                }
                ControlCommand::InventoryEquip => {
                    let Some(slot) = inventory_action_slot_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing inventory equip payload"}"#.to_string(),
                        );
                    };

                    snapshot.inventory_equip_slot = Some(slot);
                    snapshot.apply_command(command)
                }
                ControlCommand::InventoryUnequip => {
                    let Some(slot) = inventory_action_slot_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing inventory unequip payload"}"#.to_string(),
                        );
                    };

                    snapshot.inventory_unequip_slot = Some(slot);
                    snapshot.apply_command(command)
                }
                ControlCommand::InventoryMove => {
                    let Some((from_slot, to_slot)) = inventory_move_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing inventory move payload"}"#.to_string(),
                        );
                    };

                    snapshot.inventory_move_from_slot = Some(from_slot);
                    snapshot.inventory_move_to_slot = Some(to_slot);
                    snapshot.apply_command(command)
                }
                ControlCommand::VaultDeposit | ControlCommand::VaultWithdraw => {
                    let Some(amount) = vault_money_amount_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing vault amount"}"#.to_string(),
                        );
                    };

                    if amount == 0 {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"vault amount must be positive"}"#.to_string(),
                        );
                    }

                    snapshot.vault_money_amount = Some(amount);
                    snapshot.apply_command(command)
                }
                ControlCommand::CreateCharacter => {
                    let Some(character_name) = character_name_from_request(&request) else {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"missing character name"}"#.to_string(),
                        );
                    };

                    if character_name.trim().len() < CHARACTER_CREATE_NAME_MIN_LENGTH {
                        return HttpResponse::json(
                            400,
                            "Bad Request",
                            r#"{"error":"character name must be at least 4 characters"}"#
                                .to_string(),
                        );
                    }

                    snapshot.selected_character_name = Some(character_name);
                    snapshot.apply_command(command)
                }
                _ => snapshot.apply_command(command),
            };
            let response = HttpResponse::json(200, "OK", snapshot.to_json());

            if should_shutdown {
                shutdown.store(true, Ordering::Release);
            }

            response
        }
        ("GET", "/__shutdown") => {
            shutdown.store(true, Ordering::Release);
            HttpResponse::empty(204, "No Content")
        }
        _ => HttpResponse::json(404, "Not Found", r#"{"error":"not found"}"#.to_string()),
    }
}

fn read_request(reader: &mut BufReader<TcpStream>) -> io::Result<HttpRequest> {
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    if request_line.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing request line",
        ));
    }

    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing request method"))?;
    let target = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing request target"))?;

    let mut content_length = 0usize;

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }

        if let Some((name, value)) = trimmed.split_once(':') {
            if name.trim().eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }
    }

    let mut body = vec![0; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }

    let body = String::from_utf8(body)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "request body is not utf-8"))?;

    let (path, query) = split_target(target);
    Ok(HttpRequest {
        method: method.to_string(),
        path,
        query,
        body,
    })
}

fn split_target(target: &str) -> (String, String) {
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    (path.to_string(), query.to_string())
}

fn poke_shutdown(address: SocketAddr) -> io::Result<()> {
    let mut stream = TcpStream::connect(wakeup_target(address))?;
    write_all(
        &mut stream,
        "GET /__shutdown HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )?;
    let mut response = Vec::new();
    let _ = stream.read_to_end(&mut response);
    Ok(())
}

fn wakeup_target(address: SocketAddr) -> SocketAddr {
    if !address.ip().is_unspecified() {
        return address;
    }

    let ip = match address {
        SocketAddr::V4(_) => IpAddr::V4(Ipv4Addr::LOCALHOST),
        SocketAddr::V6(_) => IpAddr::V6(Ipv6Addr::LOCALHOST),
    };

    SocketAddr::new(ip, address.port())
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    content_type: &str,
    body: &str,
) -> io::Result<()> {
    let body_bytes = body.as_bytes();
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        status,
        reason,
        content_type,
        body_bytes.len()
    );

    write_all(stream, &response)?;
    write_all(stream, body)
}

fn write_all(stream: &mut TcpStream, body: &str) -> io::Result<()> {
    stream.write_all(body.as_bytes())?;
    stream.flush()
}

fn query_value<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        if name == key {
            Some(value)
        } else {
            None
        }
    })
}

fn character_name_from_request(request: &HttpRequest) -> Option<String> {
    query_value(&request.query, "character")
        .or_else(|| query_value(&request.body, "character"))
        .map(str::to_string)
        .or_else(|| {
            let body = request.body.trim();
            if body.is_empty() {
                None
            } else {
                Some(body.to_string())
            }
        })
}

fn friend_name_from_request(request: &HttpRequest) -> Option<String> {
    query_value(&request.query, "friend")
        .or_else(|| query_value(&request.body, "friend"))
        .map(str::to_string)
        .or_else(|| {
            let body = request.body.trim();
            if body.is_empty() {
                None
            } else {
                Some(body.to_string())
            }
        })
}

fn guild_join_from_request(request: &HttpRequest) -> Option<u16> {
    query_value(&request.query, "master_id")
        .or_else(|| query_value(&request.query, "master-id"))
        .or_else(|| query_value(&request.body, "master_id"))
        .or_else(|| query_value(&request.body, "master-id"))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            let body = request.body.trim();
            if body.is_empty() {
                None
            } else {
                Some(body)
            }
        })?
        .parse::<u16>()
        .ok()
}

fn guild_create_from_request(request: &HttpRequest) -> Option<(String, [u8; 32])> {
    let guild_name = query_value(&request.query, "guild_name")
        .or_else(|| query_value(&request.query, "guild-name"))
        .or_else(|| query_value(&request.body, "guild_name"))
        .or_else(|| query_value(&request.body, "guild-name"))?
        .trim()
        .to_string();

    let guild_name_length = guild_name.chars().count();
    if guild_name.is_empty() || guild_name_length < 4 || guild_name_length > MAX_GUILD_NAME_LENGTH {
        return None;
    }

    let guild_emblem = guild_emblem_from_request(request)?;

    Some((guild_name, guild_emblem))
}

fn guild_emblem_from_request(request: &HttpRequest) -> Option<[u8; 32]> {
    let value = query_value(&request.query, "guild_emblem")
        .or_else(|| query_value(&request.query, "guild-emblem"))
        .or_else(|| query_value(&request.body, "guild_emblem"))
        .or_else(|| query_value(&request.body, "guild-emblem"))?
        .trim();

    decode_guild_emblem_hex(value)
}

fn decode_guild_emblem_hex(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 {
        return None;
    }

    let bytes = value.as_bytes();
    let mut emblem = [0u8; 32];

    let mut index = 0;
    while index < emblem.len() {
        emblem[index] = decode_hex_byte(bytes[index * 2], bytes[index * 2 + 1])?;
        index += 1;
    }

    Some(emblem)
}

fn decode_hex_byte(high: u8, low: u8) -> Option<u8> {
    Some((decode_hex_digit(high)? << 4) | decode_hex_digit(low)?)
}

fn decode_hex_digit(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn guild_emblem_to_hex(value: &[u8; 32]) -> String {
    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

    let mut output = String::with_capacity(64);
    for &byte in value {
        output.push(HEX_DIGITS[(byte >> 4) as usize] as char);
        output.push(HEX_DIGITS[(byte & 0x0f) as usize] as char);
    }

    output
}

fn guild_role_assign_from_request(request: &HttpRequest) -> Option<(String, u8, u8)> {
    let player_name = query_value(&request.query, "player")
        .or_else(|| query_value(&request.body, "player"))?
        .trim()
        .to_string();

    if player_name.is_empty() {
        return None;
    }

    let role = query_value(&request.query, "role")
        .or_else(|| query_value(&request.body, "role"))?
        .trim()
        .parse::<u8>()
        .ok()?;

    let assignment_type = query_value(&request.query, "type")
        .or_else(|| query_value(&request.body, "type"))?
        .trim()
        .parse::<u8>()
        .ok()?;

    Some((player_name, role, assignment_type))
}

fn guild_fire_from_request(request: &HttpRequest) -> Option<(String, String)> {
    let player_name = query_value(&request.query, "player")
        .or_else(|| query_value(&request.body, "player"))?
        .trim()
        .to_string();

    if player_name.is_empty() {
        return None;
    }

    let security_code = query_value(&request.query, "security_code")
        .or_else(|| query_value(&request.query, "security-code"))
        .or_else(|| query_value(&request.query, "authority_code"))
        .or_else(|| query_value(&request.query, "authority-code"))
        .or_else(|| query_value(&request.body, "security_code"))
        .or_else(|| query_value(&request.body, "security-code"))
        .or_else(|| query_value(&request.body, "authority_code"))
        .or_else(|| query_value(&request.body, "authority-code"))
        .map(str::trim)
        .filter(|value| !value.is_empty())?
        .to_string();

    Some((player_name, security_code))
}

fn guild_ban_union_from_request(request: &HttpRequest) -> Option<String> {
    if let Some(guild_name) = query_value(&request.query, "guild_name")
        .or_else(|| query_value(&request.query, "guild-name"))
        .or_else(|| query_value(&request.query, "union_name"))
        .or_else(|| query_value(&request.query, "union-name"))
        .or_else(|| query_value(&request.body, "guild_name"))
        .or_else(|| query_value(&request.body, "guild-name"))
        .or_else(|| query_value(&request.body, "union_name"))
        .or_else(|| query_value(&request.body, "union-name"))
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return Some(guild_name.to_string());
    }

    let body = request.body.trim();
    if body.is_empty() || body.contains('=') {
        None
    } else {
        Some(body.to_string())
    }
}

fn duel_start_from_request(request: &HttpRequest) -> Option<(u16, String)> {
    let player_id = query_value(&request.query, "player_id")
        .or_else(|| query_value(&request.query, "player-id"))
        .or_else(|| query_value(&request.body, "player_id"))
        .or_else(|| query_value(&request.body, "player-id"))?
        .trim()
        .parse::<u16>()
        .ok()?;

    let player_name = query_value(&request.query, "player_name")
        .or_else(|| query_value(&request.query, "player-name"))
        .or_else(|| query_value(&request.query, "player"))
        .or_else(|| query_value(&request.body, "player_name"))
        .or_else(|| query_value(&request.body, "player-name"))
        .or_else(|| query_value(&request.body, "player"))?
        .trim()
        .to_string();

    if player_name.is_empty() {
        return None;
    }

    Some((player_id, player_name))
}

fn duel_channel_join_from_request(request: &HttpRequest) -> Option<u8> {
    if let Some(channel_id) = query_value(&request.query, "channel_id")
        .or_else(|| query_value(&request.query, "channel-id"))
        .or_else(|| query_value(&request.query, "channel"))
        .or_else(|| query_value(&request.body, "channel_id"))
        .or_else(|| query_value(&request.body, "channel-id"))
        .or_else(|| query_value(&request.body, "channel"))
        .and_then(|value| value.trim().parse::<u8>().ok())
    {
        return Some(channel_id);
    }

    let body = request.body.trim();
    if body.is_empty() || body.contains('=') {
        None
    } else {
        body.parse::<u8>().ok()
    }
}

fn skill_targeted_from_request(request: &HttpRequest) -> Option<(u16, u16)> {
    let skill_id = query_value(&request.query, "skill_id")
        .or_else(|| query_value(&request.query, "skill-id"))
        .or_else(|| query_value(&request.body, "skill_id"))
        .or_else(|| query_value(&request.body, "skill-id"))?
        .trim()
        .parse::<u16>()
        .ok()?;

    let target_id = query_value(&request.query, "target_id")
        .or_else(|| query_value(&request.query, "target-id"))
        .or_else(|| query_value(&request.body, "target_id"))
        .or_else(|| query_value(&request.body, "target-id"))?
        .trim()
        .parse::<u16>()
        .ok()?;

    Some((skill_id, target_id))
}

fn inventory_move_from_request(request: &HttpRequest) -> Option<(u8, u8)> {
    let from_slot = query_value(&request.query, "from_slot")
        .or_else(|| query_value(&request.body, "from_slot"))?
        .trim()
        .parse::<u8>()
        .ok()?;

    let to_slot = query_value(&request.query, "to_slot")
        .or_else(|| query_value(&request.body, "to_slot"))?
        .trim()
        .parse::<u8>()
        .ok()?;

    Some((from_slot, to_slot))
}

fn inventory_action_slot_from_request(request: &HttpRequest) -> Option<u8> {
    query_value(&request.query, "slot")
        .or_else(|| query_value(&request.query, "item_slot"))
        .or_else(|| query_value(&request.body, "slot"))
        .or_else(|| query_value(&request.body, "item_slot"))?
        .trim()
        .parse::<u8>()
        .ok()
}

fn inventory_use_from_request(request: &HttpRequest) -> Option<(u8, u8, bool)> {
    let slot = inventory_action_slot_from_request(request)?;

    let target = match query_value(&request.query, "target")
        .or_else(|| query_value(&request.body, "target"))
    {
        Some(value) => value.trim().parse::<u8>().ok()?,
        None => 0,
    };

    let add_points = match query_value(&request.query, "add_points")
        .or_else(|| query_value(&request.query, "add-points"))
        .or_else(|| query_value(&request.query, "fruit"))
        .or_else(|| query_value(&request.body, "add_points"))
        .or_else(|| query_value(&request.body, "add-points"))
        .or_else(|| query_value(&request.body, "fruit"))
    {
        Some(value) => parse_bool_value(value)?,
        None => true,
    };

    Some((slot, target, add_points))
}

fn vault_money_amount_from_request(request: &HttpRequest) -> Option<u32> {
    query_value(&request.query, "amount")
        .or_else(|| query_value(&request.body, "amount"))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            let body = request.body.trim();
            if body.is_empty() {
                None
            } else {
                Some(body)
            }
        })?
        .parse::<u32>()
        .ok()
}

fn parse_bool_value(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

#[derive(Debug, Clone)]
struct HttpRequest {
    method: String,
    path: String,
    query: String,
    body: String,
}

impl HttpRequest {
    fn command_name(&self) -> Option<&str> {
        query_value(&self.query, "name")
            .or_else(|| query_value(&self.body, "name"))
            .or_else(|| {
                let body = self.body.trim();
                if body.is_empty() {
                    None
                } else {
                    Some(body)
                }
            })
    }
}

#[derive(Debug, Clone)]
struct HttpResponse {
    status: u16,
    reason: &'static str,
    content_type: &'static str,
    body: String,
}

impl HttpResponse {
    fn json(status: u16, reason: &'static str, body: String) -> Self {
        Self {
            status,
            reason,
            content_type: "application/json",
            body,
        }
    }

    fn empty(status: u16, reason: &'static str) -> Self {
        Self {
            status,
            reason,
            content_type: "text/plain; charset=utf-8",
            body: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{route_request, spawn, ControlCommand, ControlSnapshot, HttpRequest};
    use crate::{AppState, SessionPhase};
    use mu_ui::{FriendScreenState, GuildScreenState, UiRoute};
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::atomic::AtomicBool;
    use std::sync::{Arc, Mutex};

    fn read_response(stream: &mut TcpStream) -> (String, String) {
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        let (head, body) = response.split_once("\r\n\r\n").unwrap();
        (head.to_string(), body.to_string())
    }

    fn send_request(address: std::net::SocketAddr, request: &str) -> (String, String) {
        let mut stream = TcpStream::connect(address).unwrap();
        stream.write_all(request.as_bytes()).unwrap();
        stream.flush().unwrap();
        read_response(&mut stream)
    }

    #[test]
    fn snapshot_serializes_current_state() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);
        snapshot.apply_command(ControlCommand::Ping);

        assert_eq!(
            snapshot.to_json(),
            r#"{"state":"ready-for-login","ui_route":"login","session_phase":"ready-for-login","last_command":"ping","selected_character_name":null,"friend_name":null,"guild_master_player_id":null,"guild_player_name":null,"guild_create_name":null,"guild_create_emblem":null,"duel_player_id":null,"duel_player_name":null,"duel_channel_id":null,"skill_id":null,"skill_target_id":null,"guild_role":null,"guild_assignment_type":null,"guild_security_code":null,"guild_union_name":null,"inventory_use_slot":null,"inventory_use_target":null,"inventory_use_add_points":null,"inventory_equip_slot":null,"inventory_unequip_slot":null,"friend_screen_state":null,"guild_screen_state":null,"siege_screen_state":null,"vault_money_amount":null,"inventory_move_from_slot":null,"inventory_move_to_slot":null,"command_count":1}"#
        );
    }

    #[test]
    fn command_parser_recognizes_game_shop_mu_helper_events_gens_social_vault_and_inventory_aliases(
    ) {
        assert_eq!(
            ControlCommand::parse("game-shop"),
            Some(ControlCommand::GameShop)
        );
        assert_eq!(
            ControlCommand::parse("game_shop"),
            Some(ControlCommand::GameShop)
        );
        assert_eq!(ControlCommand::GameShop.as_str(), "game-shop");
        assert_eq!(
            ControlCommand::parse("marketplace"),
            Some(ControlCommand::Marketplace)
        );
        assert_eq!(
            ControlCommand::parse("player-shop"),
            Some(ControlCommand::Marketplace)
        );
        assert_eq!(
            ControlCommand::parse("player_shop"),
            Some(ControlCommand::Marketplace)
        );
        assert_eq!(ControlCommand::Marketplace.as_str(), "marketplace");
        assert_eq!(
            ControlCommand::parse("mu-helper"),
            Some(ControlCommand::MuHelper)
        );
        assert_eq!(
            ControlCommand::parse("mu_helper"),
            Some(ControlCommand::MuHelper)
        );
        assert_eq!(ControlCommand::MuHelper.as_str(), "mu-helper");
        assert_eq!(
            ControlCommand::parse("select-character"),
            Some(ControlCommand::SelectCharacter)
        );
        assert_eq!(
            ControlCommand::parse("select_character"),
            Some(ControlCommand::SelectCharacter)
        );
        assert_eq!(ControlCommand::SelectCharacter.as_str(), "select-character");
        assert_eq!(
            ControlCommand::parse("character-create"),
            Some(ControlCommand::CharacterCreate)
        );
        assert_eq!(
            ControlCommand::parse("character_create"),
            Some(ControlCommand::CharacterCreate)
        );
        assert_eq!(ControlCommand::CharacterCreate.as_str(), "character-create");
        assert_eq!(
            ControlCommand::parse("create-character"),
            Some(ControlCommand::CreateCharacter)
        );
        assert_eq!(
            ControlCommand::parse("create_character"),
            Some(ControlCommand::CreateCharacter)
        );
        assert_eq!(ControlCommand::CreateCharacter.as_str(), "create-character");
        assert_eq!(
            ControlCommand::parse("guild-join"),
            Some(ControlCommand::GuildJoin)
        );
        assert_eq!(
            ControlCommand::parse("guild_join"),
            Some(ControlCommand::GuildJoin)
        );
        assert_eq!(ControlCommand::GuildJoin.as_str(), "guild-join");
        assert_eq!(
            ControlCommand::parse("guild-create"),
            Some(ControlCommand::GuildCreate)
        );
        assert_eq!(
            ControlCommand::parse("guild_create"),
            Some(ControlCommand::GuildCreate)
        );
        assert_eq!(ControlCommand::GuildCreate.as_str(), "guild-create");
        assert_eq!(
            ControlCommand::parse("guild-role-assign"),
            Some(ControlCommand::GuildRoleAssign)
        );
        assert_eq!(
            ControlCommand::parse("guild_role_assign"),
            Some(ControlCommand::GuildRoleAssign)
        );
        assert_eq!(
            ControlCommand::GuildRoleAssign.as_str(),
            "guild-role-assign"
        );
        assert_eq!(
            ControlCommand::parse("guild-fire"),
            Some(ControlCommand::GuildFire)
        );
        assert_eq!(
            ControlCommand::parse("guild_fire"),
            Some(ControlCommand::GuildFire)
        );
        assert_eq!(
            ControlCommand::parse("guild-kick-player"),
            Some(ControlCommand::GuildFire)
        );
        assert_eq!(ControlCommand::GuildFire.as_str(), "guild-fire");
        assert_eq!(
            ControlCommand::parse("guild-ban-union"),
            Some(ControlCommand::GuildBanUnion)
        );
        assert_eq!(
            ControlCommand::parse("guild_ban_union"),
            Some(ControlCommand::GuildBanUnion)
        );
        assert_eq!(
            ControlCommand::parse("guild-remove-alliance"),
            Some(ControlCommand::GuildBanUnion)
        );
        assert_eq!(
            ControlCommand::parse("guild_remove_alliance"),
            Some(ControlCommand::GuildBanUnion)
        );
        assert_eq!(ControlCommand::GuildBanUnion.as_str(), "guild-ban-union");
        assert_eq!(
            ControlCommand::parse("inventory-use"),
            Some(ControlCommand::InventoryUse)
        );
        assert_eq!(
            ControlCommand::parse("inventory_use"),
            Some(ControlCommand::InventoryUse)
        );
        assert_eq!(
            ControlCommand::parse("item-use"),
            Some(ControlCommand::InventoryUse)
        );
        assert_eq!(
            ControlCommand::parse("item_use"),
            Some(ControlCommand::InventoryUse)
        );
        assert_eq!(ControlCommand::InventoryUse.as_str(), "inventory-use");
        assert_eq!(
            ControlCommand::parse("inventory-equip"),
            Some(ControlCommand::InventoryEquip)
        );
        assert_eq!(
            ControlCommand::parse("inventory_equip"),
            Some(ControlCommand::InventoryEquip)
        );
        assert_eq!(
            ControlCommand::parse("item-equip"),
            Some(ControlCommand::InventoryEquip)
        );
        assert_eq!(
            ControlCommand::parse("item_equip"),
            Some(ControlCommand::InventoryEquip)
        );
        assert_eq!(ControlCommand::InventoryEquip.as_str(), "inventory-equip");
        assert_eq!(
            ControlCommand::parse("inventory-unequip"),
            Some(ControlCommand::InventoryUnequip)
        );
        assert_eq!(
            ControlCommand::parse("inventory_unequip"),
            Some(ControlCommand::InventoryUnequip)
        );
        assert_eq!(
            ControlCommand::parse("item-unequip"),
            Some(ControlCommand::InventoryUnequip)
        );
        assert_eq!(
            ControlCommand::parse("item_unequip"),
            Some(ControlCommand::InventoryUnequip)
        );
        assert_eq!(
            ControlCommand::InventoryUnequip.as_str(),
            "inventory-unequip"
        );
        assert_eq!(
            ControlCommand::parse("inventory-move"),
            Some(ControlCommand::InventoryMove)
        );
        assert_eq!(
            ControlCommand::parse("inventory_move"),
            Some(ControlCommand::InventoryMove)
        );
        assert_eq!(
            ControlCommand::parse("item-move"),
            Some(ControlCommand::InventoryMove)
        );
        assert_eq!(
            ControlCommand::parse("item_move"),
            Some(ControlCommand::InventoryMove)
        );
        assert_eq!(ControlCommand::InventoryMove.as_str(), "inventory-move");
        assert_eq!(
            ControlCommand::parse("options"),
            Some(ControlCommand::Options)
        );
        assert_eq!(ControlCommand::Options.as_str(), "options");
        assert_eq!(
            ControlCommand::parse("friend"),
            Some(ControlCommand::Friend)
        );
        assert_eq!(ControlCommand::Friend.as_str(), "friend");
        assert_eq!(ControlCommand::parse("guild"), Some(ControlCommand::Guild));
        assert_eq!(ControlCommand::Guild.as_str(), "guild");
        assert_eq!(
            ControlCommand::parse("events"),
            Some(ControlCommand::Events)
        );
        assert_eq!(ControlCommand::Events.as_str(), "events");
        assert_eq!(ControlCommand::parse("gens"), Some(ControlCommand::Gens));
        assert_eq!(ControlCommand::Gens.as_str(), "gens");
        assert_eq!(ControlCommand::parse("siege"), Some(ControlCommand::Siege));
        assert_eq!(ControlCommand::Siege.as_str(), "siege");
        assert_eq!(
            ControlCommand::parse("siege-inactive"),
            Some(ControlCommand::SiegeInactive)
        );
        assert_eq!(
            ControlCommand::parse("siege_inactive"),
            Some(ControlCommand::SiegeInactive)
        );
        assert_eq!(ControlCommand::SiegeInactive.as_str(), "siege-inactive");
        assert_eq!(
            ControlCommand::parse("siege-soldier"),
            Some(ControlCommand::SiegeSoldier)
        );
        assert_eq!(
            ControlCommand::parse("siege_soldier"),
            Some(ControlCommand::SiegeSoldier)
        );
        assert_eq!(ControlCommand::SiegeSoldier.as_str(), "siege-soldier");
        assert_eq!(
            ControlCommand::parse("siege-commander"),
            Some(ControlCommand::SiegeCommander)
        );
        assert_eq!(
            ControlCommand::parse("siege_commander"),
            Some(ControlCommand::SiegeCommander)
        );
        assert_eq!(ControlCommand::SiegeCommander.as_str(), "siege-commander");
        assert_eq!(
            ControlCommand::parse("friend-add"),
            Some(ControlCommand::FriendAdd)
        );
        assert_eq!(
            ControlCommand::parse("friend_add"),
            Some(ControlCommand::FriendAdd)
        );
        assert_eq!(ControlCommand::FriendAdd.as_str(), "friend-add");
        assert_eq!(
            ControlCommand::parse("friend-delete"),
            Some(ControlCommand::FriendDelete)
        );
        assert_eq!(
            ControlCommand::parse("friend_delete"),
            Some(ControlCommand::FriendDelete)
        );
        assert_eq!(ControlCommand::FriendDelete.as_str(), "friend-delete");
        assert_eq!(
            ControlCommand::parse("friend-roster"),
            Some(ControlCommand::FriendRoster)
        );
        assert_eq!(
            ControlCommand::parse("friend_inbox"),
            Some(ControlCommand::FriendInbox)
        );
        assert_eq!(
            ControlCommand::parse("friend-compose"),
            Some(ControlCommand::FriendCompose)
        );
        assert_eq!(
            ControlCommand::parse("friend-chat-rooms"),
            Some(ControlCommand::FriendChatRooms)
        );
        assert_eq!(
            ControlCommand::parse("guild-summary"),
            Some(ControlCommand::GuildSummary)
        );
        assert_eq!(
            ControlCommand::parse("guild_members"),
            Some(ControlCommand::GuildMembers)
        );
        assert_eq!(
            ControlCommand::parse("guild-union"),
            Some(ControlCommand::GuildUnion)
        );
        assert_eq!(
            ControlCommand::parse("guild-no-guild"),
            Some(ControlCommand::GuildNoGuild)
        );
        assert_eq!(
            ControlCommand::parse("guild-error"),
            Some(ControlCommand::GuildError)
        );
        assert_eq!(ControlCommand::parse("duel"), Some(ControlCommand::Duel));
        assert_eq!(ControlCommand::Duel.as_str(), "duel");
        assert_eq!(
            ControlCommand::parse("duel-start"),
            Some(ControlCommand::DuelStart)
        );
        assert_eq!(
            ControlCommand::parse("duel_start"),
            Some(ControlCommand::DuelStart)
        );
        assert_eq!(ControlCommand::DuelStart.as_str(), "duel-start");
        assert_eq!(
            ControlCommand::parse("duel-stop"),
            Some(ControlCommand::DuelStop)
        );
        assert_eq!(
            ControlCommand::parse("duel_stop"),
            Some(ControlCommand::DuelStop)
        );
        assert_eq!(ControlCommand::DuelStop.as_str(), "duel-stop");
        assert_eq!(
            ControlCommand::parse("duel-channel-join"),
            Some(ControlCommand::DuelChannelJoin)
        );
        assert_eq!(
            ControlCommand::parse("duel_channel_join"),
            Some(ControlCommand::DuelChannelJoin)
        );
        assert_eq!(
            ControlCommand::DuelChannelJoin.as_str(),
            "duel-channel-join"
        );
        assert_eq!(
            ControlCommand::parse("duel-channel-quit"),
            Some(ControlCommand::DuelChannelQuit)
        );
        assert_eq!(
            ControlCommand::parse("duel_channel_quit"),
            Some(ControlCommand::DuelChannelQuit)
        );
        assert_eq!(
            ControlCommand::DuelChannelQuit.as_str(),
            "duel-channel-quit"
        );
        assert_eq!(
            ControlCommand::parse("skill-targeted"),
            Some(ControlCommand::SkillTargeted)
        );
        assert_eq!(
            ControlCommand::parse("skill_targeted"),
            Some(ControlCommand::SkillTargeted)
        );
        assert_eq!(
            ControlCommand::parse("skill-target"),
            Some(ControlCommand::SkillTargeted)
        );
        assert_eq!(ControlCommand::SkillTargeted.as_str(), "skill-targeted");
        assert_eq!(
            ControlCommand::parse("vault-deposit"),
            Some(ControlCommand::VaultDeposit)
        );
        assert_eq!(
            ControlCommand::parse("vault_deposit"),
            Some(ControlCommand::VaultDeposit)
        );
        assert_eq!(
            ControlCommand::parse("vault-withdraw"),
            Some(ControlCommand::VaultWithdraw)
        );
        assert_eq!(
            ControlCommand::parse("vault_withdraw"),
            Some(ControlCommand::VaultWithdraw)
        );
        assert_eq!(ControlCommand::VaultDeposit.as_str(), "vault-deposit");
        assert_eq!(ControlCommand::VaultWithdraw.as_str(), "vault-withdraw");
    }

    #[test]
    fn snapshot_tracks_social_view_overrides() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.apply_command(ControlCommand::FriendCompose);

        assert_eq!(snapshot.ui_route, UiRoute::Friend);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(
            snapshot.friend_screen_state,
            Some(FriendScreenState::Compose)
        );
        assert_eq!(snapshot.guild_screen_state, None);

        snapshot.apply_command(ControlCommand::GuildMembers);

        assert_eq!(snapshot.ui_route, UiRoute::Guild);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(
            snapshot.friend_screen_state,
            Some(FriendScreenState::Compose)
        );
        assert_eq!(snapshot.guild_screen_state, Some(GuildScreenState::Members));

        snapshot.apply_command(ControlCommand::Friend);

        assert_eq!(snapshot.ui_route, UiRoute::Friend);
        assert_eq!(snapshot.friend_screen_state, None);
        assert_eq!(snapshot.guild_screen_state, Some(GuildScreenState::Members));

        snapshot.apply_command(ControlCommand::Guild);

        assert_eq!(snapshot.ui_route, UiRoute::Guild);
        assert_eq!(snapshot.guild_screen_state, None);
    }

    #[test]
    fn snapshot_tracks_siege_view_overrides() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.apply_command(ControlCommand::Siege);
        assert_eq!(snapshot.ui_route, UiRoute::Siege);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(
            snapshot.siege_screen_state,
            Some(mu_ui::SiegeScreenState::Observer)
        );

        snapshot.apply_command(ControlCommand::SiegeInactive);
        assert_eq!(
            snapshot.siege_screen_state,
            Some(mu_ui::SiegeScreenState::Inactive)
        );

        snapshot.apply_command(ControlCommand::SiegeSoldier);
        assert_eq!(
            snapshot.siege_screen_state,
            Some(mu_ui::SiegeScreenState::Soldier)
        );

        snapshot.apply_command(ControlCommand::SiegeCommander);
        assert_eq!(
            snapshot.siege_screen_state,
            Some(mu_ui::SiegeScreenState::Commander)
        );

        snapshot.apply_command(ControlCommand::Chat);
        assert_eq!(snapshot.siege_screen_state, None);
    }

    #[test]
    fn snapshot_tracks_inventory_move_payload() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.inventory_move_from_slot = Some(0x12);
        snapshot.inventory_move_to_slot = Some(0x34);
        snapshot.apply_command(ControlCommand::InventoryMove);

        assert_eq!(snapshot.ui_route, UiRoute::Inventory);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.inventory_move_from_slot, Some(0x12));
        assert_eq!(snapshot.inventory_move_to_slot, Some(0x34));
    }

    #[test]
    fn snapshot_tracks_inventory_item_action_payloads() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.inventory_use_slot = Some(7);
        snapshot.inventory_use_target = Some(3);
        snapshot.inventory_use_add_points = Some(false);
        snapshot.inventory_equip_slot = Some(9);
        snapshot.inventory_unequip_slot = Some(2);
        snapshot.apply_command(ControlCommand::InventoryUse);

        let body = snapshot.to_json();
        assert!(body.contains(r#""inventory_use_slot":7"#));
        assert!(body.contains(r#""inventory_use_target":3"#));
        assert!(body.contains(r#""inventory_use_add_points":false"#));
        assert!(body.contains(r#""inventory_equip_slot":9"#));
        assert!(body.contains(r#""inventory_unequip_slot":2"#));
        assert!(body.contains(r#""ui_route":"inventory""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));
    }

    #[test]
    fn snapshot_tracks_guild_join_payload() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.guild_master_player_id = Some(0x1234);
        snapshot.apply_command(ControlCommand::GuildJoin);

        assert_eq!(snapshot.ui_route, UiRoute::Guild);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.guild_master_player_id, Some(0x1234));
        assert_eq!(snapshot.guild_screen_state, None);
    }

    #[test]
    fn snapshot_tracks_guild_create_payload() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.guild_create_name = Some("Guild".to_string());
        snapshot.guild_create_emblem = Some([1u8; 32]);
        snapshot.apply_command(ControlCommand::GuildCreate);

        assert_eq!(snapshot.ui_route, UiRoute::Guild);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.guild_create_name.as_deref(), Some("Guild"));
        assert_eq!(snapshot.guild_create_emblem, Some([1u8; 32]));
        assert_eq!(snapshot.guild_screen_state, None);

        let body = snapshot.to_json();
        assert!(body.contains(r#""guild_create_name":"Guild""#));
        assert!(body.contains(&format!(r#""guild_create_emblem":"{}""#, "01".repeat(32))));
    }

    #[test]
    fn snapshot_tracks_duel_start_payload() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.duel_player_id = Some(0x1234);
        snapshot.duel_player_name = Some("Astra".to_string());
        snapshot.apply_command(ControlCommand::DuelStart);

        assert_eq!(snapshot.ui_route, UiRoute::Duel);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.duel_player_id, Some(0x1234));
        assert_eq!(snapshot.duel_player_name.as_deref(), Some("Astra"));

        let body = snapshot.to_json();
        assert!(body.contains(r#""duel_player_id":4660"#));
        assert!(body.contains(r#""duel_player_name":"Astra""#));
    }

    #[test]
    fn control_http_route_accepts_duel_start_payload() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));
        let shutdown = Arc::new(AtomicBool::new(false));

        let response = route_request(
            HttpRequest {
                method: "POST".to_string(),
                path: "/command".to_string(),
                query: "name=duel-start&player_id=4660&player_name=Astra".to_string(),
                body: String::new(),
            },
            &snapshot,
            &shutdown,
        );

        assert_eq!(response.status, 200);
        let snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
        assert_eq!(snapshot.last_command, Some(ControlCommand::DuelStart));
        assert_eq!(snapshot.duel_player_id, Some(4660));
        assert_eq!(snapshot.duel_player_name.as_deref(), Some("Astra"));
        assert_eq!(snapshot.ui_route, UiRoute::Duel);
    }

    #[test]
    fn control_http_route_accepts_duel_channel_join_payload() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));
        let shutdown = Arc::new(AtomicBool::new(false));

        let response = route_request(
            HttpRequest {
                method: "POST".to_string(),
                path: "/command".to_string(),
                query: "name=duel-channel-join&channel_id=7".to_string(),
                body: String::new(),
            },
            &snapshot,
            &shutdown,
        );

        assert_eq!(response.status, 200);
        let snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
        assert_eq!(snapshot.last_command, Some(ControlCommand::DuelChannelJoin));
        assert_eq!(snapshot.duel_channel_id, Some(7));
        assert_eq!(snapshot.ui_route, UiRoute::Duel);
    }

    #[test]
    fn control_http_route_accepts_marketplace_command() {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));
        let shutdown = Arc::new(AtomicBool::new(false));

        let response = route_request(
            HttpRequest {
                method: "POST".to_string(),
                path: "/command".to_string(),
                query: "name=marketplace".to_string(),
                body: String::new(),
            },
            &snapshot,
            &shutdown,
        );

        assert_eq!(response.status, 200);
        let snapshot = snapshot.lock().expect("control snapshot mutex poisoned");
        assert_eq!(snapshot.last_command, Some(ControlCommand::Marketplace));
        assert_eq!(snapshot.ui_route, UiRoute::Marketplace);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
    }

    #[test]
    fn duel_channel_join_requests_require_a_payload() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=duel-channel-join HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing duel channel payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=duel-channel-join&channel_id=bad HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing duel channel payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=duel-channel-join&channel_id=7 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""duel_channel_id":7"#));
        assert!(body.contains(r#""ui_route":"duel""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""duel_channel_id":7"#));
        assert!(state_body.contains(r#""command_count":1"#));
    }

    #[test]
    fn skill_targeted_requests_require_a_payload() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=skill-targeted HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing skill targeted payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=skill-targeted&skill_id=bad&target_id=7 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing skill targeted payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=skill-targeted&skill_id=6&target_id=77 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""skill_id":6"#));
        assert!(body.contains(r#""skill_target_id":77"#));
        assert!(body.contains(r#""ui_route":"world""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""skill_id":6"#));
        assert!(state_body.contains(r#""skill_target_id":77"#));
        assert!(state_body.contains(r#""command_count":1"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn snapshot_tracks_guild_fire_payload() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.guild_player_name = Some("Blade".to_string());
        snapshot.guild_security_code = Some("1234".to_string());
        snapshot.apply_command(ControlCommand::GuildFire);

        assert_eq!(snapshot.ui_route, UiRoute::Guild);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.guild_player_name.as_deref(), Some("Blade"));
        assert_eq!(snapshot.guild_security_code.as_deref(), Some("1234"));
        assert_eq!(snapshot.guild_screen_state, Some(GuildScreenState::Members));
    }

    #[test]
    fn snapshot_tracks_guild_ban_union_payload() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.guild_union_name = Some("Alliance".to_string());
        snapshot.apply_command(ControlCommand::GuildBanUnion);

        assert_eq!(snapshot.ui_route, UiRoute::Guild);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.guild_union_name.as_deref(), Some("Alliance"));
        assert_eq!(snapshot.guild_screen_state, Some(GuildScreenState::Union));
    }

    #[test]
    fn snapshot_tracks_skill_targeted_payload() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        snapshot.skill_id = Some(6);
        snapshot.skill_target_id = Some(77);
        snapshot.apply_command(ControlCommand::SkillTargeted);

        assert_eq!(snapshot.ui_route, UiRoute::World);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.skill_id, Some(6));
        assert_eq!(snapshot.skill_target_id, Some(77));
    }

    #[test]
    fn snapshot_tracks_route_and_session_state() {
        let mut snapshot = ControlSnapshot::new(AppState::ReadyForLogin);

        assert_eq!(snapshot.ui_route, UiRoute::Login);
        assert_eq!(snapshot.session_phase, SessionPhase::ReadyForLogin);

        snapshot.apply_command(ControlCommand::LoginSuccess);

        assert_eq!(snapshot.ui_route, UiRoute::CharacterSelect);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.selected_character_name = Some("Astra".to_string());
        snapshot.apply_command(ControlCommand::SelectCharacter);

        assert_eq!(snapshot.ui_route, UiRoute::CharacterSelect);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.selected_character_name.as_deref(), Some("Astra"));

        snapshot.apply_command(ControlCommand::CharacterCreate);

        assert_eq!(snapshot.ui_route, UiRoute::CharacterCreate);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
        assert_eq!(snapshot.selected_character_name, None);

        snapshot.apply_command(ControlCommand::Chat);

        assert_eq!(snapshot.ui_route, UiRoute::Chat);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Options);

        assert_eq!(snapshot.ui_route, UiRoute::Options);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Npc);

        assert_eq!(snapshot.ui_route, UiRoute::Npc);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Shop);

        assert_eq!(snapshot.ui_route, UiRoute::Shop);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::GameShop);

        assert_eq!(snapshot.ui_route, UiRoute::GameShop);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Trade);

        assert_eq!(snapshot.ui_route, UiRoute::Trade);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Marketplace);

        assert_eq!(snapshot.ui_route, UiRoute::Marketplace);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Party);

        assert_eq!(snapshot.ui_route, UiRoute::Party);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Gate);

        assert_eq!(snapshot.ui_route, UiRoute::Gate);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Friend);

        assert_eq!(snapshot.ui_route, UiRoute::Friend);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Guild);

        assert_eq!(snapshot.ui_route, UiRoute::Guild);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Duel);

        assert_eq!(snapshot.ui_route, UiRoute::Duel);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::Quests);

        assert_eq!(snapshot.ui_route, UiRoute::Quests);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);

        snapshot.apply_command(ControlCommand::MuHelper);

        assert_eq!(snapshot.ui_route, UiRoute::MuHelper);
        assert_eq!(snapshot.session_phase, SessionPhase::LoggedIn);
    }

    #[test]
    fn server_exposes_state_and_applies_commands() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::Boot).unwrap();
        let address = handle.address();

        let (_, body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""state":"boot""#));
        assert!(body.contains(r#""ui_route":"boot""#));
        assert!(body.contains(r#""session_phase":"ready-for-login""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=ready-for-login HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""state":"ready-for-login""#));
        assert!(body.contains(r#""last_command":"ready-for-login""#));
        assert!(body.contains(r#""ui_route":"login""#));
        assert!(body.contains(r#""session_phase":"ready-for-login""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=login-success HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"character-select""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));
        assert!(body.contains(r#""selected_character_name":null"#));

        let (_, body) = send_request(
            address,
            "POST /command?name=character-create HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"character-create""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));
        assert!(body.contains(r#""selected_character_name":null"#));

        let (_, body) = send_request(
            address,
            "POST /command?name=create-character&character=Astra HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"character-create""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));
        assert!(body.contains(r#""last_command":"create-character""#));
        assert!(body.contains(r#""selected_character_name":"Astra""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=select-character HTTP/1.1\r\nHost: localhost\r\nContent-Length: 5\r\nConnection: close\r\n\r\nAstra",
        );
        assert!(body.contains(r#""ui_route":"character-select""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));
        assert!(body.contains(r#""selected_character_name":"Astra""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=chat HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"chat""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=options HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"options""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=npc HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"npc""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=shop HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"shop""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=game-shop HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"game-shop""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=trade HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"trade""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=vault-deposit&amount=250 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"inventory""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));
        assert!(body.contains(r#""vault_money_amount":250"#));

        let (_, body) = send_request(
            address,
            "POST /command?name=vault-withdraw&amount=125 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"inventory""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));
        assert!(body.contains(r#""vault_money_amount":125"#));

        let (_, body) = send_request(
            address,
            "POST /command?name=inventory-move&from_slot=0&to_slot=1 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"inventory""#));
        assert!(body.contains(r#""inventory_move_from_slot":0"#));
        assert!(body.contains(r#""inventory_move_to_slot":1"#));

        let (_, body) = send_request(
            address,
            "POST /command?name=quests HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"quests""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=party HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"party""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=gate HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"gate""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=siege HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"siege""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));
        assert!(body.contains(r#""siege_screen_state":"observer""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=events HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"events""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=gens HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"hud""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=friend-compose HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"friend""#));
        assert!(body.contains(r#""friend_screen_state":"compose""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=friend-add&friend=Astra HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"friend""#));
        assert!(body.contains(r#""friend_name":"Astra""#));
        assert!(body.contains(r#""last_command":"friend-add""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=guild-members HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"guild""#));
        assert!(body.contains(r#""guild_screen_state":"members""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=mu-helper HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"mu-helper""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=duel HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""ui_route":"duel""#));
        assert!(body.contains(r#""session_phase":"logged-in""#));

        let (_, body) = send_request(
            address,
            "POST /command?name=exit HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(body.contains(r#""state":"exit""#));

        let final_snapshot = handle.join().unwrap();
        assert_eq!(final_snapshot.state, AppState::Exit);
        assert_eq!(final_snapshot.command_count, 26);
    }

    #[test]
    fn create_character_rejects_short_names() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=create-character&character=Aba HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );

        assert!(head.contains("400 Bad Request"));
        assert!(body.contains("character name must be at least 4 characters"));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""ui_route":"login""#));
        assert!(state_body.contains(r#""selected_character_name":null"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn friend_actions_require_a_name() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=friend-add HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing friend name""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=friend-delete HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing friend name""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=friend-add&friend= HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing friend name""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=friend-delete HTTP/1.1\r\nHost: localhost\r\nContent-Length: 3\r\nConnection: close\r\n\r\n   ",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing friend name""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""friend_name":null"#));
        assert!(state_body.contains(r#""command_count":0"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn vault_money_actions_require_an_amount() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=vault-deposit HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing vault amount""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=vault-withdraw&amount=bad HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing vault amount""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=vault-deposit&amount=0 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"vault amount must be positive""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""vault_money_amount":null"#));
        assert!(state_body.contains(r#""command_count":0"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn inventory_move_actions_require_a_payload() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=inventory-move HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing inventory move payload""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""inventory_move_from_slot":null"#));
        assert!(state_body.contains(r#""inventory_move_to_slot":null"#));
        assert!(state_body.contains(r#""command_count":0"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn inventory_item_actions_require_payloads() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=inventory-use HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing inventory use payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=inventory-equip HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing inventory equip payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=inventory-unequip HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing inventory unequip payload""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""inventory_use_slot":null"#));
        assert!(state_body.contains(r#""inventory_equip_slot":null"#));
        assert!(state_body.contains(r#""inventory_unequip_slot":null"#));
        assert!(state_body.contains(r#""command_count":0"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn inventory_item_actions_accept_payloads() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=inventory-use&item_slot=7&target=3&fruit=false HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""last_command":"inventory-use""#));
        assert!(body.contains(r#""inventory_use_slot":7"#));
        assert!(body.contains(r#""inventory_use_target":3"#));
        assert!(body.contains(r#""inventory_use_add_points":false"#));

        let (head, body) = send_request(
            address,
            "POST /command?name=inventory-equip&slot=9 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""last_command":"inventory-equip""#));
        assert!(body.contains(r#""inventory_equip_slot":9"#));

        let (head, body) = send_request(
            address,
            "POST /command?name=inventory-unequip&item_slot=2 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""last_command":"inventory-unequip""#));
        assert!(body.contains(r#""inventory_unequip_slot":2"#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""state":"ready-for-login""#));
        assert!(state_body.contains(r#""ui_route":"inventory""#));
        assert!(state_body.contains(r#""session_phase":"logged-in""#));
        assert!(state_body.contains(r#""inventory_use_slot":7"#));
        assert!(state_body.contains(r#""inventory_equip_slot":9"#));
        assert!(state_body.contains(r#""inventory_unequip_slot":2"#));
        assert!(state_body.contains(r#""command_count":3"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn guild_join_requests_require_a_payload() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-join HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild join payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-join&master_id=bad HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild join payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-join&master_id=4660 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""guild_master_player_id":4660"#));
        assert!(body.contains(r#""guild_screen_state":null"#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""guild_master_player_id":4660"#));
        assert!(state_body.contains(r#""command_count":1"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn guild_create_requests_require_a_payload() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-create HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild create payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-create&guild_name=Guild&guild_emblem=bad HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild create payload""#));

        let emblem = "01".repeat(32);
        let (head, body) = send_request(
            address,
            format!(
                "POST /command?name=guild-create&guild_name=Guild&guild_emblem={emblem} HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .as_str(),
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""guild_create_name":"Guild""#));
        assert!(body.contains(&format!(r#""guild_create_emblem":"{}""#, emblem)));
        assert!(body.contains(r#""guild_screen_state":null"#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""guild_create_name":"Guild""#));
        assert!(state_body.contains(&format!(r#""guild_create_emblem":"{}""#, emblem)));
        assert!(state_body.contains(r#""command_count":1"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn guild_role_assign_requests_require_a_payload() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-role-assign HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild role assignment payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-role-assign&player=Astra&role=bad&type=1 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild role assignment payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-role-assign&player=Astra&role=64&type=2 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""guild_player_name":"Astra""#));
        assert!(body.contains(r#""guild_role":64"#));
        assert!(body.contains(r#""guild_assignment_type":2"#));
        assert!(body.contains(r#""guild_screen_state":"members""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""guild_player_name":"Astra""#));
        assert!(state_body.contains(r#""guild_role":64"#));
        assert!(state_body.contains(r#""guild_assignment_type":2"#));
        assert!(state_body.contains(r#""command_count":1"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn guild_fire_requests_require_a_payload() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-fire HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild fire payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-fire&player=Blade&security_code= HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild fire payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-fire&player=Blade&security_code=1234 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""guild_player_name":"Blade""#));
        assert!(body.contains(r#""guild_security_code":"1234""#));
        assert!(body.contains(r#""guild_screen_state":"members""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""guild_player_name":"Blade""#));
        assert!(state_body.contains(r#""guild_security_code":"1234""#));
        assert!(state_body.contains(r#""command_count":1"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn guild_ban_union_requests_require_a_payload() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-ban-union HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild ban union payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-ban-union&guild_name= HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild ban union payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-ban-union HTTP/1.1\r\nHost: localhost\r\nContent-Length: 11\r\nConnection: close\r\n\r\nguild_name=",
        );
        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing guild ban union payload""#));

        let (head, body) = send_request(
            address,
            "POST /command?name=guild-ban-union&guild_name=Alliance HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        assert!(head.contains("200 OK"));
        assert!(body.contains(r#""guild_union_name":"Alliance""#));
        assert!(body.contains(r#""guild_screen_state":"union""#));

        let (_, state_body) = send_request(
            address,
            "GET /state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(state_body.contains(r#""guild_union_name":"Alliance""#));
        assert!(state_body.contains(r#""command_count":1"#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn select_character_requires_a_name() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::ReadyForLogin).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=select-character HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );

        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"missing character name""#));

        handle.request_shutdown();
        let final_snapshot = handle.join().unwrap();
        assert_eq!(final_snapshot.command_count, 0);
    }

    #[test]
    fn unknown_command_returns_error() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::Boot).unwrap();
        let address = handle.address();

        let (head, body) = send_request(
            address,
            "POST /command?name=unknown HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );

        assert!(head.contains("400 Bad Request"));
        assert!(body.contains(r#""error":"unknown command""#));

        handle.request_shutdown();
        let _ = handle.join();
    }

    #[test]
    fn shutdown_request_joins_cleanly() {
        let handle = spawn("127.0.0.1:0".parse().unwrap(), AppState::Boot).unwrap();

        handle.request_shutdown();

        let final_snapshot = handle.join().unwrap();
        assert_eq!(final_snapshot.state, AppState::Boot);
        assert_eq!(final_snapshot.command_count, 0);
    }

    #[test]
    fn shutdown_request_joins_cleanly_on_wildcard_bind() {
        let handle = spawn("0.0.0.0:0".parse().unwrap(), AppState::Boot).unwrap();

        handle.request_shutdown();

        let final_snapshot = handle.join().unwrap();
        assert_eq!(final_snapshot.state, AppState::Boot);
        assert_eq!(final_snapshot.command_count, 0);
    }
}
