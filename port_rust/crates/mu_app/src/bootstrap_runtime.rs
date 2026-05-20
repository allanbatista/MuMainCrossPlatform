use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, IntoScheduleConfigs, Res, ResMut, Resource, Startup, Update};
use camino::Utf8Path;
use mu_gameplay::MovementCommand;
use mu_network::{Session, SessionEvent};
use mu_protocol::chat::public_chat_message;
use mu_protocol::guild::guild_list_request;
use mu_protocol::login::{request_character_list, select_character};
use mu_protocol::movement::{decode_movement_update, walk_request, MovementUpdate};
use mu_protocol::social::friend_list_request;
use mu_protocol::{decode_packet, PacketFrame};
use mu_ui::{
    character_select_screen, CharacterSelectCharacter, CharacterSelectScreenState, UiRoute,
    UiShellState,
};
use tokio::sync::mpsc as tokio_mpsc;

use crate::{ClientRuntime, Config, GraphicalRuntimeConfig, SessionState};

const SESSION_CONNECT_TIMEOUT: Duration = Duration::from_millis(250);
const SESSION_READ_TIMEOUT: Duration = Duration::from_millis(250);

#[derive(Debug)]
pub(crate) enum BootstrapSignal {
    Session(SessionEvent),
    ServerList,
    CharacterList,
    Movement(MovementUpdate),
    Logout(u8),
    JoinMap(u8),
    Error(String),
}

#[derive(Debug)]
enum BootstrapCommand {
    Walk(MovementCommand),
    Chat { sender: String, message: String },
    SelectCharacter(String),
    FriendListRequest,
    GuildListRequest,
}

#[derive(Debug, Resource)]
pub struct BootstrapRuntime {
    inbox: Mutex<Receiver<BootstrapSignal>>,
    command_sender: Option<tokio_mpsc::UnboundedSender<BootstrapCommand>>,
    pending_world_map: Option<u8>,
    character_list_ready: bool,
    character_select_index: Option<usize>,
    last_error: Option<String>,
}

impl BootstrapRuntime {
    fn new(
        receiver: Receiver<BootstrapSignal>,
        command_sender: Option<tokio_mpsc::UnboundedSender<BootstrapCommand>>,
    ) -> Self {
        Self {
            inbox: Mutex::new(receiver),
            command_sender,
            pending_world_map: None,
            character_list_ready: false,
            character_select_index: None,
            last_error: None,
        }
    }

    pub(crate) fn idle() -> Self {
        let (_sender, receiver) = mpsc::channel();
        Self::new(receiver, None)
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

    pub(crate) fn queue_character_select_request(&self, character_name: impl Into<String>) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::SelectCharacter(character_name.into()))
            .is_ok()
    }

    pub(crate) fn queue_friend_list_request(&self) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::FriendListRequest)
            .is_ok()
    }

    pub(crate) fn queue_guild_list_request(&self) -> bool {
        let Some(command_sender) = self.command_sender.as_ref() else {
            return false;
        };

        command_sender
            .send(BootstrapCommand::GuildListRequest)
            .is_ok()
    }

    pub(crate) fn character_list_ready(&self) -> bool {
        self.character_list_ready
    }

    pub(crate) fn set_character_list_ready(&mut self, ready: bool) {
        self.character_list_ready = ready;
    }

    pub(crate) fn clear_character_select_selection(&mut self) {
        self.character_select_index = None;
    }

    pub(crate) fn character_select_index(&self) -> Option<usize> {
        self.character_select_index
    }

    pub(crate) fn set_character_select_index(&mut self, index: Option<usize>) {
        self.character_select_index = index;
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
) {
    let signals = bootstrap.drain_signals();

    for signal in signals {
        apply_bootstrap_signal(signal, bootstrap, session_state, ui_shell, client_runtime);
    }
}

