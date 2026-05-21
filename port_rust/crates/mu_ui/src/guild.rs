use bevy::app::{App, Plugin};

use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const GUILD_TITLE: &str = "Guild";
const SUMMARY_NOTICE: &str = "Review guild status, notices, and rival info.";
const MEMBERS_NOTICE: &str = "Manage guild members.";
const UNION_NOTICE: &str = "Review allied guilds.";
const NO_GUILD_NOTICE: &str = "Join or create a guild to continue.";
const ERROR_NOTICE: &str = "Guild sync failed.";

const SUMMARY_ACTIONS: &[GuildAction] = &[GuildAction::BreakUpGuild, GuildAction::Close];
const MEMBERS_ACTIONS: &[GuildAction] = &[
    GuildAction::Appoint,
    GuildAction::Disband,
    GuildAction::Fire,
    GuildAction::Close,
];
const UNION_ACTIONS: &[GuildAction] = &[
    GuildAction::BreakUnion,
    GuildAction::BanUnion,
    GuildAction::Close,
];
const NO_GUILD_ACTIONS: &[GuildAction] = &[GuildAction::Close];
const ERROR_ACTIONS: &[GuildAction] = &[GuildAction::Retry, GuildAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildScreenState {
    Summary,
    Members,
    Union,
    NoGuild,
    Error,
}

impl GuildScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::Members => "members",
            Self::Union => "union",
            Self::NoGuild => "no-guild",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Summary => Some(SUMMARY_NOTICE),
            Self::Members => Some(MEMBERS_NOTICE),
            Self::Union => Some(UNION_NOTICE),
            Self::NoGuild => Some(NO_GUILD_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildAction {
    BreakUpGuild,
    Appoint,
    Disband,
    Fire,
    BreakUnion,
    BanUnion,
    Close,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildMemberRole {
    Master,
    SubMaster,
    BattleMaster,
    Member,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildSort {
    Name,
    Role,
    Server,
    MemberCount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildMemberEntry {
    pub name: String,
    pub number: u8,
    pub server: Option<u8>,
    pub role: GuildMemberRole,
    pub selected: bool,
    pub is_self: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildUnionEntry {
    pub name: String,
    pub member_count: u16,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuildScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: GuildScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub tab_index: usize,
    pub player_role: Option<GuildMemberRole>,
    pub guild_type: Option<&'static str>,
    pub guild_name: Option<String>,
    pub guild_score: Option<u32>,
    pub member_count: Option<usize>,
    pub rival_guild_name: Option<String>,
    pub member_sort: Option<GuildSort>,
    pub union_sort: Option<GuildSort>,
    pub guild_master_name: Option<String>,
    pub sub_master_name: Option<String>,
    pub battle_master_name: Option<String>,
    pub break_up_caption: Option<&'static str>,
    pub break_union_caption: Option<&'static str>,
    pub ban_union_enabled: bool,
    pub member_actions_visible: bool,
    pub union_actions_visible: bool,
    pub request_union_list: bool,
    pub selected_member_name: Option<String>,
    pub selected_member_role: Option<GuildMemberRole>,
    pub selected_member_server: Option<u8>,
    pub selected_union_name: Option<String>,
    pub selected_union_member_count: Option<u16>,
    pub notices: Vec<String>,
    pub members: Vec<GuildMemberEntry>,
    pub unions: Vec<GuildUnionEntry>,
    pub actions: &'static [GuildAction],
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GuildUiPlugin;

impl Plugin for GuildUiPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn guild_screen(state: GuildScreenState) -> GuildScreen {
    let (
        tab_index,
        player_role,
        guild_type,
        guild_name,
        guild_score,
        member_count,
        rival_guild_name,
        member_sort,
        union_sort,
        guild_master_name,
        sub_master_name,
        battle_master_name,
        break_up_caption,
        break_union_caption,
        ban_union_enabled,
        member_actions_visible,
        union_actions_visible,
        request_union_list,
        selected_member_name,
        selected_member_role,
        selected_member_server,
        selected_union_name,
        selected_union_member_count,
        notices,
        members,
        unions,
        actions,
    ) = match state {
        GuildScreenState::Summary => (
            0,
            Some(GuildMemberRole::Master),
            Some("normal"),
            Some("Night Watch".to_owned()),
            Some(12_500),
            Some(4),
            Some("Red Dragons".to_owned()),
            None,
            None,
            Some("Astra".to_owned()),
            Some("Blade".to_owned()),
            Some("Selene".to_owned()),
            Some("Disband Guild"),
            None,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
            None,
            vec![
                "Guild meeting at 20:00.".to_owned(),
                "Alliance request pending.".to_owned(),
            ],
            Vec::new(),
            Vec::new(),
            SUMMARY_ACTIONS,
        ),
        GuildScreenState::Members => {
            let members = build_guild_members();
            (
                1,
                Some(GuildMemberRole::Master),
                Some("normal"),
                Some("Night Watch".to_owned()),
                Some(12_500),
                Some(members.len()),
                Some("Red Dragons".to_owned()),
                Some(GuildSort::Role),
                None,
                None,
                None,
                None,
                Some("Disband Guild"),
                None,
                false,
                true,
                false,
                false,
                Some("Blade".to_owned()),
                Some(GuildMemberRole::SubMaster),
                Some(3),
                None,
                None,
                Vec::new(),
                members,
                Vec::new(),
                MEMBERS_ACTIONS,
            )
        }
        GuildScreenState::Union => {
            let unions = build_guild_unions();
            (
                2,
                Some(GuildMemberRole::Master),
                Some("normal"),
                Some("Night Watch".to_owned()),
                Some(12_500),
                Some(4),
                Some("Red Dragons".to_owned()),
                None,
                Some(GuildSort::MemberCount),
                None,
                None,
                None,
                Some("Disband Guild"),
                Some("Break Union"),
                true,
                false,
                true,
                true,
                None,
                None,
                None,
                Some("Vanert Vanguard".to_owned()),
                Some(14),
                Vec::new(),
                Vec::new(),
                unions,
                UNION_ACTIONS,
            )
        }
        GuildScreenState::NoGuild => (
            0,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            NO_GUILD_ACTIONS,
        ),
        GuildScreenState::Error => (
            0,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            ERROR_ACTIONS,
        ),
    };

    GuildScreen {
        route: UiRoute::Guild,
        layout: UiShellLayout::for_route(UiRoute::Guild),
        widgets: UiShellWidgetSet::for_route(UiRoute::Guild),
        state,
        title: GUILD_TITLE,
        notice: state.notice(),
        tab_index,
        player_role,
        guild_type,
        guild_name,
        guild_score,
        member_count,
        rival_guild_name,
        member_sort,
        union_sort,
        guild_master_name,
        sub_master_name,
        battle_master_name,
        break_up_caption,
        break_union_caption,
        ban_union_enabled,
        member_actions_visible,
        union_actions_visible,
        request_union_list,
        selected_member_name,
        selected_member_role,
        selected_member_server,
        selected_union_name,
        selected_union_member_count,
        notices,
        members,
        unions,
        actions,
    }
}

impl GuildScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|tab_index={}|player_role={:?}|guild_type={:?}|guild_name={:?}|guild_score={:?}|member_count={:?}|rival_guild_name={:?}|member_sort={:?}|union_sort={:?}|guild_master_name={:?}|sub_master_name={:?}|battle_master_name={:?}|break_up_caption={:?}|break_union_caption={:?}|ban_union_enabled={}|member_actions_visible={}|union_actions_visible={}|request_union_list={}|selected_member_name={:?}|selected_member_role={:?}|selected_member_server={:?}|selected_union_name={:?}|selected_union_member_count={:?}|notices={:?}|members={:?}|unions={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.tab_index,
            self.player_role,
            self.guild_type,
            self.guild_name,
            self.guild_score,
            self.member_count,
            self.rival_guild_name,
            self.member_sort,
            self.union_sort,
            self.guild_master_name,
            self.sub_master_name,
            self.battle_master_name,
            self.break_up_caption,
            self.break_union_caption,
            self.ban_union_enabled,
            self.member_actions_visible,
            self.union_actions_visible,
            self.request_union_list,
            self.selected_member_name,
            self.selected_member_role,
            self.selected_member_server,
            self.selected_union_name,
            self.selected_union_member_count,
            self.notices,
            self.members,
            self.unions,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

fn build_guild_members() -> Vec<GuildMemberEntry> {
    vec![
        GuildMemberEntry {
            name: "Astra".to_owned(),
            number: 11,
            server: Some(3),
            role: GuildMemberRole::Master,
            selected: false,
            is_self: true,
        },
        GuildMemberEntry {
            name: "Blade".to_owned(),
            number: 22,
            server: Some(3),
            role: GuildMemberRole::SubMaster,
            selected: true,
            is_self: false,
        },
        GuildMemberEntry {
            name: "Selene".to_owned(),
            number: 33,
            server: Some(5),
            role: GuildMemberRole::BattleMaster,
            selected: false,
            is_self: false,
        },
        GuildMemberEntry {
            name: "Marlon".to_owned(),
            number: 44,
            server: None,
            role: GuildMemberRole::Member,
            selected: false,
            is_self: false,
        },
    ]
}

fn build_guild_unions() -> Vec<GuildUnionEntry> {
    vec![
        GuildUnionEntry {
            name: "Vanert Vanguard".to_owned(),
            member_count: 14,
            selected: true,
        },
        GuildUnionEntry {
            name: "Night Watch".to_owned(),
            member_count: 18,
            selected: false,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{guild_screen, GuildScreenState};

    #[test]
    fn guild_summary_and_members_snapshots() {
        assert_eq!(
            guild_screen(GuildScreenState::Summary).snapshot(),
            "route=guild|group=gameplay|state=summary|title=Guild|notice=Some(\"Review guild status, notices, and rival info.\")|tab_index=0|player_role=Some(Master)|guild_type=Some(\"normal\")|guild_name=Some(\"Night Watch\")|guild_score=Some(12500)|member_count=Some(4)|rival_guild_name=Some(\"Red Dragons\")|member_sort=None|union_sort=None|guild_master_name=Some(\"Astra\")|sub_master_name=Some(\"Blade\")|battle_master_name=Some(\"Selene\")|break_up_caption=Some(\"Disband Guild\")|break_union_caption=None|ban_union_enabled=false|member_actions_visible=false|union_actions_visible=false|request_union_list=false|selected_member_name=None|selected_member_role=None|selected_member_server=None|selected_union_name=None|selected_union_member_count=None|notices=[\"Guild meeting at 20:00.\", \"Alliance request pending.\"]|members=[]|unions=[]|actions=[BreakUpGuild, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            guild_screen(GuildScreenState::Members).snapshot(),
            "route=guild|group=gameplay|state=members|title=Guild|notice=Some(\"Manage guild members.\")|tab_index=1|player_role=Some(Master)|guild_type=Some(\"normal\")|guild_name=Some(\"Night Watch\")|guild_score=Some(12500)|member_count=Some(4)|rival_guild_name=Some(\"Red Dragons\")|member_sort=Some(Role)|union_sort=None|guild_master_name=None|sub_master_name=None|battle_master_name=None|break_up_caption=Some(\"Disband Guild\")|break_union_caption=None|ban_union_enabled=false|member_actions_visible=true|union_actions_visible=false|request_union_list=false|selected_member_name=Some(\"Blade\")|selected_member_role=Some(SubMaster)|selected_member_server=Some(3)|selected_union_name=None|selected_union_member_count=None|notices=[]|members=[GuildMemberEntry { name: \"Astra\", number: 11, server: Some(3), role: Master, selected: false, is_self: true }, GuildMemberEntry { name: \"Blade\", number: 22, server: Some(3), role: SubMaster, selected: true, is_self: false }, GuildMemberEntry { name: \"Selene\", number: 33, server: Some(5), role: BattleMaster, selected: false, is_self: false }, GuildMemberEntry { name: \"Marlon\", number: 44, server: None, role: Member, selected: false, is_self: false }]|unions=[]|actions=[Appoint, Disband, Fire, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn guild_union_no_guild_and_error_snapshots() {
        assert_eq!(
            guild_screen(GuildScreenState::Union).snapshot(),
            "route=guild|group=gameplay|state=union|title=Guild|notice=Some(\"Review allied guilds.\")|tab_index=2|player_role=Some(Master)|guild_type=Some(\"normal\")|guild_name=Some(\"Night Watch\")|guild_score=Some(12500)|member_count=Some(4)|rival_guild_name=Some(\"Red Dragons\")|member_sort=None|union_sort=Some(MemberCount)|guild_master_name=None|sub_master_name=None|battle_master_name=None|break_up_caption=Some(\"Disband Guild\")|break_union_caption=Some(\"Break Union\")|ban_union_enabled=true|member_actions_visible=false|union_actions_visible=true|request_union_list=true|selected_member_name=None|selected_member_role=None|selected_member_server=None|selected_union_name=Some(\"Vanert Vanguard\")|selected_union_member_count=Some(14)|notices=[]|members=[]|unions=[GuildUnionEntry { name: \"Vanert Vanguard\", member_count: 14, selected: true }, GuildUnionEntry { name: \"Night Watch\", member_count: 18, selected: false }]|actions=[BreakUnion, BanUnion, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            guild_screen(GuildScreenState::NoGuild).snapshot(),
            "route=guild|group=gameplay|state=no-guild|title=Guild|notice=Some(\"Join or create a guild to continue.\")|tab_index=0|player_role=None|guild_type=None|guild_name=None|guild_score=None|member_count=None|rival_guild_name=None|member_sort=None|union_sort=None|guild_master_name=None|sub_master_name=None|battle_master_name=None|break_up_caption=None|break_union_caption=None|ban_union_enabled=false|member_actions_visible=false|union_actions_visible=false|request_union_list=false|selected_member_name=None|selected_member_role=None|selected_member_server=None|selected_union_name=None|selected_union_member_count=None|notices=[]|members=[]|unions=[]|actions=[Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            guild_screen(GuildScreenState::Error).snapshot(),
            "route=guild|group=gameplay|state=error|title=Guild|notice=Some(\"Guild sync failed.\")|tab_index=0|player_role=None|guild_type=None|guild_name=None|guild_score=None|member_count=None|rival_guild_name=None|member_sort=None|union_sort=None|guild_master_name=None|sub_master_name=None|battle_master_name=None|break_up_caption=None|break_union_caption=None|ban_union_enabled=false|member_actions_visible=false|union_actions_visible=false|request_union_list=false|selected_member_name=None|selected_member_role=None|selected_member_server=None|selected_union_name=None|selected_union_member_count=None|notices=[]|members=[]|unions=[]|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
