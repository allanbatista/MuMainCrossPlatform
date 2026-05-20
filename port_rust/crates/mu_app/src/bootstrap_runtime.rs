use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, IntoScheduleConfigs, Res, ResMut, Resource, Startup, Update};
use mu_network::{Session, SessionEvent};
use mu_protocol::{decode_packet, PacketFrame};
use mu_ui::{UiRoute, UiShellState};

use crate::{ClientRuntime, GraphicalRuntimeConfig, SessionState};

const SESSION_CONNECT_TIMEOUT: Duration = Duration::from_millis(250);
const SESSION_READ_TIMEOUT: Duration = Duration::from_millis(250);

#[derive(Debug)]
pub(crate) enum BootstrapSignal {
    Session(SessionEvent),
    ServerList,
    CharacterList,
    Logout(u8),
    JoinMap(u8),
    Error(String),
}

#[derive(Debug, Resource)]
pub struct BootstrapRuntime {
    inbox: Mutex<Receiver<BootstrapSignal>>,
    pending_world_map: Option<u8>,
    last_error: Option<String>,
}

impl BootstrapRuntime {
    pub(crate) fn new(receiver: Receiver<BootstrapSignal>) -> Self {
        Self {
            inbox: Mutex::new(receiver),
            pending_world_map: None,
            last_error: None,
        }
    }

    pub(crate) fn idle() -> Self {
        let (_sender, receiver) = mpsc::channel();
        Self::new(receiver)
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

    let (sender, receiver) = mpsc::channel();
    spawn_bootstrap_worker(address, sender);
    commands.insert_resource(BootstrapRuntime::new(receiver));
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
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::ServerSelect);
        }
        BootstrapSignal::CharacterList => {
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::CharacterSelect);
        }
        BootstrapSignal::Logout(kind) => apply_logout(kind, bootstrap, ui_shell),
        BootstrapSignal::JoinMap(map) => {
            bootstrap.pending_world_map = Some(map);
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::Loading);
        }
        BootstrapSignal::Error(message) => {
            bootstrap.pending_world_map = None;
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
            bootstrap.last_error = None;
            ui_shell.set_route(UiRoute::CharacterSelect);
        }
        SessionEvent::LoginFailure => {
            session_state.apply_event(event);
            bootstrap.pending_world_map = None;
            bootstrap.last_error = Some("login failed".to_string());
            ui_shell.set_route(UiRoute::Login);
        }
        SessionEvent::Logout => {
            session_state.apply_event(event);
        }
        SessionEvent::Disconnect => {
            if bootstrap.pending_world_map.is_some() || client_runtime.world_ready() {
                return;
            }

            session_state.apply_event(event);
            bootstrap.pending_world_map = None;
            bootstrap.last_error = Some("connection lost".to_string());
            ui_shell.set_route(UiRoute::Error);
        }
    }
}

fn apply_logout(kind: u8, bootstrap: &mut BootstrapRuntime, ui_shell: &mut UiShellState) {
    bootstrap.pending_world_map = None;
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

fn spawn_bootstrap_worker(address: SocketAddr, sender: Sender<BootstrapSignal>) {
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

                let packet = match session.receive().await {
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
        });
    });
}

fn classify_bootstrap_packet(frame: &PacketFrame<'_>) -> Option<BootstrapSignal> {
    match (frame.headcode, frame.subcode) {
        (0xF4, 0x06) => Some(BootstrapSignal::ServerList),
        (0xF4, 0x05) => Some(BootstrapSignal::Error("server is busy".to_string())),
        (0xF3, 0x00) => Some(BootstrapSignal::CharacterList),
        (0xF3, 0x03) => frame.payload.get(2).copied().map(BootstrapSignal::JoinMap),
        (0xF1, 0x02) => frame.payload.first().copied().map(BootstrapSignal::Logout),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        apply_bootstrap_signal, finish_world_bootstrap, BootstrapRuntime, BootstrapSignal,
    };
    use crate::bootstrap_runtime::spawn_bootstrap_worker;
    use crate::{ClientRuntime, GraphicalRuntimeConfig, SessionState};
    use camino::Utf8PathBuf;
    use mu_network::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::encode_packet;
    use mu_protocol::session::{character_list_extended, game_server_entered, CharacterListEntry};
    use mu_ui::{UiRoute, UiShellState};
    use std::time::Duration;

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

        apply_bootstrap_signal(
            BootstrapSignal::CharacterList,
            &mut bootstrap,
            &mut session_state,
            &mut ui_shell,
            &mut client_runtime,
        );
        assert_eq!(ui_shell.current(), UiRoute::CharacterSelect);

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
                    .send_packet(character_list.clone())
                    .send_packet(join_map.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut bootstrap = {
            let (sender, receiver) = std::sync::mpsc::channel();
            spawn_bootstrap_worker(server.address(), sender);
            BootstrapRuntime::new(receiver)
        };
        let config = config(Some(server.address().to_string()));
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

            finish_world_bootstrap(&mut bootstrap, &mut client_runtime, &config, &mut ui_shell);

            if ui_shell.current() == UiRoute::World {
                break;
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        assert_eq!(ui_shell.current(), UiRoute::World);
        assert!(client_runtime.world_ready());
        assert_eq!(session_state.phase(), mu_network::SessionPhase::LoggedIn);
        assert!(bootstrap.last_error().is_none());

        server.finish().await.unwrap();
    }
}
