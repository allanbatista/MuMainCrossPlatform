use bevy::prelude::Resource;

pub use mu_network::session::{SessionEvent, SessionPhase};

#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct SessionState {
    phase: SessionPhase,
    last_event: Option<SessionEvent>,
}

impl SessionState {
    pub fn new() -> Self {
        Self::default()
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

    pub fn as_str(&self) -> &'static str {
        self.phase.as_str()
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

    pub fn login_success(&mut self) -> bool {
        self.apply_event(SessionEvent::LoginSuccess)
    }

    pub fn login_failure(&mut self) -> bool {
        self.apply_event(SessionEvent::LoginFailure)
    }

    pub fn logout(&mut self) -> bool {
        self.apply_event(SessionEvent::Logout)
    }

    pub fn disconnect(&mut self) -> bool {
        self.apply_event(SessionEvent::Disconnect)
    }

    pub fn apply_event(&mut self, event: SessionEvent) -> bool {
        self.last_event = Some(event);

        let next_phase = event.next_phase();
        let changed = self.phase != next_phase;
        self.phase = next_phase;

        tracing::info!(
            component = "session-state",
            event = event.as_str(),
            phase = self.phase.as_str(),
            changed = changed
        );

        changed
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            phase: SessionPhase::ReadyForLogin,
            last_event: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionEvent, SessionPhase, SessionState};
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::fmt::MakeWriter;

    struct BufferWriter(Arc<Mutex<Vec<u8>>>);

    impl<'a> MakeWriter<'a> for BufferWriter {
        type Writer = BufferSink;

        fn make_writer(&'a self) -> Self::Writer {
            BufferSink(Arc::clone(&self.0))
        }
    }

    struct BufferSink(Arc<Mutex<Vec<u8>>>);

    impl Write for BufferSink {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0
                .lock()
                .expect("log buffer poisoned")
                .extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn capture_logs(f: impl FnOnce()) -> String {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(false)
            .without_time()
            .with_target(false)
            .compact()
            .with_writer(BufferWriter(Arc::clone(&buffer)))
            .finish();

        tracing::subscriber::with_default(subscriber, f);

        let output = buffer.lock().expect("log buffer poisoned").clone();
        String::from_utf8(output).expect("log output is utf-8")
    }

    #[test]
    fn tracks_login_logout_and_disconnect_transitions() {
        let output = capture_logs(|| {
            let mut state = SessionState::new();

            assert_eq!(state.phase(), SessionPhase::ReadyForLogin);
            assert!(state.login_success());
            assert_eq!(state.phase(), SessionPhase::LoggedIn);
            assert_eq!(state.last_event(), Some(SessionEvent::LoginSuccess));

            assert!(state.logout());
            assert_eq!(state.phase(), SessionPhase::ReadyForLogin);
            assert_eq!(state.last_event(), Some(SessionEvent::Logout));

            assert!(state.disconnect());
            assert_eq!(state.phase(), SessionPhase::Disconnected);
            assert_eq!(state.last_event(), Some(SessionEvent::Disconnect));
        });

        assert!(output.contains("component=\"session-state\""), "{output}");
        assert!(output.contains("login-success"), "{output}");
        assert!(output.contains("logout"), "{output}");
        assert!(output.contains("disconnect"), "{output}");
    }

    #[test]
    fn login_failure_keeps_the_login_screen_state() {
        let mut state = SessionState::new();

        assert!(!state.login_failure());
        assert_eq!(state.phase(), SessionPhase::ReadyForLogin);
        assert_eq!(state.last_event(), Some(SessionEvent::LoginFailure));
        assert!(state.is_ready_for_login());
    }
}
