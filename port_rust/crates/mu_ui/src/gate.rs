use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const GATE_TITLE: &str = "Gate";
const GUEST_PUBLIC_NOTICE: &str = "Public gate access requires the entrance fee.";
const GUEST_PRIVATE_NOTICE: &str = "This gate is private.";
const GUILD_MEMBER_NOTICE: &str = "Guild members may enter.";
const GUILD_MASTER_NOTICE: &str = "Configure gate access and fees.";

const GUEST_PUBLIC_ACTIONS: &[GateAction] = &[GateAction::Enter, GateAction::Close];
const GUEST_PRIVATE_ACTIONS: &[GateAction] = &[GateAction::Enter, GateAction::Close];
const GUILD_MEMBER_ACTIONS: &[GateAction] = &[GateAction::Enter, GateAction::Close];
const GUILD_MASTER_ACTIONS: &[GateAction] = &[
    GateAction::TogglePublic,
    GateAction::FeeUp,
    GateAction::FeeDown,
    GateAction::SetEntranceFee,
    GateAction::Enter,
    GateAction::Close,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateScreenState {
    GuestPublic,
    GuestPrivate,
    GuildMember,
    GuildMaster,
}

impl GateScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GuestPublic => "guest-public",
            Self::GuestPrivate => "guest-private",
            Self::GuildMember => "guild-member",
            Self::GuildMaster => "guild-master",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::GuestPublic => Some(GUEST_PUBLIC_NOTICE),
            Self::GuestPrivate => Some(GUEST_PRIVATE_NOTICE),
            Self::GuildMember => Some(GUILD_MEMBER_NOTICE),
            Self::GuildMaster => Some(GUILD_MASTER_NOTICE),
        }
    }

    pub const fn actions(self) -> &'static [GateAction] {
        match self {
            Self::GuestPublic => GUEST_PUBLIC_ACTIONS,
            Self::GuestPrivate => GUEST_PRIVATE_ACTIONS,
            Self::GuildMember => GUILD_MEMBER_ACTIONS,
            Self::GuildMaster => GUILD_MASTER_ACTIONS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateAction {
    TogglePublic,
    FeeUp,
    FeeDown,
    SetEntranceFee,
    Enter,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GateScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: GateScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub public: bool,
    pub entrance_fee: u32,
    pub view_entrance_fee: u32,
    pub add_entrance_fee: u32,
    pub max_entrance_fee: u32,
    pub player_gold: Option<u32>,
    pub enter_enabled: bool,
    pub actions: &'static [GateAction],
}

pub fn gate_screen(state: GateScreenState) -> GateScreen {
    let (
        public,
        entrance_fee,
        view_entrance_fee,
        add_entrance_fee,
        max_entrance_fee,
        player_gold,
        enter_enabled,
    ) = match state {
        GateScreenState::GuestPublic => (true, 25_000, 25_000, 5_000, 50_000, Some(12_000), false),
        GateScreenState::GuestPrivate => (false, 0, 0, 0, 0, None, false),
        GateScreenState::GuildMember => (true, 0, 0, 0, 0, None, true),
        GateScreenState::GuildMaster => (false, 25_000, 30_000, 5_000, 50_000, None, true),
    };

    GateScreen {
        route: UiRoute::Gate,
        layout: UiShellLayout::for_route(UiRoute::Gate),
        widgets: UiShellWidgetSet::for_route(UiRoute::Gate),
        state,
        title: GATE_TITLE,
        notice: state.notice(),
        public,
        entrance_fee,
        view_entrance_fee,
        add_entrance_fee,
        max_entrance_fee,
        player_gold,
        enter_enabled,
        actions: state.actions(),
    }
}

impl GateScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|public={}|entrance_fee={}|view_entrance_fee={}|add_entrance_fee={}|max_entrance_fee={}|player_gold={:?}|enter_enabled={}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.public,
            self.entrance_fee,
            self.view_entrance_fee,
            self.add_entrance_fee,
            self.max_entrance_fee,
            self.player_gold,
            self.enter_enabled,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{gate_screen, GateScreenState};

    #[test]
    fn guest_gate_snapshots_cover_public_and_private_variants() {
        assert_eq!(
            gate_screen(GateScreenState::GuestPublic).snapshot(),
            "route=gate|group=gameplay|state=guest-public|title=Gate|notice=Some(\"Public gate access requires the entrance fee.\")|public=true|entrance_fee=25000|view_entrance_fee=25000|add_entrance_fee=5000|max_entrance_fee=50000|player_gold=Some(12000)|enter_enabled=false|actions=[Enter, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            gate_screen(GateScreenState::GuestPrivate).snapshot(),
            "route=gate|group=gameplay|state=guest-private|title=Gate|notice=Some(\"This gate is private.\")|public=false|entrance_fee=0|view_entrance_fee=0|add_entrance_fee=0|max_entrance_fee=0|player_gold=None|enter_enabled=false|actions=[Enter, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn guild_gate_snapshots_cover_member_and_master_variants() {
        assert_eq!(
            gate_screen(GateScreenState::GuildMember).snapshot(),
            "route=gate|group=gameplay|state=guild-member|title=Gate|notice=Some(\"Guild members may enter.\")|public=true|entrance_fee=0|view_entrance_fee=0|add_entrance_fee=0|max_entrance_fee=0|player_gold=None|enter_enabled=true|actions=[Enter, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            gate_screen(GateScreenState::GuildMaster).snapshot(),
            "route=gate|group=gameplay|state=guild-master|title=Gate|notice=Some(\"Configure gate access and fees.\")|public=false|entrance_fee=25000|view_entrance_fee=30000|add_entrance_fee=5000|max_entrance_fee=50000|player_gold=None|enter_enabled=true|actions=[TogglePublic, FeeUp, FeeDown, SetEntranceFee, Enter, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
