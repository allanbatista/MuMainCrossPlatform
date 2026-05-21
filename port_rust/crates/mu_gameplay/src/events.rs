use bevy::prelude::{App, Plugin, Resource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventMode {
    #[default]
    Idle,
    Entry,
    Countdown,
    Reward,
    Result,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventKind {
    #[default]
    None,
    BloodCastle,
    ChaosCastle,
    Doppelganger,
    CursedTemple,
    DevilSquare,
    LuckyCoin,
    EmpireGuardian,
    IllusionTemple,
    Kanturu,
    CryWolf,
    BattleSoccer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventManager {
    mode: EventMode,
    kind: EventKind,
    event_name: String,
    event_level: Option<u8>,
    ticket_slot: Option<u8>,
    countdown_seconds: Option<u32>,
    reward_text: Option<String>,
    result_text: Option<String>,
}

impl Default for EventManager {
    fn default() -> Self {
        Self {
            mode: EventMode::Idle,
            kind: EventKind::None,
            event_name: String::new(),
            event_level: None,
            ticket_slot: None,
            countdown_seconds: None,
            reward_text: None,
            result_text: None,
        }
    }
}

impl Resource for EventManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventManager>();
    }
}

impl EventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn mode(&self) -> EventMode {
        self.mode
    }

    pub fn kind(&self) -> EventKind {
        self.kind
    }

    pub fn event_name(&self) -> &str {
        self.event_name.as_str()
    }

    pub fn event_level(&self) -> Option<u8> {
        self.event_level
    }

    pub fn ticket_slot(&self) -> Option<u8> {
        self.ticket_slot
    }

    pub fn countdown_seconds(&self) -> Option<u32> {
        self.countdown_seconds
    }

    pub fn reward_text(&self) -> Option<&str> {
        self.reward_text.as_deref()
    }

    pub fn result_text(&self) -> Option<&str> {
        self.result_text.as_deref()
    }

    pub fn set_event(&mut self, kind: EventKind, event_name: impl Into<String>) {
        self.kind = kind;
        self.event_name = event_name.into();
        self.event_level = None;
        self.ticket_slot = None;
        self.countdown_seconds = None;
        self.reward_text = None;
        self.result_text = None;
        self.mode = EventMode::Entry;
    }

    pub fn set_entry(&mut self, event_level: Option<u8>, ticket_slot: Option<u8>) {
        self.event_level = event_level;
        self.ticket_slot = ticket_slot;
        self.mode = EventMode::Entry;
    }

    pub fn start_countdown(&mut self, countdown_seconds: u32) {
        self.countdown_seconds = Some(countdown_seconds);
        self.mode = EventMode::Countdown;
    }

    pub fn set_reward(&mut self, reward_text: impl Into<String>) {
        self.reward_text = Some(reward_text.into());
        self.result_text = None;
        self.mode = EventMode::Reward;
    }

    pub fn set_result(&mut self, result_text: impl Into<String>) {
        self.result_text = Some(result_text.into());
        self.reward_text = None;
        self.mode = EventMode::Result;
    }

    pub fn mark_error(&mut self) {
        self.mode = EventMode::Error;
    }

    pub fn clear_error(&mut self) {
        self.mode = if self.reward_text.is_some() {
            EventMode::Reward
        } else if self.result_text.is_some() {
            EventMode::Result
        } else if self.countdown_seconds.is_some() {
            EventMode::Countdown
        } else if self.kind == EventKind::None {
            EventMode::Idle
        } else {
            EventMode::Entry
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{EventKind, EventManager, EventMode, EventPlugin};
    use bevy::prelude::App;

    #[test]
    fn event_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(EventPlugin);

        let manager = app.world().resource::<EventManager>();
        assert_eq!(manager.mode(), EventMode::Idle);
        assert_eq!(manager.kind(), EventKind::None);
    }

    #[test]
    fn event_manager_tracks_countdown_reward_and_result() {
        let mut manager = EventManager::new();
        manager.set_event(EventKind::BloodCastle, "Blood Castle");
        manager.set_entry(Some(7), Some(3));
        manager.start_countdown(45);
        manager.set_reward("Gold Box");
        manager.set_result("Second place secured.");

        assert_eq!(manager.kind(), EventKind::BloodCastle);
        assert_eq!(manager.event_name(), "Blood Castle");
        assert_eq!(manager.event_level(), Some(7));
        assert_eq!(manager.ticket_slot(), Some(3));
        assert_eq!(manager.countdown_seconds(), Some(45));
        assert_eq!(manager.reward_text(), None);
        assert_eq!(manager.result_text(), Some("Second place secured."));
        assert_eq!(manager.mode(), EventMode::Result);
    }
}
