use bevy::prelude::{App, Plugin, Resource};

use crate::MAX_USERNAME_SIZE;

pub const MAX_MAIL_RECIPIENT_LENGTH: usize = MAX_USERNAME_SIZE;
pub const MAX_MAIL_DATE_LENGTH: usize = 10;
pub const MAX_MAIL_SUBJECT_LENGTH: usize = 60;
pub const MAX_MAIL_TIME_LENGTH: usize = 8;
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
pub struct MailLetterEntry {
    pub id: u32,
    pub sender: String,
    pub subject: String,
    pub date: String,
    pub time: String,
    pub read: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailManager {
    mode: MailMode,
    new_mail_alert: bool,
    webzen_mail: bool,
    letters_loaded: bool,
    selected_letter_id: Option<u32>,
    draft_recipient: String,
    draft_subject: String,
    draft_body: String,
    letters: Vec<MailLetterEntry>,
}

impl Default for MailManager {
    fn default() -> Self {
        Self {
            mode: MailMode::Inbox,
            new_mail_alert: false,
            webzen_mail: false,
            letters_loaded: false,
            selected_letter_id: None,
            draft_recipient: String::new(),
            draft_subject: String::new(),
            draft_body: String::new(),
            letters: Vec::new(),
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

    pub fn letters_loaded(&self) -> bool {
        self.letters_loaded
    }

    pub fn selected_letter_id(&self) -> Option<u32> {
        self.selected_letter_id
    }

    pub fn letters(&self) -> &[MailLetterEntry] {
        self.letters.as_slice()
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

    pub fn set_letters_loaded(&mut self, letters_loaded: bool) {
        self.letters_loaded = letters_loaded;
    }

    pub fn select_letter(&mut self, letter_id: u32) {
        self.selected_letter_id = Some(letter_id);
        self.mode = MailMode::Reading;
    }

    pub fn mark_letter_read(&mut self, letter_id: u32) {
        if let Some(letter) = self.letters.iter_mut().find(|entry| entry.id == letter_id) {
            letter.read = true;
            self.sync_new_mail_alert();
        }
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

    pub fn upsert_letter(&mut self, letter: MailLetterEntry) {
        let letter = normalize_mail_letter_entry(letter);

        if let Some(existing) = self.letters.iter_mut().find(|entry| entry.id == letter.id) {
            *existing = letter;
        } else {
            self.letters.push(letter);
        }

        self.letters_loaded = true;
        self.sync_new_mail_alert();
        self.sort_letters();
    }

    pub fn remove_letter(&mut self, letter_id: u32) {
        let was_selected = self.selected_letter_id == Some(letter_id);
        self.letters.retain(|entry| entry.id != letter_id);

        if was_selected {
            self.clear_selected_letter();
        }

        self.sync_new_mail_alert();
    }

    pub fn mark_error(&mut self) {
        self.mode = MailMode::Error;
    }

    fn sync_new_mail_alert(&mut self) {
        self.new_mail_alert = self.letters.iter().any(|letter| !letter.read);
    }

    fn sort_letters(&mut self) {
        self.letters.sort_by(|left, right| {
            right
                .date
                .cmp(&left.date)
                .then_with(|| right.time.cmp(&left.time))
                .then_with(|| right.id.cmp(&left.id))
        });
    }
}

fn normalize_mail_recipient(value: String) -> String {
    value.chars().take(MAX_MAIL_RECIPIENT_LENGTH).collect()
}

fn normalize_mail_subject(value: String) -> String {
    value.chars().take(MAX_MAIL_SUBJECT_LENGTH).collect()
}

fn normalize_mail_letter_entry(mut letter: MailLetterEntry) -> MailLetterEntry {
    letter.sender = normalize_mail_recipient(letter.sender);
    letter.subject = normalize_mail_subject(letter.subject);
    letter.date = letter.date.chars().take(MAX_MAIL_DATE_LENGTH).collect();
    letter.time = letter.time.chars().take(MAX_MAIL_TIME_LENGTH).collect();
    letter
}

fn normalize_mail_body(value: String) -> String {
    value.chars().take(MAX_MAIL_BODY_LENGTH).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        MailLetterEntry, MailManager, MailMode, MailPlugin, MAX_MAIL_BODY_LENGTH,
        MAX_MAIL_SUBJECT_LENGTH,
    };
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

    #[test]
    fn mail_letter_updates_track_live_state() {
        let mut manager = MailManager::new();
        manager.upsert_letter(MailLetterEntry {
            id: 3,
            sender: "Selene".to_owned(),
            subject: "Castle prep".to_owned(),
            date: "05/18/2026".to_owned(),
            time: "21:40".to_owned(),
            read: true,
        });
        manager.upsert_letter(MailLetterEntry {
            id: 1,
            sender: "Astra".to_owned(),
            subject: "Potion run".to_owned(),
            date: "05/19/2026".to_owned(),
            time: "10:12".to_owned(),
            read: false,
        });

        assert!(manager.letters_loaded());
        assert!(manager.new_mail_alert());
        assert_eq!(manager.letters().len(), 2);
        assert_eq!(manager.letters()[0].id, 1);
        assert_eq!(manager.letters()[1].id, 3);

        manager.select_letter(1);
        manager.remove_letter(1);

        assert_eq!(manager.mode(), MailMode::Inbox);
        assert_eq!(manager.selected_letter_id(), None);
        assert_eq!(manager.letters().len(), 1);
        assert!(!manager.new_mail_alert());
    }

    #[test]
    fn mail_mark_letter_read_clears_the_unread_alert() {
        let mut manager = MailManager::new();
        manager.upsert_letter(MailLetterEntry {
            id: 1,
            sender: "Astra".to_owned(),
            subject: "Potion run".to_owned(),
            date: "05/19/2026".to_owned(),
            time: "10:12".to_owned(),
            read: false,
        });

        assert!(manager.new_mail_alert());

        manager.mark_letter_read(1);

        assert!(manager.letters()[0].read);
        assert!(!manager.new_mail_alert());
    }
}
