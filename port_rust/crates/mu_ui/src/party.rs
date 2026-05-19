use bevy::app::{App, Plugin};

use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_gameplay::{PartyManager, PartyMemberInfo};

const PARTY_TITLE: &str = "Party";
const INFO_NOTICE: &str = "View party member status.";
const LIST_NOTICE: &str = "Select a party member.";
const EMPTY_NOTICE: &str = "Invite a player to start a party.";
const ERROR_NOTICE: &str = "Party sync failed.";

const INFO_ACTIONS: &[PartyAction] = &[PartyAction::LeaveParty, PartyAction::Close];
const LIST_ACTIONS: &[PartyAction] = &[
    PartyAction::SelectMember,
    PartyAction::LeaveParty,
    PartyAction::Close,
];
const EMPTY_ACTIONS: &[PartyAction] = &[PartyAction::InviteMember, PartyAction::Close];
const ERROR_ACTIONS: &[PartyAction] = &[PartyAction::Retry, PartyAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyScreenState {
    Info,
    List,
    Empty,
    Error,
}

impl PartyScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::List => "list",
            Self::Empty => "empty",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Info => Some(INFO_NOTICE),
            Self::List => Some(LIST_NOTICE),
            Self::Empty => Some(EMPTY_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }

    pub const fn actions(self) -> &'static [PartyAction] {
        match self {
            Self::Info => INFO_ACTIONS,
            Self::List => LIST_ACTIONS,
            Self::Empty => EMPTY_ACTIONS,
            Self::Error => ERROR_ACTIONS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyAction {
    LeaveParty,
    SelectMember,
    InviteMember,
    Retry,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyMemberPresence {
    Unsearched,
    Hero,
    NotFound,
    Found,
}

impl PartyMemberPresence {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsearched => "unsearched",
            Self::Hero => "hero",
            Self::NotFound => "not-found",
            Self::Found => "found",
        }
    }

    pub const fn from_index(index: i16) -> Self {
        match index {
            -3 => Self::Hero,
            -2 => Self::Unsearched,
            -1 => Self::NotFound,
            _ => Self::Found,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyRowColor {
    Default,
    Red,
    Green,
}

impl PartyRowColor {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Red => "red",
            Self::Green => "green",
        }
    }

    pub const fn from_presence(presence: PartyMemberPresence) -> Self {
        match presence {
            PartyMemberPresence::NotFound => Self::Red,
            PartyMemberPresence::Found => Self::Green,
            PartyMemberPresence::Hero | PartyMemberPresence::Unsearched => Self::Default,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyMemberView {
    pub name: String,
    pub number: u8,
    pub map_id: u8,
    pub position: (u8, u8),
    pub curr_hp: i32,
    pub max_hp: i32,
    pub hp_bar_width: u16,
    pub hp_step: u8,
    pub index: i16,
    pub presence: PartyMemberPresence,
    pub row_color: PartyRowColor,
    pub is_leader: bool,
    pub can_leave: bool,
    pub selected: bool,
}

impl PartyMemberView {
    fn from_member(
        member_num: usize,
        member: &PartyMemberInfo,
        hero_id: Option<&str>,
        leader_name: Option<&str>,
        selected_member_index: Option<usize>,
    ) -> Self {
        let presence = PartyMemberPresence::from_index(member.index);
        let row_color = PartyRowColor::from_presence(presence);
        let is_leader = member_num == 0;
        let selected = selected_member_index == Some(member_num);
        let can_leave =
            hero_id.is_some_and(|hero| hero == leader_name.unwrap_or("") || hero == member.name);

        Self {
            name: member.name.clone(),
            number: member.number,
            map_id: member.map,
            position: (member.x, member.y),
            curr_hp: member.curr_hp,
            max_hp: member.max_hp,
            hp_bar_width: hp_bar_width(member.curr_hp, member.max_hp),
            hp_step: member.step_hp.min(10),
            index: member.index,
            presence,
            row_color,
            is_leader,
            can_leave,
            selected,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartyScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: PartyScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub party_present: bool,
    pub party_number: usize,
    pub hero_id: Option<String>,
    pub leader_name: Option<String>,
    pub selected_member_index: Option<usize>,
    pub selected_character_index: Option<i16>,
    pub members: Vec<PartyMemberView>,
    pub actions: &'static [PartyAction],
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PartyUiPlugin;

impl Plugin for PartyUiPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn party_screen(
    state: PartyScreenState,
    manager: &PartyManager,
    hero_id: Option<&str>,
    selected_member_index: Option<usize>,
) -> PartyScreen {
    let party_number = manager.party_number().min(manager.members().len());
    let leader_name = manager
        .members()
        .first()
        .filter(|member| !member.name.is_empty())
        .map(|member| member.name.clone());
    let members = match state {
        PartyScreenState::Info | PartyScreenState::List => build_members(
            manager,
            hero_id,
            leader_name.as_deref(),
            selected_member_index,
            party_number,
        ),
        PartyScreenState::Empty | PartyScreenState::Error => Vec::new(),
    };
    let leader_name = match state {
        PartyScreenState::Info | PartyScreenState::List => leader_name,
        PartyScreenState::Empty | PartyScreenState::Error => None,
    };
    let selected_character_index = match (state, selected_member_index) {
        (PartyScreenState::List, Some(index)) if index < party_number => {
            Some(manager.member(index).index)
        }
        _ => None,
    };
    let party_present =
        !matches!(state, PartyScreenState::Empty | PartyScreenState::Error) && party_number > 0;
    let hero_id = hero_id.map(str::to_owned);

    PartyScreen {
        route: UiRoute::Party,
        layout: UiShellLayout::for_route(UiRoute::Party),
        widgets: UiShellWidgetSet::for_route(UiRoute::Party),
        state,
        title: PARTY_TITLE,
        notice: state.notice(),
        party_present,
        party_number: if party_present { party_number } else { 0 },
        hero_id,
        leader_name,
        selected_member_index,
        selected_character_index,
        members,
        actions: state.actions(),
    }
}

impl PartyScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|party_present={}|party_number={}|hero_id={:?}|leader_name={:?}|selected_member_index={:?}|selected_character_index={:?}|members={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.party_present,
            self.party_number,
            self.hero_id,
            self.leader_name,
            self.selected_member_index,
            self.selected_character_index,
            self.members,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

fn build_members(
    manager: &PartyManager,
    hero_id: Option<&str>,
    leader_name: Option<&str>,
    selected_member_index: Option<usize>,
    party_number: usize,
) -> Vec<PartyMemberView> {
    manager
        .members()
        .iter()
        .take(party_number)
        .enumerate()
        .map(|(member_num, member)| {
            PartyMemberView::from_member(
                member_num,
                member,
                hero_id,
                leader_name,
                selected_member_index,
            )
        })
        .collect()
}

fn hp_bar_width(curr_hp: i32, max_hp: i32) -> u16 {
    if curr_hp <= 0 || max_hp <= 0 {
        return 0;
    }

    let curr_hp = curr_hp.min(max_hp) as u32;
    let max_hp = max_hp as u32;

    ((curr_hp * 147) / max_hp) as u16
}

#[cfg(test)]
mod tests {
    use super::{party_screen, PartyScreenState};
    use mu_gameplay::{PartyManager, PartyMemberInfo};

    fn manager() -> PartyManager {
        let mut manager = PartyManager::new();

        manager.set_member(
            0,
            PartyMemberInfo {
                name: "Astra".into(),
                number: 11,
                map: 3,
                x: 12,
                y: 21,
                curr_hp: 480,
                max_hp: 600,
                step_hp: 10,
                index: 4,
            },
        );
        manager.set_member(
            1,
            PartyMemberInfo {
                name: "Blade".into(),
                number: 22,
                map: 6,
                x: 44,
                y: 15,
                curr_hp: 220,
                max_hp: 450,
                step_hp: 7,
                index: -1,
            },
        );
        manager.set_member(
            2,
            PartyMemberInfo {
                name: "Selene".into(),
                number: 33,
                map: 9,
                x: 8,
                y: 3,
                curr_hp: 100,
                max_hp: 380,
                step_hp: 3,
                index: -3,
            },
        );

        manager
    }

    #[test]
    fn info_snapshot_projects_party_status() {
        assert_eq!(
            party_screen(
                PartyScreenState::Info,
                &manager(),
                Some("Astra"),
                None,
            )
            .snapshot(),
            "route=party|group=gameplay|state=info|title=Party|notice=Some(\"View party member status.\")|party_present=true|party_number=3|hero_id=Some(\"Astra\")|leader_name=Some(\"Astra\")|selected_member_index=None|selected_character_index=None|members=[PartyMemberView { name: \"Astra\", number: 11, map_id: 3, position: (12, 21), curr_hp: 480, max_hp: 600, hp_bar_width: 117, hp_step: 10, index: 4, presence: Found, row_color: Green, is_leader: true, can_leave: true, selected: false }, PartyMemberView { name: \"Blade\", number: 22, map_id: 6, position: (44, 15), curr_hp: 220, max_hp: 450, hp_bar_width: 71, hp_step: 7, index: -1, presence: NotFound, row_color: Red, is_leader: false, can_leave: true, selected: false }, PartyMemberView { name: \"Selene\", number: 33, map_id: 9, position: (8, 3), curr_hp: 100, max_hp: 380, hp_bar_width: 38, hp_step: 3, index: -3, presence: Hero, row_color: Default, is_leader: false, can_leave: true, selected: false }]|actions=[LeaveParty, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn list_snapshot_projects_selection_and_row_colors() {
        assert_eq!(
            party_screen(
                PartyScreenState::List,
                &manager(),
                Some("Selene"),
                Some(2),
            )
            .snapshot(),
            "route=party|group=gameplay|state=list|title=Party|notice=Some(\"Select a party member.\")|party_present=true|party_number=3|hero_id=Some(\"Selene\")|leader_name=Some(\"Astra\")|selected_member_index=Some(2)|selected_character_index=Some(-3)|members=[PartyMemberView { name: \"Astra\", number: 11, map_id: 3, position: (12, 21), curr_hp: 480, max_hp: 600, hp_bar_width: 117, hp_step: 10, index: 4, presence: Found, row_color: Green, is_leader: true, can_leave: false, selected: false }, PartyMemberView { name: \"Blade\", number: 22, map_id: 6, position: (44, 15), curr_hp: 220, max_hp: 450, hp_bar_width: 71, hp_step: 7, index: -1, presence: NotFound, row_color: Red, is_leader: false, can_leave: false, selected: false }, PartyMemberView { name: \"Selene\", number: 33, map_id: 9, position: (8, 3), curr_hp: 100, max_hp: 380, hp_bar_width: 38, hp_step: 3, index: -3, presence: Hero, row_color: Default, is_leader: false, can_leave: true, selected: true }]|actions=[SelectMember, LeaveParty, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn empty_and_error_snapshots_stay_distinct() {
        assert_eq!(
            party_screen(PartyScreenState::Empty, &manager(), None, None).snapshot(),
            "route=party|group=gameplay|state=empty|title=Party|notice=Some(\"Invite a player to start a party.\")|party_present=false|party_number=0|hero_id=None|leader_name=None|selected_member_index=None|selected_character_index=None|members=[]|actions=[InviteMember, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            party_screen(PartyScreenState::Error, &manager(), None, None).snapshot(),
            "route=party|group=gameplay|state=error|title=Party|notice=Some(\"Party sync failed.\")|party_present=false|party_number=0|hero_id=None|leader_name=None|selected_member_index=None|selected_character_index=None|members=[]|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
