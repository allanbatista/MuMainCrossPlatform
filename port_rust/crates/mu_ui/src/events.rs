use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const EVENTS_TITLE: &str = "Events";
const ENTRY_NOTICE: &str = "Choose an event and enter.";
const COUNTDOWN_NOTICE: &str = "The event is about to start.";
const REWARD_NOTICE: &str = "Claim the event reward.";
const RESULT_NOTICE: &str = "Review the event result.";
const ERROR_NOTICE: &str = "Event sync failed.";

const ENTRY_ACTIONS: &[EventAction] =
    &[EventAction::Join, EventAction::Details, EventAction::Close];
const COUNTDOWN_ACTIONS: &[EventAction] = &[EventAction::Join, EventAction::Close];
const REWARD_ACTIONS: &[EventAction] = &[EventAction::Claim, EventAction::Close];
const RESULT_ACTIONS: &[EventAction] = &[EventAction::Acknowledge, EventAction::Close];
const ERROR_ACTIONS: &[EventAction] = &[EventAction::Retry, EventAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventScreenState {
    Entry,
    Countdown,
    Reward,
    Result,
    Error,
}

impl EventScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entry => "entry",
            Self::Countdown => "countdown",
            Self::Reward => "reward",
            Self::Result => "result",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Entry => Some(ENTRY_NOTICE),
            Self::Countdown => Some(COUNTDOWN_NOTICE),
            Self::Reward => Some(REWARD_NOTICE),
            Self::Result => Some(RESULT_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventAction {
    Join,
    Details,
    Claim,
    Acknowledge,
    Close,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: EventScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub event_name: Option<&'static str>,
    pub event_level: Option<u8>,
    pub ticket_slot: Option<u8>,
    pub countdown_seconds: Option<u32>,
    pub reward_text: Option<&'static str>,
    pub result_text: Option<&'static str>,
    pub actions: &'static [EventAction],
}

pub fn events_screen(state: EventScreenState) -> EventScreen {
    let (
        event_name,
        event_level,
        ticket_slot,
        countdown_seconds,
        reward_text,
        result_text,
        actions,
    ) = match state {
        EventScreenState::Entry => (
            Some("Devil Square"),
            Some(4),
            Some(1),
            None,
            None,
            None,
            ENTRY_ACTIONS,
        ),
        EventScreenState::Countdown => (
            Some("Chaos Castle"),
            Some(7),
            Some(3),
            Some(45),
            None,
            None,
            COUNTDOWN_ACTIONS,
        ),
        EventScreenState::Reward => (
            Some("Doppelganger"),
            Some(2),
            Some(4),
            None,
            Some("Lucky Coin Bundle"),
            None,
            REWARD_ACTIONS,
        ),
        EventScreenState::Result => (
            Some("Battle Soccer"),
            Some(1),
            None,
            None,
            None,
            Some("Second place secured."),
            RESULT_ACTIONS,
        ),
        EventScreenState::Error => (None, None, None, None, None, None, ERROR_ACTIONS),
    };

    EventScreen {
        route: UiRoute::Events,
        layout: UiShellLayout::for_route(UiRoute::Events),
        widgets: UiShellWidgetSet::for_route(UiRoute::Events),
        state,
        title: EVENTS_TITLE,
        notice: state.notice(),
        event_name,
        event_level,
        ticket_slot,
        countdown_seconds,
        reward_text,
        result_text,
        actions,
    }
}

impl EventScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|event_name={:?}|event_level={:?}|ticket_slot={:?}|countdown_seconds={:?}|reward_text={:?}|result_text={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.event_name,
            self.event_level,
            self.ticket_slot,
            self.countdown_seconds,
            self.reward_text,
            self.result_text,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{events_screen, EventScreenState};

    #[test]
    fn event_entry_and_countdown_snapshots() {
        assert_eq!(
            events_screen(EventScreenState::Entry).snapshot(),
            "route=events|group=gameplay|state=entry|title=Events|notice=Some(\"Choose an event and enter.\")|event_name=Some(\"Devil Square\")|event_level=Some(4)|ticket_slot=Some(1)|countdown_seconds=None|reward_text=None|result_text=None|actions=[Join, Details, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            events_screen(EventScreenState::Countdown).snapshot(),
            "route=events|group=gameplay|state=countdown|title=Events|notice=Some(\"The event is about to start.\")|event_name=Some(\"Chaos Castle\")|event_level=Some(7)|ticket_slot=Some(3)|countdown_seconds=Some(45)|reward_text=None|result_text=None|actions=[Join, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn event_reward_result_and_error_snapshots() {
        assert_eq!(
            events_screen(EventScreenState::Reward).snapshot(),
            "route=events|group=gameplay|state=reward|title=Events|notice=Some(\"Claim the event reward.\")|event_name=Some(\"Doppelganger\")|event_level=Some(2)|ticket_slot=Some(4)|countdown_seconds=None|reward_text=Some(\"Lucky Coin Bundle\")|result_text=None|actions=[Claim, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            events_screen(EventScreenState::Result).snapshot(),
            "route=events|group=gameplay|state=result|title=Events|notice=Some(\"Review the event result.\")|event_name=Some(\"Battle Soccer\")|event_level=Some(1)|ticket_slot=None|countdown_seconds=None|reward_text=None|result_text=Some(\"Second place secured.\")|actions=[Acknowledge, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            events_screen(EventScreenState::Error).snapshot(),
            "route=events|group=gameplay|state=error|title=Events|notice=Some(\"Event sync failed.\")|event_name=None|event_level=None|ticket_slot=None|countdown_seconds=None|reward_text=None|result_text=None|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
