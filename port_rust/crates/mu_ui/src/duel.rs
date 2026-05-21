use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const DUEL_TITLE: &str = "Duel";
const CHALLENGE_NOTICE: &str = "Duel request received.";
const CHANNEL_NOTICE: &str = "Choose a duel channel.";
const WATCH_NOTICE: &str = "Watching duel channels.";
const ERROR_NOTICE: &str = "Duel sync failed.";

const CHALLENGE_ACTIONS: &[DuelAction] =
    &[DuelAction::Accept, DuelAction::Decline, DuelAction::Close];
const CHANNEL_ACTIONS: &[DuelAction] = &[
    DuelAction::JoinChannel,
    DuelAction::QuitChannel,
    DuelAction::Close,
];
const WATCH_ACTIONS: &[DuelAction] = &[DuelAction::LeaveWatch, DuelAction::Close];
const ERROR_ACTIONS: &[DuelAction] = &[DuelAction::Retry, DuelAction::Close];
const CHANNEL_LIST_CHANNELS: &[DuelChannelCard] = &[
    DuelChannelCard::new(true, true, "Astra", "Blade"),
    DuelChannelCard::new(true, false, "Selene", "Orion"),
    DuelChannelCard::new(false, false, "", ""),
];
const WATCH_CHANNELS: &[DuelChannelCard] = &[
    DuelChannelCard::new(true, true, "Astra", "Blade"),
    DuelChannelCard::new(true, false, "Selene", "Orion"),
];
const WATCH_USERS: &[&str] = &["WatcherOne", "WatcherTwo"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuelScreenState {
    Challenge,
    ChannelList,
    Watching,
    Error,
}

impl DuelScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Challenge => "challenge",
            Self::ChannelList => "channel-list",
            Self::Watching => "watching",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Challenge => Some(CHALLENGE_NOTICE),
            Self::ChannelList => Some(CHANNEL_NOTICE),
            Self::Watching => Some(WATCH_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuelAction {
    Accept,
    Decline,
    JoinChannel,
    QuitChannel,
    LeaveWatch,
    Retry,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DuelChannelCard {
    pub enabled: bool,
    pub joinable: bool,
    pub player_one_id: &'static str,
    pub player_two_id: &'static str,
}

impl DuelChannelCard {
    pub const fn new(
        enabled: bool,
        joinable: bool,
        player_one_id: &'static str,
        player_two_id: &'static str,
    ) -> Self {
        Self {
            enabled,
            joinable,
            player_one_id,
            player_two_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DuelScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: DuelScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub duel_enabled: bool,
    pub pet_duel_enabled: bool,
    pub fighter_regenerated: bool,
    pub current_channel: i32,
    pub hero_id: Option<&'static str>,
    pub enemy_id: Option<&'static str>,
    pub hero_score: Option<i32>,
    pub enemy_score: Option<i32>,
    pub hero_hp_rate: Option<f32>,
    pub enemy_hp_rate: Option<f32>,
    pub hero_sd_rate: Option<f32>,
    pub enemy_sd_rate: Option<f32>,
    pub channels: &'static [DuelChannelCard],
    pub watch_users: &'static [&'static str],
    pub actions: &'static [DuelAction],
}

pub fn duel_screen(state: DuelScreenState) -> DuelScreen {
    let (
        duel_enabled,
        pet_duel_enabled,
        fighter_regenerated,
        current_channel,
        hero_id,
        enemy_id,
        hero_score,
        enemy_score,
        hero_hp_rate,
        enemy_hp_rate,
        hero_sd_rate,
        enemy_sd_rate,
        channels,
        watch_users,
        actions,
    ) = match state {
        DuelScreenState::Challenge => (
            true,
            false,
            false,
            -1,
            Some("Astra"),
            Some("Blade"),
            Some(7),
            Some(5),
            Some(0.84),
            Some(0.71),
            Some(0.62),
            Some(0.55),
            &[][..],
            &[][..],
            CHALLENGE_ACTIONS,
        ),
        DuelScreenState::ChannelList => (
            true,
            false,
            false,
            2,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            CHANNEL_LIST_CHANNELS,
            &[][..],
            CHANNEL_ACTIONS,
        ),
        DuelScreenState::Watching => (
            true,
            true,
            true,
            3,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            WATCH_CHANNELS,
            WATCH_USERS,
            WATCH_ACTIONS,
        ),
        DuelScreenState::Error => (
            false,
            false,
            false,
            -1,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            &[][..],
            &[][..],
            ERROR_ACTIONS,
        ),
    };

    DuelScreen {
        route: UiRoute::Duel,
        layout: UiShellLayout::for_route(UiRoute::Duel),
        widgets: UiShellWidgetSet::for_route(UiRoute::Duel),
        state,
        title: DUEL_TITLE,
        notice: state.notice(),
        duel_enabled,
        pet_duel_enabled,
        fighter_regenerated,
        current_channel,
        hero_id,
        enemy_id,
        hero_score,
        enemy_score,
        hero_hp_rate,
        enemy_hp_rate,
        hero_sd_rate,
        enemy_sd_rate,
        channels,
        watch_users,
        actions,
    }
}

impl DuelScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|duel_enabled={}|pet_duel_enabled={}|fighter_regenerated={}|current_channel={}|hero_id={:?}|enemy_id={:?}|hero_score={:?}|enemy_score={:?}|hero_hp_rate={:?}|enemy_hp_rate={:?}|hero_sd_rate={:?}|enemy_sd_rate={:?}|channels={:?}|watch_users={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.duel_enabled,
            self.pet_duel_enabled,
            self.fighter_regenerated,
            self.current_channel,
            self.hero_id,
            self.enemy_id,
            self.hero_score,
            self.enemy_score,
            self.hero_hp_rate,
            self.enemy_hp_rate,
            self.hero_sd_rate,
            self.enemy_sd_rate,
            self.channels,
            self.watch_users,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{duel_screen, DuelScreenState};

    #[test]
    fn duel_challenge_and_channel_snapshots() {
        assert_eq!(
            duel_screen(DuelScreenState::Challenge).snapshot(),
            "route=duel|group=gameplay|state=challenge|title=Duel|notice=Some(\"Duel request received.\")|duel_enabled=true|pet_duel_enabled=false|fighter_regenerated=false|current_channel=-1|hero_id=Some(\"Astra\")|enemy_id=Some(\"Blade\")|hero_score=Some(7)|enemy_score=Some(5)|hero_hp_rate=Some(0.84)|enemy_hp_rate=Some(0.71)|hero_sd_rate=Some(0.62)|enemy_sd_rate=Some(0.55)|channels=[]|watch_users=[]|actions=[Accept, Decline, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            duel_screen(DuelScreenState::ChannelList).snapshot(),
            "route=duel|group=gameplay|state=channel-list|title=Duel|notice=Some(\"Choose a duel channel.\")|duel_enabled=true|pet_duel_enabled=false|fighter_regenerated=false|current_channel=2|hero_id=None|enemy_id=None|hero_score=None|enemy_score=None|hero_hp_rate=None|enemy_hp_rate=None|hero_sd_rate=None|enemy_sd_rate=None|channels=[DuelChannelCard { enabled: true, joinable: true, player_one_id: \"Astra\", player_two_id: \"Blade\" }, DuelChannelCard { enabled: true, joinable: false, player_one_id: \"Selene\", player_two_id: \"Orion\" }, DuelChannelCard { enabled: false, joinable: false, player_one_id: \"\", player_two_id: \"\" }]|watch_users=[]|actions=[JoinChannel, QuitChannel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn duel_watching_and_error_snapshots() {
        assert_eq!(
            duel_screen(DuelScreenState::Watching).snapshot(),
            "route=duel|group=gameplay|state=watching|title=Duel|notice=Some(\"Watching duel channels.\")|duel_enabled=true|pet_duel_enabled=true|fighter_regenerated=true|current_channel=3|hero_id=None|enemy_id=None|hero_score=None|enemy_score=None|hero_hp_rate=None|enemy_hp_rate=None|hero_sd_rate=None|enemy_sd_rate=None|channels=[DuelChannelCard { enabled: true, joinable: true, player_one_id: \"Astra\", player_two_id: \"Blade\" }, DuelChannelCard { enabled: true, joinable: false, player_one_id: \"Selene\", player_two_id: \"Orion\" }]|watch_users=[\"WatcherOne\", \"WatcherTwo\"]|actions=[LeaveWatch, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            duel_screen(DuelScreenState::Error).snapshot(),
            "route=duel|group=gameplay|state=error|title=Duel|notice=Some(\"Duel sync failed.\")|duel_enabled=false|pet_duel_enabled=false|fighter_regenerated=false|current_channel=-1|hero_id=None|enemy_id=None|hero_score=None|enemy_score=None|hero_hp_rate=None|enemy_hp_rate=None|hero_sd_rate=None|enemy_sd_rate=None|channels=[]|watch_users=[]|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
