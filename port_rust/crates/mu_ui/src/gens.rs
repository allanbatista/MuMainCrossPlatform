use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const TITLE: &str = "Gens Ranking";
const READY_NOTICE: &str = "Review Gens ranking and contribution.";
const JOIN_NOTICE: &str = "Choose a Gens faction.";
const REWARD_NOTICE: &str = "Claim the Gens reward.";
const ERROR_NOTICE: &str = "Gens ranking sync failed.";

const READY_ACTIONS: &[GensAction] = &[
    GensAction::Join,
    GensAction::Leave,
    GensAction::Reward,
    GensAction::Ranking,
    GensAction::Close,
];
const JOIN_ACTIONS: &[GensAction] = &[GensAction::Join, GensAction::Close];
const REWARD_ACTIONS: &[GensAction] = &[GensAction::Reward, GensAction::Close];
const ERROR_ACTIONS: &[GensAction] = &[GensAction::Retry, GensAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GensRankingScreenState {
    Ready,
    Join,
    Reward,
    Error,
}

impl GensRankingScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Join => "join",
            Self::Reward => "reward",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Ready => Some(READY_NOTICE),
            Self::Join => Some(JOIN_NOTICE),
            Self::Reward => Some(REWARD_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GensAction {
    Join,
    Leave,
    Reward,
    Ranking,
    Close,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GensRankingScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: GensRankingScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub gens_type: Option<&'static str>,
    pub team_name: Option<&'static str>,
    pub ranking: Option<u16>,
    pub contribution: Option<u32>,
    pub next_contribution: Option<u32>,
    pub title_name: Option<&'static str>,
    pub reward_available: bool,
    pub actions: &'static [GensAction],
}

pub fn gens_ranking_screen(state: GensRankingScreenState) -> GensRankingScreen {
    let (
        gens_type,
        team_name,
        ranking,
        contribution,
        next_contribution,
        title_name,
        reward_available,
        actions,
    ) = match state {
        GensRankingScreenState::Ready => (
            Some("Duprian"),
            Some("Night Watch"),
            Some(12),
            Some(4_300),
            Some(5_000),
            Some("Lieutenant"),
            true,
            READY_ACTIONS,
        ),
        GensRankingScreenState::Join => (
            Some("Vanert"),
            Some("Vanert Vanguard"),
            None,
            None,
            None,
            None,
            false,
            JOIN_ACTIONS,
        ),
        GensRankingScreenState::Reward => (
            Some("Duprian"),
            Some("Night Watch"),
            Some(5),
            Some(9_500),
            Some(10_000),
            Some("Viscount"),
            true,
            REWARD_ACTIONS,
        ),
        GensRankingScreenState::Error => (None, None, None, None, None, None, false, ERROR_ACTIONS),
    };

    GensRankingScreen {
        route: UiRoute::Hud,
        layout: UiShellLayout::for_route(UiRoute::Hud),
        widgets: UiShellWidgetSet::for_route(UiRoute::Hud),
        state,
        title: TITLE,
        notice: state.notice(),
        gens_type,
        team_name,
        ranking,
        contribution,
        next_contribution,
        title_name,
        reward_available,
        actions,
    }
}

impl GensRankingScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|panel=gens-ranking|state={}|title={}|notice={:?}|gens_type={:?}|team_name={:?}|ranking={:?}|contribution={:?}|next_contribution={:?}|title_name={:?}|reward_available={}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.gens_type,
            self.team_name,
            self.ranking,
            self.contribution,
            self.next_contribution,
            self.title_name,
            self.reward_available,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{gens_ranking_screen, GensRankingScreenState};

    #[test]
    fn gens_ready_and_join_snapshots() {
        assert_eq!(
            gens_ranking_screen(GensRankingScreenState::Ready).snapshot(),
            "route=hud|group=world|panel=gens-ranking|state=ready|title=Gens Ranking|notice=Some(\"Review Gens ranking and contribution.\")|gens_type=Some(\"Duprian\")|team_name=Some(\"Night Watch\")|ranking=Some(12)|contribution=Some(4300)|next_contribution=Some(5000)|title_name=Some(\"Lieutenant\")|reward_available=true|actions=[Join, Leave, Reward, Ranking, Close]|layout=UiShellLayout { group: World, outer_margin: 16.0, panel_gap: 12.0, sidebar_width: 360.0, content_max_width: 1280.0, footer_height: 0.0 }|widgets=[Body, Sidebar, StatusStrip, Overlay]"
        );
        assert_eq!(
            gens_ranking_screen(GensRankingScreenState::Join).snapshot(),
            "route=hud|group=world|panel=gens-ranking|state=join|title=Gens Ranking|notice=Some(\"Choose a Gens faction.\")|gens_type=Some(\"Vanert\")|team_name=Some(\"Vanert Vanguard\")|ranking=None|contribution=None|next_contribution=None|title_name=None|reward_available=false|actions=[Join, Close]|layout=UiShellLayout { group: World, outer_margin: 16.0, panel_gap: 12.0, sidebar_width: 360.0, content_max_width: 1280.0, footer_height: 0.0 }|widgets=[Body, Sidebar, StatusStrip, Overlay]"
        );
    }

    #[test]
    fn gens_reward_and_error_snapshots() {
        assert_eq!(
            gens_ranking_screen(GensRankingScreenState::Reward).snapshot(),
            "route=hud|group=world|panel=gens-ranking|state=reward|title=Gens Ranking|notice=Some(\"Claim the Gens reward.\")|gens_type=Some(\"Duprian\")|team_name=Some(\"Night Watch\")|ranking=Some(5)|contribution=Some(9500)|next_contribution=Some(10000)|title_name=Some(\"Viscount\")|reward_available=true|actions=[Reward, Close]|layout=UiShellLayout { group: World, outer_margin: 16.0, panel_gap: 12.0, sidebar_width: 360.0, content_max_width: 1280.0, footer_height: 0.0 }|widgets=[Body, Sidebar, StatusStrip, Overlay]"
        );
        assert_eq!(
            gens_ranking_screen(GensRankingScreenState::Error).snapshot(),
            "route=hud|group=world|panel=gens-ranking|state=error|title=Gens Ranking|notice=Some(\"Gens ranking sync failed.\")|gens_type=None|team_name=None|ranking=None|contribution=None|next_contribution=None|title_name=None|reward_available=false|actions=[Retry, Close]|layout=UiShellLayout { group: World, outer_margin: 16.0, panel_gap: 12.0, sidebar_width: 360.0, content_max_width: 1280.0, footer_height: 0.0 }|widgets=[Body, Sidebar, StatusStrip, Overlay]"
        );
    }
}
