use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::{App, MessageReader, Plugin, Res, ResMut, Resource, Update};
use mu_ui::UiRoute;

use crate::bootstrap_runtime::BootstrapRuntime;
use crate::ClientRuntime;
use crate::SessionState;

pub(crate) const CHAT_DRAFT_CHAR_LIMIT: usize = 90;

const CHAT_DRAFT_EMPTY_STATUS: &str = "Draft is empty.";
const CHAT_DRAFT_CLEARED_STATUS: &str = "Draft cleared.";
const CHAT_SEND_FAILED_STATUS: &str = "Chat send unavailable.";

#[derive(Debug, Default, Clone, PartialEq, Eq, Resource)]
pub(crate) struct ChatComposerState {
    draft: String,
    last_sent: Option<String>,
    last_status: Option<String>,
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ChatComposerPlugin;

impl Plugin for ChatComposerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChatComposerState>()
            .add_systems(Update, apply_chat_input_system);
    }
}

impl ChatComposerState {
    pub fn draft(&self) -> &str {
        self.draft.as_str()
    }

    pub fn last_sent(&self) -> Option<&str> {
        self.last_sent.as_deref()
    }

    pub fn last_status(&self) -> Option<&str> {
        self.last_status.as_deref()
    }

    pub fn draft_char_count(&self) -> usize {
        self.draft.chars().count()
    }

    pub fn draft_limit_remaining(&self) -> usize {
        CHAT_DRAFT_CHAR_LIMIT.saturating_sub(self.draft_char_count())
    }

    pub fn push_text(&mut self, text: &str) {
        let remaining = self.draft_limit_remaining();
        if remaining == 0 {
            return;
        }

        self.draft.extend(text.chars().take(remaining));
    }

    pub fn backspace(&mut self) {
        self.draft.pop();
    }

    pub fn clear_draft(&mut self) {
        self.draft.clear();
    }

    pub fn record_status(&mut self, status: impl Into<String>) {
        self.last_status = Some(status.into());
    }

    pub fn mark_sent(&mut self, sender: &str, message: impl Into<String>) {
        let message = message.into();
        self.last_sent = Some(message.clone());
        self.last_status = Some(format!("Sent as {sender}."));
        self.draft.clear();
    }

    pub fn mark_send_failed(&mut self) {
        self.last_status = Some(CHAT_SEND_FAILED_STATUS.to_string());
    }
}

pub(crate) fn handle_chat_keyboard_input(
    composer: &mut ChatComposerState,
    input: &KeyboardInput,
) -> Option<String> {
    if !input.state.is_pressed() {
        return None;
    }

    match (&input.logical_key, &input.text) {
        (Key::Enter, _) => {
            if composer.draft().is_empty() {
                composer.record_status(CHAT_DRAFT_EMPTY_STATUS);
                None
            } else {
                Some(composer.draft().to_string())
            }
        }
        (Key::Backspace, _) => {
            composer.backspace();
            None
        }
        (Key::Escape, _) => {
            composer.clear_draft();
            composer.record_status(CHAT_DRAFT_CLEARED_STATUS);
            None
        }
        (_, Some(text)) if text.chars().all(is_printable_char) => {
            composer.push_text(text);
            None
        }
        _ => None,
    }
}

fn apply_chat_input_system(
    mut composer: ResMut<ChatComposerState>,
    ui_shell: Res<mu_ui::UiShellState>,
    session_state: Res<SessionState>,
    bootstrap_runtime: Option<Res<BootstrapRuntime>>,
    client_runtime: Res<ClientRuntime>,
    mut keyboard_input_reader: MessageReader<KeyboardInput>,
) {
    let chat_active = ui_shell.current() == UiRoute::Chat && session_state.is_logged_in();

    for keyboard_input in keyboard_input_reader.read() {
        if !chat_active {
            continue;
        }

        let Some(message) = handle_chat_keyboard_input(&mut composer, keyboard_input) else {
            continue;
        };

        let Some(bootstrap_runtime) = bootstrap_runtime.as_deref() else {
            composer.mark_send_failed();
            continue;
        };

        let sender = client_runtime.local_player_label().unwrap_or("Player");

        if bootstrap_runtime.queue_chat_message_request(sender, &message) {
            composer.mark_sent(sender, message);
        } else {
            composer.mark_send_failed();
        }
    }
}

fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}

#[cfg(test)]
mod tests {
    use super::{handle_chat_keyboard_input, ChatComposerState, CHAT_DRAFT_CHAR_LIMIT};
    use crate::bootstrap_runtime::{BootstrapCommand, BootstrapRuntime};
    use crate::ClientRuntime;
    use bevy::input::keyboard::{Key, KeyCode, KeyboardInput};
    use bevy::input::ButtonState;
    use bevy::prelude::Entity;
    use camino::Utf8PathBuf;
    use mu_assets::load_terrain_world_bundle;
    use std::path::Path;
    use std::sync::mpsc;
    use tokio::sync::mpsc as tokio_mpsc;

    fn keyboard_input(key_code: KeyCode, logical_key: Key, text: Option<&str>) -> KeyboardInput {
        KeyboardInput {
            key_code,
            logical_key,
            state: ButtonState::Pressed,
            text: text.map(Into::into),
            repeat: false,
            window: Entity::PLACEHOLDER,
        }
    }

    #[test]
    fn composer_appends_text_and_limits_length() {
        let mut composer = ChatComposerState::default();
        composer.push_text("hello");
        composer.push_text(" world");

        assert_eq!(composer.draft(), "hello world");

        let limit = "a".repeat(CHAT_DRAFT_CHAR_LIMIT + 5);
        composer.push_text(&limit);

        assert_eq!(composer.draft_char_count(), CHAT_DRAFT_CHAR_LIMIT);
    }

    #[test]
    fn composer_backspace_escape_and_enter_match_keyboard_events() {
        let mut composer = ChatComposerState::default();

        assert_eq!(
            handle_chat_keyboard_input(
                &mut composer,
                &keyboard_input(KeyCode::KeyA, Key::Character("a".into()), Some("a")),
            ),
            None
        );
        assert_eq!(composer.draft(), "a");

        assert_eq!(
            handle_chat_keyboard_input(
                &mut composer,
                &keyboard_input(KeyCode::Backspace, Key::Backspace, None),
            ),
            None
        );
        assert_eq!(composer.draft(), "");

        composer.push_text("hi");
        assert_eq!(
            handle_chat_keyboard_input(
                &mut composer,
                &keyboard_input(KeyCode::Enter, Key::Enter, None)
            ),
            Some("hi".to_string())
        );

        assert_eq!(
            handle_chat_keyboard_input(
                &mut composer,
                &keyboard_input(KeyCode::Escape, Key::Escape, None),
            ),
            None
        );
        assert_eq!(composer.draft(), "");
        assert_eq!(composer.last_status(), Some("Draft cleared."));
    }

    fn repo_world_root() -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../port_rust/assets"),
        )
        .unwrap()
    }

    #[test]
    fn composer_uses_the_selected_character_as_chat_sender() {
        let world_root = repo_world_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();
        let mut client_runtime = ClientRuntime::new();
        client_runtime.load_world_bundle_with_local_player_label(bundle, Some("Selene"));

        let (_signal_sender, signal_receiver) = mpsc::channel();
        let (command_sender, mut command_receiver) = tokio_mpsc::unbounded_channel();
        let bootstrap = BootstrapRuntime::new(signal_receiver, Some(command_sender));

        assert!(bootstrap.queue_chat_message_request(
            client_runtime.local_player_label().unwrap_or("Player"),
            "hello"
        ));

        match command_receiver.try_recv().expect("chat command missing") {
            BootstrapCommand::Chat { sender, message } => {
                assert_eq!(sender, "Selene");
                assert_eq!(message, "hello");
            }
            other => panic!("unexpected bootstrap command: {other:?}"),
        }
    }
}
