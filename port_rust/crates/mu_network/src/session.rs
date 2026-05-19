use std::net::SocketAddr;
use std::time::Duration;

use crate::{Client, ClientError};
use mu_protocol::PacketFrame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionPhase {
    ReadyForLogin,
    LoggedIn,
    Disconnected,
}

impl SessionPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyForLogin => "ready-for-login",
            Self::LoggedIn => "logged-in",
            Self::Disconnected => "disconnected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionEvent {
    LoginSuccess,
    LoginFailure,
    Logout,
    Disconnect,
}

impl SessionEvent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LoginSuccess => "login-success",
            Self::LoginFailure => "login-failure",
            Self::Logout => "logout",
            Self::Disconnect => "disconnect",
        }
    }

    pub fn next_phase(self) -> SessionPhase {
        match self {
            Self::LoginSuccess => SessionPhase::LoggedIn,
            Self::LoginFailure | Self::Logout => SessionPhase::ReadyForLogin,
            Self::Disconnect => SessionPhase::Disconnected,
        }
    }
}

#[derive(Debug)]
pub struct Session {
    client: Client,
    phase: SessionPhase,
    last_event: Option<SessionEvent>,
}

impl Session {
    pub async fn connect(
        endpoint: SocketAddr,
        connect_timeout: Duration,
        read_timeout: Duration,
    ) -> Result<Self, ClientError> {
        let client = Client::connect(endpoint, connect_timeout, read_timeout).await?;
        Ok(Self {
            client,
            phase: SessionPhase::ReadyForLogin,
            last_event: None,
        })
    }

    pub fn phase(&self) -> SessionPhase {
        self.phase
    }

    pub fn state(&self) -> SessionPhase {
        self.phase()
    }

    pub fn last_event(&self) -> Option<SessionEvent> {
        self.last_event
    }

    pub fn is_ready_for_login(&self) -> bool {
        self.phase == SessionPhase::ReadyForLogin
    }

    pub fn is_logged_in(&self) -> bool {
        self.phase == SessionPhase::LoggedIn
    }

    pub fn is_disconnected(&self) -> bool {
        self.phase == SessionPhase::Disconnected
    }

    pub fn disconnect(&mut self) {
        self.client.disconnect();
        self.transition(SessionEvent::Disconnect);
    }

    pub async fn reconnect(&mut self) -> Result<(), ClientError> {
        self.client.reconnect().await?;
        self.phase = SessionPhase::ReadyForLogin;
        tracing::info!(
            component = "network-session",
            action = "reconnect",
            phase = self.phase.as_str()
        );
        Ok(())
    }

    pub async fn send(&mut self, packet: impl AsRef<[u8]>) -> Result<(), ClientError> {
        self.client.send(packet).await
    }

