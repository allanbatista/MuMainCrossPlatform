use bevy::prelude::{App, Plugin, Resource};

use crate::MAX_USERNAME_SIZE;

pub const MAX_MAIL_RECIPIENT_LENGTH: usize = MAX_USERNAME_SIZE;
pub const MAX_MAIL_SUBJECT_LENGTH: usize = 60;
pub const MAX_MAIL_BODY_LENGTH: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MailMode {
    #[default]
    Inbox,
    Compose,
    Reading,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailManager {
    mode: MailMode,
    new_mail_alert: bool,
    webzen_mail: bool,
    selected_letter_id: Option<u32>,
    draft_recipient: String,
    draft_subject: String,
    draft_body: String,
}

impl Default for MailManager {
    fn default() -> Self {
        Self {
            mode: MailMode::Inbox,
            new_mail_alert: false,
            webzen_mail: false,
            selected_letter_id: None,
            draft_recipient: String::new(),
            draft_subject: String::new(),
            draft_body: String::new(),
        }
    }
}

impl Resource for MailManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct MailPlugin;

impl Plugin for MailPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MailManager>();
    }
}

impl MailManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn mode(&self) -> MailMode {
        self.mode
    }

    pub fn new_mail_alert(&self) -> bool {
        self.new_mail_alert
    }

    pub fn webzen_mail(&self) -> bool {
        self.webzen_mail
    }

    pub fn selected_letter_id(&self) -> Option<u32> {
        self.selected_letter_id
    }

    pub fn draft_recipient(&self) -> &str {
        self.draft_recipient.as_str()
    }

    pub fn draft_subject(&self) -> &str {
        self.draft_subject.as_str()
    }

    pub fn draft_body(&self) -> &str {
        self.draft_body.as_str()
    }

    pub fn set_new_mail_alert(&mut self, new_mail_alert: bool) {
        self.new_mail_alert = new_mail_alert;
    }

    pub fn set_webzen_mail(&mut self, webzen_mail: bool) {
        self.webzen_mail = webzen_mail;
    }

    pub fn select_letter(&mut self, letter_id: u32) {
        self.selected_letter_id = Some(letter_id);
        self.mode = MailMode::Reading;
    }

    pub fn clear_selected_letter(&mut self) {
        self.selected_letter_id = None;
        if self.mode != MailMode::Error {
            self.mode = MailMode::Inbox;
        }
    }

    pub fn set_compose(
        &mut self,
        recipient: impl Into<String>,
        subject: impl Into<String>,
        body: impl Into<String>,
    ) {
        self.selected_letter_id = None;
        self.draft_recipient = normalize_mail_recipient(recipient.into());
        self.draft_subject = normalize_mail_subject(subject.into());
        self.draft_body = normalize_mail_body(body.into());
        self.mode = MailMode::Compose;
    }

    pub fn clear_compose(&mut self) {
        self.draft_recipient.clear();
        self.draft_subject.clear();
        self.draft_body.clear();
        if self.mode != MailMode::Error {
            self.mode = MailMode::Inbox;
        }
    }

    pub fn mark_error(&mut self) {
        self.mode = MailMode::Error;
    }
}

fn normalize_mail_recipient(value: String) -> String {
    value.chars().take(MAX_MAIL_RECIPIENT_LENGTH).collect()
}

fn normalize_mail_subject(value: String) -> String {
    value.chars().take(MAX_MAIL_SUBJECT_LENGTH).collect()
}

fn normalize_mail_body(value: String) -> String {
    value.chars().take(MAX_MAIL_BODY_LENGTH).collect()
}

#[cfg(test)]
mod tests {
    use super::{MailManager, MailMode, MailPlugin, MAX_MAIL_BODY_LENGTH, MAX_MAIL_SUBJECT_LENGTH};
    use bevy::prelude::App;

    #[test]
    fn mail_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(MailPlugin);

        let manager = app.world().resource::<MailManager>();
        assert_eq!(manager.mode(), MailMode::Inbox);
        assert!(!manager.new_mail_alert());
    }

    #[test]
    fn mail_compose_state_truncates_inputs() {
        let mut manager = MailManager::new();
        manager.set_new_mail_alert(true);
        manager.set_webzen_mail(true);
        manager.set_compose(
            "LongRecipientForMail",
            "A very long letter subject that should be truncated cleanly and safely",
            "A long message body ".repeat(80),
        );

        assert_eq!(manager.mode(), MailMode::Compose);
        assert_eq!(manager.selected_letter_id(), None);
        assert_eq!(manager.draft_recipient().chars().count(), 10);
        assert_eq!(
            manager.draft_subject().chars().count(),
            MAX_MAIL_SUBJECT_LENGTH
        );
        assert_eq!(manager.draft_body().chars().count(), MAX_MAIL_BODY_LENGTH);
        assert!(manager.new_mail_alert());
        assert!(manager.webzen_mail());
    }

    #[test]
    fn mail_reading_state_clears_selection() {
        let mut manager = MailManager::new();
        manager.select_letter(0x0102_0304);
        manager.clear_selected_letter();

        assert_eq!(manager.mode(), MailMode::Inbox);
        assert_eq!(manager.selected_letter_id(), None);
    }
}
