use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const NPC_TITLE: &str = "NPC";
const DIALOGUE_NOTICE: &str = "NPC dialogue is open.";
const QUEST_LIST_NOTICE: &str = "Choose a quest or response.";
const CONFIRM_NOTICE: &str = "Confirm the NPC action.";
const ERROR_NOTICE: &str = "NPC dialogue sync failed.";

const DIALOGUE_ACTIONS: &[NpcAction] = &[
    NpcAction::Talk,
    NpcAction::QuestList,
    NpcAction::Confirm,
    NpcAction::Cancel,
    NpcAction::Close,
];

const QUEST_LIST_ACTIONS: &[NpcAction] = &[
    NpcAction::PreviousPage,
    NpcAction::NextPage,
    NpcAction::Confirm,
    NpcAction::Cancel,
    NpcAction::Close,
];

const CONFIRM_ACTIONS: &[NpcAction] = &[NpcAction::Confirm, NpcAction::Cancel, NpcAction::Close];

const ERROR_ACTIONS: &[NpcAction] = &[NpcAction::Retry, NpcAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcScreenState {
    Dialogue,
    QuestList,
    Confirming,
    Error,
}

impl NpcScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dialogue => "dialogue",
            Self::QuestList => "quest-list",
            Self::Confirming => "confirming",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Dialogue => Some(DIALOGUE_NOTICE),
            Self::QuestList => Some(QUEST_LIST_NOTICE),
            Self::Confirming => Some(CONFIRM_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcAction {
    Talk,
    QuestList,
    PreviousPage,
    NextPage,
    Confirm,
    Cancel,
    Close,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NpcScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: NpcScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub npc_name: Option<&'static str>,
    pub dialogue_index: Option<u32>,
    pub quest_list_mode: bool,
    pub page_index: Option<u8>,
    pub max_page: Option<u8>,
    pub selected_answer: Option<u8>,
    pub contribute_point: Option<u32>,
    pub actions: &'static [NpcAction],
}

pub fn npc_screen(state: NpcScreenState) -> NpcScreen {
    let (
        notice,
        npc_name,
        dialogue_index,
        quest_list_mode,
        page_index,
        max_page,
        selected_answer,
        contribute_point,
        actions,
    ) = match state {
        NpcScreenState::Dialogue => (
            state.notice(),
            Some("Marlon"),
            Some(18),
            false,
            Some(0),
            Some(2),
            None,
            Some(1200),
            DIALOGUE_ACTIONS,
        ),
        NpcScreenState::QuestList => (
            state.notice(),
            Some("Marlon"),
            Some(18),
            true,
            Some(1),
            Some(3),
            Some(2),
            Some(1200),
            QUEST_LIST_ACTIONS,
        ),
        NpcScreenState::Confirming => (
            state.notice(),
            Some("Marlon"),
            Some(18),
            false,
            Some(0),
            Some(0),
            Some(1),
            Some(1200),
            CONFIRM_ACTIONS,
        ),
        NpcScreenState::Error => (
            state.notice(),
            None,
            None,
            false,
            None,
            None,
            None,
            None,
            ERROR_ACTIONS,
        ),
    };

    NpcScreen {
        route: UiRoute::Npc,
        layout: UiShellLayout::for_route(UiRoute::Npc),
        widgets: UiShellWidgetSet::for_route(UiRoute::Npc),
        state,
        title: NPC_TITLE,
        notice,
        npc_name,
        dialogue_index,
        quest_list_mode,
        page_index,
        max_page,
        selected_answer,
        contribute_point,
        actions,
    }
}

impl NpcScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|npc_name={:?}|dialogue_index={:?}|quest_list_mode={}|page_index={:?}|max_page={:?}|selected_answer={:?}|contribute_point={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.npc_name,
            self.dialogue_index,
            self.quest_list_mode,
            self.page_index,
            self.max_page,
            self.selected_answer,
            self.contribute_point,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{npc_screen, NpcScreenState};

    #[test]
    fn npc_dialogue_and_quest_list_snapshots() {
        assert_eq!(
            npc_screen(NpcScreenState::Dialogue).snapshot(),
            "route=npc|group=gameplay|state=dialogue|title=NPC|notice=Some(\"NPC dialogue is open.\")|npc_name=Some(\"Marlon\")|dialogue_index=Some(18)|quest_list_mode=false|page_index=Some(0)|max_page=Some(2)|selected_answer=None|contribute_point=Some(1200)|actions=[Talk, QuestList, Confirm, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            npc_screen(NpcScreenState::QuestList).snapshot(),
            "route=npc|group=gameplay|state=quest-list|title=NPC|notice=Some(\"Choose a quest or response.\")|npc_name=Some(\"Marlon\")|dialogue_index=Some(18)|quest_list_mode=true|page_index=Some(1)|max_page=Some(3)|selected_answer=Some(2)|contribute_point=Some(1200)|actions=[PreviousPage, NextPage, Confirm, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn npc_confirming_and_error_snapshots() {
        assert_eq!(
            npc_screen(NpcScreenState::Confirming).snapshot(),
            "route=npc|group=gameplay|state=confirming|title=NPC|notice=Some(\"Confirm the NPC action.\")|npc_name=Some(\"Marlon\")|dialogue_index=Some(18)|quest_list_mode=false|page_index=Some(0)|max_page=Some(0)|selected_answer=Some(1)|contribute_point=Some(1200)|actions=[Confirm, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            npc_screen(NpcScreenState::Error).snapshot(),
            "route=npc|group=gameplay|state=error|title=NPC|notice=Some(\"NPC dialogue sync failed.\")|npc_name=None|dialogue_index=None|quest_list_mode=false|page_index=None|max_page=None|selected_answer=None|contribute_point=None|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
