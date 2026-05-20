use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::{AppState, SessionPhase};
use bevy::prelude::Resource;
use mu_ui::{FriendScreenState, GuildScreenState, UiRoute, CHARACTER_CREATE_NAME_MIN_LENGTH};

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
    Party,
    Gate,
    Friend,
    Guild,
    FriendAdd,
    FriendDelete,
    GuildJoin,
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
    Duel,
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
            Self::Party => "party",
            Self::Gate => "gate",
            Self::Friend => "friend",
            Self::Guild => "guild",
            Self::FriendAdd => "friend-add",
            Self::FriendDelete => "friend-delete",
            Self::GuildJoin => "guild-join",
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
            Self::Duel => "duel",
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
            "party" => Some(Self::Party),
            "gate" => Some(Self::Gate),
            "friend" => Some(Self::Friend),
            "guild" => Some(Self::Guild),
            "friend-add" | "friend_add" => Some(Self::FriendAdd),
            "friend-delete" | "friend_delete" => Some(Self::FriendDelete),
            "guild-join" | "guild_join" => Some(Self::GuildJoin),
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
            "duel" => Some(Self::Duel),
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
    pub guild_role: Option<u8>,
    pub guild_assignment_type: Option<u8>,
    pub friend_screen_state: Option<FriendScreenState>,
    pub guild_screen_state: Option<GuildScreenState>,
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
            guild_role: None,
            guild_assignment_type: None,
            friend_screen_state: None,
            guild_screen_state: None,
            command_count: 0,
        }
    }

    pub fn apply_command(&mut self, command: ControlCommand) -> bool {
        self.last_command = Some(command);
        self.command_count = self.command_count.saturating_add(1);

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
            ControlCommand::Duel => {
                self.state = AppState::ReadyForLogin;
                self.ui_route = UiRoute::Duel;
                self.session_phase = SessionPhase::LoggedIn;
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
        let guild_role = self
            .guild_role
            .map(|value| value.to_string())
            .unwrap_or_else(|| "null".to_string());
        let guild_assignment_type = self
            .guild_assignment_type
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

        format!(
            "{{\"state\":\"{}\",\"ui_route\":\"{}\",\"session_phase\":\"{}\",\"last_command\":{},\"selected_character_name\":{},\"friend_name\":{},\"guild_master_player_id\":{},\"guild_player_name\":{},\"guild_role\":{},\"guild_assignment_type\":{},\"friend_screen_state\":{},\"guild_screen_state\":{},\"command_count\":{}}}",
            self.state.as_str(),
            self.ui_route.slug(),
            self.session_phase.as_str(),
            last_command,
            selected_character_name,
            friend_name,
            guild_master_player_id,
            guild_player_name,
            guild_role,
            guild_assignment_type,
            friend_screen_state,
            guild_screen_state,
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
    use super::{spawn, ControlCommand, ControlSnapshot};
    use crate::{AppState, SessionPhase};
    use mu_ui::{FriendScreenState, GuildScreenState, UiRoute};
    use std::io::{Read, Write};
    use std::net::TcpStream;

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
            r#"{"state":"ready-for-login","ui_route":"login","session_phase":"ready-for-login","last_command":"ping","selected_character_name":null,"friend_name":null,"guild_master_player_id":null,"guild_player_name":null,"guild_role":null,"guild_assignment_type":null,"friend_screen_state":null,"guild_screen_state":null,"command_count":1}"#
        );
    }

    #[test]
    fn command_parser_recognizes_game_shop_mu_helper_and_social_aliases() {
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
        assert_eq!(final_snapshot.command_count, 20);
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