    pub async fn receive(&mut self) -> Result<Option<Vec<u8>>, ClientError> {
        match self.client.receive().await {
            Ok(Some(packet)) => {
                if let Some(event) = classify_event(&packet) {
                    self.transition(event);
                }
                Ok(Some(packet))
            }
            Ok(None) | Err(ClientError::NotConnected) => {
                self.transition(SessionEvent::Disconnect);
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }

    fn transition(&mut self, event: SessionEvent) {
        self.last_event = Some(event);

        let next_phase = event.next_phase();
        let changed = self.phase != next_phase;
        self.phase = next_phase;

        tracing::info!(
            component = "network-session",
            event = event.as_str(),
            phase = self.phase.as_str(),
            changed = changed
        );
    }
}

fn classify_event(packet: &[u8]) -> Option<SessionEvent> {
    let (headcode, subcode, payload_index) = packet_metadata(packet)?;

    if headcode != 0xF1 {
        return None;
    }

    match subcode {
        0x00 => match packet.get(payload_index).copied()? {
            0x00 => Some(SessionEvent::LoginFailure),
            _ => Some(SessionEvent::LoginSuccess),
        },
        0x01 => match packet.get(payload_index).copied()? {
            0x01 | 0x20 => Some(SessionEvent::LoginSuccess),
            _ => Some(SessionEvent::LoginFailure),
        },
        0x02 => Some(SessionEvent::Logout),
        _ => None,
    }
}

fn packet_metadata(packet: &[u8]) -> Option<(u8, u8, usize)> {
    let code = *packet.first()?;
    let header_len = PacketFrame::header_len(code);
    if packet.len() < header_len {
        return None;
    }

    let headcode = *packet.get(header_len - 2)?;
    let subcode = *packet.get(header_len - 1)?;
    Some((headcode, subcode, header_len))
}

#[cfg(test)]
mod tests {
    use super::{Session, SessionEvent, SessionPhase};
    use crate::fake_server::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::session::{game_server_entered, login_response, logout_response};
    use std::time::Duration;

    #[tokio::test]
    async fn fake_server_drives_login_logout_and_disconnect_transitions() {
        let login_success = game_server_entered(true, 7, b"1.0.0").unwrap();
        let logout = logout_response(2).unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .send_packet(login_success.clone())
                    .send_packet(logout.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut session = Session::connect(
            server.address(),
            Duration::from_millis(250),
            Duration::from_millis(250),
        )
        .await
        .unwrap();

        assert_eq!(session.state(), SessionPhase::ReadyForLogin);
        assert_eq!(session.receive().await.unwrap(), Some(login_success));
        assert_eq!(session.state(), SessionPhase::LoggedIn);
        assert_eq!(session.last_event(), Some(SessionEvent::LoginSuccess));

        assert_eq!(session.receive().await.unwrap(), Some(logout));
        assert_eq!(session.state(), SessionPhase::ReadyForLogin);
        assert_eq!(session.last_event(), Some(SessionEvent::Logout));

        assert_eq!(session.receive().await.unwrap(), None);
        assert_eq!(session.state(), SessionPhase::Disconnected);
        assert_eq!(session.last_event(), Some(SessionEvent::Disconnect));

        server.finish().await.unwrap();
    }

    #[test]
    fn login_failure_keeps_the_session_on_the_login_screen() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");

        runtime.block_on(async {
            let login_failure = login_response(0).unwrap();
            let server = FakeServer::spawn(
                "127.0.0.1:0".parse().unwrap(),
                FakeServerScenario::single(
                    ConnectionScript::new()
                        .send_packet(login_failure.clone())
                        .close(),
                ),
            )
            .await
            .unwrap();

            let mut session = Session::connect(
                server.address(),
                Duration::from_millis(250),
                Duration::from_millis(250),
            )
            .await
            .unwrap();

            assert_eq!(session.receive().await.unwrap(), Some(login_failure));
            assert_eq!(session.state(), SessionPhase::ReadyForLogin);
            assert_eq!(session.last_event(), Some(SessionEvent::LoginFailure));

            session.disconnect();
            assert_eq!(session.state(), SessionPhase::Disconnected);
            assert_eq!(session.last_event(), Some(SessionEvent::Disconnect));

            server.finish().await.unwrap();
        });
    }

    #[test]
    fn disconnect_without_packets_marks_the_session_disconnected() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");

        runtime.block_on(async {
            let server = FakeServer::spawn(
                "127.0.0.1:0".parse().unwrap(),
                FakeServerScenario::single(ConnectionScript::new().close()),
            )
            .await
            .unwrap();

            let mut session = Session::connect(
                server.address(),
                Duration::from_millis(250),
                Duration::from_millis(250),
            )
            .await
            .unwrap();

            assert_eq!(session.receive().await.unwrap(), None);
            assert_eq!(session.state(), SessionPhase::Disconnected);
            assert_eq!(session.last_event(), Some(SessionEvent::Disconnect));

            server.finish().await.unwrap();
        });
    }
}