pub(crate) fn poll_bootstrap_signals_system(
    mut bootstrap: ResMut<BootstrapRuntime>,
    mut session_state: ResMut<SessionState>,
    mut ui_shell: ResMut<UiShellState>,
    mut client_runtime: ResMut<ClientRuntime>,
) {
    poll_bootstrap_signals(
        &mut bootstrap,
        &mut session_state,
        &mut ui_shell,
        &mut client_runtime,
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

    match client_runtime.load_world_from_assets(asset_root, u32::from(world_map)) {
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

fn apply_bootstrap_signal(
    signal: BootstrapSignal,
    bootstrap: &mut BootstrapRuntime,
    session_state: &mut SessionState,
    ui_shell: &mut UiShellState,
    client_runtime: &mut ClientRuntime,
) {
    match signal {
        BootstrapSignal::Session(event) => {
            apply_session_event(event, bootstrap, session_state, ui_shell, client_runtime)
        }
        BootstrapSignal::ServerList => {
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::ServerSelect);
        }
        BootstrapSignal::CharacterList => {
            bootstrap.set_character_list_ready(true);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::CharacterSelect);
        }
        BootstrapSignal::Movement(update) => apply_movement_update(update, client_runtime),
        BootstrapSignal::Logout(kind) => apply_logout(kind, bootstrap, ui_shell),
        BootstrapSignal::JoinMap(map) => {
            bootstrap.pending_world_map = Some(map);
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::Loading);
        }
        BootstrapSignal::Error(message) => {
            bootstrap.pending_world_map = None;
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = Some(message);
            ui_shell.set_route(UiRoute::Error);
        }
    }
}

fn apply_session_event(
    event: SessionEvent,
    bootstrap: &mut BootstrapRuntime,
    session_state: &mut SessionState,
    ui_shell: &mut UiShellState,
    client_runtime: &mut ClientRuntime,
) {
    match event {
        SessionEvent::LoginSuccess => {
            session_state.apply_event(event);
            bootstrap.pending_world_map = None;
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::CharacterSelect);
        }
        SessionEvent::LoginFailure => {
            session_state.apply_event(event);
            bootstrap.pending_world_map = None;
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = Some("login failed".to_string());
            ui_shell.set_route(UiRoute::Login);
        }
        SessionEvent::Logout => {
            session_state.apply_event(event);
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
        }
        SessionEvent::Disconnect => {
            let pending_world_map = bootstrap.pending_world_map.is_some();
            let world_ready = client_runtime.world_ready();
            session_state.apply_event(event);

            if pending_world_map {
                return;
            }

            bootstrap.pending_world_map = None;
            bootstrap.set_character_list_ready(false);
            bootstrap.clear_character_select_selection();
            bootstrap.last_error = Some("connection lost".to_string());

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

fn apply_logout(kind: u8, bootstrap: &mut BootstrapRuntime, ui_shell: &mut UiShellState) {
    bootstrap.pending_world_map = None;
    bootstrap.set_character_list_ready(false);
    bootstrap.clear_character_select_selection();
    bootstrap.last_error = None;

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
        (0xF4, 0x06) => Some(BootstrapSignal::ServerList),
        (0xF4, 0x05) => Some(BootstrapSignal::Error("server is busy".to_string())),
        (0xF3, 0x00) => Some(BootstrapSignal::CharacterList),
        (0xF3, 0x03) => frame.payload.get(2).copied().map(BootstrapSignal::JoinMap),
        (0xF1, 0x02) => frame.payload.first().copied().map(BootstrapSignal::Logout),
        _ => None,
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
        BootstrapCommand::FriendListRequest => {
            let packet = friend_list_request().map_err(|error| error.to_string())?;

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
        apply_bootstrap_signal, apply_character_select_input, finish_world_bootstrap,
        legacy_language_byte, BootstrapCommand, BootstrapRuntime, BootstrapSignal,
    };
    use crate::bootstrap_runtime::spawn_bootstrap_worker;
    use crate::{ClientRuntime, GraphicalRuntimeConfig, SessionState};
    use bevy::input::keyboard::KeyCode;
    use bevy::input::ButtonInput;
    use camino::Utf8PathBuf;
    use mu_gameplay::MovementCommand;
    use mu_network::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::chat::public_chat_message;
    use mu_protocol::encode_packet;
    use mu_protocol::guild::guild_list_request;
    use mu_protocol::login::{request_character_list, select_character};
    use mu_protocol::movement::{encode_move_position_update, walk_request};
    use mu_protocol::session::{character_list_extended, game_server_entered, CharacterListEntry};
    use mu_protocol::social::friend_list_request;
    use mu_ui::{UiRoute, UiShellState};
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
        assert!(bootstrap.last_error().is_none());
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
        assert!(bootstrap.queue_guild_list_request());

        match command_receiver
            .try_recv()
            .expect("friend list command missing")
        {
            BootstrapCommand::FriendListRequest => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }

        match command_receiver
            .try_recv()
            .expect("guild list command missing")
        {
            BootstrapCommand::GuildListRequest => {}
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
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
