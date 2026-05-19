use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const QUESTS_TITLE: &str = "Quests";
const JOURNAL_NOTICE: &str = "Review active quests and NPC progress.";
const DIALOGUE_NOTICE: &str = "Quest dialogue is open.";
const REWARD_NOTICE: &str = "Complete the selected quest.";
const ETC_NOTICE: &str = "Quest entries from other sources are listed here.";
const ERROR_NOTICE: &str = "Quest sync failed.";

const JOURNAL_ACTIONS: &[QuestAction] =
    &[QuestAction::Open, QuestAction::GiveUp, QuestAction::Close];
const DIALOGUE_ACTIONS: &[QuestAction] = &[
    QuestAction::PreviousPage,
    QuestAction::NextPage,
    QuestAction::Confirm,
    QuestAction::Cancel,
    QuestAction::Close,
];
const REWARD_ACTIONS: &[QuestAction] = &[
    QuestAction::RequestReward,
    QuestAction::Cancel,
    QuestAction::Close,
];
const ETC_ACTIONS: &[QuestAction] = &[QuestAction::SelectEtc, QuestAction::Close];
const ERROR_ACTIONS: &[QuestAction] = &[QuestAction::Retry, QuestAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestScreenState {
    Journal,
    Dialogue,
    Reward,
    Etc,
    Error,
}

impl QuestScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Journal => "journal",
            Self::Dialogue => "dialogue",
            Self::Reward => "reward",
            Self::Etc => "etc",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Journal => Some(JOURNAL_NOTICE),
            Self::Dialogue => Some(DIALOGUE_NOTICE),
            Self::Reward => Some(REWARD_NOTICE),
            Self::Etc => Some(ETC_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestAction {
    Open,
    GiveUp,
    PreviousPage,
    NextPage,
    Confirm,
    Cancel,
    Close,
    Retry,
    RequestReward,
    SelectEtc,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuestScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: QuestScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub npc_name: Option<&'static str>,
    pub npc_index: Option<i32>,
    pub selected_quest_index: Option<u32>,
    pub current_quests: &'static [u32],
    pub etc_quests: &'static [u32],
    pub selected_answer: Option<u8>,
    pub dialogue_page: Option<u8>,
    pub max_page: Option<u8>,
    pub request_reward_quest: Option<u32>,
    pub request_reward_visible: bool,
    pub actions: &'static [QuestAction],
}

pub fn quests_screen(state: QuestScreenState) -> QuestScreen {
    let (
        notice,
        npc_name,
        npc_index,
        selected_quest_index,
        current_quests,
        etc_quests,
        selected_answer,
        dialogue_page,
        max_page,
        request_reward_quest,
        request_reward_visible,
        actions,
    ) = match state {
        QuestScreenState::Journal => (
            state.notice(),
            Some("Elpis"),
            Some(18),
            Some(1001),
            &[1001, 1002, 1008][..],
            &[2001, 2002][..],
            None,
            Some(0),
            Some(4),
            None,
            false,
            JOURNAL_ACTIONS,
        ),
        QuestScreenState::Dialogue => (
            state.notice(),
            Some("Elpis"),
            Some(18),
            Some(1002),
            &[1001, 1002, 1008][..],
            &[2001, 2002][..],
            Some(2),
            Some(1),
            Some(4),
            None,
            false,
            DIALOGUE_ACTIONS,
        ),
        QuestScreenState::Reward => (
            state.notice(),
            Some("Elpis"),
            Some(18),
            Some(1008),
            &[1001, 1002, 1008][..],
            &[2001, 2002][..],
            Some(1),
            Some(3),
            Some(4),
            Some(1008),
            true,
            REWARD_ACTIONS,
        ),
        QuestScreenState::Etc => (
            state.notice(),
            None,
            None,
            None,
            &[][..],
            &[3001, 3002, 3003][..],
            None,
            None,
            None,
            None,
            false,
            ETC_ACTIONS,
        ),
        QuestScreenState::Error => (
            state.notice(),
            None,
            None,
            None,
            &[][..],
            &[][..],
            None,
            None,
            None,
            None,
            false,
            ERROR_ACTIONS,
        ),
    };

    QuestScreen {
        route: UiRoute::Quests,
        layout: UiShellLayout::for_route(UiRoute::Quests),
        widgets: UiShellWidgetSet::for_route(UiRoute::Quests),
        state,
        title: QUESTS_TITLE,
        notice,
        npc_name,
        npc_index,
        selected_quest_index,
        current_quests,
        etc_quests,
        selected_answer,
        dialogue_page,
        max_page,
        request_reward_quest,
        request_reward_visible,
        actions,
    }
}

impl QuestScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|npc_name={:?}|npc_index={:?}|selected_quest_index={:?}|current_quests={:?}|etc_quests={:?}|selected_answer={:?}|dialogue_page={:?}|max_page={:?}|request_reward_quest={:?}|request_reward_visible={}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.npc_name,
            self.npc_index,
            self.selected_quest_index,
            self.current_quests,
            self.etc_quests,
            self.selected_answer,
            self.dialogue_page,
            self.max_page,
            self.request_reward_quest,
            self.request_reward_visible,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{quests_screen, QuestScreenState};

    #[test]
    fn quest_journal_and_dialogue_snapshots() {
        assert_eq!(
            quests_screen(QuestScreenState::Journal).snapshot(),
            "route=quests|group=gameplay|state=journal|title=Quests|notice=Some(\"Review active quests and NPC progress.\")|npc_name=Some(\"Elpis\")|npc_index=Some(18)|selected_quest_index=Some(1001)|current_quests=[1001, 1002, 1008]|etc_quests=[2001, 2002]|selected_answer=None|dialogue_page=Some(0)|max_page=Some(4)|request_reward_quest=None|request_reward_visible=false|actions=[Open, GiveUp, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            quests_screen(QuestScreenState::Dialogue).snapshot(),
            "route=quests|group=gameplay|state=dialogue|title=Quests|notice=Some(\"Quest dialogue is open.\")|npc_name=Some(\"Elpis\")|npc_index=Some(18)|selected_quest_index=Some(1002)|current_quests=[1001, 1002, 1008]|etc_quests=[2001, 2002]|selected_answer=Some(2)|dialogue_page=Some(1)|max_page=Some(4)|request_reward_quest=None|request_reward_visible=false|actions=[PreviousPage, NextPage, Confirm, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn quest_reward_etc_and_error_snapshots() {
        assert_eq!(
            quests_screen(QuestScreenState::Reward).snapshot(),
            "route=quests|group=gameplay|state=reward|title=Quests|notice=Some(\"Complete the selected quest.\")|npc_name=Some(\"Elpis\")|npc_index=Some(18)|selected_quest_index=Some(1008)|current_quests=[1001, 1002, 1008]|etc_quests=[2001, 2002]|selected_answer=Some(1)|dialogue_page=Some(3)|max_page=Some(4)|request_reward_quest=Some(1008)|request_reward_visible=true|actions=[RequestReward, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            quests_screen(QuestScreenState::Etc).snapshot(),
            "route=quests|group=gameplay|state=etc|title=Quests|notice=Some(\"Quest entries from other sources are listed here.\")|npc_name=None|npc_index=None|selected_quest_index=None|current_quests=[]|etc_quests=[3001, 3002, 3003]|selected_answer=None|dialogue_page=None|max_page=None|request_reward_quest=None|request_reward_visible=false|actions=[SelectEtc, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            quests_screen(QuestScreenState::Error).snapshot(),
            "route=quests|group=gameplay|state=error|title=Quests|notice=Some(\"Quest sync failed.\")|npc_name=None|npc_index=None|selected_quest_index=None|current_quests=[]|etc_quests=[]|selected_answer=None|dialogue_page=None|max_page=None|request_reward_quest=None|request_reward_visible=false|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
